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

#if !defined(luma_hdr_luma)
#define luma_hdr_luma

#include <stdbool.h>
#include <stdint.h>
#include <stdio.h>

#define luma_ver 0x18

typedef uint8_t  luma_byte;
typedef uint16_t luma_ptr;

extern char const * luma_bootlder;
extern char const * luma_cart;
extern bool         luma_dead;
extern luma_ptr     luma_instrPtr;

extern luma_byte luma_mem[0x10000]; /* 65536-bit address space. */

#define luma_result (luma_mem[0xE800])
#define luma_param0 (luma_mem[0xE801])
#define luma_param1 (luma_mem[0xE802])
#define luma_param2 (luma_mem[0xE803])
#define luma_param3 (luma_mem[0xE804])
#define luma_param4 (luma_mem[0xE805])
#define luma_param5 (luma_mem[0xE806])
#define luma_param6 (luma_mem[0xE807])
#define luma_param7 (luma_mem[0xE808])

#define luma_getDbl(_addr) ((luma_ptr)(luma_mem[_addr] | ((luma_ptr)(luma_mem[_addr + 0x1]) << 0x8)))

_Noreturn void luma_abrt(      void);
          void luma_initMem(   void);
          void luma_ldBank(    luma_byte    num);
          void luma_ldBootlder(void);
          void luma_ldRom(     char const * file,luma_byte banknum,luma_ptr addr);
          void luma_memDump(   void);
          void luma_proc(      void);
          void luma_setByte(   luma_ptr     addr,luma_byte val);
          void luma_setDbl(    luma_ptr     addr,luma_ptr  val);

#if defined(NDEBUG)
#define luma_log(...)
#else
#define luma_log(...) (fprintf(stderr,__VA_ARGS__))
#endif

#endif
