# if !defined(LUMA__HEADER__MAIN)
# define LUMA__HEADER__MAIN
# include <vector>
// constexpr -> const -> normal
// typedef -> enum class -> class -> operator -> variable -> function
namespace luma {
	typedef decltype(nullptr) nullptr_t; // Official way to define nullptr_t
	enum class arch_t {
		aarch64,
		amd64,
		ia64,
		ppc64,
		unknown,
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
	class app_t {
	public:
		app_t(int const argc, char const * * argv);
		~app_t();
	private:
		bool constexpr static debug =
# if defined(NDEBUG)
		false;
# else
		true;
# endif
		luma::arch_t constexpr static arch = luma::arch_t::
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
		luma::kernel_t constexpr static kernel = luma::kernel_t::
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
		int               stderr = 0x0;
		int               stdout = 0x0;
		char const *      archstr(luma::arch_t arch) noexcept;
		char const *      getenv(char const * envvar);
		char const *      kernelstr(luma::kernel_t kernel) noexcept;
		char const *      strcut(char const * str,int pos,int len);
		int               strcmp(char const * lstr,char const * rstr) noexcept;
		int               strlen(char const * str) noexcept;
		void              arghandl(char const * arg);
		void              dbgmsgf(char const * msg);
		[[noreturn]] void exit() noexcept;
		//template<typename T,typename ... Args>
		//void              msgf(char const * msg, Args const & ... args);
		void              msgf(int pipe,char const * buf);
		void              msgferr(char const * buf);
		void              msgfout(char const * buf);
	};
}
# endif
