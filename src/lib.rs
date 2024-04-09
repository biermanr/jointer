use pyo3::prelude::*;

#[pyfunction]
fn join(file_path1: &str, file_path2: &str, out_path: &str) {
    //NOTE TODO!!
    println!("GOT HERE");
}

#[pymodule]
fn rust_utils(_py: Python, m: &PyModule) -> PyResult<()> {
    m.add_function(wrap_pyfunction!(join, m)?)?;
    Ok(())
}
