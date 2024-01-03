use std::cmp::Ordering;
use std::io::BufRead;
use std::path::Path;
use std::{fs, io, ops, fmt};

#[derive(Eq,PartialEq,Debug,Clone,Copy)]
pub struct CountData {
    chrom: u8, //NOTE handle the chromosomes better! new struct?
    pos: u32,
    ref_base: char,
    alt_base: char,
    ref_count: u16,
    alt_count: u16,
}

impl fmt::Display for CountData {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(f, "{}\t{}\t{}\t{}\t{}\t{}",
               self.chrom, self.pos, self.ref_base, self.alt_base,
               self.ref_count, self.alt_count,
               )
    }
}

impl ops::Add<CountData> for CountData {
    type Output = CountData;

    fn add(self, other: Self) -> Self {
        //Create a new CountData object
        //as the addition of two CountData's
        //they are assumed to be at the same chrom/pos
        CountData {
            chrom: self.chrom,
            pos: self.pos,
            ref_base: self.ref_base, //The ref_base should be the same for self and other
            alt_base: self.alt_base, //NOTE this is wrong, the ALTs might be different
            ref_count: self.ref_count+other.ref_count,
            alt_count: self.alt_count+other.alt_count, //Also wrong
        }
    }
}


impl ops::AddAssign for CountData {
    fn add_assign(&mut self, other: Self) {
        //Increment the count data for self using other
        self.ref_count += other.ref_count; //Also wrong
        self.alt_count += other.alt_count; //Also wrong
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

        CountFile{f, data}
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
            .take(6)
            .collect();

        CountData {
            chrom: parts[0].parse().unwrap(),
            pos: parts[1].parse().unwrap(),
            ref_base: parts[2].chars().next().unwrap(),
            alt_base: parts[3].chars().next().unwrap(),
            ref_count: parts[4].parse().unwrap(),
            alt_count: parts[5].parse().unwrap(),
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
        let line = "22	10513926	A	G	10	2";
        let cf = CountFile::parse_line(line);

        let ground_truth = CountData {
            chrom: 22,
            pos: 10513926,
            ref_base: 'A',
            alt_base: 'G',
            ref_count: 10,
            alt_count: 2,
        };

        assert_eq!(cf,ground_truth);
    }

    #[test]
    fn test_add_count_data() {
        let cd1 = CountData {
            chrom: 22,
            pos: 10513926,
            ref_base: 'A',
            alt_base: '.',
            ref_count: 1,
            alt_count: 0,
        };

        let cd2 = CountData {
            chrom: 22,
            pos: 10513926,
            ref_base: 'A',
            alt_base: '.',
            ref_count: 3,
            alt_count: 9,
        };

        let ground_truth = CountData {
            chrom: 22,
            pos: 10513926,
            ref_base: 'A',
            alt_base: '.',
            ref_count: 4,
            alt_count: 9,
        };

        let s = cd1+cd2;
        assert_eq!(s,ground_truth);
    }

    #[test]
    fn test_iter_sum_count_data() {
        let cd1 = CountData {
            chrom: 22,
            pos: 10513926,
            ref_base: 'A',
            alt_base: '.',
            ref_count: 1,
            alt_count: 0,
        };

        let cd2 = CountData {
            chrom: 22,
            pos: 10513926,
            ref_base: 'A',
            alt_base: '.',
            ref_count: 3,
            alt_count: 9,
        };

        let ground_truth = CountData {
            chrom: 22,
            pos: 10513926,
            ref_base: 'A',
            alt_base: '.',
            ref_count: 4,
            alt_count: 9,
        };

        let cd_sum = cd1 + cd2;
        assert_eq!(cd_sum, ground_truth);
    }

    #[test]
    fn test_string_representation() {
        let cd = CountData {
            chrom: 22,
            pos: 10513926,
            ref_base: 'A',
            alt_base: '.',
            ref_count: 1,
            alt_count: 0,
        };

        assert_eq!(cd.to_string(), "22\t10513926\tA\t.\t1\t0".to_owned())
    }

}
