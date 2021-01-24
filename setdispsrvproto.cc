# include <luma/main.hh>
void luma::setdispsrvproto() {
	luma::dat.dispsrvproto = luma::dispsrvproto_t::wayland;
# if 0
	char const * envval = luma::getenv("LUMA__DISPSRVPROTO");
	if(envval != "") {
		if(envval == "x") {
			luma::dbgmsg("Setting the display server protocol to X.\n");
			luma::dat.dispsrvproto = luma::dispsrvproto_t::x;
		}
		else if(envval == "wayland") {
			luma::dbgmsg("Setting the display server protocol to Wayland.\n");
			luma::dat.dispsrvproto = luma::dispsrvproto_t::wayland;
		}
		else {
			//std::cerr << "$LUMA__DISPSRVPROTO is set to \"" + envval + "\", which is an unrecognized display server protocol.\n";
		}
	}
	else {
		luma::dbgmsg("Getting current display server protocol.\n");
		std::string xdgsesstype = std::getenv("XDG_SESSION_TYPE");
		if(xdgsesstype == "wayland") {
			luma::dbgmsg("It appears to be Wayland.\n");
			luma::dat.dispsrvproto = luma::dispsrvproto_t::wayland;
		}
		else if(xdgsesstype == "x11") {
			luma::dbgmsg("It appears to be X.\n");
			luma::dat.dispsrvproto = luma::dispsrvproto_t::x;
		}
		else {
			luma::dbgmsg("Error\n");
		}
	}
# endif
}
