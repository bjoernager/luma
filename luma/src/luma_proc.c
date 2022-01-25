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
	HEX : NAME : PARAMETERS : FULL NAME

	 00 : IGN  :            : IGNORE
	 01 : CPY  : ptr ,ptr   : COPY
	 02 : SET  : ptr ,byte  : SET
	 03 : INC  : ptr        : INCREMENT
	 04 : DEC  : ptr        : DECREMENT
	 05 : JMP  : ptr        : JUMP
	 06 : DIE  :            : DIE
	 07 : BNK  : byte       : LOAD BANK
	 08 : TRP  :            : TRAP
	 09 : CMP  : ptr ,ptr   : COMPARE
	 0A : JEQ  : ptr        : JUMP IF EQUAL
	 0B : JLT  : ptr        : JUMP IF LESS THAN
	 0C : JGT  : ptr        : JUMP IF GREATER THAN
	 0D : JLE  : ptr        : JUMP IF LESS THAN OR EQUAL
	 0E : JGE  : ptr        : JUMP IF GREATER THAN OR EQUAL
	 0F : ICD  : ptr        : INCREMENT DOUBLE
	 10 : DCD  : ptr        : DECREMENT DOUBLE
	 11 : CPD  : ptr ,ptr   : COMPARE DOUBLE
*/

typedef void (* luma_opcdHandlTer)(void);

static void luma_opcdHandl_ilg(void);
static void luma_opcdHandl_ign(void);
static void luma_opcdHandl_cpy(void);
static void luma_opcdHandl_set(void);
static void luma_opcdHandl_inc(void);
static void luma_opcdHandl_dec(void);
static void luma_opcdHandl_jmp(void);
static void luma_opcdHandl_die(void);
static void luma_opcdHandl_bnk(void);
static void luma_opcdHandl_trp(void);
static void luma_opcdHandl_cmp(void);
static void luma_opcdHandl_jeq(void);
static void luma_opcdHandl_jlt(void);
static void luma_opcdHandl_jgt(void);
static void luma_opcdHandl_jle(void);
static void luma_opcdHandl_jge(void);
static void luma_opcdHandl_icd(void);
static void luma_opcdHandl_dcd(void);
static void luma_opcdHandl_cpd(void);

