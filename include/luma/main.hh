# if __cplusplus < 202002L
# error The compiler seems to not have support for C++20 or newer (__cplusplus is less than 202002L), which is required to build Luma.
# endif
# if !defined(LUMA__HEADER__MAIN)
# define LUMA__HEADER__MAIN
# if !defined(LUMA__)
# if (defined(__DragonFlyBSD__) || defined(__FreeBSD__) || defined(__linux__))
# define LUMA__X false
# else
# define LUMA__X true
# endif
# endif
# include <cstdint>
# include <iostream>
# include <luma/stdlibsock.hh>
# include <luma/stdlibsock/gfx.hh>
# include <string>
# include <vector>
# include <vulkan/vulkan.h>
# include <wayland-client.h>
# include <xcb/xcb.h>
using namespace std::literals::string_literals;
namespace luma {
	bool constexpr debug =
# if defined(NDEBUG)
	false;
# else
	true;
# endif
	bool constexpr usex = LUMA__X;
	void inline dbgmsg(char const * msg) {
		if constexpr(debug) {
			std::cerr << msg;
		}
	}
	void initgfx();
	void termgfx();
	class dat_t {
	private:
		bool                               gfxisinit;
		std::vector<VkExtensionProperties> vkexts;
		std::vector<VkPhysicalDevice>      vkphysdevs;
		::VkApplicationInfo                vkappinf {};
		::VkInstance                       vkinst;
		::VkInstanceCreateInfo             vkinstcrtinf {};
		::VkResult                         vkreslt;
		::wl_buffer *                      wlbuff = nullptr;
		::wl_display *                     wldisp = nullptr;
		::wl_shell_surface *               wlsurf = nullptr;
		::xcb_connection_t *               xconn = nullptr;
		::xcb_screen_t *                   xscrn = nullptr;
		::xcb_window_t                     xwin;
		friend void luma::initgfx();
		friend std::uint8_t luma::stdlibsock::gfx::crtwin(std::basic_string<char> nm,std::uint16_t pos_x,std::uint16_t pos_y,std::uint16_t res_x,std::uint16_t res_y,bool flscrn);
		friend void luma::termgfx();
	} extern dat;
}
# endif
