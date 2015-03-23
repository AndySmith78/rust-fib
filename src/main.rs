use std::old_io;

fn main() {
  
  let input = start_fib();
  let real_num = string_to_number(input);
  generate_sequence(real_num); 
}

fn start_fib() -> String {
  let mut reader = std::old_io::stdin();
  println!("How far on the Fibanoci do you want to display: ");
  reader.read_line().ok().expect("Failed to Read")
}

fn string_to_number(input: String) -> i32 {
  let input_num: Result<i32, _> = input.trim().parse();
   
  match input_num {
        Ok(num) => num,
        Err(_) => 0,
  }
}
fn generate_sequence(input: i32) {  
  let almost_last_num: isize = 0;
  let last_num: isize = 1;  

  print_number(1, almost_last_num);
  print_number(2, last_num);
  
  fib_loop(input, last_num, almost_last_num);
}

fn fib_loop(input: i32, mut last_num: isize, mut almost_last_num: isize) {
  for x in 3..input + 1{
    let next_num = last_num + almost_last_num;
    print_number(x, next_num);
    almost_last_num = last_num;
    last_num = next_num;
  }
}

fn print_number(input: i32, fib_number: isize) {
  println!("Digit number {} is: {}", input, fib_number);
}