luma_opcdHandlTer luma_opcdHandls[0x100] = {
	luma_opcdHandl_ign,
	luma_opcdHandl_cpy,
	luma_opcdHandl_set,
	luma_opcdHandl_inc,
	luma_opcdHandl_dec,
	luma_opcdHandl_jmp,
	luma_opcdHandl_die,
	luma_opcdHandl_bnk,
	luma_opcdHandl_trp,
	luma_opcdHandl_cmp,
	luma_opcdHandl_jeq,
	luma_opcdHandl_jlt,
	luma_opcdHandl_jgt,
	luma_opcdHandl_jle,
	luma_opcdHandl_jge,
	luma_opcdHandl_icd,
	luma_opcdHandl_dcd,
	luma_opcdHandl_cpd,
	luma_opcdHandl_ilg,
	luma_opcdHandl_ilg,
	luma_opcdHandl_ilg,
	luma_opcdHandl_ilg,
	luma_opcdHandl_ilg,
	luma_opcdHandl_ilg,
	luma_opcdHandl_ilg,
	luma_opcdHandl_ilg,
	luma_opcdHandl_ilg,
	luma_opcdHandl_ilg,
	luma_opcdHandl_ilg,
	luma_opcdHandl_ilg,
	luma_opcdHandl_ilg,
	luma_opcdHandl_ilg,
	luma_opcdHandl_ilg,
	luma_opcdHandl_ilg,
	luma_opcdHandl_ilg,
	luma_opcdHandl_ilg,
	luma_opcdHandl_ilg,
	luma_opcdHandl_ilg,
	luma_opcdHandl_ilg,
	luma_opcdHandl_ilg,
	luma_opcdHandl_ilg,
	luma_opcdHandl_ilg,
	luma_opcdHandl_ilg,
	luma_opcdHandl_ilg,
	luma_opcdHandl_ilg,
	luma_opcdHandl_ilg,
	luma_opcdHandl_ilg,
	luma_opcdHandl_ilg,
	luma_opcdHandl_ilg,
	luma_opcdHandl_ilg,
	luma_opcdHandl_ilg,
	luma_opcdHandl_ilg,
	luma_opcdHandl_ilg,
	luma_opcdHandl_ilg,
	luma_opcdHandl_ilg,
	luma_opcdHandl_ilg,
	luma_opcdHandl_ilg,
	luma_opcdHandl_ilg,
	luma_opcdHandl_ilg,
	luma_opcdHandl_ilg,
	luma_opcdHandl_ilg,
	luma_opcdHandl_ilg,
	luma_opcdHandl_ilg,
	luma_opcdHandl_ilg,
	luma_opcdHandl_ilg,
	luma_opcdHandl_ilg,
	luma_opcdHandl_ilg,
	luma_opcdHandl_ilg,
	luma_opcdHandl_ilg,
	luma_opcdHandl_ilg,
	luma_opcdHandl_ilg,
	luma_opcdHandl_ilg,
	luma_opcdHandl_ilg,
	luma_opcdHandl_ilg,
	luma_opcdHandl_ilg,
	luma_opcdHandl_ilg,
	luma_opcdHandl_ilg,
	luma_opcdHandl_ilg,
	luma_opcdHandl_ilg,
	luma_opcdHandl_ilg,
	luma_opcdHandl_ilg,
	luma_opcdHandl_ilg,
	luma_opcdHandl_ilg,
	luma_opcdHandl_ilg,
	luma_opcdHandl_ilg,
	luma_opcdHandl_ilg,
	luma_opcdHandl_ilg,
	luma_opcdHandl_ilg,
	luma_opcdHandl_ilg,
	luma_opcdHandl_ilg,
	luma_opcdHandl_ilg,
	luma_opcdHandl_ilg,
	luma_opcdHandl_ilg,
	luma_opcdHandl_ilg,
	luma_opcdHandl_ilg,
	luma_opcdHandl_ilg,
	luma_opcdHandl_ilg,
	luma_opcdHandl_ilg,
	luma_opcdHandl_ilg,
	luma_opcdHandl_ilg,
	luma_opcdHandl_ilg,
	luma_opcdHandl_ilg,
	luma_opcdHandl_ilg,
	luma_opcdHandl_ilg,
	luma_opcdHandl_ilg,
	luma_opcdHandl_ilg,
	luma_opcdHandl_ilg,
	luma_opcdHandl_ilg,
	luma_opcdHandl_ilg,
	luma_opcdHandl_ilg,
	luma_opcdHandl_ilg,
	luma_opcdHandl_ilg,
	luma_opcdHandl_ilg,
	luma_opcdHandl_ilg,
	luma_opcdHandl_ilg,
	luma_opcdHandl_ilg,
	luma_opcdHandl_ilg,
	luma_opcdHandl_ilg,
	luma_opcdHandl_ilg,
	luma_opcdHandl_ilg,
	luma_opcdHandl_ilg,
	luma_opcdHandl_ilg,
	luma_opcdHandl_ilg,
	luma_opcdHandl_ilg,
	luma_opcdHandl_ilg,
	luma_opcdHandl_ilg,
	luma_opcdHandl_ilg,
	luma_opcdHandl_ilg,
	luma_opcdHandl_ilg,
	luma_opcdHandl_ilg,
	luma_opcdHandl_ilg,
	luma_opcdHandl_ilg,
	luma_opcdHandl_ilg,
	luma_opcdHandl_ilg,
	luma_opcdHandl_ilg,
	luma_opcdHandl_ilg,
	luma_opcdHandl_ilg,
	luma_opcdHandl_ilg,
	luma_opcdHandl_ilg,
	luma_opcdHandl_ilg,
	luma_opcdHandl_ilg,
	luma_opcdHandl_ilg,
	luma_opcdHandl_ilg,
	luma_opcdHandl_ilg,
	luma_opcdHandl_ilg,
	luma_opcdHandl_ilg,
	luma_opcdHandl_ilg,
	luma_opcdHandl_ilg,
	luma_opcdHandl_ilg,
	luma_opcdHandl_ilg,
	luma_opcdHandl_ilg,
	luma_opcdHandl_ilg,
	luma_opcdHandl_ilg,
	luma_opcdHandl_ilg,
	luma_opcdHandl_ilg,
	luma_opcdHandl_ilg,
	luma_opcdHandl_ilg,
	luma_opcdHandl_ilg,
	luma_opcdHandl_ilg,
	luma_opcdHandl_ilg,
	luma_opcdHandl_ilg,
	luma_opcdHandl_ilg,
	luma_opcdHandl_ilg,
	luma_opcdHandl_ilg,
	luma_opcdHandl_ilg,
	luma_opcdHandl_ilg,
	luma_opcdHandl_ilg,
	luma_opcdHandl_ilg,
	luma_opcdHandl_ilg,
	luma_opcdHandl_ilg,
	luma_opcdHandl_ilg,
	luma_opcdHandl_ilg,
	luma_opcdHandl_ilg,
	luma_opcdHandl_ilg,
	luma_opcdHandl_ilg,
	luma_opcdHandl_ilg,
	luma_opcdHandl_ilg,
	luma_opcdHandl_ilg,
	luma_opcdHandl_ilg,
	luma_opcdHandl_ilg,
	luma_opcdHandl_ilg,
	luma_opcdHandl_ilg,
	luma_opcdHandl_ilg,
	luma_opcdHandl_ilg,
	luma_opcdHandl_ilg,
	luma_opcdHandl_ilg,
	luma_opcdHandl_ilg,
	luma_opcdHandl_ilg,
	luma_opcdHandl_ilg,
	luma_opcdHandl_ilg,
	luma_opcdHandl_ilg,
	luma_opcdHandl_ilg,
	luma_opcdHandl_ilg,
	luma_opcdHandl_ilg,
	luma_opcdHandl_ilg,
	luma_opcdHandl_ilg,
	luma_opcdHandl_ilg,
	luma_opcdHandl_ilg,
	luma_opcdHandl_ilg,
	luma_opcdHandl_ilg,
	luma_opcdHandl_ilg,
	luma_opcdHandl_ilg,
	luma_opcdHandl_ilg,
	luma_opcdHandl_ilg,
	luma_opcdHandl_ilg,
	luma_opcdHandl_ilg,
	luma_opcdHandl_ilg,
	luma_opcdHandl_ilg,
	luma_opcdHandl_ilg,
	luma_opcdHandl_ilg,
	luma_opcdHandl_ilg,
	luma_opcdHandl_ilg,
	luma_opcdHandl_ilg,
	luma_opcdHandl_ilg,
	luma_opcdHandl_ilg,
	luma_opcdHandl_ilg,
	luma_opcdHandl_ilg,
	luma_opcdHandl_ilg,
	luma_opcdHandl_ilg,
	luma_opcdHandl_ilg,
	luma_opcdHandl_ilg,
	luma_opcdHandl_ilg,
	luma_opcdHandl_ilg,
	luma_opcdHandl_ilg,
	luma_opcdHandl_ilg,
	luma_opcdHandl_ilg,
	luma_opcdHandl_ilg,
	luma_opcdHandl_ilg,
	luma_opcdHandl_ilg,
	luma_opcdHandl_ilg,
	luma_opcdHandl_ilg,
	luma_opcdHandl_ilg,
	luma_opcdHandl_ilg,
	luma_opcdHandl_ilg,
	luma_opcdHandl_ilg,
	luma_opcdHandl_ilg,
	luma_opcdHandl_ilg,
	luma_opcdHandl_ilg,
	luma_opcdHandl_ilg,
	luma_opcdHandl_ilg,
	luma_opcdHandl_ilg,
	luma_opcdHandl_ilg,
	luma_opcdHandl_ilg,
	luma_opcdHandl_ilg,
	luma_opcdHandl_ilg,
	luma_opcdHandl_ilg,
	luma_opcdHandl_ilg,
	luma_opcdHandl_ilg,
	luma_opcdHandl_ilg,
	luma_opcdHandl_ilg,
	luma_opcdHandl_ilg,
	luma_opcdHandl_ilg,
	luma_opcdHandl_ilg,
	luma_opcdHandl_ilg,
	luma_opcdHandl_ilg,
	luma_opcdHandl_ilg,
};

