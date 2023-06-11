from . import tabler
from typing import Iterable, Any


def from_plaintext(text: Iterable[Iterable[str]]) -> tabler.table.Table:
    """
    Create a table from some plain text.
    """
    tabler.table.Table(
        list(
            map(
                lambda row: list(
                    map(lambda s: tabler.table.Cell(tabler.table.Content(s)), row)
                ),
                text,
            )
        )
    )


def create_from_plaintext(
    header: Iterable[str], content: Iterable[Iterable[str]], splitter: bool
) -> tabler.table.Table:
    """
    Create a table from header and content.
    """
    tabler.table.Table.create(
        header=list(
            map(lambda cell: tabler.table.Cell(tabler.table.Content(cell)), header)
        ),
        content=list(
            map(
                lambda row: list(
                    map(lambda s: tabler.table.Cell(tabler.table.Content(s)), row)
                ),
                content,
            )
        ),
        splitter=splitter,
    )


def auto_cell(v: Any) -> tabler.table.Cell:
    """
    Automatically create a new table cell.
    """
    if v is None:
        return tabler.table.Cell(tabler.table.Content.Empty)
    elif isinstance(v, tabler.table.Cell):
        return v
    elif isinstance(v, tabler.table.Content):
        return tabler.table.Cell(v)
    elif isinstance(v, str):
        return tabler.table.Cell(tabler.table.Content(v))
    else:
        return tabler.table.Cell(tabler.table.Content(str(v)))


def auto_table_from(v: Iterable[Iterable[Any]]) -> tabler.table.Table:
    """
    Create a table with automatical cells.
    """
    return tabler.table.Table(
        list(map(lambda row: list(map(lambda cell: auto_cell(cell), row)), v))
    )


def auto_table_create(
    header: Iterable[Iterable[Any]], content: Iterable[Iterable[Any]], splitter: bool
) -> tabler.table.Table:
    """
    Create a table with automatical cells.
    """
    return tabler.table.Table.create(
        header=list(map(lambda cell: auto_cell(cell), header)),
        content=list(
            map(lambda row: list(map(lambda cell: auto_cell(cell), row)), content)
        ),
        splitter=splitter,
    )
