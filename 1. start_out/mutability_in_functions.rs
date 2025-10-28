fn prints_number(x: i32) -> i32 {
  let mut x = x;
  x = x + 2;
  println!("{x}");
  x
}
fn main() {
  let mut num = 10;
  let y = prints_number(num);
  y = y + 3;
}
