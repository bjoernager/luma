# include <luma/main.hh>
void luma::app_t::setdispsrvproto() {
	this->dispsrvproto = this->dispsrvproto_t::wayland;
# if 0
	char const * envval = this->getenv("LUMA__DISPSRVPROTO");
	if(envval != "") {
		if(envval == "x") {
			this->dbgmsg("Setting the display server protocol to X.\n");
			this->dispsrvproto = this->dispsrvproto_t::x;
		}
		else if(envval == "wayland") {
			this->dbgmsg("Setting the display server protocol to Wayland.\n");
			this->dispsrvproto = this->dispsrvproto_t::wayland;
		}
		else {
			//std::cerr << "$LUMA__DISPSRVPROTO is set to \"" + envval + "\", which is an unrecognized display server protocol.\n";
		}
	}
	else {
		this->dbgmsg("Getting current display server protocol.\n");
		std::string xdgsesstype = std::getenv("XDG_SESSION_TYPE");
		if(xdgsesstype == "wayland") {
			this->dbgmsg("It appears to be Wayland.\n");
			this->dispsrvproto = this->dispsrvproto_t::wayland;
		}
		else if(xdgsesstype == "x11") {
			this->dbgmsg("It appears to be X.\n");
			this->dispsrvproto = this->dispsrvproto_t::x;
		}
		else {
			this->dbgmsg("Error\n");
		}
	}
# endif
}
