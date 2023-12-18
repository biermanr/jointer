use std::cmp::Ordering;
use std::io::BufRead;
use std::path::Path;
use std::{fs, io, ops};

#[derive(Eq,PartialEq,Debug)]
pub struct CountData {
    chrom: String, //NOTE handle the chromosomes better! new struct?
    pos: u32,
    ref_base: char,
    alt_base: char,
    bef_base: char,
    aft_base: char,
    ref_count: u16,
    alt_count: u16,
    ref_indel: u16,
    alt_indel: u16,
    ref_fwd: u16,
    alt_fwd: u16,
    ref_rev: u16,
    alt_rev: u16,
}

impl ops::Add<CountData> for CountData {
    type Output = CountData;

    fn add(self, other: Self) -> Self {
        //Create a new CountData object
        //as the addition of two CountData's
        //they should be at the same chrom/pos
        CountData {
            chrom: self.chrom,
            pos: self.pos,
            ref_base: self.ref_base,
            alt_base: self.alt_base, //NOTE this is wrong
            bef_base: self.bef_base,
            aft_base: self.aft_base,
            ref_count: self.ref_count+other.ref_count,
            alt_count: self.alt_count+other.alt_count,
            ref_indel: self.ref_indel+other.ref_indel,
            alt_indel: self.alt_indel+other.alt_indel,
            ref_fwd: self.ref_fwd+other.ref_fwd,
            alt_fwd: self.alt_fwd+other.alt_fwd,
            ref_rev: self.ref_rev+other.ref_rev,
            alt_rev: self.alt_rev+other.alt_rev,
        }
    }
}


impl ops::AddAssign for CountData {
    fn add_assign(&mut self, other: Self) {
        //Increment the count data for self using other
        self.ref_count += other.ref_count;
        self.alt_count += other.alt_count;
        self.ref_indel += other.ref_indel;
        self.alt_indel += other.alt_indel;
        self.ref_fwd += other.ref_fwd;
        self.alt_fwd += other.alt_fwd;
        self.ref_rev += other.ref_rev;
        self.alt_rev += other.alt_rev;
    }
}


impl Ord for CountData {
    fn cmp(&self, other: &Self) -> Ordering {
        self.chrom.cmp(&other.chrom)
            .then_with(|| self.pos.cmp(&other.pos))
    }
}

impl PartialOrd for CountData {
    fn partial_cmp(&self, other: &Self) -> Option<Ordering> {
        Some(self.cmp(other))
    }
}



pub struct CountFile {
    f: io::Lines<io::BufReader<fs::File>>,
    pub data: CountData,
}

impl CountFile {
    pub fn new(count_path: &str) -> CountFile {
        let mut f = Self::read_lines(count_path).expect(
            "Error, could not read file");

        f.next(); //skip header

        //NOTE handle this better
        let line = f.next().unwrap().expect(
            "Error, could not read line");

        let data = Self::parse_line(&line);

        CountFile{f:f, data:data}
    }

    ///Read the next line and update fields
    ///updates the CountData
    ///returns true if successful, otherwise false
    ///NOTE refactor this to use an Optional return?
    pub fn next(&mut self) -> bool {
        if let Some(Ok(line)) = self.f.next() {
            self.data = Self::parse_line(&line);
            true
        } else {
            false
        }
    }

    ///Parse the count file line into a CountData object
    fn parse_line(line: &str) -> CountData {
        let parts: Vec<&str> = line
            .split('\t')
            .take(14)
            .collect();

        CountData {
            chrom: parts[0].into(),
            pos: parts[1].parse().unwrap(),
            ref_base: parts[2].chars().next().unwrap(),
            alt_base: parts[3].chars().next().unwrap(),
            bef_base: parts[4].chars().next().unwrap(),
            aft_base: parts[5].chars().next().unwrap(),
            ref_count: parts[6].parse().unwrap(),
            alt_count: parts[7].parse().unwrap(),
            ref_indel: parts[8].parse().unwrap(),
            alt_indel: parts[9].parse().unwrap(),
            ref_fwd: parts[10].parse().unwrap(),
            alt_fwd: parts[11].parse().unwrap(),
            ref_rev: parts[12].parse().unwrap(),
            alt_rev: parts[13].parse().unwrap(),
        }
    }

