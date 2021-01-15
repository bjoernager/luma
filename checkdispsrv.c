# include <luma/main.h>
# include <stdint.h>
# include <stdio.h>
# include <stdlib.h>
# include <string.h>
void luma__checkdispsrv(uint32_t * retval) {
	retval = 0x0;
	char const * sesstype = getenv("XDG_SESSION_TYPE");
	if(strncmp(sesstype,"wayland",0x10)) {
		luma__dat.dispsrv = "wayland";
	}
	else if(strncmp(sesstype,"x11",0x10)) {
		luma__dat.dispsrv = "x";
	}
	else {
		printf("Environment variable $XDG_SESSION_TYPE is \"%s\", which is not understood.\n");
		*retval = 0x1;
	}
}
