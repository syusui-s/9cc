MAKEOPTS := -j5
CFLAGS=-Wall -g

9cc: 9cc.c

test: 9cc
	./test.sh

clean:
	rm -f 9cc *.o *~ tmp*

format:
	clang-format 9cc.c