void luma_proc() {
	luma_opcdHandls[luma_mem[luma_instrPtr]]();
	luma_instrPtr += 0x1;
}

static void luma_opcdHandl_ilg(void) {
	fprintf(stderr,"ILG-%" PRIX8 " @%" PRIX16 "\n",luma_mem[luma_instrPtr],luma_instrPtr);
}

static void luma_opcdHandl_ign(void) {}

static void luma_opcdHandl_cpy(void) {
	luma_ptr const dest = luma_getDbl(luma_instrPtr + 0x1);
	luma_ptr const src  = luma_getDbl(luma_instrPtr + 0x3);
	luma_log("CPY %" PRIX16 " %" PRIX16 "\n",src,dest);
	luma_setByte(dest,luma_mem[src]);
	luma_instrPtr += 0x4;
}

static void luma_opcdHandl_set(void) {
	luma_ptr const  dest = luma_getDbl(luma_instrPtr + 0x1);
	luma_byte const val  = luma_mem[luma_instrPtr + 0x3];
	luma_log("SET %" PRIX16 " %" PRIX8 "\n",dest,val);
	luma_setByte(dest,val);
	luma_instrPtr += 0x3;
}

static void luma_opcdHandl_inc(void) {
	luma_ptr const addr = luma_getDbl(luma_instrPtr + 0x1);
	luma_log("INC %" PRIX16 "\n",addr);
	luma_setByte(addr,luma_mem[addr] + 0x1);
	luma_instrPtr += 0x2;
}

