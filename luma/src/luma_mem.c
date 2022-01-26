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
	ADDRESS SPACE:
	0000-3FFF : BOOTLOADER OR CARTRIDGE ROM BANK 01-FF
	4000-7FFF : CARTRIDGE ROM BANK 00
	8000-BFFF : VRAM
	C000-DFFF : SYSTEM RAM
	E000-E7FF : CARTRIDGE RAM
	E800-E800 : RESULT REGISTER
	E801-E801 : GENERAL REGISTER 0
	E802-E802 : GENERAL REGISTER 1
	E803-E803 : GENERAL REGISTER 2
	E804-E804 : GENERAL REGISTER 3
	E805-E805 : GENERAL REGISTER 4
	E806-E806 : GENERAL REGISTER 5
	E807-E807 : GENERAL REGISTER 6
	E808-E808 : GENERAL REGISTER 7
	E809-E809 : RANDOM REGISTER
	E90A-FFFF : NOTHING
*/
luma_byte luma_mem[0x10000];
