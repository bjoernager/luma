# include <stdint.h>
# include <unistd.h>
# include <xcb/xcb.h>
uint8_t luma__stdlibsock__gfx__crtwin(void) {
	xcb_connection_t * luma__dat__xcbconn;
	xcb_screen_t *     luma__dat__xcbscrn;
	xcb_window_t       luma__dat__xcbwin;
	luma__dat__xcbconn = xcb_connect(NULL,NULL);
	luma__dat__xcbscrn = xcb_setup_roots_iterator(xcb_get_setup(luma__dat__xcbconn)).data;
	luma__dat__xcbwin  = xcb_generate_id(luma__dat__xcbconn);
	xcb_create_window(luma__dat__xcbconn,XCB_COPY_FROM_PARENT,luma__dat__xcbwin,luma__dat__xcbscrn->root,0x0,0x0,0x400,0x300,0xa,XCB_WINDOW_CLASS_INPUT_OUTPUT,luma__dat__xcbscrn->root_visual,0x0,NULL);
	xcb_map_window(luma__dat__xcbconn,luma__dat__xcbwin);
	xcb_flush(luma__dat__xcbconn);
	sleep(0x6);
	xcb_disconnect(luma__dat__xcbconn);
	return 0x0;
}
