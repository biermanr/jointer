echo "Merging 4 counts files each with 1M lines"

echo "Python takes:"
time merge-counts use-py \
    --output py_small.counts \
    data/small/*.counts

echo ""

echo "Rust takes:"
