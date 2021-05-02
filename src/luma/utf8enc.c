# include <luma/utf8enc.h>
# include <stdint.h>
# include <stdio.h>
# include <stdlib.h>
uint8_t const * luma_utf8enc(uint32_t * codeps,size_t * outszptr) {
	size_t sz    = (size_t){0x0}; // Size of input array (bytes).
	size_t outsz = (size_t){0x0}; // Size of output array /bytes).
	for(size_t n = (size_t){0x0};;n += (size_t){0x1}) { // First pass: get size of input array, and determine size of output array.
		uint32_t codep = codeps[n]; // Current Unicode codepoint.
		if(codep == (uint32_t){0x0}) { // U+0000 is Null.
			sz = n;
			break;
		}
		if(codep >= (uint32_t){0x110000}) { // Codepoint out of range.
			return NULL;
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
	uint8_t * str              = malloc(outsz); // Allocate space for output array.
	str[outsz - (size_t){0x1}] = (uint8_t){0x0}; // Create null-terminator on output array.
	for(size_t n = (size_t){0x0}, outn = (size_t){0x0};n < sz;n += (size_t){0x1},outn += (size_t){0x1}) { // Second pass: encode each codepoint into UTF-8.
		uint32_t codep = codeps[n]; // Current Unicode codepoint.
		if(codep >= 0x10000) { // Four bytes.
			str[outn] = (uint8_t){0xF0 + (codep >> 0x12)};
			outn      += (size_t){0x1};
			str[outn] =  (uint8_t){0x80 + ((codep >> 0xC) & 0x3F)};
			outn      += (size_t){0x1};
			str[outn] =  (uint8_t){0x80 + ((codep >> 0x6) & 0x3F)};
			outn      += (size_t){0x1};
			str[outn] =  (uint8_t){0x80 + (codep & 0x3F)};
			continue;
		}
		if(codep >= 0x800) { // Three bytes.
			str[outn] =  (uint8_t){0xE0 + (codep >> 0xC)};
			outn      += (size_t){0x1};
			str[outn] =  (uint8_t){0x80 + ((codep >> 0x6) & 0x3F)};
			outn      += (size_t){0x1};
			str[outn] =  (uint8_t){0x80 + (codep & 0x3F)};
			continue;
		}
		if(codep >= 0x80) { // Two bytes.
			str[outn] =  (uint8_t){0xC0 + (codep >> 0x6)};
			outn      += (size_t){0x1};
			str[outn] =  (uint8_t){0x80 + (codep & 0x3F)};
			continue;
		}
		// One byte.
		str[outn] =  codep;
	}
	return (uint8_t const *){str};
}
