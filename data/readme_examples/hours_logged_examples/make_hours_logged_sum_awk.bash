awk 'NR==FNR {a[$1$2]=$3; next} $1$2 in a {print $1,$2,$3+a[$1$2]}' \
    application_A.tsv application_B.tsv
