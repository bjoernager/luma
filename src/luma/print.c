# include <luma/utf8enc.h>
# include <stdarg.h>
# include <stdint.h>
# include <stdio.h>
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
			uint8_t const * chr   = luma_utf8enc((uint32_t[]){va_arg(args,uint32_t),0x0},&chrsz);
			fwrite(chr,0x1,chrsz - (size_t){0x1},stdout);
			continue;
		}
		size_t          chrsz = (size_t){0x0};
		uint8_t const * chr   = luma_utf8enc((uint32_t[]){msg[n],0x0,0x0},&chrsz);
		fwrite(chr,0x1,chrsz - (size_t){0x1},stdout);
	}
	va_end(args);
}
