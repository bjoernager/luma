CC      = clang
CFLAGS  = -std=c2x -Wall -Wextra -Wpedantic -I include -march=native -mtune=native -O3
ifneq ($(debug),1)
CFLAGS += -DNDEBUG
else
CFLAGS += -g
endif
LDFLAGS =
SRCS = \
	src/luma/print.c \
	src/luma/utf8dec.c \
	src/luma/utf8enc.c \
	src/main.c
HDRS = \
	include/luma/arch.h \
	include/luma/print.h \
	include/luma/utf8dec.h \
	include/luma/utf8enc.h
OBJS = $(SRCS:.c=.o)
luma: $(OBJS)
	$(CC) $(LDFLAGS) $^ -o $@
$(OBJS): $(HDRS)
.PHONY: run
run: luma
	./luma
.PHONY: clean
clean:
	rm $(OBJS)
.PHONT: purge
purge:
	rm luma $(OBJS)
