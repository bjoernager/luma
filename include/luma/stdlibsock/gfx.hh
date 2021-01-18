# if !defined(LUMA__HEADER__STDLIBSOCK__GFX)
# define LUMA__HEADER__STDLIBSOCK__GFX
# include <cstdint>
# include <string>
namespace luma {
	namespace stdlibsock {
		namespace gfx {
			uint8_t crtwin(std::basic_string<char> nm, uint16_t pos_x, uint16_t pos_y, uint16_t res_x, uint16_t res_y, bool flscrn);
			void    destwin();
		}
	}
}
# endif
