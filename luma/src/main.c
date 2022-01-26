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

#include <signal.h>
#include <stdio.h>
#include <stdlib.h>
#include <string.h>
#include <sys/random.h>
#include <time.h>

static volatile sig_atomic_t luma_hasSig;

static void luma_sigHandl(int const _sig) {
	(void)_sig;
	if (luma_hasSig) {
		quick_exit(EXIT_FAILURE); /* We must call quick_exit, as the normal exit causes UB when invoked from a signal handler. */
	}
	luma_hasSig = 0x1;
}

int main(void) {
	printf("luma %i\n",luma_ver);
	printf("Copyright 2021, 2022 Gabriel Jensen\n\n");
	signal(SIGINT,luma_sigHandl);
	memset(luma_mem,0x0,0x10000); /* We initialise all of the memory to zero so the behaviour is not UB. */
	luma_ldBootlder();
	luma_log("\nBootstrapping...\n");
	struct timespec const sleeptimespec = {.tv_sec = 0x0,.tv_nsec = 0x7D}; /* We want to run at 8Mhz. 0x7D = 125 = 1000000000/8000000 */
	while (!luma_dead) {
		if (luma_hasSig) {
			luma_dead = true;
			break;
		}
		luma_proc();
		getrandom(&luma_rnd,sizeof (luma_byte),GRND_RANDOM);
		nanosleep(&sleeptimespec,NULL);
	}
#if !defined(NDEBUG)
	luma_memDump();
#endif
}
