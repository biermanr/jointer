from pathlib import Path
import sys


def join(file_path1: Path, file_path2: Path):
    """
    Join two tabular files into one based on the first column.
    The files must be sorted by the first column.

    :param file_path1: path to the first file
    :type file_path1: Path

    :param file_path2: path to the second file
    :type file_path2: Path

    :param out_path: path to the output file
    :type out_path: Path
    """
    with open(file_path1, "r") as f1, open(file_path2, "r") as f2:
        line1 = f1.readline()
        line2 = f2.readline()

        while line1 and line2:
            key1, *rest1 = line1.strip().split("\t")
            key2, *rest2 = line2.strip().split("\t")

            if key1 < key2:
                line1 = f1.readline()
            elif key1 > key2:
                line2 = f2.readline()
            else:
                sys.stdout.write("\t".join([key1] + rest1 + rest2) + "\n")
                line1 = f1.readline()
                line2 = f2.readline()
