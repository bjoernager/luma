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

#include <stdio.h>

void luma_loadRom(char const * const restrict _file,luma_byte const _banknum) {
	FILE * file = fopen(_file,"r");
	void * const buf = luma_mem + (_banknum == 0x0 ? 0x0 : 0x2000);
	fread(buf,sizeof (luma_byte),0x4000,file);
	fclose(file);
}
