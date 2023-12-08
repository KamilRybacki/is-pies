use std::env;
use std::fs;

mod tbp;
use tbp::parse::check_if_raw_string_is_a_pies;


fn is_pies(thing: String) -> bool {
  // Check if file exists
  if !fs::metadata(&thing).is_ok() {
    tbp::parse::check_if_raw_string_is_a_pies(thing)
  }
  let contents = std::fs::read_to_string(thing)
    .expect("Something went wrong reading the file");
  // Check if contents are empty
  if contents.len() == 0 {
    return false;
  }
  return true;
}

fn main() {
  let args: Vec<String> = env::args().collect();
  if args.len() < 2 {
    println!("Please provide a filename");
    return;
  }
  let input_thing = args[1];
  let is_the_thing_a_pies = is_pies(input_thing);
  if is_the_thing_a_pies {
    println!("pies");
  } else {
    println!("not pies");
  }
}
