CXX=clang++
CXXFLAGS=-Iinclude -D_ATFILE_SOURCE -D_FORTIFY_SOURCE=2 -D_LARGEFILE_SOURCE -D_LARGEFILE64_SOURCE -D_ISOC99_SOURCE -D_ISOC11_SOURCE -D_ISOC2X_SOURCE -D_POSIX_C_SOURCE=200809L -D_XOPEN_SOURCE -D_XOPEN_SOURCE_EXTENDED -D__STDC_WANT_IEC_60559_BFP_EXT__ -D__STDC_WANT_IEC_60559_FUNCS_EXT__ -D__STDC_WANT_IEC_60559_TYPES_EXT__ -D__STDC_WANT_LIB_EXT2__=0x1
ifneq ($(DEBUG),1)
CXXFLAGS += -DNDEBUG
endif
ifeq ($(LUMA__X),1)
CXXFLAGS += -DLUMA__X=true
endif
CXXFLAGS += -std=c++20 -Wall -Wextra -Wpedantic
CXXFLAGS += -march=native -mtune=native -O3
LDFLAGS = -lgmp -lmpfr -lpthread -lvulkan -lwayland-client -lxcb
HDRS_CXX = \
	include/luma/main.hh
SRCS_CXX = \
	src/luma/app_t/app_t.cc \
	src/luma/app_t/archstr.cc \
	src/luma/app_t/arghandl.cc \
	src/luma/app_t/dbgmsgf.cc \
	src/luma/app_t/exit.cc \
	src/luma/app_t/getenv.cc \
	src/luma/app_t/msgf.cc \
	src/luma/app_t/msgferr.cc \
	src/luma/app_t/msgfout.cc \
	src/luma/app_t/kernelstr.cc \
	src/luma/app_t/strcmp.cc \
	src/luma/app_t/strcut.cc \
	src/luma/app_t/strlen.cc \
	src/main.cc
SRCS=$(SRCS_CXX)
OBJS=$(SRCS:.cc=.o)
luma.elf: $(OBJS)
	$(CXX) $(LDFLAGS) -o $@ $(OBJS)
$(OBJS): $(HDRS_CXX) $(SRCS_CXX)
.PHONY: clean
clean:
	rm $(OBJS)
