# if !defined(LUMA__HEADER__MAIN)
# if (!defined(__STDC_VERSION__) && (__STDC_VERSION__ <= 201710L))
# error The compiler seems to not have support for C17 or newer (__STDC_VERSION__ is less than 201710L), which is required to build Luma.
# endif
# define LUMA__HEADER__MAIN
# define _ATFILE_SOURCE
# define _FORTIFY_SOURCE 2
# define _LARGEFILE_SOURCE
# define _LARGEFILE64_SOURCE
# define _ISOC99_SOURCE
# define _ISOC11_SOURCE
# define _ISOC2X_SOURCE
# define _POSIX_C_SOURCE 200809L
# define _XOPEN_SOURCE
# define _XOPEN_SOURCE_EXTENDED
# define __STDC_WANT_IEC_60559_BFP_EXT__
# define __STDC_WANT_IEC_60559_FUNCS_EXT__
# define __STDC_WANT_IEC_60559_TYPES_EXT__
# define __STDC_WANT_LIB_EXT2__ 0x1
# include <luma/stdlibsock.h>
# include <luma/stdlibsock/gfx.h>
# include <stdint.h>
# include <xcb/xcb.h>
struct luma__dat_t {
	char *             dispsrv;
	xcb_connection_t * xconn;
	xcb_screen_t *     xscrn;
	xcb_window_t       xwin;
};
struct luma__dat_t extern luma__dat;
void luma__initx(uint32_t * retval);
# endif
