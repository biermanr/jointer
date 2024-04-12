use pyo3::prelude::*;
use std::fs::File;
use std::io::{self, BufRead, BufReader, Write};
use std::path::Path;

#[pyfunction]
fn join(file_path1: &str, file_path2: &str) {
    inner_join(Path::new(file_path1), Path::new(file_path2), io::stdout());
}

fn inner_join(file_path1: &Path, file_path2: &Path, mut out: impl Write) {
    let file1 = File::open(file_path1).expect("Could not open file 1");
    let file2 = File::open(file_path2).expect("Could not open file 2");

    let mut lines1 = BufReader::new(file1).lines().peekable();
    let mut lines2 = BufReader::new(file2).lines().peekable();

    while let (Some(line1), Some(line2)) = (lines1.peek(), lines2.peek()) {
        let (key1, rest1) = line1.as_ref().expect("Error reading line in file 1")
            .split_once("\t").expect("Could not split line in file 1");
        let (key2, rest2) = line2.as_ref().expect("Error reading line in file 2")
            .split_once("\t").expect("Could not split line in file 2");

        if key1 == key2 {
            writeln!(out, "{}\t{}\t{}", key1, rest1, rest2).expect("Error writing output");
            lines1.next();
            lines2.next();
        } else if key1 < key2 {
            lines1.next();
        } else {
            lines2.next();
        }
    }
}

#[pymodule]
fn rust_utils(_py: Python, m: &PyModule) -> PyResult<()> {
    m.add_function(wrap_pyfunction!(join, m)?)?;
    Ok(())
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
        let country_codes_path = country_codes_file.path();

        let mut country_populations_file = NamedTempFile::new().unwrap();
        writeln!(country_populations_file, "country_code\tpopulation").unwrap();
        writeln!(country_populations_file, "BRA\t209").unwrap();
        writeln!(country_populations_file, "IND\t1380").unwrap();
        writeln!(country_populations_file, "USA\t328").unwrap();
        country_populations_file.flush().unwrap();
        let country_populations_path = country_populations_file.path();

        let expected_output = "country_code\tcountry_name\tpopulation\nBRA\tBrazil\t209\nUSA\tUnited-States\t328\n";

        let mut captured_stdout = Vec::new();
        inner_join(country_codes_path, country_populations_path, &mut captured_stdout);

        assert_eq!(String::from_utf8(captured_stdout).unwrap(), expected_output);
    }
}
