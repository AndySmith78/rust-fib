fn main() {

  generate_sequence();
  
}

fn generate_sequence() {  
  let mut almost_last_num = 0i32;
  let mut last_num = 1i32;  

  println!("{}", almost_last_num);
  println!("{}", last_num);
  

  for x in 0..10 {
    let next_num = last_num + almost_last_num;

    println!("{}", next_num);
    almost_last_num = last_num;
    last_num = next_num;
  }

}

