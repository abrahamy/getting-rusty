pub fn fizz_buzz() {
  for i in 1..101 {
    let moduli: (u8, u8) = (i % 3, i % 5);
    let s = match moduli {
      (0, 0) => String::from("fizzbuzz"),
      (0, _) => String::from("fizz"),
      (_, 0) => String::from("buzz"),
      _ => format!("{}", i)
    };
    
    println!("{}", s)
  }
}