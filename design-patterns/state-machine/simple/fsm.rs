// State machine basic structure gets declared here
#[derive(Debug)]
struct StateMachine<S> {
    some_unrelated_value: usize,
    state: S,
}

// First transition into State A
impl StateMachine<InitialState> {
     fn new(val: String) -> Self {
         StateMachine {
             some_unrelated_value: 0,
             state: InitialState::new(val)
         }
     }
}

// Initial State
#[derive(Debug)]
struct InitialState {
    start_value: String,
}

// Initial State records string as initial value
impl InitialState {
    fn new(start_value: String) -> Self {
        InitialState {
            start_value: start_value,
        }
    }
}

// State B starts and breaks up given string into words ( using vector for reprensentation )
#[derive(Debug)]
struct StringTokenizerState {
    intern_value: Vec<String>,
}

impl From<StateMachine<InitialState>> for StateMachine<StringTokenizerState> {
     fn from(val: StateMachine<InitialState>) -> StateMachine<StringTokenizerState> {
         StateMachine {
             some_unrelated_value: val.some_unrelated_value,
             state: StringTokenizerState {
                 // break up string into words from initial string
                 intern_value: val.state.start_value.split(" ").map(|x| x.into()).collect(),
             }
         }
     }
}

// Final state where word count is determined
#[derive(Debug)]
struct WordCounterState {
    final_value: usize,
}

impl From<StateMachine<StringTokenizerState>> for StateMachine<WordCounterState> {
     fn from(val: StateMachine<StringTokenizerState>) -> StateMachine<WordCounterState> {
         StateMachine {
             some_unrelated_value: val.some_unrelated_value,
             state: WordCounterState {
                 final_value: val.state.intern_value.len(),
             }
         }
     }
}

fn main() {
    // Create a new StateMachine, so it can be transitioned into initial state
    let init_state = StateMachine::new("Hi This is a test state machine example".into());

    init_state.some_unrelated_value;
    println!("Initial Value: {}", init_state.state.start_value);

    // Transition to next state. It consumes initial state.
    // Here type annotations are needed since all the state machines are linear in their state
    let tokenizer_state = StateMachine::<StringTokenizerState>::from(init_state);
    
    // Previous state is not available now as value has been moved into next state. values are only available via
    // current state
    tokenizer_state.some_unrelated_value;
    println!("Intern Value: {:?}", tokenizer_state.state.intern_value);

    // Transition into final State
    let final_state = StateMachine::<WordCounterState>::from(tokenizer_state);
    println!("Final State: {}", final_state.state.final_value);
}