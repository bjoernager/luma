# include <locale.h>
# include <luma/arch.h>
# include <luma/print.h>
# include <luma/utf8dec.h>
# include <luma/utf8enc.h>
# include <stdint.h>
# include <stdio.h>
# include <stdlib.h>
int main(void) {
	setlocale(LC_ALL,"en_GB.UTF-8");
	enum luma_arch code[] = {
		luma_arch_lab,
		luma_arch_hello,
	};
	for(size_t i = (size_t){0x0};i < sizeof code / sizeof code[0x0];++i) {
		printf("Got code %d.\n",code[i]);
	}
	uint8_t const * msg = luma_utf8enc((uint32_t[]){0x00A2,0x2C,0x939,0x2C,0x10348,0x2C,0x20AC,0x2C,0x218A,0x2C,0x1F44B,0x0},NULL);
	printf("Array: %s\n",msg);
	free((void *)msg);
	uint32_t * codeps = luma_utf8dec(luma_utf8enc((uint32_t[]){0x1F44B,0x0},NULL),NULL);
	printf("It is %u.\n",codeps[0x0]);
	luma_print((uint32_t[]){0x48,0x65,0x6C,0x6C,0x6F,0x20,0xFFFD,0x65,0x72,0x65,0x21,0x0},(uint32_t){0xF0});
	free((void *)codeps);
	exit(EXIT_SUCCESS);
}
