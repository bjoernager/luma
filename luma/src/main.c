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
#include <stdlib.h>
#include <string.h>
#include <sys/random.h>

int main(void) {
	printf("luma %i\n",luma_ver);
	printf("Copyright 2021, 2022 Gabriel Jensen\n\n");
	luma_initDat();
	memset(luma_mem,0x0,0x10000); /* We initialise all of the memory to zero so the behaviour is not UB. */
	if (luma_initWin()) {
		return EXIT_FAILURE;
	}
	luma_ldBootlder();
	luma_log("\nBootstrapping...\n");
	while (!luma_dat.dead) {
		if (luma_checkEvts()) {
			goto stop;
		}
		getrandom(&luma_rnd,sizeof (luma_byte),GRND_RANDOM);
		luma_proc();
	}
	while (!luma_checkEvts()) {}
stop:;
#if !defined(NDEBUG)
	luma_memDump();
#endif
	SDL_DestroyWindow(luma_dat.win);
	SDL_Quit();
}
