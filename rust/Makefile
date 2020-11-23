all: debug

debug: libpintos.a
	cargo rustc -- -C link-arg= -L.

release: libpintos.a
	cargo rustc --release -- -C link-arg= -L.

libpintos.a:
	make -C ../src/examples libc.a
	cp ../src/examples/libc.a libpintos.a

clean:
	cargo clean

.PHONY: all clean debug release libpintos.a
