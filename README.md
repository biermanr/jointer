# jointer
Command line utility to aid in joining/merging tabular data, extending the functionality of the `join` command and 'two-file' awk idioms.

## Example use cases

### Simple: Mapping country codes to country names
Maybe you have a file with two columns, (1) a 3-letter country code, and
(2) the number of people from that country in millions (I don't know if these
numbers are accurate):

Example file `country_populations.tsv`:
| country_code |  population |
| ------------ | ----------- |
| BRA          |  209        |
| CHN          |  1439       |
| IND          |  1380       |
| USA          |  328        |

then you have another file with two columns, (1) a 3-letter country code, and
(2) the name of the country:

Example file `country_codes.tsv`:
| country_code |  country_name |
| ------------ | ------------- |
| BRA          |  Brazil       |
| CHN          |  China        |
| IND          |  India        |
| USA          |  United States|

Your goal is to merge these two files to get a table with three columns:
(1) the country code, (2) the country name, and (3) the population. You
could achieve this using the `join` command:

```bash
join country_populations.tsv country_codes.tsv
```

This would give you the following merged table:

| country_code |  country_name |  population |
| ------------ | ------------- | ----------- |
| BRA          |  Brazil       |  209        |
| CHN          |  China        |  1439       |
| IND          |  India        |  1380       |
| USA          |  United States|  328        |

You could also use `jointer` to achieve the same result:

```bash
jointer country_populations.tsv country_codes.tsv
```

By default `join` and `jointer` will join on the first field of each file. If the
fields to join on are not the first field, you can specify the fields to join on
using the `-1` and `-2` options. For example, the country code could be the second field in
the `country_codes_col_reorder.tsv` file

| country_name |  country_code |
| ------------ | ------------- |
| Brazil       |  BRA          |
| China        |  CHN          |
| India        |  IND          |
| United States|  USA          |

In which case you would use the following command to perform the join:
```bash
join -1 1 -2 2 country_populations.tsv country_codes_col_reorder.tsv
```
or
```bash
jointer -1 1 -2 2 country_populations.tsv country_codes_col_reorder.tsv
```

If the join column is the 2nd for both files, you can use the `-j` option instead,
even though this is marked as a "compatibility" option that is not recommended for
use in new scripts:
```bash
join -j 2 country_populations_col_reorder.tsv country_codes_col_reorder.tsv
```
```bash
jointer -j 2 country_populations_col_reorder.tsv country_codes_col_reorder.tsv
```

!TODO add in examples from join man page TODO!

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
join \
    <(awk '{print $1"-"$2,$1,$2,$3}' < application_A.tsv) \
    <(awk '{print $1"-"$2,$3}' < application_B.tsv) \
    | awk 'BEGIN{OFS="\t"} {print $2,$3,$4+$5}'
```

Alternatively, we can use idiomatic `awk` two-file processing
(good explanation [here](https://backreference.org/2010/02/10/idiomatic-awk/))
```bash
awk 'NR==FNR {a[$1$2]=$3; next} $1$2 in a {print $1,$2,$3+a[$1$2]}' \
    application_A.tsv application_B.tsv
```

Or we can use `jointer`:
```bash
jointer \
    -j 1,eq 2,eq \
    -m all \
    -c 1,2,sum(3) \
    application_A.tsv application_B.tsv
```

### Merging BED files sorted by chrom, start, end
This functionality is already covered by the `bedtools` suite
of software. However, it is just an abstraction of the general
merging that is possible with the `jointer` tool.

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

or using `jointer`:
```bash
jointer \
    -j 1,eq 2-3,ov \
    -c 1,min(2),max(3),max(4) \
    source_a.bed source_b.bed > merged_max_outer_jointer.bed
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
jointer \
    -j 1,eq 2<>3 \
    -c 1,max(2),min(3),sum(4) \
    source_a.bed source_b.bed > merged.bed
```
