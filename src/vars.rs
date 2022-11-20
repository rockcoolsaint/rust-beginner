pub fn run() {
  let name = "Inno";
  let age = 37;
  println!("My name is {} and I am {}", name, age);
  let age = 38;
  println!("My name is {} and I am {}", name, age);

  // Define constant
  const ID: i32 = 001;
  println!("ID: {}", ID);

  // Assign multiple vars
  let (my_name, my_age) = ("Inno", 37);
  println!("{} is {}", my_name, my_age);
}