static void luma_opcdHandl_dec(void) {
	luma_ptr const addr = luma_getDbl(luma_instrPtr + 0x1);
	luma_log("DEC %" PRIX16 "\n",addr);
	luma_setByte(addr,luma_mem[addr] - 0x1);
	luma_instrPtr += 0x2;
}

static void luma_opcdHandl_jmp(void) {
	luma_ptr const dest = luma_getDbl(luma_instrPtr + 0x1);
	luma_log("JMP  %" PRIX16 "\n",dest);
	luma_instrPtr = dest;
}

static void luma_opcdHandl_die(void) {
	luma_log("DIE @%" PRIX16 "\n",luma_instrPtr);
	luma_dead = true;
}

static void luma_opcdHandl_bnk(void) {
	luma_byte const banknum = luma_mem[luma_instrPtr + 0x1];
	luma_ldBank(banknum);
	luma_instrPtr += 0x1;
}

static void luma_opcdHandl_trp(void) {
	luma_log("TRP @%" PRIX16 "\n",luma_instrPtr);
	luma_memDump();
	for (;;) {}
}

static void luma_opcdHandl_cmp(void) {
	luma_byte const lval = luma_mem[luma_getDbl(luma_instrPtr + 0x1)];
	luma_byte const rval = luma_mem[luma_getDbl(luma_instrPtr + 0x3)];
	luma_log("CMP %" PRIX8 " %" PRIX8 ": ",lval,rval);
	if (lval < rval) {
		luma_result = 0x0;
	}
	else if (lval > rval) {
		luma_result = 0x2;
	}
	else {
		luma_result = 0x1;
	}
	luma_log("%" PRIX8 "\n",luma_result);
	luma_instrPtr += 0x4;
}

static void luma_opcdHandl_jeq(void) {

}

static void luma_opcdHandl_jlt(void) {

}

static void luma_opcdHandl_jgt(void) {

}

static void luma_opcdHandl_jle(void) {

}

static void luma_opcdHandl_jge(void) {

}

static void luma_opcdHandl_icd(void) {

}

static void luma_opcdHandl_dcd(void) {

}

static void luma_opcdHandl_cpd(void) {
	luma_ptr const lval = luma_mem[luma_instrPtr + 0x1];
	luma_ptr const rval = luma_mem[luma_instrPtr + 0x2];
	luma_log("CPD %" PRIX16 " %" PRIX16 ": ",lval,rval);
	if (lval < rval) {
		luma_result = 0x0;
	}
	else if (lval > rval) {
		luma_result = 0x2;
	}
	else {
		luma_result = 0x1;
	}
	luma_log("%" PRIX8 "\n",luma_result);
	luma_instrPtr += 0x2;
}

