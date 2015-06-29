extern crate regex;

use std::env;
use std::io::*;
use std::process::exit;
use std::str::FromStr;
use regex::Regex;

fn main() {
  let arguments: Vec<_> = env::args().collect();
  let column_numbers = parse_args(arguments);

  handle_input(&column_numbers); 
}

fn parse_args(arguments: Vec<String>) -> Vec<usize> {
  if arguments.len() <= 1 {
    usage_err_exit();
  }
  let mut columns = arguments.clone();
  columns.remove(0);
  let column_numbers: Vec<usize> = columns.iter().map(|x| {
    let col_arg = FromStr::from_str(x);
    if !col_arg.is_ok() {
      usage_err_exit();
    }
    return col_arg.unwrap();
  }).collect();

  return column_numbers;
}

fn usage_err_exit() {
  println!("usage: nth <columns>");
  exit(1);
}

fn handle_input(column_numbers: &Vec<usize>) {
  let splitter = Regex::new(r"\s+").unwrap();

  let mut input = stdin();
  let mut read_buf: String = String::new();
  let mut line = input.read_line(&mut read_buf);

  while line.is_ok() && line.unwrap() > 0 {
    let line_val = read_buf.clone();
    let column_vals: Vec<&str> = splitter.split(&line_val).collect();

    handle_line(column_vals, &column_numbers);

    read_buf = String::new();
    line = input.read_line(&mut read_buf);
  }
}

fn handle_line(column_vals: Vec<&str>, column_numbers: &Vec<usize>) {
  for col_num in column_numbers.iter() {
    if (col_num - 1) >= column_vals.len() { continue; }
    print!("{} ", column_vals[col_num - 1]);
  }
  println!("");
}

