# include <cstdlib>
# include <iostream>
# include <luma/main.hh>
# include <string>
# include <unistd.h>
luma::dat_t luma::dat;
int main(int argc, char * * argv) {
	if(argc < 0x2) {
		std::cout << "Missing argument \"file\".\n";
		exit(EXIT_FAILURE);
	}
	if((access(argv[0x1], F_OK) == 0)) {
		luma::stdlibsock::gfx::crtwin("luma test"s,0x0,0x0,0x400,0x300,false);
	}
	else {
		std::cout << "The file doesn't exist.\n";
		exit(EXIT_FAILURE);
	}
	exit(EXIT_SUCCESS);
}
