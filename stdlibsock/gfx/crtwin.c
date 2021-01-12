# include <luma/main.h>
# include <stdint.h>
# include <string.h>
# include <unistd.h>
# include <xcb/xcb.h>
# include <xcb/xcb_atom.h>
uint8_t luma__stdlibsock__gfx__crtwin(char * nm,uint16_t pos_x,uint16_t pos_y,uint16_t res_x,uint16_t res_y,bool flscreen) {
	xcb_connection_t * luma__dat__xcbconn;
	xcb_screen_t *     luma__dat__xcbscrn;
	xcb_window_t       luma__dat__xcbwin;
	luma__dat__xcbconn = xcb_connect(NULL,NULL);
	luma__dat__xcbscrn = xcb_setup_roots_iterator(xcb_get_setup(luma__dat__xcbconn)).data;
	luma__dat__xcbwin  = xcb_generate_id(luma__dat__xcbconn);
	xcb_create_window(luma__dat__xcbconn,XCB_COPY_FROM_PARENT,luma__dat__xcbwin,luma__dat__xcbscrn->root,pos_y,pos_x,res_x,res_y,0xa,XCB_WINDOW_CLASS_INPUT_OUTPUT,luma__dat__xcbscrn->root_visual,0x0,NULL);
	xcb_change_property(luma__dat__xcbconn,XCB_PROP_MODE_REPLACE,luma__dat__xcbwin,XCB_ATOM_WM_NAME,XCB_ATOM_wSTRING,0x8,strlen(nm),nm);
	xcb_map_window(luma__dat__xcbconn,luma__dat__xcbwin);
	xcb_flush(luma__dat__xcbconn);
	sleep(0x6);
	xcb_disconnect(luma__dat__xcbconn);
	return 0x0;
}
