# include <luma/main.hh>
void luma::app_t::arghandl(char const * arg) {
	char const * appinf = "Luma ()";
	if(this->strcmp(arg,"--help")) {
		this->msgfout("\n");
		this->exit();
	}
	else if(this->strcmp(arg,"--version")) {
		this->msgfout("\n");
		this->exit();
	}
}
