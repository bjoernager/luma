# include <cstdint>
# include <iostream>
# include <luma/main.hh>
# include <string>
# include <unistd.h>
# include <wayland-client.h>
uint8_t luma::stdlibsock::gfx::crtwin(std::basic_string<char>,uint16_t pos_x,uint16_t pos_y,uint16_t res_x,uint16_t res_y,bool flscrn) {
	if(flscrn) {
		std::cout << "Fullscreen is not supported yet!\n";
	}
	luma::dat.wldisp = wl_display_connect(NULL);
	if(luma::dat.wldisp == nullptr) {
		std::cout << "Unable to make a Wayland connection!\n";
	}
	else {
		std::cout << "A Wayland connection has been made.\n";
	}
	::sleep(0x6);
	::wl_display_disconnect(luma::dat.wldisp);
	return 0x0;
}
