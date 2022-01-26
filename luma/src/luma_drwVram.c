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

#include <SDL2/SDL.h>

#include <stdio.h>

void luma_drwVram(void) {
	SDL_Rect rect;
	rect.w = 0x4;
	rect.h = 0x4;
	for (luma_dbl y = 0x0;y < 0x80;y += 0x1) {
		rect.y = y * 0x4;
		for (luma_dbl x = 0x0;x < 0x80;x += 0x1) {
			rect.x = x * 0x4;
			luma_byte const memVal = luma_mem[0x8000 + x + y * 0x80];
			Uint32 const col = (Uint32)((memVal << 0x10) + (memVal << 0x8) + memVal);
			SDL_FillRect(luma_dat.srf,&rect,col);
		}
	}
	SDL_UpdateWindowSurface(luma_dat.win);
}
