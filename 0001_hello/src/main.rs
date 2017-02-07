fn main() {
    println!("{}", hello("Samwise"));
    println!("{}", hello("Marvin"));
}

// TODO: Implement the hello function
fn hello() {
  
}

#[test]
fn test_hello() {
  assert_eq!("Hello Samwise", hello("Samwise"));
  assert_eq!("Hello Marvin", hello("Marvin"));
}