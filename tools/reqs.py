#!/usr/bin/env python3
"""Check the requirements that Rautosa derives from specifications which have
no numbered requirements of their own (OSEK OS).

Usage:

    python3 tools/reqs.py            check every file under requirements/
    python3 tools/reqs.py --list     also print every requirement
    python3 tools/reqs.py --strict   treat style warnings as errors

Needs Python 3.11 or newer and nothing else. The files it reads are described
in docs/requirements-guide.md:

    requirements/<document>/document.toml   title, ID prefix, section headings
    requirements/<document>/*.toml          [[requirement]] entries

Errors are structural (a missing key, a duplicate ID, an unknown section) and
make the exit code 1. Warnings point at wording that usually means the
requirement is not atomic or not testable yet.
"""

from __future__ import annotations

import argparse
import re
import sys
import tomllib
from dataclasses import dataclass, field
from pathlib import Path

REPO = Path(__file__).resolve().parent.parent
ROOT = REPO / "requirements"

MANDATORY = {"id", "text", "source", "page"}
OPTIONAL = {"where", "level", "kind", "verify", "stage", "note", "see", "withdrawn"}
LEVELS = {"standard", "extended"}
KINDS = {"requirement", "constraint"}
VERIFY = {"test", "review", "analysis"}
STAGES = {1, 2, 3, 4}

SWS_ID = re.compile(r"SWS_Os(?:_[A-Z]+)?_\d{5}")
VAGUE = re.compile(
    r"\b(should|may|might|could|appropriate(?:ly)?|proper(?:ly)?|correct(?:ly)?|"
    r"as needed|if necessary|if possible|etc|and/or|depends|usually|normally|"
    r"quickly|fast|efficient(?:ly)?)\b",
    re.IGNORECASE,
)
MAX_LENGTH = 300


@dataclass
class Result:
    documents: list[dict] = field(default_factory=list)
    requirements: list[dict] = field(default_factory=list)
    errors: list[str] = field(default_factory=list)
    warnings: list[str] = field(default_factory=list)
    per_file: dict[str, int] = field(default_factory=dict)


def id_regex(prefix: str) -> str:
    """Regular expression source matching the requirement IDs of one document."""
    return rf"{re.escape(prefix)}_[A-Z][A-Za-z0-9]*_\d{{2,}}"


def section_key(number: str) -> tuple:
    return tuple(int(part) if part.isdigit() else part for part in number.split("."))


def section_path(number: str, sections: dict[str, str]) -> list[dict]:
    parts = number.split(".")
    path = []
    for depth in range(1, len(parts) + 1):
        ancestor = ".".join(parts[:depth])
        if ancestor in sections:
            path.append({"number": ancestor, "title": sections[ancestor]})
    return path


def style_warnings(text: str) -> list[str]:
    found = []
    shall = len(re.findall(r"\bshall\b", text))
    if shall == 0:
        found.append('has no "shall": say who shall do what')
    elif shall > 1:
        found.append(f'has {shall} "shall": probably more than one requirement')
    vague = sorted({m.group(1).lower() for m in VAGUE.finditer(text)})
    if vague:
        found.append(f"uses wording that cannot be tested: {', '.join(vague)}")
    if len(text) > MAX_LENGTH:
        found.append(f"is {len(text)} characters long: probably more than one requirement")
    if not text.endswith("."):
        found.append("does not end with a full stop")
    return found


