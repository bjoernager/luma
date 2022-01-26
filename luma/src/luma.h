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

#include <SDL2/SDL.h>
#include <stdbool.h>
#include <stdint.h>
#include <stdio.h>

#define luma_ver 0x1C

typedef uint8_t  luma_byte;
typedef uint16_t luma_dbl;

typedef struct {
	char const *  bootlder;
	char const *  cart;
	bool          dead;
	luma_dbl      instrPtr;
	SDL_Surface * srf;
	SDL_Window *  win;
} luma_dattyp;
extern luma_dattyp luma_dat;

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
#define luma_rnd    (luma_mem[0xE809])

#define luma_getDbl(_addr) ((luma_dbl)(luma_mem[_addr] | ((luma_dbl)(luma_mem[_addr + 0x1]) << 0x8)))

_Noreturn void luma_abrt(      void);
          bool luma_checkEvts( void);
          void luma_drwVram(   void);
          void luma_initDat(   void);
          bool luma_initWin(   void);
          void luma_ldBank(    luma_byte    num);
          void luma_ldBootlder(void);
          void luma_ldRom(     char const * file,luma_byte banknum,luma_dbl addr);
          void luma_memDump(   void);
          void luma_proc(      void);
          void luma_setByte(   luma_dbl     addr,luma_byte val);
          void luma_setDbl(    luma_dbl     addr,luma_dbl  val);

#define luma_noLog true
#if defined(NDEBUG) || luma_noLog
#define luma_log(...)
#else
#define luma_log(...) (fprintf(stderr,__VA_ARGS__))
#endif

#endif
