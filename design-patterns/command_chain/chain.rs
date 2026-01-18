// Sample uses traits to demonstrate chain of commands pattern in rust-lang.

// `Command` trait for default actions
trait Command {

    // function to accept `name` and `args` for update functionality
    fn on(&self, name: &str, args: &[&str]);
}

// `CommandChain` definition for execution chain
struct CommandChain<'a> {
    commands: Vec<Box<Command + 'a>>,
}
impl<'a> CommandChain<'a> {
    // default command-chain with new vector
    fn new() -> CommandChain<'a> {
        CommandChain {
            commands: Vec::new(),
        }
    }

    // object setter
    fn add_command(&mut self, command: Box<Command + 'a>) {
        // push provided command to defined vector
        self.commands.push(command);
    }

    // iterate over commands from configured chain and invoke defined trait
    fn run(&self, name: &str, args: &[&str]) {
        for cmd in self.commands.iter() {
            cmd.on(name, args);
        }
    }
}

// Concrete commands ....
struct UserCommand;
impl UserCommand {
    fn new() -> UserCommand {
        UserCommand
    }
}
// Lets define trait on the command so execution is fine in Command-chain
impl Command for UserCommand {
    // trait implementation
    fn on(&self, name: &str, args: &[&str]) {
        if name == "adduser" {
            println!("UserCommand handling '{}' with args {:?}", name, args);
        }
    }
}

struct MailCommand;
impl MailCommand {
    fn new() -> MailCommand {
        MailCommand
    }
}
// Lets define trait on the command so execution is fine in Command-chain
impl Command for MailCommand {
    // trait implementation
    fn on(&self, name: &str, args: &[&str]) {
        if name == "adduser" {
            println!("MailCommand handling '{}' with args {:?}", name, args);
        } else if name == "mail" {
            println!("MailCommand handling '{}' with args {:?}", name, args);
        }
    }
}

fn main() {
    let mut chain = CommandChain::new();
    chain.add_command(Box::new(UserCommand::new()));
    chain.add_command(Box::new(MailCommand::new()));
    chain.run("adduser", &["John", "users"]);

    chain.run("mail", &["Jane", "Subject"]);

    //chain.run("something-else", &["test", "cc"]);
    // TRY ^ uncommenting this to check behaviour when trait is not configured...
}
