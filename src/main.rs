mod command {
    pub trait Command {
        fn execute(&self)->Box<Command>;
    }
    pub struct DoNothing;
    impl Command for DoNothing {
        fn execute(&self)->Box<Command> {
            println!("Do Nothing");
            return Box::new(DoNothing);
        }
    }
}

mod light {
    use command::Command;

    #[derive(Clone,Debug)]
    pub struct Light;
    impl Light {
        pub fn on(&self) {
            println!("Включить свет");
        }
        pub fn off(&self) {
            println!("Выключить свет");
        }
    }

    pub struct LightOnCommand{
        light : Light,
    }
    impl LightOnCommand {
        pub fn new(light : &Light) -> Self {
            LightOnCommand { light : light.clone() }
        }
    }
    impl Command for LightOnCommand {
        fn execute(&self) -> Box<Command>{
            self.light.on();
            Box::new(LightOffCommand::new(&self.light))
        }
    }

    pub struct LightOffCommand{
        light : Light
    }
    impl LightOffCommand {
        pub fn new(light : &Light) -> Self {
            LightOffCommand { light : light.clone() }
        }
    }
    impl Command for LightOffCommand {
        fn execute(&self) -> Box<Command>{
            self.light.off();
            Box::new(LightOnCommand::new(&self.light))
        }
    }
}

use command::Command;
use light::{Light, LightOnCommand, LightOffCommand};

fn main() {
    let light = Light;
    println!("Here is a Light: {:?}", &light);

    let commands: Vec<Box<Command>> = vec![
            Box::new(LightOnCommand::new(&light)),
            Box::new(LightOffCommand::new(&light))
        ];

    let mut undo : Vec<Box<Command>> = vec![];
    let mut redo : Vec<Box<Command>> = vec![];
    undo.push(commands[0].execute());
    undo.push(commands[1].execute());

    // Undo all operations
    while let Some(cmd) = undo.pop() {
        redo.push(cmd.execute());
    }
    // Redo all operations
    while let Some(cmd) = redo.pop() {
        undo.push(cmd.execute());
    }
 }
