# include <luma/stdlibsock.h>
# include <stdio.h>
# include <stdlib.h>
# include <unistd.h>

int main(int argc, char * * argv) {
	if(argc < 0x2) {
		printf("Missing argument \"file\".\n");
		exit(EXIT_FAILURE);
	}
	if((access(argv[0x1], F_OK) == 0)) {
		printf("The file exists.\n");
		luma__stdlibsock__gfx__crtwin();
	}
	else {
		printf("The file doesn't exist.\n");
		exit(EXIT_FAILURE);
	}
	exit(EXIT_SUCCESS);
}
