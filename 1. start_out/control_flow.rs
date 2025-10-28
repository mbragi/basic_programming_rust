fn main() {
  // Loops
  'outer: loop {
      loop {
          println!("Simple loop");
          break 'outer;
      }
  }

  let a = loop {
      break 5;
  };

  // For loops
  let vec = vec![45, 30, 85, 90, 41, 39];
  for i in vec {
      println!("{i}");
  }

  // Compound Data Types versus Collections

  // While loops
  let mut num = 0;
  while num < 10 {
      num = num + 1;
  }
}