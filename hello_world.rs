fn greeting() -> str {
  "Hello There!"
}

fn main() {
  let my_greeting = ~greeting();
  println!(my_greeting);
}

#[test]
fn test_hello_world() {
  if greeting() != "Hello There!" {
    fail!("Wrong Greeting")
  }
}
