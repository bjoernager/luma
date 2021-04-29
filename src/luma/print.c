# include <luma/utf8enc.h>
# include <stdio.h>
# include <string.h>
void luma_print([[maybe_unused]] char * str,...) {
	/*for(size_t n = 0x0;;++n) {
		if(str[n] == 0x0) {
			fwrite(&(char){0xA},0x1,0x1,stdout);
			break;
		}
		if(!strcmp(&str[n],"\uFFFD")) {
			fwrite(&(char){0x20},0x1,0x1,stdout);
			continue;
		}
		fwrite(&str[n],0x1,0x1,stdout);
	}*/
}
