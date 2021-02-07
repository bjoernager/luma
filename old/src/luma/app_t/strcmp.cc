# include <luma/main.hh>
int luma::app_t::strcmp(char const * lstr,char const * rstr) noexcept {
	for(int i = 0x0;;++i) {
		if(lstr[i] != rstr[i]) {
			return lstr[i] < rstr[i] ? -0x1 : 0x1;
		}
		if(lstr[i] == '\0') {
			return 0x0;
		}
	}
}
