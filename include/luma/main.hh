# if !defined(LUMA__HEADER__MAIN)
# if defined(__cplusplus)
# if __cplusplus < 202002L
# error The compiler seems to not have support for C++20 or newer½ (__cplusplus is less than 202002L), which is required to build Luma.
# endif
# else
# error The compiler appears to not support C++ at all (__cplusplus is not defined).
# endif
# define LUMA__HEADER__MAIN
# include <cstdint>
# include <luma/stdlibsock.hh>
# include <luma/stdlibsock/gfx.hh>
# include <string>
# include <wayland-client.h>
using namespace std::literals::string_literals;
namespace luma {
	class dat_t {
	public:
		char *              dispsrv;
		struct wl_display * wldisp;
	};
	luma::dat_t extern dat;
	void               initgfx(uint32_t * retval);
}
# endif
