/*
	Copyright 2021 Gabriel Jensen

	This file is part of luma.

	luma is free software: you can redistribute it and/or modify it under the terms of the GNU Affero General Public License as published by the Free Software Foundation, either version 3 of the License, or	(at your option) any later version.

	luma is distributed in the hope that it will be useful,	but WITHOUT ANY WARRANTY; without even the implied warranty of MERCHANTABILITY or FITNESS FOR A PARTICULAR PURPOSE.

	See the	GNU Affero General Public License for more details.

	You should have received a copy of the GNU Affero General Public License along with luma.

	If not, see <https://www.gnu.org/licenses/>.
*/
# include <luma/utf8enc.h>
# include <stdarg.h>
# include <stdint.h>
# include <stdio.h>
# include <stdlib.h>
void luma_print(uint32_t * msg,...) {
	va_list args;
	va_start(args,msg);
	for(size_t n = (size_t){0x0};;n += (size_t){0x1}) {
		if(msg[n] == (uint32_t){0x0}) {
			fwrite(&(uint8_t){0xA},0x1,0x1,stdout);
			break;
		}
		if(msg[n] == (uint32_t){0xFFFD}) {
			size_t          chrsz = (size_t){0x0};
			uint8_t * chr   = NULL;
			luma_utf8enc((uint32_t[]){va_arg(args,uint32_t),0x0},&chr,&chrsz);
			fwrite(chr,0x1,chrsz - (size_t){0x1},stdout);
			free((void *)chr);
			continue;
		}
		size_t          chrsz = (size_t){0x0};
		uint8_t * chr   = NULL;
		luma_utf8enc((uint32_t[]){msg[n],0x0,0x0},&chr,&chrsz);
		fwrite(chr,0x1,chrsz - (size_t){0x1},stdout);
		free((void *)chr);
	}
	va_end(args);
}
