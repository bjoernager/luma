# if !defined(LUMA__HEADER__STDLIBSOCK__GFX)
# define LUMA__HEADER__STDLIBSOCK__GFX
# include <stdbool.h>
# include <stdint.h>
uint8_t luma__stdlibsock__gfx__crtwin(char * nm, uint16_t pos_x, uint16_t pos_y, uint16_t res_x, uint16_t res_y, bool flscrn);
void    luma__stdlibsock__gfx__destwin(void);
# endif
