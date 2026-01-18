trait SharedFunctionality {
     fn get_shared_value(&self) -> usize;
}

#[derive(Debug)]
struct Waiting {
    waiting_time: std::time::Duration,
    shared_value: usize,
}
impl Waiting {
     fn new() -> Self {
         Waiting {
             waiting_time: std::time::Duration::new(0,0),
             shared_value: 0,
         }
     }

     fn to_filling(self) -> Filling {
         Filling {
             rate: 1, 
             shared_value: 0,
         }
     }
}
impl SharedFunctionality for Waiting {
    fn get_shared_value(&self) -> usize {
        self.shared_value
    }
}

#[derive(Debug)]
struct Filling {
    rate: usize,
    shared_value: usize,
}
impl SharedFunctionality for Filling {
     fn get_shared_value(&self) -> usize {
         self.shared_value
     }
}

fn main() {
    let in_waiting_state = Waiting::new();
    println!("Initial State :{}", in_waiting_state.shared_value);

    let in_filling_state = in_waiting_state.to_filling();
    println!("Initial State :{:?}", in_filling_state.shared_value);
}