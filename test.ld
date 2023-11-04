OUTPUT_ARCH(arm);

MEMORY {
	bios  : ORIGIN = 0x00000000, LENGTH = 0x00004000
	ewram : ORIGIN = 0x02000000, LENGTH = 0x00040000
	iwram : ORIGIN = 0x03000000, LENGTH = 0x00008000
	reg   : ORIGIN = 0x04000000, LENGTH = 0x000003FF
	pal   : ORIGIN = 0x05000000, LENGTH = 0x00000400
	vram  : ORIGIN = 0x06000000, LENGTH = 0x00018000
	oam   : ORIGIN = 0x07000000, LENGTH = 0x00000400
	rom   : ORIGIN = 0x08000000, LENGTH = 0x02000000
	sram  : ORIGIN = 0x0E000000, LENGTH = 0x00010000
};

SECTIONS {
	.bss    : {*(.bss*)}    > ewram
	.data   : {*(.data*)}   > ewram
	.text   : {*(.text*)}   > rom
	.rodata : {*(.rodata*)} > rom
};
