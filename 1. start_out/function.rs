n my_fn(s: &str) {
  println!("{s}");
}

fn multiplication(num1: i32, num2: i32) -> i32 {
  // return 45;
  println!("Computing Multiplication");
  num1 * num2
}

fn basic_math(num1: i32, num2: i32) -> (i32, i32, i32) {
  (num1 * num2, num1 + num2, num1 - num2)
}
// Expression versus Statements
// Expression returns a valid value
// Statement returns a unit value
fn main() {
  my_fn("This is my function");
  let str = "Function call with a variable";
  my_fn(str);
  let answer = multiplication(10, 15);
  let result = basic_math(10, 15);
  let (multiplication, addition, subtraction) = basic_math(10, 15);

  // Code Blocks

  let full_name = {
      let first_name = "Nouman";
      let last_name = "Azam";
      println!("{answer}");
      format!("{first_name} {last_name}")
  };
  // println!("{first_name}");
}
