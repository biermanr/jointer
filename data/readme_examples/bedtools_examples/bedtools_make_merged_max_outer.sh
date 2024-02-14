bedtools intersect -wa -wb -a source_a.bed -b source_b.bed | \
    awk 'BEGIN{OFS="\t"}{print $1,($2<$6)?$2:$6,($3>$7)?$3:$7,($4>$8)?$4:$8}' \
    > merged_max_outer_bedtools_awk.bed

#!TODO implement this functionality after deciding on API
#counts_merge \
#    -k 1,eq 2<>3 \
#    -r 1,min(2),max(3),max(4) \
#    source_a.bed source_b.bed > merged_max_outer_counts_merge.bed

#make sure the files are identical
#diff merged_max_outer_bedtools_awk.bed merged_max_outer_counts_merge.bed
