use std::cmp::Ordering;
use std::io::BufRead;
use std::path::Path;
use std::fs;
use std::io;

#[derive(PartialEq,Debug)]
struct CountData {
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

pub struct CountFile {
    f: io::Lines<io::BufReader<fs::File>>,
    data: CountData,
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
        self.data.chrom.cmp(&other.data.chrom)
            .then_with(|| self.data.pos.cmp(&other.data.pos))
    }
}

impl PartialOrd for CountFile {
    fn partial_cmp(&self, other: &Self) -> Option<Ordering> {
        Some(self.cmp(other))
    }
}

impl PartialEq for CountFile {
    fn eq(&self, other: &Self) -> bool {
        self.data.chrom.eq(&other.data.chrom) && self.data.pos.eq(&other.data.pos)
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


}
