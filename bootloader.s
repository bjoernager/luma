.cpu arm7tdmi

.arm
b reset

.arm
b undefined_instruction

.arm
b software_interrupt

.arm
b prefetch_abort

.arm
b data_abort

.arm
b interrupt_request

.arm
b fast_interrupt_request

.func
reset:
	ldr lr, .image_entry_point
	bx  lr

.endfunc

.align
.image_entry_point:
	.word 0x08000000

.func
undefined_instruction:
	b undefined_instruction
.endfunc

.func
software_interrupt:
	ldr  r0, =.software_interrupt_impl
	mov  r1, pc
	bx   r0
	movs pc, lr
.endfunc

.thumb

.func
.thumb_func
.software_interrupt_impl:
	mov r0, lr
	sub r0, #0x4
	ldr r0, [r0]
	bx  r1

.endfunc

.arm

.func
prefetch_abort:
	b prefetch_abort
.endfunc

.func
data_abort:
	b data_abort
.endfunc

.func
interrupt_request:
	b interrupt_request
.endfunc

.func
fast_interrupt_request:
	b fast_interrupt_request
.endfunc
