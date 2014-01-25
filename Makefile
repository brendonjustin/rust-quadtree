RUSTFLAGS ?=

RUST_SRC = $(shell find src/. -type f -name '*.rs')

.PHONY: all
all: libquadtree.dummy

libquadtree.dummy: src/quadtree/lib.rs $(RUST_SRC)
	rustpkg build quadtree $(RUSTFLAGS)
	touch $@

compile_demo: src/demo/main.rs libquadtree.dummy
	rustpkg install demo

demo: compile_demo
	./bin/demo

.PHONY: clean
clean:
	rustpkg clean quadtree
	rustpkg clean demo
	rustpkg uninstall demo
	rm -f *.dummy
