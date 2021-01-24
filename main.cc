# include <fcntl.h>
# include <luma/main.hh>
# include <unistd.h>
luma::dat_t luma::dat;
int main(int argc, char * * argv) {
	if(argc < 0x2) {
		luma::msgerr("Missing argument \"file\".\n");
		::_exit(0x1);
	}
	luma::msgout(luma::archstr(luma::arch));
	luma::msgout(luma::kernelstr(luma::kernel));
	if(!::access(argv[0x1],R_OK)) {
	 	int lumafile = ::open(argv[0x1],O_RDONLY);
		/*
		char16_t const * alphtokens {
			u'\u0061', // a
			u'\u0062', // b
			u'\u0063', // c
			u'\u0064', // d
			u'\u0065', // e
			u'\u0066', // f
			u'\u0067', // g
			u'\u0068', // h
			u'\u0069', // i
			u'\u006a', // j
			u'\u006b', // k
			u'\u006c', // l
			u'\u006d', // m
			u'\u006e', // n
			u'\u006f', // o
			u'\u0070', // p
			u'\u0071', // q
			u'\u0072', // r
			u'\u0073', // s
			u'\u0074', // t
			u'\u0075', // u
			u'\u0076', // v
			u'\u0077', // w
			u'\u0078', // x
			u'\u0079', // y
			u'\u007a'  // z
		};
		char16_t const * alphtokens {
			u'\u0028', // (
			u'\u0029', // )
			u'\u002b', // +
			u'\u00d7', // ×
			u'\u00f7', // ÷
			u'\u2044', // ⁄
			u'\u2212', //
			u'\u221a', // −
			u'\u2329', // 〈
			u'\u232a'  // 〉
		};
		char16_t const * alphtokens {
			u'\u0030', // 0
			u'\u0031', // 1
			u'\u0032', // 2
			u'\u0033', // 3
			u'\u0034', // 4
			u'\u0035', // 5
			u'\u0036', // 6
			u'\u0037', // 7
			u'\u0038', // 8
			u'\u0039', // 9
			u'\u218a', // ↊
			u'\u218b'  // ↋
		};
		*/
		while(lumafile) {
			luma::stdlibsock::gfx::crtwin("luma test",0x0,0x0,0x400,0x300,false);
		}
	}
	else {
		luma::msgerr("The file doesn't exist.\n");
		::_exit(0x1);
	}
	::_exit(0x0);
}
