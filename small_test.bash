
echo "Maturin developing the python/rust code"
maturin develop --release

echo "Merging 4 counts files each with 1M lines"

echo "Python takes:"
time merge-counts use-py \
    --output py_small.counts \
    data/small/*counts.txt

echo ""


echo "Rust takes:"
time merge-counts use-rs \
    --output rs_small.counts \
    data/small/*counts.txt
