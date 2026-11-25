#!/usr/bin/env python3
"""Extract every requirement from the AUTOSAR CP "Specification of Operating
System" (SWS OS) PDF and build a searchable, offline requirement browser.

Usage:

    python3 -m venv tools/.venv && tools/.venv/bin/pip install pypdf
    tools/.venv/bin/python tools/sws_extract.py spec/AUTOSAR_CP_SWS_OS.pdf

Outputs (both git-ignored, see the note below):

    spec/sws-os-requirements.json   machine-readable requirement list
    spec/sws-os-requirements.html   self-contained viewer, open it in a browser

The PDF is a free download from https://www.autosar.org (Classic Platform,
document ID 34). AUTOSAR permits reproduction "for informational purposes
only", so the generated files contain specification text and must stay local:
do not commit or publish them. This script contains no specification text.

The script also scans the repository for requirement IDs mentioned in Rust
sources, so the viewer shows which requirements the code already refers to.
"""

from __future__ import annotations

import argparse
import datetime
import json
import re
import statistics
import sys
from collections import Counter
from pathlib import Path

try:
    from pypdf import PdfReader
except ImportError:  # pragma: no cover
    sys.exit("pypdf is missing: tools/.venv/bin/pip install pypdf")

REPO = Path(__file__).resolve().parent.parent
TEMPLATE = Path(__file__).with_name("sws_viewer.template.html")

REQ_START = re.compile(r"^\[(SWS_Os(?:_[A-Z]+)?_\d{5})\]\s*(.*)$")
REQ_ID = re.compile(r"SWS_Os(?:_[A-Z]+)?_\d{5}")
UPSTREAM_ID = re.compile(r"\b(?:SRS|RS)_[A-Za-z]+_\d{5}\b")
HEADING_NUM = re.compile(r"^((?:\d+|[A-Z])(?:\.\d+)*)\s+(.*)$")
PAGE_NOISE = [
    re.compile(r"^AUTOSAR CP R\d\d-\d\d$"),
    re.compile(r"^\d+ of \d+ Document ID \d+: \S+$"),
    re.compile(r"^[▽△]$"),
]
LIST_ITEM = re.compile(r"^(?:•|[–-]\s|\d+\.\s+\S|\(\d+\)\s|[a-z]\)\s|\d+ [A-Za-z_]+\()")

# Roadmap stage per section (longest matching section-number prefix wins).
# This mapping is Rautosa's own reading of the document structure, not part of
# the specification. See docs/roadmap.md.
STAGE_1 = "Stage 1 · SC1 / OSEK core"
STAGE_2 = "Stage 2 · Schedule tables"
STAGE_3 = "Stage 3 · SC3 protection"
STAGE_4 = "Stage 4 · Multi-core & IOC"
SC24 = "SC2/SC4 · timing & sync"
OPTIONAL = "Optional features"
GENERAL = "General"
ARTI = "ARTI tracing"
CONFIG = "Configuration"
NOT_APPLICABLE = "Not applicable"
STAGE_ORDER = [
    STAGE_1, STAGE_2, STAGE_3, STAGE_4, SC24, OPTIONAL, GENERAL, ARTI, CONFIG,
    NOT_APPLICABLE,
]

SECTION_STAGE = {
    "7.1": STAGE_1, "7.2": STAGE_1, "7.3": STAGE_2, "7.4": SC24,
    "7.5": STAGE_1, "7.6": STAGE_3, "7.7.1": STAGE_3, "7.7.2": SC24,
    "7.7.3": STAGE_3, "7.7.4": STAGE_3, "7.7.5": STAGE_3, "7.8": STAGE_3,
    "7.9": STAGE_4, "7.10": STAGE_4, "7.11": GENERAL, "7.12": STAGE_3,
    "7.13": STAGE_3, "7.14": OPTIONAL, "7.15": GENERAL, "7.16": ARTI,
    "7.17": ARTI, "8.1": GENERAL, "8.2": GENERAL, "8.5": STAGE_4,
    "8.6": STAGE_4, "8.7": STAGE_3, "8.8": GENERAL, "9": GENERAL,
    "10": CONFIG, "11": CONFIG, "A": NOT_APPLICABLE,
}
# Sections 8.3 (types) and 8.4 (functions) are mapped by the name in the title.
NAME_STAGE = [
    (SC24, r"Synchron|SyncScheduleTable|SetScheduleTableAsync|BudgetReplenish"),
    (STAGE_2, r"ScheduleTable"),
    (STAGE_1, r"Counter|GetElapsedValue|isOsStarted|PhysicalTime"),
    (STAGE_4, r"Core|Spinlock|Asyn\b|Ioc"),
    (OPTIONAL, r"InterruptSource|ClearPendingInterrupt"),
    (STAGE_3, r"."),
]
ID_STAGE = {"SWS_Os_00439": STAGE_1, "SWS_Os_00367": STAGE_1}


