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

fn string_to_number(input: String) -> isize {
  let input_num: Result<isize, _> = input.trim().parse();
   
  match input_num {
        Ok(num) => num,
        Err(_) => 0,
  }
}
fn generate_sequence(input: isize) {  
  let mut almost_last_num: isize = 0;
  let mut last_num: isize = 1;  

  println!("{}", almost_last_num);
  println!("{}", last_num);
  
  fib_loop(input, last_num, almost_last_num);
}

fn fib_loop(mut input: isize, mut last_num: isize, mut almost_last_num: isize) {
  for x in 0..input {
    let next_num = last_num + almost_last_num;

    println!("{}", next_num);
    almost_last_num = last_num;
    last_num = next_num;
  }
}
