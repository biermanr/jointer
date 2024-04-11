use pyo3::prelude::*;
use std::fs::File;
use std::io::{self, BufRead, Lines, BufReader, Write};
use std::path::Path;

#[pyfunction]
fn join(file_path1: &str, file_path2: &str) {
    inner_join(file_path1, file_path2, io::stdout());
}

fn inner_join(file_path1: &str, file_path2: &str, mut out: impl Write) {
    let mut f1_lines = read_lines(file_path1).expect("Could not read lines from file 1");
    let mut f2_lines = read_lines(file_path2).expect("Could not read lines from file 2");

    let mut f1_line = f1_lines.next().expect("Could not iterate file1").unwrap();
    let mut f2_line = f2_lines.next().expect("Could not iterate file2").unwrap();

    let (mut f1_key,  mut _f1_rest) = f1_line.split_once("\t").expect("Could not split line in f1");
    let (mut f2_key,  mut  f2_rest) = f2_line.split_once("\t").expect("Could not split line in f2");

    loop {

        if f1_key == f2_key {
            writeln!(out,"{}\t{}",f1_line, f2_rest).unwrap();
        }

        //Advance the iterators or break the loop if exhuasted
        let advance_f1 = f1_key <= f2_key;
        let advance_f2 = f2_key <= f1_key;

        if advance_f1 {
            f1_line = match f1_lines.next() {
                Some(Ok(l)) => l,
                _ => break,
            };
            (f1_key, _f1_rest) = f1_line.split_once("\t").expect("Could not split line in f1");
        }

        if advance_f2 {
            f2_line = match f2_lines.next() {
                Some(Ok(l)) => l,
                _ => break,
            };
            (f2_key,  f2_rest) = f2_line.split_once("\t").expect("Could not split line in f2");
        }
    }
}

#[pymodule]
fn rust_utils(_py: Python, m: &PyModule) -> PyResult<()> {
    m.add_function(wrap_pyfunction!(join, m)?)?;
    Ok(())
}

//Buffered read lines copied from "Rust By Example"
fn read_lines<P>(filename: P) -> io::Result<Lines<BufReader<File>>>
where P: AsRef<Path>, {
    let file = File::open(filename)?;
    Ok(BufReader::new(file).lines())
}

#[cfg(test)]
mod tests {
    use super::*;
    use tempfile::NamedTempFile;
    use std::io::Write;

    #[test]
    fn test_country_codes() {
        let mut country_codes_file = NamedTempFile::new().unwrap();
        writeln!(country_codes_file, "country_code\tcountry_name").unwrap();
        writeln!(country_codes_file, "BRA\tBrazil").unwrap();
        writeln!(country_codes_file, "CHN\tChina").unwrap();
        writeln!(country_codes_file, "USA\tUnited-States").unwrap();
        country_codes_file.flush().unwrap();
        let country_codes_path = country_codes_file.path().to_str().unwrap();

        let mut country_populations_file = NamedTempFile::new().unwrap();
        writeln!(country_populations_file, "country_code\tpopulation").unwrap();
        writeln!(country_populations_file, "BRA\t209").unwrap();
        writeln!(country_populations_file, "IND\t1380").unwrap();
        writeln!(country_populations_file, "USA\t328").unwrap();
        country_populations_file.flush().unwrap();
        let country_populations_path = country_populations_file.path().to_str().unwrap();

        let expected_output = "country_code\tcountry_name\tpopulation\nBRA\tBrazil\t209\nUSA\tUnited-States\t328\n";

        let mut captured_stdout = Vec::new();
        inner_join(country_codes_path, country_populations_path, &mut captured_stdout);

        assert_eq!(String::from_utf8(captured_stdout).unwrap(), expected_output);
    }
}
