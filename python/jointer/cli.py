"""CLI for counts file merging."""

import sys

import click

from . import __version__

CONTEXT_SETTINGS = dict(help_option_names=["-h", "--help"])


@click.command(context_settings=CONTEXT_SETTINGS)
@click.version_option(version=__version__)
@click.option(
    "-1",
    "--first",
    default=1,
    type=int,
    help="Field number to join on from file 1.",
)
@click.option(
    "-2",
    "--second",
    default=1,
    type=int,
    help="Field number to join on from file 2.",
)
@click.argument("fpaths", type=click.Path(exists=True), nargs=2)
def jointer(first, second, fpaths) -> None:
    """Tabular file joining."""
    with open(fpaths[0], "r") as f1, open(fpaths[1], "r") as f2:
        l1 = f1.readline()
        l2 = f2.readline()
        while True:
            if not l1 or not l2:
                break

            l1_fields = l1.strip().split("\t")
            l2_fields = l2.strip().split("\t")

            if l1_fields[first - 1] == l2_fields[second - 1]:
                sys.stdout.write("\t".join(l1_fields + l2_fields[1:]) + "\n")
                l1 = f1.readline()
                l2 = f2.readline()

            elif l1_fields[first - 1] < l2_fields[second - 1]:
                l1 = f1.readline()

            else:
                l2 = f2.readline()
