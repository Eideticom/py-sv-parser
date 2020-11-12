# py-sv-parser

This is a simple set of bindings for the wonderful
[sv-parser](https://github.com/dalance/sv-parser) library. Bindings are
written in Rust using PyO3 to provide the Python interface.

Currently it only supports the "standard" parsing function and operations on
the tree that function generates. It currently iterates over the provided
tree and generates a new tree with all of the data copied out, so it's quite
a bit less performant than pure native Rust, but all the functionality is
there.

The intent is to support 100% of `sv-parser`, and eventually map to the
native library's Rust iterators directly for better performance.

## Installation

```
pip install .
```

I have not yet tested on a machine without the Rust toolchain installed,
so you may have to install that first if you're getting build errors.

## Documentation

The Rust docs aren't particularly helpful, as PyO3 consumes the doc comments
and all structure before it gets to rustdoc, so don't bother building those.
To get some API documentation, install and use `pdoc3`. You must have built
and installed the py_sv_parser package.

```
pip install pdoc3
pdoc3 --html py_sv_parser
```