def load(root: Path = ROOT) -> Result:
    """Read and validate every requirement file below `root`."""
    result = Result()
    seen: dict[str, str] = {}
    pending_see: list[tuple[str, str]] = []

    for directory in sorted(p for p in root.iterdir() if p.is_dir()) if root.is_dir() else []:
        meta_path = directory / "document.toml"
        where = meta_path.relative_to(REPO).as_posix()
        if not meta_path.is_file():
            result.errors.append(f"{where}: missing")
            continue
        try:
            meta = tomllib.loads(meta_path.read_text(encoding="utf-8"))
        except tomllib.TOMLDecodeError as error:
            result.errors.append(f"{where}: {error}")
            continue
        missing = {"title", "short", "prefix", "sections"} - meta.keys()
        if missing:
            result.errors.append(f"{where}: missing key(s) {', '.join(sorted(missing))}")
            continue
        sections: dict[str, str] = meta["sections"]
        prefix: str = meta["prefix"]
        id_pattern = re.compile(id_regex(prefix))
        result.documents.append({
            "key": directory.name, "title": meta["title"], "short": meta["short"], "prefix": prefix,
        })

        for path in sorted(directory.glob("*.toml")):
            if path.name == "document.toml":
                continue
            name = path.relative_to(REPO).as_posix()
            try:
                data = tomllib.loads(path.read_text(encoding="utf-8"))
            except tomllib.TOMLDecodeError as error:
                result.errors.append(f"{name}: {error}")
                continue
            extra = data.keys() - {"requirement"}
            if extra:
                result.errors.append(f"{name}: unknown top-level key(s) {', '.join(sorted(extra))}")
            entries = data.get("requirement", [])
            result.per_file[name] = len(entries)

            for position, entry in enumerate(entries, start=1):
                req_id = entry.get("id", f"entry #{position}")
                label = f"{name}: {req_id}"
                before = len(result.errors)

                missing = MANDATORY - entry.keys()
                if missing:
                    result.errors.append(f"{label}: missing key(s) {', '.join(sorted(missing))}")
                unknown = entry.keys() - MANDATORY - OPTIONAL
                if unknown:
                    result.errors.append(f"{label}: unknown key(s) {', '.join(sorted(unknown))}")

                if "id" in entry:
                    if not isinstance(req_id, str) or not id_pattern.fullmatch(req_id):
                        result.errors.append(f"{label}: ID must look like {prefix}_<Topic>_<NN>")
                    elif req_id in seen:
                        result.errors.append(f"{label}: ID already used in {seen[req_id]}")
                    else:
                        seen[req_id] = name

                text = entry.get("text", "")
                if not isinstance(text, str) or not text.strip():
                    if "text" in entry:
                        result.errors.append(f"{label}: text is empty")
                    text = ""
                text = " ".join(text.split())

                source = entry.get("source")
                if source is not None and source not in sections:
                    result.errors.append(f"{label}: section {source!r} is not listed in {where}")
                if "page" in entry and (not isinstance(entry["page"], int) or entry["page"] < 1):
                    result.errors.append(f"{label}: page must be a positive number")
                for key, allowed in (("level", LEVELS), ("kind", KINDS), ("verify", VERIFY), ("stage", STAGES)):
                    if key in entry and entry[key] not in allowed:
                        choices = ", ".join(str(a) for a in sorted(allowed, key=str))
                        result.errors.append(f"{label}: {key} must be one of {choices}")
                for key in ("where", "note", "withdrawn"):
                    if key in entry and (not isinstance(entry[key], str) or not entry[key].strip()):
                        result.errors.append(f"{label}: {key} must be a non-empty string")
                see = entry.get("see", [])
                if not isinstance(see, list) or not all(isinstance(item, str) for item in see):
                    result.errors.append(f"{label}: see must be a list of requirement IDs")
                    see = []
                pending_see.extend((label, item) for item in see)

                if len(result.errors) > before:
                    continue
                if "withdrawn" not in entry:
                    result.warnings.extend(f"{label}: {w}" for w in style_warnings(text))

                result.requirements.append({
                    "id": req_id,
                    "doc": directory.name,
                    "text": text,
                    "section": {"number": source, "title": sections[source]},
                    "path": section_path(source, sections),
                    "page": entry["page"],
                    "where": entry.get("where", ""),
                    "level": entry.get("level", ""),
                    "kind": entry.get("kind", "requirement"),
                    "verify": entry.get("verify", "test"),
                    "stage": entry.get("stage", 1),
                    "note": " ".join(entry.get("note", "").split()),
                    "see": see,
                    "withdrawn": entry.get("withdrawn", ""),
                    "file": name,
                })

    for label, target in pending_see:
        if target not in seen and not SWS_ID.fullmatch(target):
            result.errors.append(f"{label}: see refers to unknown requirement {target}")

    order = {r["id"]: index for index, r in enumerate(result.requirements)}
    result.requirements.sort(
        key=lambda r: (r["doc"], section_key(r["section"]["number"]), order[r["id"]])
    )
    return result


def main() -> None:
    parser = argparse.ArgumentParser(description=__doc__.split("\n\n")[0])
    parser.add_argument("--list", action="store_true", help="print every requirement")
    parser.add_argument("--strict", action="store_true", help="treat warnings as errors")
    args = parser.parse_args()

    result = load()
    if args.list:
        section = None
        for requirement in result.requirements:
            here = (requirement["doc"], requirement["section"]["number"])
            if here != section:
                section = here
                print(f'\n{requirement["section"]["number"]} {requirement["section"]["title"]}')
            flags = [requirement[k] for k in ("level", "where") if requirement[k]]
            if requirement["kind"] != "requirement":
                flags.insert(0, requirement["kind"])
            if requirement["withdrawn"]:
                flags.insert(0, "withdrawn")
            print(f'  {requirement["id"]}  {requirement["text"]}')
            if flags:
                print(f'  {" " * len(requirement["id"])}  [{"; ".join(flags)}]')
        print()

    for name, count in result.per_file.items():
        print(f"{count:4d}  {name}")
    for warning in result.warnings:
        print(f"warning: {warning}")
    for error in result.errors:
        print(f"error: {error}")

    failed = bool(result.errors) or (args.strict and bool(result.warnings))
    total = sum(result.per_file.values())
    print(
        f"{'FAILED' if failed else 'ok'}: {total} requirement(s), "
        f"{len(result.errors)} error(s), {len(result.warnings)} warning(s)"
    )
    sys.exit(1 if failed else 0)


if __name__ == "__main__":
    main()
