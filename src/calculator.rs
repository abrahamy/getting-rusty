fn div (x: f64, y: f64) -> Result<f64, String> {
  match y {
    0.0 => Err(String::from("DivisionByZero")),
    _ => Ok(x / y)
  }
}

fn mult(x: i64, y: i64) -> i64 {
  x * y
}

fn fac(x: i64) -> i64 {
  match x {
    0 => 1,
    1 => 1,
    n => n * fac(n-1)
  }
}

pub fn run() -> Result<(), String> {
  let matches = clap_app!(Calculator =>
    (version: "1.0")
    (author: "Abraham Yusuf <aaondowasey@gmail.com>")
    (about: "A simple calculator for rust.")
    (@subcommand div =>
      (about: "Divide x by y")
      (@arg x: +required "The numerator")
      (@arg y: +required "The denominator")
    )
    (@subcommand mult =>
      (about: "Multiply x by y")
      (@arg x: +required "The first operand")
      (@arg y: +required "The second operand")
    )
    (@subcommand fac =>
      (about: "Compute the factorial of n")
      (@arg n: +required "The number whose factorial should be computed")
    )
  ).get_matches();
  
  // handle division command
  if let Some(cmd) = matches.subcommand_matches("div") {
    let x = value_t_or_exit!(cmd.value_of("x"), f64);
    let y = value_t_or_exit!(cmd.value_of("y"), f64);
    let rs = div(x, y).expect("Division by zero is not possible.");
    
    println!("{} / {} = {}", x, y, rs);
  }
  
  // handle multiplication command
  if let Some(cmd) = matches.subcommand_matches("mult") {
    let x = value_t_or_exit!(cmd.value_of("x"), i64);
    let y = value_t_or_exit!(cmd.value_of("y"), i64);
    
    println!("{} * {} = {}", x, y, mult(x, y));
  }
  
  // handle factorial command
  if let Some(cmd) = matches.subcommand_matches("fac") {
    let n = value_t_or_exit!(cmd.value_of("n"), i64);
    
    println!("{}! = {}", n, fac(n));
  }

  Ok(())
}