fn main() {
  // Associativity
  let x = 8 / 4 / 2; // (8 / 4) / 2 = 1
  let mut y = 42;
  // x = y = 0; // x = (y = 0) // x = ()

  // right to left
  // =, +=, -=, *=, /=, %=, &=, |=, ^=, <<=, >>=

  let mut a = 1;
  let mut b = 2;
  let mut c = 3;
  // b += c += 42;

  // Explicit Boolean in Conditionals
  let x = 0;
  if x != 0 {}

  // Operator Overloading
  let a = 10 + 20;
  let b = String::from("1") + " 2";
  println!("{}", b);
}
