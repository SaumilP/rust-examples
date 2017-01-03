struct Greeter {
  s: String,
}

struct GreetBuilder {
  s: String
}

impl GreetBuilder {
  fn new() -> GreetBuilder {
    GreetBuilder { s : "".to_string() }
  }

  fn set_s(mut self, s: String) -> GreetBuilder {
    self.s = s;
    self
  }

  fn build(self) -> Greeter {
    Greeter{ s: self.s }
  }
}

fn main(){
    let g = GreetBuilder::new()
        .set_s("Greeting Rustacean!!!".to_string())
        .build();
    println!("string: {}", g.s);
}