def read_pages(reader: PdfReader) -> list[list[str]]:
    title = " ".join((reader.pages[0].extract_text() or "").split("\n")[:1]).strip()
    pages = []
    for page in reader.pages:
        lines = []
        for raw in (page.extract_text() or "").split("\n"):
            line = raw.rstrip()
            stripped = line.strip()
            if not stripped or stripped == title:
                continue
            if any(p.match(stripped) for p in PAGE_NOISE):
                continue
            lines.append(line)
        pages.append(lines)
    return pages


def read_outline(reader: PdfReader) -> list[dict]:
    headings = []

    def walk(items):
        for item in items:
            if isinstance(item, list):
                walk(item)
                continue
            text = " ".join(item.title.split())
            match = HEADING_NUM.match(text)
            if not match:
                continue
            headings.append({
                "number": match.group(1),
                "title": match.group(2),
                "page": reader.get_destination_page_number(item) + 1,
            })

    walk(reader.outline)
    return headings


def build_vocabulary(pages: list[list[str]]) -> Counter:
    vocab: Counter = Counter()
    for lines in pages:
        for line in lines:
            vocab.update(w.lower() for w in re.findall(r"[A-Za-z][A-Za-z_-]*[A-Za-z]", line))
    return vocab


def join_lines(lines: list[str], vocab: Counter) -> str:
    """Re-flow hard-wrapped PDF lines into paragraphs, undoing hyphenation."""
    out = ""
    for line in (l.strip() for l in lines):
        if not line:
            continue
        if not out:
            out = line
        elif LIST_ITEM.match(line):
            out += "\n" + line
        elif out.endswith("-") and re.search(r"[A-Za-z]-$", out) and re.match(r"[A-Za-z]", line):
            left = re.search(r"([A-Za-z_]+)-$", out).group(1)
            right = re.match(r"[A-Za-z_]+", line).group(0)
            keep_hyphen = vocab[f"{left}-{right}".lower()] > vocab[f"{left}{right}".lower()]
            out = out if keep_hyphen else out[:-1]
            out += line
        else:
            out += " " + line
    return re.sub(r"[ \t]+", " ", out).strip()


def looks_tabular(title: str, body_lines: list[str]) -> bool:
    if title.startswith("Definition of"):
        return True
    lengths = [len(l.strip()) for l in body_lines if l.strip()]
    return len(lengths) >= 8 and statistics.median(lengths) < 45


def stage_for(req_id: str, section_number: str, section_title: str, title: str) -> str:
    if req_id in ID_STAGE:
        return ID_STAGE[req_id]
    if section_number.startswith(("8.3", "8.4")):
        name = f"{section_title} {title}"
        return next(stage for stage, pattern in NAME_STAGE if re.search(pattern, name))
    parts = section_number.split(".")
    while parts:
        stage = SECTION_STAGE.get(".".join(parts))
        if stage:
            return stage
        parts.pop()
    return GENERAL


def kind_for(req_id: str, title: str) -> str:
    if "_CONSTR_" in req_id:
        return "Constraint"
    if "_NA_" in req_id:
        return "Not applicable"
    match = re.match(r"Definition of (API function|datatype|configurable interface|"
                     r"imported datatypes|optional interfaces|development errors|"
                     r"mandatory interfaces|callout function|ImplementationDataType|"
                     r"[A-Za-z ]+?) ", title + " ")
    if match:
        what = match.group(1)
        if "function" in what or "interface" in what.lower() and "interfaces" not in what:
            return "API"
        if "datatype" in what.lower() or "DataType" in what:
            return "Type"
        return "Definition"
    return "Requirement"


