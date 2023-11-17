.cpu arm7tdmi

.arm
b _start

.byte 0x24, 0xFF, 0xAE, 0x51
.byte 0x69, 0x9A, 0xA2, 0x21
.byte 0x3D, 0x84, 0x82, 0x0A
.byte 0x84, 0xE4, 0x09, 0xAD
.byte 0x11, 0x24, 0x8B, 0x98
.byte 0xC0, 0x81, 0x7F, 0x21
.byte 0xA3, 0x52, 0xBE, 0x19
.byte 0x93, 0x09, 0xCE, 0x20
.byte 0x10, 0x46, 0x4A, 0x4A
.byte 0xF8, 0x27, 0x31, 0xEC
.byte 0x58, 0xC7, 0xE8, 0x33
.byte 0x82, 0xE3, 0xCE, 0xBF
.byte 0x85, 0xF4, 0xDF, 0x94
.byte 0xCE, 0x4B, 0x09, 0xC1
.byte 0x94, 0x56, 0x8A, 0xC0
.byte 0x13, 0x72, 0xA7, 0xFC
.byte 0x9F, 0x84, 0x4D, 0x73
.byte 0xA3, 0xCA, 0x9A, 0x61
.byte 0x58, 0x97, 0xA3, 0x27
.byte 0xFC, 0x03, 0x98, 0x76
.byte 0x23, 0x1D, 0xC7, 0x61
.byte 0x03, 0x04, 0xAE, 0x56
.byte 0xBF, 0x38, 0x84, 0x00
.byte 0x40, 0xA7, 0x0E, 0xFD
.byte 0xFF, 0x52, 0xFE, 0x03
.byte 0x6F, 0x95, 0x30, 0xF1
.byte 0x97, 0xFB, 0xC0, 0x85
.byte 0x60, 0xD6, 0x80, 0x25
.byte 0xA9, 0x63, 0xBE, 0x03
.byte 0x01, 0x4E, 0x38, 0xE2
.byte 0xF9, 0xA2, 0x34, 0xFF
.byte 0xBB, 0x3E, 0x03, 0x44
.byte 0x78, 0x00, 0x90, 0xCB
.byte 0x88, 0x11, 0x3A, 0x94
.byte 0x65, 0xC0, 0x7C, 0x63
.byte 0x87, 0xF0, 0x3C, 0xAF
.byte 0xD6, 0x25, 0xE4, 0x8B
.byte 0x38, 0x0A, 0xAC, 0x72
.byte 0x21, 0xD4, 0xF8, 0x07

.ascii "LUMA\x0\x0\x0\x0\x0\x0\x0\x0"

.ascii "AMUL"

.ascii "00"

.byte 0x96

.byte 0x0

.byte 0x0

.fill 0x7,0x1,0x0

.byte 0x45

.byte 0x0

.fill 0x2,0x1,0x0

.arm
nop

.byte 0x0

.byte 0x0

.fill 0x1A,0x1,0x0

.arm
nop

.arm
.func
_start:
	mov  r9, #0x1
	rsbs r9, #0x0
	adds r9, #0x2
	mov  r9, r9, LSL #0x1F
	subs r9, #0x1
	mov  sp, #0x450
	swi  #0x3 @ Comment on other emulators.
	ldr  lr, =start
	bx   lr
.endfunc

.thumb
.func
.thumb_func
start:
	@ Set up video mode:
	ldr  r0, .ioAddr
	@ Already at DISPCNT.
	ldr  r1, .displayControl
	strh r1, [r0]

	@ Set up palette:
	ldr  r0, .paletteAddr
	ldr  r1, .backgroundColour
	strh r1, [r0]
	ldr  r1, .paletteIndex
	lsl  r1, #0x1 @ Multiply by two.
	add  r0, r1 @ Apply index.
	ldr  r1, .foregroundColour
	strh r1, [r0]

	@ Set up loop:
	@ - r0 is the current pixel address.
	@ - r1 is the palette value/index.
	@ - r2 is the addend.
	@ - r3 is the last pixel address.
	ldr  r0, .videoAddr
	ldr  r1, .paletteIndex
	bic  r2, r2
	mov  r3, #0x4B
	lsl  r3, #0x9
	add  r3, r0

	@ Plot pixels:
	@ TO-DO: Plot correctly (bytewise).
.loop:
	strb r1, [r0]
	add  r0, r2
	add  r2, #0x1
	cmp  r0, r3
	bge  .restart
	@bge  .stop
	b    .loop @ Repeat loop.

	@ Restart loop:
.restart:
	ldr  r0, .videoAddr
	b    .loop

.stop:
	b .stop
.endfunc

.align
.ioAddr:
	.word 0x04000000

.align
.displayControl:
	.word 0x0404

.align
.paletteAddr:
	.word 0x05000000

.align
.paletteIndex:
	.word 0xFF

.align
.backgroundColour:
	.word 0b0000100001000010

.align
.foregroundColour:
	.word 0b0001001010011110

.align
.videoAddr:
	.word 0x06000000