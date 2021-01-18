# include <cstdint>
# include <iostream>
# include <luma/main.hh>
# include <string>
# include <unistd.h>
# include <vulkan/vulkan.hpp>
# if defined(LUMA__USE_X)
# include <wayland-client.h>
# else
# include <xcb/xcb.h>
# include <xcb/xcb_atom.h>
# endif
uint8_t luma::stdlibsock::gfx::crtwin(std::basic_string<char> nm,uint16_t pos_x,uint16_t pos_y,uint16_t res_x,uint16_t res_y,bool flscrn) {
	if(flscrn) {
		std::cout << "Fullscreen is not supported yet!\n";
	}
# if defined(LUMA__USE_X)
	luma::dat.wldisp = wl_display_connect(NULL);
	if(luma::dat.wldisp == nullptr) {
		std::cout << "Unable to make a Wayland connection!\n";
	}
	else {
		std::cout << "A Wayland connection has been made.\n";
	}
	::sleep(0x6);
	::wl_display_disconnect(luma::dat.wldisp);
# else
	luma::dat.xconn = xcb_connect(NULL,NULL);
	luma::dat.xscrn = xcb_setup_roots_iterator(xcb_get_setup(luma::dat.xconn)).data;
	luma::dat.xwin  = xcb_generate_id(luma::dat.xconn);
	xcb_create_window(luma::dat.xconn,XCB_COPY_FROM_PARENT,luma::dat.xwin,luma::dat.xscrn->root,pos_y,pos_x,res_x,res_y,0xa,XCB_WINDOW_CLASS_INPUT_OUTPUT,luma::dat.xscrn->root_visual,0x0,NULL);
	xcb_change_property(luma::dat.xconn,XCB_PROP_MODE_REPLACE,luma::dat.xwin,XCB_ATOM_WM_NAME,XCB_ATOM_STRING,0x8,nm.size(),nm.c_str());
	xcb_map_window(luma::dat.xconn,luma::dat.xwin);
	xcb_flush(luma::dat.xconn);
	sleep(0x6);
	xcb_disconnect(luma::dat.xconn);
# endif
	return 0x0;
}
