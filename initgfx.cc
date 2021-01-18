# include <luma/main.hh>
# include <stdint.h>
# include <stdio.h>
# if defined(LUMA__USE_X)
# include <wayland-client.h>
# else
# include <xcb/xcb.h>
# endif
void luma::initgfx(uint32_t * retval) {
}
