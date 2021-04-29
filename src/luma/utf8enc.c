# include <luma/utf8enc.h>
# include <stdint.h>
# include <stdio.h>
# include <stdlib.h>
uint8_t const * luma_utf8enc(uint32_t * codeps) {
	size_t sz    = (size_t){0x0}; // Size of input array (bytes).
	size_t outsz = (size_t){0x0}; // Size of output array /bytes).
	for(size_t n = (size_t){0x0};;n += (size_t){0x1}) { // First pass: get size of input array, and determine size of output array.
		uint32_t codep = codeps[n]; // Current Unicode codepoint.
		if(codep == (uint32_t){0x0}) { // U+0000 is Null.
			sz = n;
			break;
		}
		if(codep > 0x10FFFF) { // Codepoint out of range.
			return NULL;
		}
		if(codep > 0xFFFF) { // 4 bytes.
			outsz += (size_t){0x2};
			continue;
		}
		if(codep > 0x7FF) { // 3 bytes.
			outsz += (size_t){0x3};
			continue;
		}
		if(codep > 0x7F) { // 2 bytes.
			outsz += (size_t){0x2};
			continue;
		}
		// 1 byte.
		outsz += (size_t){0x1};
	}
	outsz += (size_t){0x1}; // Add space for null-terminator.
	printf("There are %zu element(s).\n",sz);
	printf("The output will have %zu element(s).\n",outsz);
	uint8_t * outstr              = malloc(outsz); // Allocate space for output array.
	outstr[outsz - (size_t){0x1}] = (uint8_t){0x0}; // Create null-terminator on output array.
	size_t outn = (size_t){0x0}; // Keep track of position in output array.
	for(size_t n = (size_t){0x0};n < sz;n += (size_t){0x1}) {
		uint32_t codep = codeps[n]; // Current Unicode codepoint.
		if(codep > 0xFFFF) {
			outstr[outn] = (uint8_t){0x3F};
			outn += (size_t){0x1};
			continue;
		}
		if(codep > 0x7FF) {
			outstr[outn] = (uint8_t){0x3F};
			outn += (size_t){0x1};
			continue;
		}
		if(codep > 0x7F) {
			outstr[outn] = (uint8_t){0xC0 + (codep >> 0x6)};
			outn += (size_t){0x1};
			outstr[outn] = (uint8_t){0x80 + ((uint8_t){codep << 0x2} >> 0x2)};
			outn += (size_t){0x1};
			continue;
		}
		outstr[outn] = codep;
		outn += (size_t){0x1};
	}
	return (uint8_t const *){outstr};
}
