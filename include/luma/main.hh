# if !defined(LUMA__HEADER__MAIN)
# define LUMA__HEADER__MAIN
# include <fcntl.h>
# include <luma/stdlibsock.hh>
# include <luma/stdlibsock/gfx.hh>
# include <vector>
# include <vulkan/vulkan.h>
# include <wayland-client.h>
# include <xcb/xcb.h>
// enum class -> class -> operator -> constexpr -> function -> variable -> inline function
namespace luma {
	enum class arch_t {
		aarch64,
		amd64,
		ia64,
		ppc64,
		unknown,
	};
	enum class dispsrvproto_t {
		unknown,
		wayland,
		x,
	};
	enum class kernel_t {
		darwinos,
		dragonflybsd,
		freebsd,
		hurd,
		linux,
		minix,
		netbsd,
		openbsd,
		unknown,
	};
	class dat_t {
	public:
		bool                               gfxisinit;
		luma::dispsrvproto_t               dispsrvproto= luma::dispsrvproto_t::wayland;
		std::vector<VkExtensionProperties> vkexts;
		std::vector<VkPhysicalDevice>      vkphysdevs;
		::VkApplicationInfo                vkappinf     {};
		::VkInstance                       vkinst;
		::VkInstanceCreateInfo             vkinstcrtinf {};
		::VkResult                         vkreslt;
		::wl_buffer *                      wlbuff       = nullptr;
		::wl_display *                     wldisp       = nullptr;
		::wl_shell_surface *               wlsurf       = nullptr;
		::xcb_connection_t *               xconn        = nullptr;
		::xcb_screen_t *                   xscrn        = nullptr;
		::xcb_window_t                     xwin;
	} extern dat;
	bool constexpr debug =
# if defined(NDEBUG)
	false;
# else
	true;
# endif
	luma::arch_t constexpr arch = luma::arch_t::
# if defined(__aarch64__)
	aarch64;
# elif (defined(_M_AMD64) || defined(__amd64) || defined(__amd64__) || defined(__x86_64) || defined(x86_64__))
	amd64;
# elif (defined(_IA64) defined(_M_IA64) || defined(__IA64__) || defined(__ia64__) || defined(__itanium__))
	ia64;
# elif (defined(_ARCH_PPC64) || defined(__powerpc64__) || defined(__PPC64__) || defined(__ppc64__))
	ppc64;
# else
	unknown;
# endif
	luma::kernel_t constexpr kernel = luma::kernel_t::
# if defined(__APPLE__)
	darwinos;
# elif defined(__DragonFly__)
	dragonflybsd;
# elif defined(__FreeBSD__)
	freebsd;
# elif (defined(__GNU__) || defined(__gnu_hurd__))
	hurd;
# elif defined(__linux__)
	linux;
# elif defined(__minix)
	minix;
# elif defined(__NetBSD__)
	netbsd;
# elif defined(__OpenBSD__)
	openbsd;
# else
	unknown;
# endif
	char const * archstr(luma::arch_t arch);
	char const * getenv(char const * envvar);
	char const * kernelstr(luma::kernel_t kernel);
	int          strlen(char const * str);
	void         dbgmsg(char const * msg);
	void         initgfx();
	void         msg(int pipe,char const * msg);
	void         msgerr(char const * msg);
	void         msgout(char const * msg);
	void         setdispsrvproto();
	void         termgfx();
}
# endif
