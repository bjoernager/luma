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
	 12 : ADB  : ptr ,ptr   : ADD BYTE
	 13 : ADD  : ptr ,ptr   : ADD DOUBLE
	 14 : SUB  : ptr ,ptr   : SUBTRACT
	 15 : SBD  : ptr ,ptr   : SUBTRACT DOUBLE
	 16 : MUL  : ptr ,ptr   : MULTIPLY
	 17 : MLD  : ptr ,ptr   : MULTIPLY DOUBLE
	 18 : DIV  : ptr ,ptr   : DIVIDE
	 19 : DVD  : ptr ,ptr   : DIVIDE DOUBLE
	 1A : OUT  : byte,ptr   : OUTPUT
	 1B : INP  : byte,ptr   : INPUT
	 1C : DRW  :            : DRAW
	 1D : CPP  : ptr ,byte  : COPY POINTER
	 1E : STP  : ptr ,byte  : SET POINTER
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
static void luma_opcdHandl_drw(void);
static void luma_opcdHandl_cpp(void);
static void luma_opcdHandl_stp(void);

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
	luma_opcdHandl_drw,
	luma_opcdHandl_cpp,
	luma_opcdHandl_stp,
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
	luma_opcdHandls[luma_mem[luma_dat.instrPtr]]();
	luma_dat.instrPtr += 0x1;
}

static void luma_opcdHandl_ilg(void) {
	fprintf(stderr,"! ILG-%" PRIX8 " @%" PRIX16 "\n",luma_mem[luma_dat.instrPtr],luma_dat.instrPtr);
}

static void luma_opcdHandl_ign(void) {}

static void luma_opcdHandl_cpy(void) {
	luma_dbl const dest = luma_getDbl(luma_dat.instrPtr + 0x1);
	luma_dbl const src  = luma_getDbl(luma_dat.instrPtr + 0x3);
	luma_log("CPY @%" PRIX16 " %" PRIX16 " %" PRIX16 "\n",luma_dat.instrPtr,src,dest);
	luma_setByte(dest,luma_mem[src]);
	luma_dat.instrPtr += 0x4;
}

static void luma_opcdHandl_set(void) {
	luma_dbl const  dest = luma_getDbl(luma_dat.instrPtr + 0x1);
	luma_byte const val  = luma_mem[luma_dat.instrPtr + 0x3];
	luma_log("SET @%" PRIX16 " %" PRIX16 " %" PRIX8 "\n",luma_dat.instrPtr,dest,val);
	luma_setByte(dest,val);
	luma_dat.instrPtr += 0x3;
}

static void luma_opcdHandl_inc(void) {
	luma_dbl const  addr = luma_getDbl(luma_dat.instrPtr + 0x1);
	luma_byte const val  = luma_mem[addr];
	luma_log("INC %" PRIX16 "=%" PRIX8 "\n",addr,val);
	luma_setByte(addr,val + 0x1);
	luma_dat.instrPtr += 0x2;
}

static void luma_opcdHandl_dec(void) {
	luma_dbl const  addr = luma_getDbl(luma_dat.instrPtr + 0x1);
	luma_byte const val  = luma_mem[addr];
	luma_log("DEC %" PRIX16 "=%" PRIX8 "\n",addr,val);
	luma_setByte(addr,val - 0x1);
	luma_dat.instrPtr += 0x2;
}

static void luma_opcdHandl_jmp(void) {
	luma_dbl const dest = luma_getDbl(luma_dat.instrPtr + 0x1);
	luma_log("JMP @%" PRIX16 " %" PRIX16 "\n",luma_dat.instrPtr,dest);
	luma_dat.instrPtr = dest - 0x1; /* Compensate for the incremention by luma_proc. */
}

static void luma_opcdHandl_die(void) {
	luma_log("DIE @%" PRIX16 "\n",luma_dat.instrPtr);
	luma_dat.dead = true;
}

static void luma_opcdHandl_bnk(void) {
	luma_byte const banknum = luma_mem[luma_dat.instrPtr + 0x1];
	luma_log("BNK @%" PRIX16 " %" PRIX8 "\n",luma_dat.instrPtr,banknum);
	luma_ldBank(banknum);
	luma_dat.instrPtr += 0x1;
}

static void luma_opcdHandl_trp(void) {
	luma_log("TRP @%" PRIX16 "\n",luma_dat.instrPtr);
	luma_memDump();
	for (;;) {}
}

static void luma_opcdHandl_cmp(void) {
	luma_dbl const  laddr = luma_getDbl(luma_dat.instrPtr + 0x1);
	luma_dbl const  raddr = luma_getDbl(luma_dat.instrPtr + 0x3);
	luma_byte const lval  = luma_mem[laddr];
	luma_byte const rval  = luma_mem[raddr];
	luma_log("CMP @%" PRIX16 " %" PRIX8 "=%" PRIX8 " %" PRIX8 "=%" PRIX8 ": ",luma_dat.instrPtr,laddr,lval,raddr,rval);
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
	luma_dat.instrPtr += 0x4;
}

static void luma_opcdHandl_jeq(void) {
	luma_log("JEQ @%" PRIX16 ": ",luma_dat.instrPtr);
	if (luma_result == 0x1) {
		luma_dbl const dest = luma_getDbl(luma_dat.instrPtr + 0x1);
		luma_log("%" PRIX16 "\n",dest);
		luma_dat.instrPtr = dest - 0x1;
	}
	else {
		luma_log("%" PRIX16 "\n",luma_dat.instrPtr);
		luma_dat.instrPtr += 0x2;
	}
}

