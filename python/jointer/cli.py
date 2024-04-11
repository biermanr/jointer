import click
from . import __version__

from . import jointer
from . import rust_utils  # type: ignore

CONTEXT_SETTINGS = dict(help_option_names=["-h", "--help"])


@click.command(context_settings=CONTEXT_SETTINGS)
@click.version_option(version=__version__)
@click.argument("input_file1", type=click.Path(exists=True))
@click.argument("input_file2", type=click.Path(exists=True))
def jointerPY(input_file1, input_file2):
    """
    Jointer: A tool to flexibly merge sorted tabular files.
    Python implementation.
    """
    jointer.join(input_file1, input_file2)


@click.command(context_settings=CONTEXT_SETTINGS)
@click.version_option(version=__version__)
@click.argument("input_file1", type=click.Path(exists=True))
@click.argument("input_file2", type=click.Path(exists=True))
def jointerRS(input_file1, input_file2):
    """
    Jointer: A tool to flexibly merge sorted tabular files.
    Rust implementation.
    """
    rust_utils.join(input_file1, input_file2)
