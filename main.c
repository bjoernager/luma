# include <luma/main.h>
# include <stdio.h>
# include <stdlib.h>
# include <unistd.h>
struct luma__dat_t luma__dat;
int main(int argc, char * * argv) {
	if(argc < 0x2) {
		printf("Missing argument \"file\".\n");
		exit(EXIT_FAILURE);
	}
	if((access(argv[0x1], F_OK) == 0)) {
		printf("\f");
		luma__initstdlib__gfx();
		luma__stdlibsock__gfx__crtwin("luma test",0x0,0x0,0x400,0x300,false);
	}
	else {
		printf("The file doesn't exist.\n");
		exit(EXIT_FAILURE);
	}
	exit(EXIT_SUCCESS);
}
