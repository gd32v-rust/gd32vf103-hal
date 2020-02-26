/* author:  Luo Jia <luojia65@hust.edu.cn> 2019-11-29 */
.section .vectors, "ax"
.globl _gd32vf103_vectors
_gd32vf103_vectors:
/* source: Firmware\RISCV\env_Eclipse\start.S */
    .weak  eclic_msip_handler
    .weak  eclic_mtip_handler
    .weak  eclic_bwei_handler
    .weak  eclic_pmovi_handler
    .weak  WWDGT_IRQHandler
    .weak  LVD_IRQHandler
    .weak  TAMPER_IRQHandler
    .weak  RTC_IRQHandler
    .weak  FMC_IRQHandler
    .weak  RCU_IRQHandler
    .weak  EXTI0_IRQHandler
    .weak  EXTI1_IRQHandler
    .weak  EXTI2_IRQHandler
    .weak  EXTI3_IRQHandler
    .weak  EXTI4_IRQHandler
    .weak  DMA0_Channel0_IRQHandler
    .weak  DMA0_Channel1_IRQHandler
    .weak  DMA0_Channel2_IRQHandler
    .weak  DMA0_Channel3_IRQHandler
    .weak  DMA0_Channel4_IRQHandler
    .weak  DMA0_Channel5_IRQHandler
    .weak  DMA0_Channel6_IRQHandler
    .weak  ADC0_1_IRQHandler
    .weak  CAN0_TX_IRQHandler
    .weak  CAN0_RX0_IRQHandler
    .weak  CAN0_RX1_IRQHandler
    .weak  CAN0_EWMC_IRQHandler
    .weak  EXTI5_9_IRQHandler
    .weak  TIMER0_BRK_IRQHandler
    .weak  TIMER0_UP_IRQHandler
    .weak  TIMER0_TRG_CMT_IRQHandler
    .weak  TIMER0_Channel_IRQHandler
    .weak  TIMER1_IRQHandler
    .weak  TIMER2_IRQHandler
    .weak  TIMER3_IRQHandler
    .weak  I2C0_EV_IRQHandler
    .weak  I2C0_ER_IRQHandler
    .weak  I2C1_EV_IRQHandler
    .weak  I2C1_ER_IRQHandler
    .weak  SPI0_IRQHandler
    .weak  SPI1_IRQHandler
    .weak  USART0_IRQHandler
    .weak  USART1_IRQHandler
    .weak  USART2_IRQHandler
    .weak  EXTI10_15_IRQHandler
    .weak  RTC_Alarm_IRQHandler
    .weak  USBFS_WKUP_IRQHandler
    .weak  EXMC_IRQHandler
    .weak  TIMER4_IRQHandler
    .weak  SPI2_IRQHandler
    .weak  UART3_IRQHandler
    .weak  UART4_IRQHandler
    .weak  TIMER5_IRQHandler
    .weak  TIMER6_IRQHandler
    .weak  DMA1_Channel0_IRQHandler
    .weak  DMA1_Channel1_IRQHandler
    .weak  DMA1_Channel2_IRQHandler
    .weak  DMA1_Channel3_IRQHandler
    .weak  DMA1_Channel4_IRQHandler
    .weak  CAN1_TX_IRQHandler
    .weak  CAN1_RX0_IRQHandler
    .weak  CAN1_RX1_IRQHandler
    .weak  CAN1_EWMC_IRQHandler
    .weak  USBFS_IRQHandler
    jal x0, _start_logical_addr
    .word     0
    .word     0
    .word     eclic_msip_handler
    .word     0
    .word     0
    .word  	  0
    .word  	  eclic_mtip_handler
    .word  	  0
    .word  	  0
    .word  	  0
    .word  	  0
    .word  	  0
    .word  	  0
    .word  	  0
    .word 	  0
    .word  	  0
    .word  	  eclic_bwei_handler
    .word  	  eclic_pmovi_handler
    .word  	  WWDGT_IRQHandler
	.word     LVD_IRQHandler
	.word     TAMPER_IRQHandler
	.word     RTC_IRQHandler
	.word     FMC_IRQHandler
	.word     RCU_IRQHandler
	.word     EXTI0_IRQHandler
	.word     EXTI1_IRQHandler
	.word     EXTI2_IRQHandler
	.word     EXTI3_IRQHandler
	.word     EXTI4_IRQHandler
	.word     DMA0_Channel0_IRQHandler
	.word     DMA0_Channel1_IRQHandler
	.word     DMA0_Channel2_IRQHandler
	.word     DMA0_Channel3_IRQHandler
	.word     DMA0_Channel4_IRQHandler
	.word     DMA0_Channel5_IRQHandler
	.word     DMA0_Channel6_IRQHandler
	.word     ADC0_1_IRQHandler
	.word     CAN0_TX_IRQHandler
	.word     CAN0_RX0_IRQHandler
	.word     CAN0_RX1_IRQHandler
	.word     CAN0_EWMC_IRQHandler
	.word     EXTI5_9_IRQHandler
	.word     TIMER0_BRK_IRQHandler
	.word     TIMER0_UP_IRQHandler
	.word     TIMER0_TRG_CMT_IRQHandler
	.word     TIMER0_Channel_IRQHandler
	.word     TIMER1_IRQHandler
	.word     TIMER2_IRQHandler
	.word     TIMER3_IRQHandler
	.word     I2C0_EV_IRQHandler
	.word     I2C0_ER_IRQHandler
	.word     I2C1_EV_IRQHandler
	.word     I2C1_ER_IRQHandler
	.word     SPI0_IRQHandler
	.word     SPI1_IRQHandler
	.word     USART0_IRQHandler
	.word     USART1_IRQHandler
	.word     USART2_IRQHandler
	.word     EXTI10_15_IRQHandler
	.word     RTC_Alarm_IRQHandler
	.word     USBFS_WKUP_IRQHandler
    .word     0
    .word     0
    .word     0
    .word     0
	.word     0
	.word     EXMC_IRQHandler
	.word     0
	.word     TIMER4_IRQHandler
	.word     SPI2_IRQHandler
	.word     UART3_IRQHandler
	.word     UART4_IRQHandler
	.word     TIMER5_IRQHandler
	.word     TIMER6_IRQHandler
	.word     DMA1_Channel0_IRQHandler
	.word     DMA1_Channel1_IRQHandler
	.word     DMA1_Channel2_IRQHandler
	.word     DMA1_Channel3_IRQHandler
	.word     DMA1_Channel4_IRQHandler
    .word     0
    .word     0
	.word     CAN1_TX_IRQHandler
	.word     CAN1_RX0_IRQHandler
	.word     CAN1_RX1_IRQHandler
	.word     CAN1_EWMC_IRQHandler
	.word     USBFS_IRQHandler

