/*
	Copyright 2021, 2022 Gabriel Jensen

	This file is part of luma.

	luma is free software: you can redistribute it and/or modify it under the
	terms of the GNU Affero General Public License as published by the Free
	Software Foundation, either version 3 of the License, or (at your
	option) any later version.

	luma is distributed in the hope that it will be useful, but WITHOUT ANY
	WARRANTY; without even the implied warranty of MERCHANTABILITY or
	FITNESS FOR A PARTICULAR PURPOSE. See the GNU General Public
	License for more details.

	You should have received a copy of the GNU Affero General Public License
	along with luma. If not, see <https://www.gnu.org/licenses/>.
*/

#include "luma.h"

/*
	MEMORY MODEL:
	0000-3FFF : BOOTLOADER OR CARTRIDGE ROM BANK 1+
	4000-7FFF : CARTRIDGE ROM BANK 0
	8000-BFFF : VRAM
	C000-DFFF : SYSTEM RAM
	C000-C3FF : I/O BUFFER
*/
luma_byte luma_mem[0x10000];
