fn main() {
  // Comments

  // The current line is a comment line
  // This is a second line of comment

  /* This is a
  multi-line
  comment
  */

  // More on Printing

  print!("This is a print command");
  print!("This is going to be printed on the same line");

  // Escape Sequences
  // \escape sequence character

  /*
  \n : Newline character
  \t : Tab space
  \r : Carriage Return
  \" : Double quote
  \\ : Backward slash
  */

  println!("\n will be printed after one empty line");
  println!("\t A tab space at the start");

  println!("This will be overwritten \r This text will only appear on the screen");
  println!("Prints double quotes \", Prints backslash \\");

  // Positional Arguments
  println!(
      "I am doing {2} from {1} year and i {0} it",
      "like", 20, "programming"
  );
  // Named Arguments
  println!(
      "{language} is a system programming language which is cool to {activity} in",
      activity = "code",
      language = "Rust"
  );

  // Printing compound data types
  let my_info = ("Salary", 40000, "Age", 40);
  println!("{:?}", my_info);
  println!("{:#?}", my_info);
}
