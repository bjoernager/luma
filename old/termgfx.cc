# include <luma/main.hh>
# include <vulkan/vulkan.h>
# include <wayland-client.h>
# include <xcb/xcb.h>
void luma::app_t::termgfx() {
	::vkDestroyInstance(this->vkinst,nullptr);
	if(this->dispsrvproto == this->dispsrvproto_t::x) {
		::xcb_disconnect(this->xconn);
	}
	else if(this->dispsrvproto == this->dispsrvproto_t::wayland) {
		::wl_display_disconnect(this->wldisp);
	}
}
