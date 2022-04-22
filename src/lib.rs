use std::collections::HashMap;

use handlebars::Handlebars;
use pyo3::prelude::*;
use rayon::iter::{IntoParallelRefIterator, ParallelIterator};
use regex::RegexBuilder;
use serde_json::json;
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
    let mut input_val = fs::read_to_string(file_name).expect("Unable to read file");

    let reg_for_removing_extrat_white_space =
        RegexBuilder::new(r" +").build().expect("Invalid Regex");

    input_val = reg_for_removing_extrat_white_space
        .replace_all(&input_val, " ")
        .to_string();

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
    let mut input_val = fs::read_to_string(file_name).expect("Unable to read file");
    let reg_for_removing_extrat_white_space =
        RegexBuilder::new(r" +").build().expect("Invalid Regex");

    input_val = reg_for_removing_extrat_white_space
        .replace_all(&input_val, " ")
        .to_string();

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

fn search_in_string(
    all_ngram: Vec<String>,
    input_val: &str,
    possible_word: &str,
) -> HashMap<String, usize> {
    let mut temp: Vec<String> = vec![];
    let reg = Handlebars::new();
    for i in all_ngram.iter() {
        temp.push(format!(
            "{}",
            reg.render_template(i, &json!({ "word": possible_word }))
                .expect("unable to build word")
        ));
    }
    let temp_hash = temp
        .par_iter()
        .map(|value| {
            return (value.clone(), find_count_using_regex(&input_val, &value));
        })
        .collect();

    return temp_hash;
}

#[pyfunction]
fn search_from_file_all_possible_combination_rayon(
    all_ngram: Vec<String>,
    file_name: String,
    all_possible_words: Vec<String>,
) -> HashMap<String, HashMap<String, usize>> {
    /*
       1. this first generate all possible words grams which has been passed in formated system
       2. will run in parallel for all combination
       3. return the result
    */

    let mut input_val = fs::read_to_string(file_name).expect("Unable to read file");
    let reg_for_removing_extrat_white_space =
        RegexBuilder::new(r" +").build().expect("Invalid Regex");

    input_val = reg_for_removing_extrat_white_space
        .replace_all(&input_val, " ")
        .to_string();

    // a {} b
    let temp: HashMap<String, HashMap<String, usize>> = all_possible_words
        .par_iter()
        .map(|val| {
            return (
                val.clone(),
                search_in_string(all_ngram.clone(), &input_val, val),
            );
        })
        .collect();
    temp
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
    std::env::set_var("RAYON_NUM_THREADS", "50");
    m.add_function(wrap_pyfunction!(sum_as_string, m)?)?;
    m.add_function(wrap_pyfunction!(search_from_string, m)?)?;
    m.add_function(wrap_pyfunction!(search_from_file, m)?)?;
    m.add_function(wrap_pyfunction!(search_from_file_rayon, m)?)?;
    m.add_function(wrap_pyfunction!(search_from_string_rayon, m)?)?;
    m.add_function(wrap_pyfunction!(
        search_from_file_all_possible_combination_rayon,
        m
    )?)?;
    Ok(())
}
