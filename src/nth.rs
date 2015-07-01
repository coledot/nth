use std::env;
use std::io::{stderr, stdin, stdout, BufWriter, Error, Write};
use std::process::exit;

fn main() {
  let column_numbers = parse_args();
  if column_numbers.is_empty() {
    usage_err_exit(None);
  }

  if let Err(err) = handle_input(&column_numbers) {
    writeln!(stderr(), "Error when reading from stdin: {}", err).ok();
    exit(1);
  }
}

fn parse_args() -> Vec<usize> {
  env::args().skip(1).map(|x| {
    match x.parse::<isize>() {
      Ok(val) if val < 1 => usage_err_exit(Some("Column number must be a positive integer".to_string())),
      Ok(val)            => val as usize,
      Err(err)           => usage_err_exit(Some(format!("Failed to parse '{}' with error: {}", x, err))),
    }
  }).collect()
}

fn usage_err_exit(msg: Option<String>) -> ! {
  writeln!(stderr(), "usage: nth <column-no> [<column-no> ...]").ok();
  if let Some(msg) = msg {
    writeln!(stderr(), "\n{}", msg).ok();
  }
  exit(1);
}

fn handle_input(column_numbers: &[usize]) -> Result<(), Error> {
  let mut input = stdin();
  let mut out_buf = BufWriter::new(stdout());
  let mut read_buf = String::new();
  let mut write_buf = String::new();

  loop {
    let bytes_read = try!(input.read_line(&mut read_buf));
    if bytes_read == 0 {
      return Ok(())
    }

    parse_line(&mut write_buf, &read_buf, &column_numbers);
    writeln!(out_buf, "{}", write_buf).unwrap();
    read_buf.clear();
    write_buf.clear();
  }
}

fn parse_line(out_buf: &mut String, read_buf: &String, column_numbers: &[usize]) {
  let line_values: Vec<&str> = read_buf.split_whitespace().collect();
  for col_num in column_numbers.iter() {
    if let Some(val) = line_values.get(col_num - 1) {
      out_buf.push_str(val);
      out_buf.push_str(" ");
    }
  }
}

#[test]
fn parse_line_test() {
    let mut input = String::new();
    let mut line = String::new();
    input.push_str("a b  c   d     e");

    parse_line(&mut line, &input, &[1, 3]);
    assert!(line == "a c ");
    line.clear();

    parse_line(&mut line, &input, &[2, 5]);
    assert!(line == "b e ");
    line.clear();

    parse_line(&mut line, &input, &[1, 2, 1]);
    assert!(line == "a b a ");
    line.clear();

    parse_line(&mut line, &input, &[6, 5, 4]);
    assert!(line == "e d ");
    line.clear();
}
