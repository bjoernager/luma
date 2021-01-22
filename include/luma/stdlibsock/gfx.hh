# if !defined(LUMA__HEADER__STDLIBSOCK__GFX)
# define LUMA__HEADER__STDLIBSOCK__GFX
# include <cstdint>
# include <string>
namespace luma {
	namespace stdlibsock {
		namespace gfx {
			std::uint8_t crtwin(std::basic_string<char> nm, std::uint16_t pos_x, std::uint16_t pos_y, std::uint16_t res_x, std::uint16_t res_y, bool flscrn);
			void    destwin();
		}
	}
}
# endif
