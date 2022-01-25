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

#include <inttypes.h>
#include <stdio.h>
#include <string.h>

/*
	OPCODES:
	00 : IGN :           : IGNORE
	01 : CPY : ptr ,ptr  : COPY
	02 : SET : ptr ,byte : SET
	03 : INC : ptr       : INCREMENT
	04 : DEC : ptr       : DECREMENT
	05 : JMP : ptr       : JUMP
	06 : DIE :           : DIE
	07 : LDB : byte      : LOAD BANK
*/

#include <assert.h>

void luma_opcd() {
	luma_byte const _opcd = luma_mem[luma_instrPtr];
	switch (_opcd) {
	default:
		fprintf(stderr,"Unknown opcode %" PRIx16 " at %" PRIx16 "\n",luma_mem[luma_instrPtr],luma_instrPtr);
		luma_abrt();
	case 0x0:
		break;
	case 0x1:
		{
			luma_ptr const dest = luma_getPtrVal(luma_instrPtr + 0x1);
			luma_ptr const src  = luma_getPtrVal(luma_instrPtr + 0x3);
			luma_log("Copying byte at %" PRIx16 " to %" PRIx16 "\n",src,dest);
			luma_mem[dest] = luma_mem[src];
		}
		luma_instrPtr += 0x4;
		break;
	case 0x2:
		{
			luma_ptr const  dest = luma_getPtrVal(luma_instrPtr + 0x1);
			luma_byte const val  = luma_mem[luma_instrPtr + 0x3];
			luma_log("Setting byte at %" PRIx16 " to %" PRIx16 "\n",dest,val);
			luma_mem[dest] = val;
		}
		luma_instrPtr += 0x3;
		break;
	case 0x5:
		luma_instrPtr = luma_getPtrVal(luma_instrPtr + 0x1);
		luma_log("Jumping to %" PRIx16 "\n",luma_instrPtr);
		return;
	case 0x6:
		luma_dead = true;
		break;
	case 0x7:
		{
			luma_byte const banknum = luma_mem[luma_instrPtr + 0x1];
			luma_log("Loading ROM bank %" PRId8 " from \"%s\"\n",banknum,luma_cart);
			luma_loadRom(luma_cart,banknum);
		}
		luma_instrPtr += 0x1;
	}
	luma_instrPtr += 0x1;
}
