all: oak

oak: src/utils.c src/parse.c src/readall.c
	gcc -Wall -o dist/oak src/utils.c src/parse.c src/readall.c -lm