# include <cstdint>
# include <luma/main.hh>
# include <vulkan/vulkan.h>
# include <wayland-client.h>
# include <xcb/xcb.h>
void luma::termgfx() {
	::vkDestroyInstance(luma::dat.vkinst,nullptr);
	if constexpr(luma::usex == 0x1) {
		::xcb_disconnect(luma::dat.xconn);
	}
	else {
		::wl_display_disconnect(luma::dat.wldisp);
	}
}