static void luma_opcdHandl_jlt(void) {
	luma_log("JLT @%" PRIX16 ": ",luma_dat.instrPtr);
	if (luma_result == 0x0) {
		luma_dbl const dest = luma_getDbl(luma_dat.instrPtr + 0x1);
		luma_log("%" PRIX16 "\n",dest);
		luma_dat.instrPtr = dest - 0x1;
	}
	else {
		luma_log("%" PRIX16 "\n",luma_dat.instrPtr);
		luma_dat.instrPtr += 0x2;
	}
}

static void luma_opcdHandl_jgt(void) {
	luma_log("JGT @%" PRIX16 ": ",luma_dat.instrPtr);
	if (luma_result == 0x2) {
		luma_dbl const dest = luma_getDbl(luma_dat.instrPtr + 0x1);
		luma_log("%" PRIX16 "\n",dest);
		luma_dat.instrPtr = dest - 0x1;
	}
	else {
		luma_log("%" PRIX16 "\n",luma_dat.instrPtr);
		luma_dat.instrPtr += 0x2;
	}
}

static void luma_opcdHandl_jle(void) {
	luma_log("JLE @%" PRIX16 ": ",luma_dat.instrPtr);
	if (luma_result == 0x0 || luma_result == 0x1) {
		luma_dbl const dest = luma_getDbl(luma_dat.instrPtr + 0x1);
		luma_log("%" PRIX16 "\n",dest);
		luma_dat.instrPtr = dest - 0x1;
	}
	else {
		luma_log("%" PRIX16 "\n",luma_dat.instrPtr);
		luma_dat.instrPtr += 0x2;
	}
}

static void luma_opcdHandl_jge(void) {
	luma_log("JGE @%" PRIX16 ": ",luma_dat.instrPtr);
	if (luma_result == 0x2 || luma_result == 0x1) {
		luma_dbl const dest = luma_getDbl(luma_dat.instrPtr + 0x1);
		luma_log("%" PRIX16 "\n",dest);
		luma_dat.instrPtr = dest - 0x1;
	}
	else {
		luma_log("%" PRIX16 "\n",luma_dat.instrPtr);
		luma_dat.instrPtr += 0x2;
	}
}

static void luma_opcdHandl_icd(void) {
	luma_dbl const addr = luma_getDbl(luma_dat.instrPtr + 0x1);
	luma_dbl const val  = luma_getDbl(addr);
	luma_log("ICD @%" PRIX16 " %" PRIX16 "=%" PRIX16 "\n",luma_dat.instrPtr,addr,val);
	luma_setDbl(addr,val + 0x1);
	luma_dat.instrPtr += 0x2;
}

static void luma_opcdHandl_dcd(void) {
	luma_dbl const addr = luma_getDbl(luma_dat.instrPtr + 0x1);
	luma_dbl const val  = luma_getDbl(addr);
	luma_log("DCD @%" PRIX16 " %" PRIX16 "=%" PRIX16 "\n",luma_dat.instrPtr,addr,val);
	luma_setDbl(addr,val - 0x1);
	luma_dat.instrPtr += 0x2;
}

static void luma_opcdHandl_cpd(void) {
	luma_dbl const laddr = luma_getDbl(luma_dat.instrPtr + 0x1);
	luma_dbl const raddr = luma_getDbl(luma_dat.instrPtr + 0x3);
	luma_dbl const lval  = luma_getDbl(laddr);
	luma_dbl const rval  = luma_getDbl(raddr);
	luma_log("CPD @%" PRIX16 " %" PRIX16 "=%" PRIX16 " %" PRIX16 "=%" PRIX16 ": ",luma_dat.instrPtr,laddr,lval,raddr,rval);
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
	luma_dat.instrPtr += 0x4;
}

static void luma_opcdHandl_drw(void) {
	luma_log("DRW @%" PRIX16 "\n",luma_dat.instrPtr);
	luma_drwVram();
}

static void luma_opcdHandl_cpp(void) {
	luma_dbl const  destPtr = luma_getDbl(luma_dat.instrPtr + 0x1);
	luma_dbl const  dest    = luma_getDbl(destPtr);
	luma_dbl const  src     = luma_getDbl(luma_dat.instrPtr + 0x3);
	luma_byte const srcVal  = luma_mem[src];
	luma_log("CPP @%" PRIX16 " %" PRIX16 "=%" PRIX16 " %" PRIX16 "=%" PRIX8 "\n",luma_dat.instrPtr,destPtr,dest,src,srcVal);
	luma_setByte(dest,srcVal);
	luma_dat.instrPtr += 0x4;
}

static void luma_opcdHandl_stp(void) {
	luma_dbl const  destPtr = luma_getDbl(luma_dat.instrPtr + 0x1);
	luma_dbl const  dest    = luma_getDbl(destPtr);
	luma_byte const val     = luma_mem[luma_dat.instrPtr + 0x3];
	luma_log("STP @%" PRIX16 " %" PRIX16 "=%" PRIX16 " %" PRIX8 "\n",luma_dat.instrPtr,destPtr,dest,val);
	luma_setByte(dest,val);
	luma_dat.instrPtr += 0x4;
}