    ///Create buffered line reader of a file
    fn read_lines<P>(fname: P) -> io::Result<io::Lines<io::BufReader<fs::File>>>
    where P: AsRef<Path>, {
        let file = fs::File::open(fname)?;
        Ok(io::BufReader::new(file).lines())
    }

}

impl Ord for CountFile {
    fn cmp(&self, other: &Self) -> Ordering {
        self.data.cmp(&other.data)
    }
}

impl PartialOrd for CountFile {
    fn partial_cmp(&self, other: &Self) -> Option<Ordering> {
        Some(self.cmp(other))
    }
}

impl PartialEq for CountFile {
    fn eq(&self, other: &Self) -> bool {
        self.data.eq(&other.data)
    }
}

impl Eq for CountFile {}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_parse_line() {
        let line = "chr22	10513926	A	.	A	A	1	0	0	0	1	0	0	0	.	.	.	.	.	.	.	.	.	.";
        let cf = CountFile::parse_line(line);

        let ground_truth = CountData {
            chrom: "chr22".into(),
            pos: 10513926,
            ref_base: 'A',
            alt_base: '.',
            bef_base: 'A',
            aft_base: 'A',
            ref_count: 1,
            alt_count: 0,
            ref_indel: 0,
            alt_indel: 0,
            ref_fwd: 1,
            alt_fwd: 0,
            ref_rev: 0,
            alt_rev: 0,
        };

        assert_eq!(cf,ground_truth);
    }

    #[test]
    fn test_add_CountData() {
        let cd1 = CountData {
            chrom: "chr22".into(),
            pos: 10513926,
            ref_base: 'A',
            alt_base: '.',
            bef_base: 'A',
            aft_base: 'A',
            ref_count: 1,
            alt_count: 0,
            ref_indel: 3,
            alt_indel: 5,
            ref_fwd: 1,
            alt_fwd: 7,
            ref_rev: 6,
            alt_rev: 18,
        };

        let cd2 = CountData {
            chrom: "chr22".into(),
            pos: 10513926,
            ref_base: 'A',
            alt_base: '.',
            bef_base: 'A',
            aft_base: 'A',
            ref_count: 3,
            alt_count: 9,
            ref_indel: 14,
            alt_indel: 6,
            ref_fwd: 7,
            alt_fwd: 12,
            ref_rev: 4,
            alt_rev: 3,
        };

        let ground_truth = CountData {
            chrom: "chr22".into(),
            pos: 10513926,
            ref_base: 'A',
            alt_base: '.',
            bef_base: 'A',
            aft_base: 'A',
            ref_count: 4,
            alt_count: 9,
            ref_indel: 17,
            alt_indel: 11,
            ref_fwd: 8,
            alt_fwd: 19,
            ref_rev: 10,
            alt_rev: 21,
        };

        let s = cd1+cd2;
        assert_eq!(s,ground_truth);
    }

    #[test]
    fn test_iter_sum_CountData() {
        let cd1 = CountData {
            chrom: "chr22".into(),
            pos: 10513926,
            ref_base: 'A',
            alt_base: '.',
            bef_base: 'A',
            aft_base: 'A',
            ref_count: 1,
            alt_count: 0,
            ref_indel: 3,
            alt_indel: 5,
            ref_fwd: 1,
            alt_fwd: 7,
            ref_rev: 6,
            alt_rev: 18,
        };

        let cd2 = CountData {
            chrom: "chr22".into(),
            pos: 10513926,
            ref_base: 'A',
            alt_base: '.',
            bef_base: 'A',
            aft_base: 'A',
            ref_count: 3,
            alt_count: 9,
            ref_indel: 14,
            alt_indel: 6,
            ref_fwd: 7,
            alt_fwd: 12,
            ref_rev: 4,
            alt_rev: 3,
        };

        let ground_truth = CountData {
            chrom: "chr22".into(),
            pos: 10513926,
            ref_base: 'A',
            alt_base: '.',
            bef_base: 'A',
            aft_base: 'A',
            ref_count: 4,
            alt_count: 9,
            ref_indel: 17,
            alt_indel: 11,
            ref_fwd: 8,
            alt_fwd: 19,
            ref_rev: 10,
            alt_rev: 21,
        };

        let cd_sum = cd1 + cd2;
        assert_eq!(cd_sum, ground_truth);
    }
}
