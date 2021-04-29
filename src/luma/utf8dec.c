# include <luma/utf8dec.h>
# include <stdint.h>
# include <stdlib.h>
uint32_t * luma_utf8dec([[maybe_unused]] char const * str) {
	uint32_t * utf = malloc(0x4);
	utf[0x0] = (uint32_t){0x0};
	return utf;
}
