# include <luma/main.h>
# include <stdint.h>
# include <stdio.h>
# include <string.h>
# include <unistd.h>
# include <xcb/xcb.h>
# include <xcb/xcb_atom.h>
uint8_t luma__stdlibsock__gfx__crtwin(char * nm,uint16_t pos_x,uint16_t pos_y,uint16_t res_x,uint16_t res_y,bool flscrn) {
	uint32_t retval = 0x0;
	if(flscrn) {
		printf("Fullscreen is not supported yet!\n");
	}
	luma__crtxconn(&retval);
	luma__dat.xcbscrn = xcb_setup_roots_iterator(xcb_get_setup(luma__dat.xcbconn)).data;
	luma__dat.xcbwin  = xcb_generate_id(luma__dat.xcbconn);
	xcb_create_window(luma__dat.xcbconn,XCB_COPY_FROM_PARENT,luma__dat.xcbwin,luma__dat.xcbscrn->root,pos_y,pos_x,res_x,res_y,0xa,XCB_WINDOW_CLASS_INPUT_OUTPUT,luma__dat.xcbscrn->root_visual,0x0,NULL);
	xcb_change_property(luma__dat.xcbconn,XCB_PROP_MODE_REPLACE,luma__dat.xcbwin,XCB_ATOM_WM_NAME,XCB_ATOM_STRING,0x8,strlen(nm),nm);
	xcb_map_window(luma__dat.xcbconn,luma__dat.xcbwin);
	xcb_flush(luma__dat.xcbconn);
	sleep(0x6);
	xcb_disconnect(luma__dat.xcbconn);
	return 0x0;
}
