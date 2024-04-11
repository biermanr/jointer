join -j1 \
    <(awk '{print $1"-"$2,$1,$2,$3}' < application_A.tsv) \
    <(awk '{print $1"-"$2,$3}' < application_B.tsv) \
    | awk 'BEGIN{OFS="\t"} {print $2,$3,$4+$5}'
