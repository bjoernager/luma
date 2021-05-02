/*
	Copyright 2021 Gabriel Jensen

	This file is part of luma.

	luma is free software: you can redistribute it and/or modify it under the terms of the GNU Affero General Public License as published by the Free Software Foundation, either version 3 of the License, or	(at your option) any later version.

	luma is distributed in the hope that it will be useful,	but WITHOUT ANY WARRANTY; without even the implied warranty of MERCHANTABILITY or FITNESS FOR A PARTICULAR PURPOSE.

	See the	GNU Affero General Public License for more details.

	You should have received a copy of the GNU Affero General Public License along with luma.

	If not, see <https://www.gnu.org/licenses/>.
*/
# include <locale.h>
# include <luma/arch.h>
# include <luma/print.h>
# include <luma/utf8dec.h>
# include <luma/utf8enc.h>
# include <stdint.h>
# include <stdio.h>
# include <stdlib.h>
int main(void) {
	setlocale(LC_ALL,"en_GB.UTF-8");
	enum luma_arch code[] = {
		luma_arch_lab,
		luma_arch_hello,
	};
	for(size_t i = (size_t){0x0};i < sizeof code / sizeof code[0x0];++i) {
		printf("Got code %d.\n",code[i]);
	}
	{
		uint8_t * msg = NULL;
		luma_utf8enc((uint32_t[]){0x00A2,0x2C,0x939,0x2C,0x10348,0x2C,0x20AC,0x2C,0x218A,0x2C,0x1F44B,0x0},&msg,NULL);
		printf("Array: %s\n",msg);
		free((void *){msg});
	}
	{
		uint32_t * codeps = NULL;
		uint8_t *  utf    = NULL;
		luma_utf8enc((uint32_t[]){0x1F44B,0x0},&utf,NULL);
		luma_utf8dec(utf,&codeps,NULL);
		free((void *)utf);
		printf("It is %u.\n",codeps[0x0]);
		free((void *)codeps);
	}
	luma_print((uint32_t[]){0x48,0x65,0x6C,0x6C,0x6F,0x20,0xFFFD,0x65,0x72,0x65,0x21,0x0},(uint32_t){0xF0});
	exit(EXIT_SUCCESS);
}
