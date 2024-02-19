# counts_merge
PyO3 project to compare speed and implementation of sorted  counts files merging in python and rust

## Example use cases

### Multi-column merging and aggregating of two sorted tables
Let's say we have kept track of the hours logged by users using two different applications
and we're interested in knowing the total number of hours logged by each user on each day,
but only for the days that the user logged hours in both applications.

Example date sorted table `application_A.tsv`:

| date |  user |  hours_logged |
| ---- | ----- | --------------|
| 2020-01-01 |  user1 |  3.5 |
| 2020-01-02 |  user1 |  4.5 |
| 2020-01-02 |  user2 |  3   |

Example date sorted table `application_B.tsv`:

| date |  user |  hours_logged |
| ---- | ----- | --------------|
| 2020-01-01 |  user1 |  3.0 |
| 2020-01-01 |  user2 |  2.0 |
| 2020-01-02 |  user1 |  4.0 |
| 2020-01-02 |  user2 |  3.5 |

The desired merged table would be:

| date |  user |  hours_logged |
| ---- | ----- | --------------|
| 2020-01-01 |  user1 |  6.5 |
| 2020-01-02 |  user1 |  8.5 |
| 2020-01-02 |  user2 |  6.5 |

We need to join on two fields, but the `join` coreutils command only supports
joining on a single field. Instead we can first use `awk` to create a single key
to join on, and then use `join` to merge the files.
```bash
join -j1 \
    <(awk '{print $1"-"$2,$1,$2,$3}' < application_A.tsv) \
    <(awk '{print $1"-"$2,$3}' < application_B.tsv) \
    | awk 'BEGIN{OFS="\t"} {print $2,$3,$4+$5}'
```

More simply, we can use idiomatic `awk` two-file processing to do the same thing:
```bash
awk 'NR==FNR {a[$1$2]=$3; next} $1$2 in a {print $1,$2,$3+a[$1$2]}' \
    application_A.tsv application_B.tsv
```

Or we can use `counts_merge`:
```bash
counts_merge \
    -k 1,eq 2,eq \
    -m all \
    -r 1,2,sum(3) \
    application_A.tsv application_B.tsv
```

### Merging BED files sorted by chrom, start, end
This functionality is already covered by the `bedtools` suite
of software. However, it is just an abstraction of the general
merging that is possible with the `counts_merge` tool.

Example sorted BED file `source_a.bed`:
| #chrom  |  start |   end  | count  |
| ------- | ------ | ------ | -------|
| chr1    |  100   |   150  | 3      |
| chr1    |  120   |   150  | 5      |
| chr2    |  570   |   600  | 1      |
| chrX    | 1430   |  1450  | 8      |

Example sorted BED file `source_b.bed`:
| #chrom  |  start |   end  | count  |
| ------- | ------ | ------ | -------|
| chr1    |   80   |   110  | 2      |
| chr2    |  370   |   600  | 4      |
| chrX    | 1420   |  1490  | 9      |

There are multiple examples of different ways that we might want to merge these files.
For example, we might want to merge the files by taking the maximum count for each
overlapping region, and we might want to report the extended regions from the intersection
rather than the original start/end coordinates from each file. We might also want to
only report regions with overlaps from all input files. The merged file would then be:

| #chrom  |  start |   end  | count  |
| ------- | ------ | ------ | -------|
| chr1    |   80   |   150  | 3      |
| chr2    |  370   |   600  | 4      |
| chrX    | 1420   |  1490  | 9      |

To do this using `bedtools` and `awk` we could use the following command:
```bash
bedtools intersect -wa -wb -a source_a.bed -b source_b.bed | \
    awk 'BEGIN{OFS="\t"}{print $1,($2<$6)?$2:$6,($3>$7)?$3:$7,($4>$8)?$4:$8}' \
    > merged_max_outer_bedtools_awk.bed
```

or using `counts_merge`:
```bash
counts_merge \
    -k 1,eq 2<>3 \
    -r 1,min(2),max(3),max(4) \
    source_a.bed source_b.bed > merged_max_outer_counts_merge.bed
```

Or similarly we might want to merge the files by taking the sum of the counts, but
only reporting the shared parts of the regions that overlap

| #chrom  |  start |   end  | count  |
| ------- | ------ | ------ | -------|
| chr1    |  100   |   110  | 5      |
| chr2    |  570   |   600  | 5      |
| chrX    | 1430   |  1450  | 17     |

```bash
bedtools intersect -wa -wb -a source_a.bed -b source_b.bed | \
    awk 'BEGIN{OFS="\t"}{print $1,($2>$6)?$2:$6,($3<$7)?$3:$7,$4+$8}' \
    > merged_sum_inner_bedtools_awk.bed
```

```bash
counts_merge \
    -k 1,eq 2<>3 \
    -r 1,max(2),min(3),sum(4) \
    source_a.bed source_b.bed > merged.bed
```
