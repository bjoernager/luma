#if !defined(LUMA__HEADER)
# define LUMA__HEADER
# include <string>
using namespace std::literals::string_literals;
class luma {
public:
	[[noreturn]] luma(int const argc,char const * * argv);
	~luma();
private:
	void                    arghandl(int const argc, char const * * argv);
	std::basic_string<char> lumafile = ""s;
};
# endif
