# include <luma/utf8dec.h>
# include <stdint.h>
# include <stdio.h>
# include <stdlib.h>
uint32_t * luma_utf8dec(uint8_t const * str,size_t * outszptr) {
	size_t sz    = (size_t){0x0};
	size_t outsz = (size_t){0x0};
	for(size_t n = (size_t){0x0};;n += (size_t){0x1}) { // First pass: get size of input array and determine size of output array.
		uint8_t const utf = str[n];
		if(utf == (uint8_t){0x0}) { // Null-terminator.
			sz = n;
			break;
		}
		if(utf >= (uint8_t){0xF0}) { // Four byte.
			outsz += (size_t){0x4};
			n     += (size_t){0x3};
			continue;
		}
		if(utf >= (uint8_t){0xE0}) { // Three bytes.
			outsz += (size_t){0x3};
			n     += (size_t){0x2};
			continue;
		}
		if(utf >= (uint8_t){0xC0}) { // Two bytes.
			outsz += (size_t){0x2};
			n     += (size_t){0x1};
			continue;
		}
		if(utf >= (uint8_t){0x80}) { // One byte.
			outsz += (size_t){0x1};
			continue;
		}
		// Out of range.
		return NULL;
	}
	outsz += (size_t){0x1}; // Reserve space for null-terminator.
	if(outszptr != NULL) {
		*outszptr = outsz;
	}
	uint32_t * codeps             = malloc(outsz);
	codeps[outsz - (size_t){0x1}] = (uint32_t){0x0}; // Create null-terminator on output array.
	for(size_t n = (size_t){0x0}, outn = (size_t){0x0};n < sz;n += (size_t){0x1},outn += (size_t){0x1}) { // Second pass: decode UTF-8.
		uint8_t utf = str[n];
		if(utf >= (uint8_t){0xF7}) { // Out of range.
			return NULL;
		}
		if(utf >= (uint8_t){0xF0}) { // Four byte.
			uint32_t codep =  (uint32_t){(utf ^ 0xF0) << 0x12};
			n              += (size_t){0x1};
			utf            =  str[n];
			codep          += (uint32_t){(utf ^ 0x80) << 0xC};
			n              += (size_t){0x1};
			utf            =  str[n];
			codep          += (uint32_t){(utf ^ 0x80) << 0x6};
			n              += (size_t){0x1};
			utf            =  str[n];
			codep          += (uint32_t){(utf ^ 0x80)};
			codeps[outn]   =  codep;
			continue;
		}
		if(utf >= (uint8_t){0xE0}) { // Three bytes.
			uint32_t codep =  (uint32_t){(utf ^ 0xE0) << 0xC};
			n              += (size_t){0x1};
			utf            =  str[n];
			codep          += (uint32_t){(utf ^ 0x80) << 0x6};
			n              += (size_t){0x1};
			utf            =  str[n];
			codep          += (uint32_t){(utf ^ 0x80)};
			n              += (size_t){0x1};
			codeps[outn]   =  codep;
			continue;
		}
		if(utf >= (uint8_t){0xC0}) { // Two bytes.
			uint32_t codep =  (uint32_t){(utf ^ 0xC0) << 0x6};
			n              += (size_t){0x1};
			utf            =  str[n];
			codep          += (uint32_t){(utf ^ 0x80)};
			n              += (size_t){0x1};
			codeps[outn]   =  codep;
			continue;
		}
		if(utf > (uint8_t){0x7F}) { // One byte.
			uint32_t codep = (uint32_t){utf};
			codeps[outn]   = codep;
			continue;
		}
		// Out of range.
		return NULL;
	}
	return codeps;
}
