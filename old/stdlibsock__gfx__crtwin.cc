# include <cstring>
# include <iostream>
# include <luma/main.hh>
# include <string>
# include <unistd.h>
# include <vulkan/vulkan.h>
# include <wayland-client.h>
# include <xcb/xcb.h>
# include <xcb/xcb_atom.h>
int luma::app_t::stdlibsock__gfx__crtwin(char const * nm, int pos_x, int pos_y, int res_x, int res_y, bool flscrn) {
	if(!this->gfxisinit) {
		this->initgfx();
	}
	if(flscrn) {
		std::cerr << "Fullscreen is not supported yet!\n";
	}
	if(this->dispsrvproto == luma::dispsrvproto_t::wayland) {
	}
	else if(this->dispsrvproto == luma::dispsrvproto_t::x) {
		this->dbgmsg("Creating X window... ");
		::xcb_create_window(this->xconn,XCB_COPY_FROM_PARENT,this->xwin,this->xscrn->root,pos_y,pos_x,res_x,res_y,0xa,XCB_WINDOW_CLASS_INPUT_OUTPUT,this->xscrn->root_visual,0x0,nullptr);
		::xcb_change_property(this->xconn,XCB_PROP_MODE_REPLACE,this->xwin,XCB_ATOM_WM_NAME,XCB_ATOM_STRING,0x8,nm.size(),nm.c_str());
		::xcb_map_window(this->xconn,this->xwin);
		::xcb_flush(this->xconn);
		this->dbgmsg("O.K.\n");
	}
	::sleep(0x6);
	this->termgfx();
	return 0x0;
}
