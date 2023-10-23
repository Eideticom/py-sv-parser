from typing import List
from .bindings import (
    parse_lib, parse_lib_str,
    parse_sv, parse_sv_str
)
from .bindings import SyntaxTree


def parse_sv_file(
    path: str,
    pre_defines: dict = {},
    include_paths: List[str] = [],
    ignore_include: bool = False,
    allow_incomplete: bool = False,
    lib: bool = False
) -> SyntaxTree:
    """Parse a SystemVerilog file.

    Args:
        path: Path to the SystemVerilog file.
        pre_defines: Pre-define files. Defaults to {}.
        include_paths: File paths for the includes. Defaults to [].
        ignore_include: Ignore includes. Defaults to False.
        allow_incomplete: Allow incomplete source code. Defaults to False.
        lib: Whether to parse as a System Verilog library, else as a System Verilog script.
    """
    if lib:
        return parse_lib(path, pre_defines, include_paths, ignore_include, allow_incomplete)
    else:
        return parse_sv(path, pre_defines, include_paths, ignore_include, allow_incomplete)


def parse_sv_text(
    text: str,
    path: str = '',
    pre_defines: dict = {},
    include_paths: List[str] = [],
    ignore_include: bool = False,
    allow_incomplete: bool = False,
    lib: bool = False
) -> SyntaxTree:
    """Parse a SystemVerilog string.

    Args:
        text: Text containing the SystemVerilog script.
        path: Path to the SystemVerilog file.
        pre_defines: Pre-define files. Defaults to {}.
        include_paths: File paths for the includes. Defaults to [].
        ignore_include: Ignore includes. Defaults to False.
        allow_incomplete: Allow incomplete source code. Defaults to False.
        lib: Whether to parse as a System Verilog library, else as a System Verilog script.
    """
    if lib:
        return parse_lib_str(text, path, pre_defines, include_paths, ignore_include, allow_incomplete)
    else:
        return parse_sv_str(text, path, pre_defines, include_paths, ignore_include, allow_incomplete)
