RUSTFLAGS ?=

RUST_SRC = $(shell find src/. -type f -name '*.rs')

OUTDIR ?= ./build

BINDIR = $(OUTDIR)/bin
LIBDIR = $(OUTDIR)/lib
TMPDIR = $(OUTDIR)/tmp

$(BINDIR) $(LIBDIR) $(TMPDIR):
	mkdir -p '$@'

.PHONY: all
all: $(TMPDIR)/libquadtree.dummy

$(TMPDIR)/libquadtree.dummy: src/quadtree/lib.rs $(RUST_SRC) $(LIBDIR) $(TMPDIR)
	rustc --out-dir '$(LIBDIR)' src/quadtree/lib.rs $(RUSTFLAGS)
	touch $@

compile_demo: src/demo/main.rs $(TMPDIR)/libquadtree.dummy $(BINDIR)
	rustc -o '$(BINDIR)/demo' -L '$(LIBDIR)' src/demo/main.rs

demo: compile_demo
	'$(BINDIR)/demo'

.PHONY: clean
clean:
	rm -f '$(OUTDIR)'
