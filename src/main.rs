use std::old_io;

fn main() {
  let mut reader = std::old_io::stdin();
  println!("How far on the Fibanoci do you want to display: ");
  let input = reader.read_line().ok().expect("Failed to Read");
  let input_num: Result<isize, _> = input.trim().parse();
  let num = match input_num {
        Ok(n) => n,
        Err(_) => {
            println!("Please input a number!");
            return;
        }
    };
  generate_sequence(num);
  
}




fn generate_sequence(input: isize) {  
  let mut almost_last_num: isize = 0;
  let mut last_num: isize = 1;  

  println!("{}", almost_last_num);
  println!("{}", last_num);
  

  for x in 0..input {
    let next_num = last_num + almost_last_num;

    println!("{}", next_num);
    almost_last_num = last_num;
    last_num = next_num;
  }

}

