/* memory-base.x (GD32VF103 linker script) */
/* Author: Luo Jia <luojia65@hust.edu.cn> Wuhan, China 2020-2-26 */

REGION_ALIAS("REGION_TEXT", MAIN_FLASH);
REGION_ALIAS("REGION_RODATA", MAIN_FLASH);
REGION_ALIAS("REGION_DATA", SRAM);
REGION_ALIAS("REGION_BSS", SRAM);
REGION_ALIAS("REGION_HEAP", SRAM);
REGION_ALIAS("REGION_STACK", SRAM);

ENTRY(_gd32vf103_vectors)
EXTERN(_gd32vf103_vectors)
EXTERN(_gd32vf103_trap_entry)
EXTERN(_gd32vf103_irq_entry)

SECTIONS
{
  .vectors :
  {
    KEEP(*(.vectors));
  } > VECTORS
}
