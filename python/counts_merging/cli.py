import click
from . import __version__

from . import py_merge
from . import rust_utils

CONTEXT_SETTINGS = dict(help_option_names=["-h", "--help"])

@click.group(context_settings=CONTEXT_SETTINGS)
@click.version_option(version=__version__)
def merge_counts():
    """
    cli for count-file merging
    """
    pass


@merge_counts.command()
@click.argument(
    "count_paths",
    nargs = -1,
    required=True,
    type=click.Path(exists=True),
)
@click.option(
    "--output",
    help="File path to write merged output",
    required=True,
)    
def use_py(count_paths, output):
    """
    Python implementation of counts file merging.
    """
    py_merge.merge(count_paths, output)


@merge_counts.command()
@click.argument(
    "count_paths",
    nargs = -1,
    required=True,
    type=click.Path(exists=True),
)
@click.option(
    "--output",
    help="File path to write merged output",
    required=True,
)    
def use_rs(count_paths, output):
    """
    Rust implementation of counts file merging.
    """
    rust_utils.rs_merge(count_paths, output)

