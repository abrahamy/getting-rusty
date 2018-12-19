pub fn fizz_buzz() {
  for i in 1..101 {
    let t: (bool, bool) = (i % 3 == 0, i % 5 == 0);
    match t  {
      (true, true) => println!("fizzbuzz"),
      (true, false) => println!("fizz"),
      (false, true) => println!("buzz"),
      (false, false) => println!("{}", i),
    }
  }
}