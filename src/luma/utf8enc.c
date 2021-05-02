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
# include <stdint.h>
# include <stdio.h>
# include <stdlib.h>
int luma_utf8enc(uint32_t * codeps,uint8_t * * utf,size_t * outszptr) {
	size_t sz    = (size_t){0x0}; // Size of input array (bytes).
	size_t outsz = (size_t){0x0}; // Size of output array /bytes).
	for(size_t n = (size_t){0x0};;n += (size_t){0x1}) { // First pass: get size of input array, and determine size of output array.
		uint32_t codep = codeps[n]; // Current Unicode codepoint.
		if(codep == (uint32_t){0x0}) { // U+0000 is Null.
			sz = n;
			break;
		}
		if(codep >= (uint32_t){0x110000}) { // Codepoint out of range.
			return 0x1;
		}
		if(codep >= (uint32_t){0x10000}) { // 4 bytes.
			outsz += (size_t){0x4};
			continue;
		}
		if(codep >= (uint32_t){0x800}) { // 3 bytes.
			outsz += (size_t){0x3};
			continue;
		}
		if(codep >= (uint32_t){0x80}) { // 2 bytes.
			outsz += (size_t){0x2};
			continue;
		}
		// 1 byte.
		outsz += (size_t){0x1};
	}
	outsz += (size_t){0x1}; // Add space for null-terminator.
	if(outszptr != NULL) {
		*outszptr = outsz;
	}
	*utf                        = malloc(outsz); // Allocate space for output array.
	(*utf)[outsz - (size_t){0x1}] = (uint8_t){0x0}; // Create null-terminator on output array.
	for(size_t n = (size_t){0x0}, outn = (size_t){0x0};n < sz;n += (size_t){0x1},outn += (size_t){0x1}) { // Second pass: encode each codepoint into UTF-8.
		if(codeps[n] >= 0x10000) { // Four bytes.
			(*utf)[outn] = (uint8_t){0xF0 + (codeps[n] >> 0x12)};
			outn         += (size_t){0x1};
			(*utf)[outn] =  (uint8_t){0x80 + ((codeps[n] >> 0xC) & 0x3F)};
			outn         += (size_t){0x1};
			(*utf)[outn] =  (uint8_t){0x80 + ((codeps[n] >> 0x6) & 0x3F)};
			outn         += (size_t){0x1};
			(*utf)[outn] =  (uint8_t){0x80 + (codeps[n] & 0x3F)};
			continue;
		}
		if(codeps[n] >= 0x800) { // Three bytes.
			(*utf)[outn] =  (uint8_t){0xE0 + (codeps[n] >> 0xC)};
			outn         += (size_t){0x1};
			(*utf)[outn] =  (uint8_t){0x80 + ((codeps[n] >> 0x6) & 0x3F)};
			outn         += (size_t){0x1};
			(*utf)[outn] =  (uint8_t){0x80 + (codeps[n] & 0x3F)};
			continue;
		}
		if(codeps[n] >= 0x80) { // Two bytes.
			(*utf)[outn] =  (uint8_t){0xC0 + (codeps[n] >> 0x6)};
			outn         += (size_t){0x1};
			(*utf)[outn] =  (uint8_t){0x80 + (codeps[n] & 0x3F)};
			continue;
		}
		// One byte.
		(*utf)[outn] = codeps[n];
	}
	return 0x0;
}
