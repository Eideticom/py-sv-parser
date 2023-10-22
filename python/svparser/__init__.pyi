from typing import List, Optional, Tuple, Iterable


class SyntaxNode:
    location: SyntaxLocation
    type_name: str
    children: List[SyntaxNode]

    def events(self) -> NodeEventIter:
        """Creates an event iterator object.

        This object will iterate through NodeEvent objects.

        Returns:
            The node event list for this syntax node.
        """


class NodeEvent:
    event: str
    node: SyntaxNode


class NodeEventIter:
    events: Iterable[NodeEvent]


class NodeIter:
    nodes: Iterable[SyntaxNode]


class SyntaxTree:
    tree: SyntaxNode

    def get_str(self, node: SyntaxNode) -> Optional[str]:
        """Gets the original string from a node.

        Args:
            node: The node to get the original string from.

        Returns:
            Original string.
        """

    def get_origin(self, node: SyntaxNode) -> Optional[Tuple[str, int]]:
        """Gets the origin for a node.

        Args:
            node: The node to get the origin from.

        Returns:
            Original node string and offset.
        """

    def events(self) -> NodeEventIter:
        """Returns an iterator of events for traversing the tree.

        Returns:
            Iterator of events for traversing the tree
        """


class SyntaxLocation:
    offset: int
    len: int
    line: int


class Define:
    """Used to specify preprocessor defines.
    """

    def __init__(self,
                 identifier: str,
                 arguments: List[Tuple[str, Optional[str]]],
                 text: Optional[DefineText]) -> None:
        """Used to specify preprocessor defines.

        Args:
            identifier: Identifier for the definition.
            arguments: Arguments for the definition.
            text: Parsed text from original script for the definition.
        """


class DefineText:
    """Used for filling in what a preprocessor define expands to.
    """

    def __init__(self,
                 text: str,
                 origin: Tuple[str, int, int]
                 ) -> None:
        """Used for filling in what a preprocessor define expands to.

        Args:
            text: Raw text for the definition.
            origin: Origin from the script for the definition (tuple of script path, beginning and end lines).
        """


def parse_sv(
    path: str,
    pre_defines: dict,
    include_paths: List[str],
    ignore_include: bool,
    allow_incomplete: bool
) -> SyntaxTree:
    """Parse a SystemVerilog file.

    Args:
        path: Path to the SystemVerilog file.
        pre_defines: Pre-define files. Defaults to {}.
        include_paths: File paths for the includes. Defaults to [].
        ignore_include: Ignore includes. Defaults to False.
        allow_incomplete: Allow incomplete source code. Defaults to False.
    """


def parse_sv_str(
    text: str,
    path: str,
    pre_defines: dict,
    include_paths: List[str],
    ignore_include: bool,
    allow_incomplete: bool
) -> SyntaxTree:
    """Parse a SystemVerilog string.

    Args:
        text: Text containing the SystemVerilog script.
        path: Path to the SystemVerilog file.
        pre_defines: Pre-define files. Defaults to {}.
        include_paths: File paths for the includes. Defaults to [].
        ignore_include: Ignore includes. Defaults to False.
        allow_incomplete: Allow incomplete source code. Defaults to False.
    """


def parse_lib(
    path: str,
    pre_defines: dict,
    include_paths: List[str],
    ignore_include: bool,
    allow_incomplete: bool
) -> SyntaxTree:
    """Parse a SystemVerilog library file.

    Args:
        path: Path to the SystemVerilog file.
        pre_defines: Pre-define files. Defaults to {}.
        include_paths: File paths for the includes. Defaults to [].
        ignore_include: Ignore includes. Defaults to False.
        allow_incomplete: Allow incomplete source code. Defaults to False.
    """


def parse_lib_str(
    text: str,
    path: str,
    pre_defines: dict,
    include_paths: List[str],
    ignore_include: bool,
    allow_incomplete: bool
) -> SyntaxTree:
    """Parse a SystemVerilog string library.

    Args:
        text: Text containing the SystemVerilog script library.
        path: Path to the SystemVerilog file.
        pre_defines: Pre-define files. Defaults to {}.
        include_paths: File paths for the includes. Defaults to [].
        ignore_include: Ignore includes. Defaults to False.
        allow_incomplete: Allow incomplete source code. Defaults to False.
    """


def unwrap_node(node: SyntaxNode, *node_types: str):
    """Finds the first node of one of the given types descending from the provided node.

    Args:
        node: The start node from which to unwrap a node.
        *node_types: The node types to search during the unwrap.
    """


def unwrap_locate(node: SyntaxNode):
    """Finds the first locate node in the provided node.

    Note:
        Locate node represents the position of a token.

    Args:
        node: The start node from which to find the locate node.
    """
