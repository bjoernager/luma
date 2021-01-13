CC=clang
CFLAGS+=-Iinclude -std=c17 -Wall -Wextra -Wpedantic -Werror
CFLAGS+=-march=native -O3
LIBS=-lgmp -lmpfr -lOpenGL -lpthread -lX11 -lxcb -lvulkan
HDR=\
	include/luma/stdlibsock/gfx.h \
	include/luma/main.h \
	include/luma/stdlibsock.h
SRC=\
	stdlibsock/gfx/crtwin.c \
	stdlibsock/gfx/destwin.c \
	initx.c \
	main.c
OBJ=$(SRC:.c=.o)
luma.elf: $(OBJ)
	$(CC) $(CCFLAGS) $(LIBS) $(OBJ) -o $@
$(OBJ): $(HDR) $(SRC)
.PHONY: clean
clean:
	rm $(OBJ)
