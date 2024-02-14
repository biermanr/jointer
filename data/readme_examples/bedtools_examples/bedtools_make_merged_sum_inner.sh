bedtools intersect -wa -wb -a source_a.bed -b source_b.bed | \
    awk 'BEGIN{OFS="\t"}{print $1,($2>$6)?$2:$6,($3<$7)?$3:$7,$4+$8}' \
    > merged_sum_inner_bedtools_awk.bed

#!TODO
#counts_merge \
#    -k 1,eq 2<>3 \
#    -r 1,max(2),min(3),sum(4) \
#    source_a.bed source_b.bed > merged_sum_inner_counts_merge.bed

#diff merged_sum_inner_bedtools_awk.bed merged_sum_inner_counts_merge.bed
