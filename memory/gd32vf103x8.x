/* Ref: Firmware/RISCV/env_Eclipse/GD32VF103x?.lds */

MEMORY
{
    /* 87*4=348 (0x15C) */
    VECTORS     (RX): ORIGIN = 0x08000000, LENGTH = 0x015C 
    MAIN_FLASH  (RX): ORIGIN = 0x0800015C, LENGTH = 64K - 348
    SRAM        (RW): ORIGIN = 0x20000000, LENGTH = 20K
}

INCLUDE memory-base.x
