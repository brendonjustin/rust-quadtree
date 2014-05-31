RUSTFLAGS ?=

RUST_SRC = $(shell find src/. -type f -name '*.rs')

OUTDIR ?= ./build

BINDIR = $(OUTDIR)/bin
LIBDIR = $(OUTDIR)/lib
TMPDIR = $(OUTDIR)/tmp

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
	rm -rf '$(OUTDIR)'

$(BINDIR) $(LIBDIR) $(TMPDIR):
	mkdir -p '$@'
