# include <cstdint>
# include <cstring>
# include <iostream>
# include <luma/main.hh>
# include <string>
# include <unistd.h>
# include <vulkan/vulkan.h>
# include <wayland-client.h>
# include <xcb/xcb.h>
# include <xcb/xcb_atom.h>
std::uint8_t luma::stdlibsock::gfx::crtwin(std::basic_string<char> nm,std::uint16_t pos_x,std::uint16_t pos_y,std::uint16_t res_x,std::uint16_t res_y,bool flscrn) {
	if(!luma::dat.gfxisinit) {
		luma::initgfx();
	}
	if(flscrn) {
		std::cerr << "Fullscreen is not supported yet!\n";
	}
	if(luma::dat.dispsrvproto == luma::dispsrvproto_t::wayland) {
	}
	else if(luma::dat.dispsrvproto == luma::dispsrvproto_t::x) {
		luma::dbgmsg("Creating X window... ");
		::xcb_create_window(luma::dat.xconn,XCB_COPY_FROM_PARENT,luma::dat.xwin,luma::dat.xscrn->root,pos_y,pos_x,res_x,res_y,0xa,XCB_WINDOW_CLASS_INPUT_OUTPUT,luma::dat.xscrn->root_visual,0x0,nullptr);
		::xcb_change_property(luma::dat.xconn,XCB_PROP_MODE_REPLACE,luma::dat.xwin,XCB_ATOM_WM_NAME,XCB_ATOM_STRING,0x8,nm.size(),nm.c_str());
		::xcb_map_window(luma::dat.xconn,luma::dat.xwin);
		::xcb_flush(luma::dat.xconn);
		luma::dbgmsg("O.K.\n");
	}
	::sleep(0x6);
	luma::termgfx();
	return 0x0;
}
