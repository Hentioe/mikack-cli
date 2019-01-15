PREFIX ?= /usr/local/bin
TARGET ?= debug

.PHONY: all build install uninstall clean
all: build

build:
ifeq (${TARGET}, release)
	cargo build --release
else
	cargo build
endif

install:
	install -d ${DESTDIR}${PREFIX}
	install -m 755 target/${TARGET}/manga ${DESTDIR}${PREFIX}/manga

uninstall:
	rm ${DESTDIR}${PREFIX}/manga

clean:
	cargo clean