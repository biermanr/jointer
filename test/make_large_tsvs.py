import sys


def make_large_tsv(num_rows, num_cols):
    num_pad = num_rows // 10
    for i in range(num_rows):
        sys.stdout.write(str(i).zfill(num_pad) + "\t12\n")


if __name__ == "__main__":
    num_rows = 100
    if len(sys.argv) >= 2:
        num_rows = int(sys.argv[1])

    num_cols = 2
    if len(sys.argv) >= 3:
        num_rows = int(sys.argv[2])

    make_large_tsv(num_rows, num_cols)
