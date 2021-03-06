#![allow(dead_code,unused_imports,unused_mut)]
extern crate regex;
mod lib;
use std::io::Read;
use lib::{fahr_to_celsius, fahr_to_celsius_v2, celsius_to_fahr, fahr_to_celsius_v3,
          celsius_to_fahr_v2, cp_input_to_output, cp_input_to_output_final, count_input_char,
          count_input_line, count_input_whitespace, cp_input_to_output_escape,
          cp_input_to_output_trim, word_count, print_one_word_per_line, count_char,
          count_longest_line, display_and_count_line, line_trim, line_reverse,
          convert_tab_to_space, convert_space_to_tab_and_space, truncate_line, clear_comment};
fn main() {
    let mut buff = String::new();
    ::std::io::stdin()
        .read_to_string(&mut buff)
        .expect("Failed to read from stdin");
    let result = clear_comment(&buff);
    eprintln!("result =\n{:?}", result);
}
fn hellowolrd() {
    println!("Hello, world!");
}
