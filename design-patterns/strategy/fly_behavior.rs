// Sample demonstrates use of traits for strategy pattern.

// `FlyBehaviour` denotes behaviour for different objects
trait FlyBehaviour {
    fn fly(&self);
}

// `FlyWithWings` implemented with above defined behavioiur
struct FlyWithWings;
impl FlyBehaviour for FlyWithWings {
    fn fly(&self) {
        println!("I can fly using my wings!!!");
    }
}

// `DoNotFly` implemented with above defined behavioiur
struct DoNotFly;
impl FlyBehaviour for DoNotFly {
    fn fly(&self) {
        println!("I can't fly !!!");
    }
}

// Object which has-a relationship to above defined trait
struct Duck {
    // behaviour off type `FlyBehaviour`
    fly_behaviour : Box<FlyBehaviour>,
}
impl Duck {
    fn fly(&self) {
        self.fly_behaviour.fly();
    }
    fn set_fly_behaviour(&mut self, behaviour: Box<FlyBehaviour>) {
        self.fly_behaviour = behaviour;
    }
}

fn main() {
    let do_not_fly_behaviour = Box::new(DoNotFly);
    let fly_with_wings_behaviour = Box::new(FlyWithWings);
    let mut donald_duck = Duck { fly_behaviour: do_not_fly_behaviour };

    // Lets see what is printed for `Duck` on no fly behaviour
    donald_duck.fly();

    // Lets try to change the behaviour at runtime ( in other terms apply flying strategy )
    donald_duck.set_fly_behaviour(fly_with_wings_behaviour);

    // confirm if strategy is configured to target container object
    donald_duck.fly();
}
