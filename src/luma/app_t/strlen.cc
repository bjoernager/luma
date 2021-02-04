# include <luma/main.hh>
int luma::app_t::strlen(char const * str) noexcept {
	int len = 0x0;
	while(str[len] != '\0') {
		++len;
	}
	return len;
}