# RISC-V defined CSR registers
.equ mstatus, 0x300
.equ mie, 0x304
.equ mtvec, 0x305
# Bumblebee core defined CSR registers
.equ mtvt, 0x307
.equ msubm, 0x7c4
.equ mtvt2, 0x7ec
.equ jalmnxti, 0x7ed
.equ pushmcause, 0x7ee
.equ pushmepc, 0x7ef
.equ pushmsubm, 0x7eb
# Constants
.equ REGBYTES,4
.equ MSTATUS_MIE, 0x00000008

.section .init, "ax"
.globl _start_logical_addr

_start_logical_addr:
    la		a0,	_start_logical_addr
    li		a1,	1
	slli	a1,	a1, 29
    bleu	a1, a0, _start_bootstrap
    srli	a1,	a1, 2
    bleu	a1, a0, _start_bootstrap
    la		a0,	_start_bootstrap
    add		a0, a0, a1
	jr      a0

_start_bootstrap:
# Initialize mtvt
    la t0, _gd32vf103_vectors
    csrw mtvt, t0
# Initialize and enable mtvt2
    la t0, _gd32vf103_irq_entry
    csrw mtvt2, t0
    csrs mtvt2, 0x1
# Initialize mtvec for trap & NMI base addr
    la t0, _gd32vf103_trap_entry
    csrw mtvec, t0 # todo: conflict with riscv-rt?
# Start riscv-rt Rust entry which would check hart id,
# allocate stack, clear bss section 
    j _start

