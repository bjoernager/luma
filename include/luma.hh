#if !defined(LUMA__HEADER)
# define LUMA__HEADER
class luma {
public:
	[[noreturn]] luma(int const argc,char const * * argv);
	~luma();
private:
	void arghandl(int const argc, char const * * argv);
};
# endif
