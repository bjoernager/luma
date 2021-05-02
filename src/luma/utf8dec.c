/*
	Copyright 2021 Gabriel Jensen

	This file is part of luma.

	luma is free software: you can redistribute it and/or modify it under the terms of the GNU Affero General Public License as published by the Free Software Foundation, either version 3 of the License, or	(at your option) any later version.

	luma is distributed in the hope that it will be useful,	but WITHOUT ANY WARRANTY; without even the implied warranty of MERCHANTABILITY or FITNESS FOR A PARTICULAR PURPOSE.

	See the	GNU Affero General Public License for more details.

	You should have received a copy of the GNU Affero General Public License along with luma.

	If not, see <https://www.gnu.org/licenses/>.
*/
# include <luma/utf8dec.h>
# include <stdint.h>
# include <stdio.h>
# include <stdlib.h>
int luma_utf8dec(uint8_t * utf,uint32_t * * codeps,size_t * outszptr) {
	size_t sz    = (size_t){0x0};
	size_t outsz = (size_t){0x0};
	for(size_t n = (size_t){0x0};;n += (size_t){0x1}) { // First pass: get size of input array and determine size of output array.
		if(utf[n] == (uint8_t){0x0}) { // Null-terminator.
			sz = n;
			break;
		}
		if(utf[n] >= (uint8_t){0xF0}) { // Four byte.
			outsz += (size_t){0x4};
			n     += (size_t){0x3};
			continue;
		}
		if(utf[n] >= (uint8_t){0xE0}) { // Three bytes.
			outsz += (size_t){0x3};
			n     += (size_t){0x2};
			continue;
		}
		if(utf[n] >= (uint8_t){0xC0}) { // Two bytes.
			outsz += (size_t){0x2};
			n     += (size_t){0x1};
			continue;
		}
		if(utf[n] >= (uint8_t){0x80}) { // One byte.
			outsz += (size_t){0x1};
			continue;
		}
		// Out of range.
		return 0x1;
	}
	outsz += (size_t){0x1}; // Reserve space for null-terminator.
	if(outszptr != NULL) {
		*outszptr = outsz;
	}
	*codeps                          = malloc(outsz);
	(*codeps)[outsz - (size_t){0x1}] = (uint32_t){0x0}; // Create null-terminator on output array.
	for(size_t n = (size_t){0x0}, outn = (size_t){0x0};n < sz;n += (size_t){0x1},outn += (size_t){0x1}) { // Second pass: decode UTF-8.
		uint8_t chr = utf[n];
		if(chr >= (uint8_t){0xF7}) { // Out of range.
			return 0x1;
		}
		if(chr >= (uint8_t){0xF0}) { // Four byte.
			uint32_t codep  =  (uint32_t){(chr ^ 0xF0) << 0x12};
			n               += (size_t){0x1};
			chr             =  utf[n];
			codep           += (uint32_t){(chr ^ 0x80) << 0xC};
			n               += (size_t){0x1};
			chr             =  utf[n];
			codep           += (uint32_t){(chr ^ 0x80) << 0x6};
			n               += (size_t){0x1};
			chr             =  utf[n];
			codep           += (uint32_t){(chr ^ 0x80)};
			(*codeps)[outn] =  codep;
			continue;
		}
		if(chr >= (uint8_t){0xE0}) { // Three bytes.
			uint32_t codep  =  (uint32_t){(chr ^ 0xE0) << 0xC};
			n               += (size_t){0x1};
			chr             =  utf[n];
			codep           += (uint32_t){(chr ^ 0x80) << 0x6};
			n               += (size_t){0x1};
			chr             =  utf[n];
			codep           += (uint32_t){(chr ^ 0x80)};
			n               += (size_t){0x1};
			(*codeps)[outn] =  codep;
			continue;
		}
		if(chr >= (uint8_t){0xC0}) { // Two bytes.
			uint32_t codep  =  (uint32_t){(chr ^ 0xC0) << 0x6};
			n               += (size_t){0x1};
			chr             =  utf[n];
			codep           += (uint32_t){(chr ^ 0x80)};
			n               += (size_t){0x1};
			(*codeps)[outn] =  codep;
			continue;
		}
		if(chr > (uint8_t){0x7F}) { // One byte.
			uint32_t codep  = (uint32_t){chr};
			(*codeps)[outn] = codep;
			continue;
		}
		// Out of range.
		return 0x1;
	}
	return 0x0;
}
