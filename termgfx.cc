# include <luma/main.hh>
# include <vulkan/vulkan.h>
# include <wayland-client.h>
# include <xcb/xcb.h>
void luma::termgfx() {
	::vkDestroyInstance(luma::dat.vkinst,nullptr);
	if(luma::dat.dispsrvproto == luma::dispsrvproto_t::x) {
		::xcb_disconnect(luma::dat.xconn);
	}
	else if(luma::dat.dispsrvproto == luma::dispsrvproto_t::wayland) {
		::wl_display_disconnect(luma::dat.wldisp);
	}
}
