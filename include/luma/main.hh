# if defined(__cplusplus)
# if __cplusplus < 202002L
# error The compiler seems to not have support for C++20 or newer½ (__cplusplus is less than 202002L), which is required to build Luma.
# endif
# else
# error The compiler appears to not support C++ at all (__cplusplus is not defined).
# endif
# if !defined(LUMA__HEADER__MAIN)
# define LUMA__HEADER__MAIN
//# if defined(__linux__)
# define LUMA__USE_X
//# endif
# include <cstdint>
# include <luma/stdlibsock.hh>
# include <luma/stdlibsock/gfx.hh>
# include <string>
# if defined(LUMA__USE_X)
# include <wayland-client.h>
# else
# include <xcb/xcb.h>
# endif
using namespace std::literals::string_literals;
namespace luma {
	class dat_t {
	public:
		char *             dispsrv;
# if defined(LUMA__USE_X)
		wl_display *       wldisp;
		wl_shell_surface * wlsurf;
# else
		xcb_connection_t * xconn;
		xcb_screen_t *     xscrn;
		xcb_window_t       xwin;
# endif
	};
	luma::dat_t extern dat;
	void               initgfx(uint32_t * retval);
}
# endif
