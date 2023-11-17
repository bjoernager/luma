OUTPUT_ARCH(arm);

MEMORY {
	bios  : ORIGIN = 0x00000000, LENGTH = 0x00004000
};

SECTIONS {
	.bss    : {*(.bss*)}    > bios
	.data   : {*(.data*)}   > bios
	.text   : {*(.text*)}   > bios
	.rodata : {*(.rodata*)} > bios
};
