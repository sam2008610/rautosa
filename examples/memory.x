/* Memory map of the QEMU `mps2-an385` machine (ARM MPS2 board, AN385 FPGA image).
 *
 *   0x0000_0000  4 MiB  ZBT SSRAM1   -- code; the vector table is fetched from address 0
 *   0x2000_0000  4 MiB  ZBT SSRAM2+3 -- data
 *
 * Both are RAM on the real board (an FPGA image has no flash). `cortex-m-rt`
 * only needs the region that holds the vector table and code to be called FLASH.
 */
MEMORY
{
  FLASH : ORIGIN = 0x00000000, LENGTH = 4M
  RAM   : ORIGIN = 0x20000000, LENGTH = 4M
}
