// This example demonstrates how to apply command pattern in rust-lang
#![allow(dead_code)]
#![allow(unused_variables)]

trait Command {
     fn execute(&self);
}

struct NullCommand;
impl NullCommand {
     fn new() -> NullCommand {
         NullCommand
     }
}
impl Command for NullCommand {
    fn execute(&self) {
        println!("Nothing to do here...");
    }
}

// The object to handle : a Light
#[derive(Copy, Clone)]
struct Light;
impl Light {
    fn new() -> Light {
        Light
    }
    fn on(&self) {
        println!("Light is on");
    }
    fn off(&self) {
        println!("Light is off");
    }
}

// First command to turn on the light
struct LightOnCommand {
    light : Light,
}
impl LightOnCommand {
     fn new(_light: Light) -> LightOnCommand {
         LightOnCommand {
             light: _light
         }
     }
}
impl Command for LightOnCommand {
     fn execute(&self) {
         self.light.on();
     }
}

// Light off command
struct LightOffCommand {
    light: Light,
}
impl LightOffCommand {
     fn new(_light: Light) -> LightOffCommand {
         LightOffCommand {
             light: _light
         }
     }
}
impl Command for LightOffCommand {
    fn execute(&self) {
        self.light.off();
    }
}

// Remote control command
struct SimpleRemoteControl<'a> {
    command: Box<Command + 'a>,
}
impl<'a> SimpleRemoteControl<'a> {
     fn new() -> SimpleRemoteControl<'a> {
         SimpleRemoteControl {
             command: Box::new(NullCommand::new())
        }
     }
     fn set_command(&mut self, cmd: Box<Command + 'a>) {
         self.command = cmd;
     }
     fn button_was_pressed(&self) {
         self.command.execute();
     }
}

fn main() {
    let mut remote = SimpleRemoteControl::new();
    let light = Light::new();
    let light_on = LightOnCommand::new(light);
    let light_off = LightOffCommand::new(light);

    remote.button_was_pressed();
    remote.set_command(Box::new(light_on));
    remote.button_was_pressed();
    remote.set_command(Box::new(light_off));
    remote.button_was_pressed();
}