use std::collections::HashMap;

use pyo3::prelude::*;
use rayon::iter::{IntoParallelRefIterator, ParallelIterator};
use regex::RegexBuilder;
use std::fs;

#[pyfunction]
fn search_from_string(
    py: Python<'_>,
    all_ngram: Vec<String>,
    input_val: String,
) -> HashMap<String, usize> {
    let mut temp_hash: HashMap<String, usize> = HashMap::new();

    for i in all_ngram.iter() {
        temp_hash.insert(
            i.to_string(),
            py.allow_threads(|| find_count_using_regex(&input_val, &i)),
        );
    }
    return temp_hash;
}

#[pyfunction]
fn search_from_file(
    py: Python<'_>,
    all_ngram: Vec<String>,
    file_name: String,
) -> HashMap<String, usize> {
    let mut temp_hash: HashMap<String, usize> = HashMap::new();
    let input_val = fs::read_to_string(file_name).expect("Unable to read file");
    for i in all_ngram.iter() {
        temp_hash.insert(
            i.to_string(),
            py.allow_threads(|| find_count_using_regex(&input_val, &i)),
        );
    }
    return temp_hash;
}

#[pyfunction]
fn search_from_file_rayon(all_ngram: Vec<String>, file_name: String) -> HashMap<String, usize> {
    let input_val = fs::read_to_string(file_name).expect("Unable to read file");
    let temp_hash = all_ngram
        .par_iter()
        .map(|value| {
            return (value.clone(), find_count_using_regex(&input_val, &value));
        })
        .collect();
    return temp_hash;
}

#[pyfunction]
fn search_from_string_rayon(all_ngram: Vec<String>, input_val: String) -> HashMap<String, usize> {
    let temp_hash = all_ngram
        .par_iter()
        .map(|value| {
            return (value.clone(), find_count_using_regex(&input_val, &value));
        })
        .collect();
    return temp_hash;
}

/// Formats the sum of two numbers as string.
#[pyfunction]
fn sum_as_string(a: usize, b: usize) -> PyResult<String> {
    Ok((a + b).to_string())
}

fn find_count_using_regex(line: &str, needle: &str) -> usize {
    /*
    this will find words using regex
     */
    let regex_val = format!(r"\b{}\b", needle);
    let re = RegexBuilder::new(&regex_val)
        .case_insensitive(true)
        .build()
        .expect("Invalid Regex");
    let result = re.find_iter(line);
    let count = result.count();
    count
}

/// A Python module implemented in Rust.
#[pymodule]
fn ngram_reader(_py: Python, m: &PyModule) -> PyResult<()> {
    m.add_function(wrap_pyfunction!(sum_as_string, m)?)?;
    m.add_function(wrap_pyfunction!(search_from_string, m)?)?;
    m.add_function(wrap_pyfunction!(search_from_file, m)?)?;
    m.add_function(wrap_pyfunction!(search_from_file_rayon, m)?)?;
    m.add_function(wrap_pyfunction!(search_from_string_rayon, m)?)?;
    Ok(())
}