.macro SAVE_CONTEXT
	sw ra, 0*REGBYTES(sp)
    sw tp, 1*REGBYTES(sp)
    sw t0, 2*REGBYTES(sp)
    sw t1, 3*REGBYTES(sp)
    sw t2, 4*REGBYTES(sp)
    sw t3, 5*REGBYTES(sp)
    sw t4, 6*REGBYTES(sp)
    sw t5, 7*REGBYTES(sp)
    sw t6, 8*REGBYTES(sp)
    sw a0, 9*REGBYTES(sp)
    sw a1, 10*REGBYTES(sp)
    sw a2, 11*REGBYTES(sp)
    sw a3, 12*REGBYTES(sp)
    sw a4, 13*REGBYTES(sp)
    sw a5, 14*REGBYTES(sp)
    sw a6, 15*REGBYTES(sp)
    sw a7, 16*REGBYTES(sp)
.endm

.macro RESTORE_CONTEXT
	lw ra, 0*REGBYTES(sp)
    lw tp, 1*REGBYTES(sp)
    lw t0, 2*REGBYTES(sp)
    lw t1, 3*REGBYTES(sp)
    lw t2, 4*REGBYTES(sp)
    lw t3, 5*REGBYTES(sp)
    lw t4, 6*REGBYTES(sp)
    lw t5, 7*REGBYTES(sp)
    lw t6, 8*REGBYTES(sp)
    lw a0, 9*REGBYTES(sp)
    lw a1, 10*REGBYTES(sp)
    lw a2, 11*REGBYTES(sp)
    lw a3, 12*REGBYTES(sp)
    lw a4, 13*REGBYTES(sp)
    lw a5, 14*REGBYTES(sp)
    lw a6, 15*REGBYTES(sp)
    lw a7, 16*REGBYTES(sp)
.endm

.section .trap, "ax"
.align 6 # In CLIC mode, trap entry must be 64 byte aligned
.globl _gd32vf103_trap_entry
.weak _gd32vf103_trap_entry
_gd32vf103_trap_entry:
# Allocate stack space
	addi sp, sp, -19*REGBYTES
# Save ra, tp, t*, a* registers
	SAVE_CONTEXT # save 17 registers
# Store mcause, mepc and msubm into memory
	csrrwi  zero, pushmcause, 17
	csrrwi  zero, pushmepc, 18
	csrrwi	zero, pushmsubm, 19
# Set the function argument of trap handler
	csrr a0, mcause
	mv a1, sp
# Calls the trap handler
    # todo: should call function with signature 
    # extern "C" fn(mcause: usize, sp: usize) -> ()
	call _start_trap_rust # todo: replace
# Restore mcause, mepc and msubm
	lw a0, 19*REGBYTES(sp)
	csrw msubm, a0
	lw a0, 18*REGBYTES(sp)
	csrw mepc, a0
	lw a0, 17*REGBYTES(sp)
	csrw mcause, a0
# Restore ra, tp, t*, a* registers
	RESTORE_CONTEXT # restore 17 registers
# Free stack space
	addi sp, sp, 20*REGBYTES
	mret

.section .trap, "ax"
.align 2
.globl _gd32vf103_irq_entry
.weak _gd32vf103_irq_entry
_gd32vf103_irq_entry:
# Adjust stack pointer to allocate stack space
	addi sp, sp, -20*REGBYTES
# Save ra, tp, t*, a* registers
	SAVE_CONTEXT # save 17 registers
# These are special CSR read operations
# which actually uses mcause, mepc and msubm as operand 
# to directly store it to memory (stack)
	csrrwi  zero, pushmcause, 17
	csrrwi  zero, pushmepc, 18
	csrrwi	zero, pushmsubm, 19
# This special CSR read/write operation
# It claims the CLIC to find its pending highest ID.
# If the ID is not 0, then _automatically enable 
# the mstatus.MIE_, and jump to its vector-entry-label, 
# and update the link register.
# This design targets to speed up biting tail interrupts.
	csrrw ra, jalmnxti, ra 
# Disable interrupts
	csrc mstatus, MSTATUS_MIE
# Restore mcause, mepc and msubm
	lw a0, 19*REGBYTES(sp)
	csrw msubm, a0
	lw a0, 18*REGBYTES(sp)
	csrw mepc, a0
	lw a0, 17*REGBYTES(sp)
	csrw mcause, a0
# Restore ra, tp, t*, a* registers
	RESTORE_CONTEXT # restore 17 registers
# Free stack space
	addi sp, sp, 20*REGBYTES
	mret
