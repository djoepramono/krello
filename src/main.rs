use std::io;

fn main() {
  let mut input = String::new();

  io::stdin().read_line(&mut input).ok().expect("Failed to read line");

  let clean_input = input.trim();
  println!("processing {}", clean_input);
  let output = parse(clean_input);
  println!("result {}", output);

}

fn parse(input: &str) -> bool {
  println!("debug: '{}'", input);
  match input {
    "hey" => true,
    _ => false,
  }
}
