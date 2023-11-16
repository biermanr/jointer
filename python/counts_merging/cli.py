"""CLI for counts file merging."""
from typing import List

import click

from . import __version__, py_merge, rust_utils

CONTEXT_SETTINGS = dict(help_option_names=["-h", "--help"])


@click.group(context_settings=CONTEXT_SETTINGS)
@click.version_option(version=__version__)
def merge_counts() -> None:
    """CLI for count-file merging."""
    pass


@merge_counts.command()
@click.argument(
    "count_paths",
    nargs=-1,
    required=True,
    type=click.Path(exists=True),
)
@click.option(
    "--output",
    help="File path to write merged output",
    required=True,
)
def use_py(count_paths: List[str], output: str) -> None:
    """Python implementation of counts file merging.

    Args:
        count_paths: list of file paths to merge
        output: file path to write merged output
    """
    py_merge.merge(count_paths, output)


@merge_counts.command()
@click.argument(
    "count_paths",
    nargs=-1,
    required=True,
    type=click.Path(exists=True),
)
@click.option(
    "--output",
    help="File path to write merged output",
    required=True,
)
def use_rs(count_paths: List[str], output: str) -> None:
    """Rust implementation of counts file merging.

    Args:
        count_paths: list of file paths to merge
        output: file path to write merged output
    """
    rust_utils.rs_merge(count_paths, output)
