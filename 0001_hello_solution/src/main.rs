fn main() {
    println!("{}", hello("Samwise"));
    println!("{}", hello("Marvin"));
}

fn hello(name: &str) -> String {
  format!("Hello {}", name)
}

#[test]
fn test_hello() {
  assert_eq!("Hello Samwise", hello("Samwise"));
  assert_eq!("Hello Marvin", hello("Marvin"));
}