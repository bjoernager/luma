CXX=clang++
CXXFLAGS+=-Iinclude -D_ATFILE_SOURCE -D_FORTIFY_SOURCE=2 -D_LARGEFILE_SOURCE -D_LARGEFILE64_SOURCE -D_ISOC99_SOURCE -D_ISOC11_SOURCE -D_ISOC2X_SOURCE -D_POSIX_C_SOURCE=200809L -D_XOPEN_SOURCE -D_XOPEN_SOURCE_EXTENDED -D__STDC_WANT_IEC_60559_BFP_EXT__ -D__STDC_WANT_IEC_60559_FUNCS_EXT__ -D__STDC_WANT_IEC_60559_TYPES_EXT__ -D__STDC_WANT_LIB_EXT2__=0x1
CXXFLAGS+=-std=c++20 -Wall -Wextra -Wpedantic
CXXFLAGS+=-march=native -O3
LIBS=-lgmp -lmpfr -lpthread -lvulkan -lwayland-client -lxcb
HDRS_CXX=\
	include/luma/stdlibsock/gfx.hh \
	include/luma/main.hh \
	include/luma/stdlibsock.hh
SRCS_CXX=\
	stdlibsock/gfx/crtwin.cc \
	stdlibsock/gfx/destwin.cc \
	initgfx.cc \
	main.cc
SRCS=$(SRCS_CXX)
OBJ=$(SRCS:.cc=.o)
luma.elf: $(OBJ)
	$(CXX) $(LIBS) $(OBJ) -o $@
$(OBJ): $(HDRS_CXX) $(SRCS_CXX)
.PHONY: clean
clean:
	rm $(OBJ)