def extract(pdf: Path) -> dict:
    reader = PdfReader(str(pdf))
    pages = read_pages(reader)
    headings = read_outline(reader)
    vocab = build_vocabulary(pages)

    first_page = next(h["page"] for h in headings if h["number"] == "7")
    last_page = next((h["page"] for h in headings if h["number"] == "B"), len(pages) + 1)

    # Flatten to (page, line) and annotate each line with its section.
    flat: list[tuple[int, str]] = []
    for number, lines in enumerate(pages, start=1):
        if first_page <= number < last_page:
            flat.extend((number, line) for line in lines)

    pending = [h for h in headings if first_page <= h["page"] < last_page]
    section_at: list[int] = []  # index into `seen` per flat line
    seen: list[dict] = [next(h for h in headings if h["number"] == "7")]
    cursor = 0
    for page, line in flat:
        text = " ".join(line.split())
        # Headings appear in document order; look a few entries ahead only.
        for ahead in range(cursor, min(cursor + 4, len(pending))):
            heading = pending[ahead]
            full = f'{heading["number"]} {heading["title"]}'
            if heading["page"] == page and (text == full or (len(text) > 12 and full.startswith(text))):
                seen.append(heading)
                cursor = ahead + 1
                break
        section_at.append(len(seen) - 1)

    by_number = {h["number"]: h for h in headings}

    def section_path(number: str) -> list[dict]:
        parts = number.split(".")
        path = []
        for depth in range(1, len(parts) + 1):
            heading = by_number.get(".".join(parts[:depth]))
            if heading:
                path.append({"number": heading["number"], "title": heading["title"]})
        return path

    requirements = []
    index = 0
    while index < len(flat):
        page, line = flat[index]
        match = REQ_START.match(line.strip())
        if not match or match.group(2).lstrip().startswith("["):
            index += 1
            continue
        # A real requirement opens its body (⌈) before the next requirement starts.
        end = None
        opened = "⌈" in line
        probe = index
        while probe < len(flat):
            text = flat[probe][1]
            if probe > index and not opened and REQ_START.match(text.strip()):
                break
            opened = opened or "⌈" in text
            if opened and "⌋" in text:
                end = probe
                break
            probe += 1
        if end is None:
            index += 1
            continue

        req_id = match.group(1)
        block = [flat[i][1] for i in range(index, end + 1)]
        block[0] = match.group(2)
        text = "\n".join(block)
        head, _, rest = text.partition("⌈")
        body = rest.rpartition("⌋")[0]

        head_lines = [l.strip() for l in head.split("\n") if l.strip()]
        title_lines, upstream_lines, status = [], [], ""
        target = title_lines
        for head_line in head_lines:
            if head_line.startswith("Upstream requirements:"):
                target = upstream_lines
                head_line = head_line.removeprefix("Upstream requirements:")
            elif head_line.startswith("Status:"):
                status = head_line.removeprefix("Status:").strip()
                continue
            target.append(head_line)
        title = join_lines(title_lines, vocab).strip(" :")
        if title in {"DRAFT", "OBSOLETE"}:
            title, status = "", title
        upstream = UPSTREAM_ID.findall(" ".join(upstream_lines))

        body_lines = body.split("\n")
        tabular = looks_tabular(title, body_lines)
        body_text = (
            "\n".join(l.rstrip() for l in body_lines if l.strip())
            if tabular else join_lines(body_lines, vocab)
        )

        heading = seen[section_at[index]]
        refs = sorted({r for r in REQ_ID.findall(body_text) if r != req_id})
        requirements.append({
            "id": req_id,
            "title": title,
            "status": status,
            "kind": kind_for(req_id, title),
            "stage": stage_for(req_id, heading["number"], heading["title"], title),
            "section": {"number": heading["number"], "title": heading["title"]},
            "path": section_path(heading["number"]),
            "page": page,
            "upstream": upstream,
            "text": body_text,
            "tabular": tabular,
            "refs": refs,
        })
        index = end + 1

    release = ""
    for line in (reader.pages[0].extract_text() or "").split("\n"):
        found = re.search(r"R\d\d-\d\d", line)
        if found:
            release = found.group(0)
            break

    return {
        "meta": {
            "document": "AUTOSAR CP — Specification of Operating System (SWS OS)",
            "release": release,
            "source": pdf.name,
            "pages": len(pages),
            "generated": datetime.date.today().isoformat(),
            "stages": STAGE_ORDER,
        },
        "requirements": requirements,
    }


def scan_code_references(ids: set[str]) -> dict[str, list[str]]:
    """Map requirement ID -> ["path:line", ...] for Rust sources in the repo."""
    found: dict[str, list[str]] = {}
    for path in sorted(REPO.rglob("*.rs")):
        relative = path.relative_to(REPO)
        if relative.parts[0] in {"target", "spec", "tools"}:
            continue
        for number, line in enumerate(path.read_text(encoding="utf-8").splitlines(), start=1):
            for req_id in set(REQ_ID.findall(line)):
                if req_id in ids:
                    found.setdefault(req_id, []).append(f"{relative.as_posix()}:{number}")
    return found


def main() -> None:
    parser = argparse.ArgumentParser(description=__doc__.split("\n\n")[0])
    parser.add_argument("pdf", type=Path, help="path to AUTOSAR_CP_SWS_OS.pdf")
    parser.add_argument("--out", type=Path, default=REPO / "spec", help="output directory")
    args = parser.parse_args()

    data = extract(args.pdf)
    requirements = data["requirements"]
    references = scan_code_references({r["id"] for r in requirements})
    for requirement in requirements:
        requirement["code"] = references.get(requirement["id"], [])

    duplicates = [i for i, n in Counter(r["id"] for r in requirements).items() if n > 1]
    data["meta"]["count"] = len(requirements)

    args.out.mkdir(parents=True, exist_ok=True)
    json_path = args.out / "sws-os-requirements.json"
    html_path = args.out / "sws-os-requirements.html"
    json_path.write_text(json.dumps(data, ensure_ascii=False, indent=1), encoding="utf-8")
    payload = json.dumps(data, ensure_ascii=False, separators=(",", ":")).replace("</", "<\\/")
    html_path.write_text(
        TEMPLATE.read_text(encoding="utf-8").replace("__SWS_DATA__", payload), encoding="utf-8"
    )

    print(f"{len(requirements)} requirements from {args.pdf.name} ({data['meta']['release']})")
    for stage in STAGE_ORDER:
        count = sum(r["stage"] == stage for r in requirements)
        if count:
            print(f"  {count:4d}  {stage}")
    print(f"  {sum(bool(r['code']) for r in requirements):4d}  referenced from Rust sources")
    if duplicates:
        print(f"warning: IDs extracted more than once: {', '.join(duplicates)}")
    print(f"wrote {json_path.relative_to(REPO)} and {html_path.relative_to(REPO)}")


if __name__ == "__main__":
    main()
