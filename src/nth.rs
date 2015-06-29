use std::env;
use std::io::{stdin, Write, stderr, Error};
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
      Ok(val) => val as usize,
      Err(err) => usage_err_exit(Some(format!("Failed to parse '{}' with error: {}", x, err))),
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
  let mut read_buf = String::new();

  loop {
    let bytes_read = try!(input.read_line(&mut read_buf));
    if bytes_read == 0 {
      return Ok(())
    }

      
    handle_line(&read_buf, &column_numbers);
    read_buf.clear();
  }
}

fn handle_line(read_buf: &String, column_numbers: &[usize]) {
  let column_vals: Vec<&str> = read_buf.split_whitespace().collect();
  for col_num in column_numbers.iter() {
    if let Some(val) = column_vals.get(col_num - 1) {
      print!("{} ", val);
    }
  }
  println!("");
}

