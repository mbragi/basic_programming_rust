fn main() {
  // Arithmetic Operators
  // +, -, *, /, %
  // Note: ^ is not for exponential used rather for bitwise XOR (exclusive or) 
  // pow() method is used for exponential
  println!("Remainder after dividing 17 by 5: {}", 17 % 5);

  // Comparison Operators
  // Equality: == (Not to be confused with assignment, i.e., =)
  // Inequality: !=
  // Relational: >, <, >=, <=

  let a = 10;
  let b = 10;
  println!(
      "a == b: {}, a != b: {}, a < b: {}, a > b: {}, a <= b: {}, a >= b: {}",
      a == b,
      a != b,
      a < b,
      a > b,
      a <= b,
      a >= b,
  );

  // Logical Operators
  // And: &&
  // Or: ||
  // Not: !

  let a = 10;
  let b = 20;
  if (a > 5) && (b < 25) {
      println!("Conditions satisfied");
  } else {
      println!("Conditions not satisfiedd");
  }

  // Assignment Operators
  // Add and assign: +=
  // Subtract and assign: -=
  // Multiply and assign: *=
  // Divide and assign: /=
  // Remainder and assign: %=

  let mut x = 5;
  x += 5; // => x = x + 5;
  x -= 5; // => x = x - 5;
  x *= 5; // => x = x * 5;
  x /= 5; // => x = x / 5;
  x %= 5; // => x = x % 5;

  // Bitwise Operators
  // And: &
  // Or: |
  // Xor: ^
  // Not: ~
  // Left Shift: <<
  // Right Shift: >>

  let x: u8 = 4;
  println!("{}", x & x); // => 0000_0100 & 0000_0100 = 0000_0100
  let y: u8 = 4 << 1; // => 0000_0100 -> 0000_1000
}
