# py-sv-parser

This is a simple wrapper for the `sv-parser` library written using Pyo3.

Currently it only supports the "standard" parsing function and operations on the tree
that function generates. It doesn't export exactly the same way as `sv-parser`, because
that library relies heavily on the type system, but it comes close in overall form.

The intent is to support 100% of `sv-parser`.


## Building

*NEW*: You should be able to install this package directly with pip,
now that I have added pyproject.toml

```
pip install .
```

If that fails, use the following instructions:

### Backup instructions

Install cargo and the Rust toolchain with [rustup](https://rustup.rs). Or
another way if you don't like rustup.

Make sure you add the provided line to source cargo to your bashrc or
something equivalent on Linux.

As well, you will need to install `maturin` with `pip install maturin`
in your python environment.

To install directly in your environment, simply run the following command:

```
maturin develop --release
```

To generate wheel packages for distribution please refer to maturin usage info.

## Documentation

To get some API documentation, install and use `pdoc3`. You must have built
and installed the py_sv_parser package.

```
pip install pdoc3
pdoc3 --html py_sv_parser
```
