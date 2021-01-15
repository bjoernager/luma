# include <luma/main.h>
# include <stdint.h>
# include <stdio.h>
# include <xcb/xcb.h>
void luma__initx(uint32_t * retval) {
	if(luma__dat.xconn) {
		luma__dat.xconn = xcb_connect(NULL,NULL);
	}
	else {
		printf("luma__crtxconn called with a valid X connection!\n");
		*retval = 0x1;
		return;
	}
}
