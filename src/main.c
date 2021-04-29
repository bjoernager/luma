# include <locale.h>
# include <luma/arch.h>
# include <luma/print.h>
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
	uint8_t const * msg = luma_utf8enc((uint32_t[]){0x00A2,0x2C,0x20Ac,0x2C,0x218A,0x2C,0x1F44B,0x0});
	printf("%u\n",msg[0x0]);
	printf("%u\n",msg[0x1]);
	printf("%s\n",msg);
	//uint32_t * utf = luma_utf8dec(msg);
	free((void *)msg);
	/*for(size_t n = (size_t){0x0};;n += (size_t){0x1}) {
		if(utf[n] == (uint32_t){0x0}) {
			break;
		}
		printf("%d\n",utf[n]);
	}
	free((void *)utf);*/
	//luma_print("Hello world. �👋");
	exit(EXIT_SUCCESS);
}
