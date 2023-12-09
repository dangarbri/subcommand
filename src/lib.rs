mod command;

use std::error::Error;

use command::{Command, CommandResult};

/**
 * The dispatcher is responsible for choosing the appropriate subcommand to execute
 */
pub struct Dispatcher {
    commands: Vec<Command>,
}

impl Dispatcher {
    /**
     * Creates a new dispatcher with no commands associated with it
     */
    pub fn new() -> Dispatcher {
        Dispatcher { commands: vec![] }
    }

    /**
     * Register a new subcommand
     */
    pub fn register(&mut self, command: Command) {
        self.commands.push(command);
    }

    pub fn run(&self, command: String) -> CommandResult {
        for cmd in &self.commands {
            if cmd.name == command {
                return (cmd.function)()
            }
        }
        return Err(Box::<dyn Error>::from("Invalid command"));
    }

    /** Returns the help message as a string */
    pub fn get_help_message(&self) -> String {
        let mut help: String = String::new();
        let cmd_column_length = self.get_longest_command_name_length() + 4;

        for cmd in &self.commands {
            help.push_str(&format!("{1:<0$}{2}\n", cmd_column_length, cmd.name, cmd.help));
        }
        help
    }

    /** Prints all the help messages for all the sub commands in this dispatcher */
    pub fn print_help(&self) {
        let msg = self.get_help_message();
        println!("{msg}");
    }

    fn get_longest_command_name_length(&self) -> usize {
        let mut length = 0;
        for cmd in &self.commands {
            let cmd_length = cmd.name.len();
            if cmd_length > length {
                length = cmd_length
            }
        }
        return length;
    }
}

#[cfg(test)]
mod tests {
    use crate::command::CommandResult;

    use super::*;

    fn dummy_fn() -> CommandResult {
        Ok(99)
    }

    #[test]
    fn test_help() {
        let mut dispatcher = Dispatcher::new();
        dispatcher.register(Command { name: "beep".into(), function: dummy_fn, help: "boop".into() });
        dispatcher.register(Command { name: "subcmd".into(), function: dummy_fn, help: "A longer message than before".into() });

        let help = dispatcher.get_help_message();
        assert_eq!(help, "beep      boop\nsubcmd    A longer message than before\n");
    }

    #[test]
    fn test_dispatch() {
        let mut dispatcher = Dispatcher::new();
        dispatcher.register(Command { name: "beep".into(), function: dummy_fn, help: "boop".into() });
        let result = dispatcher.run("beep".into()).unwrap();
        assert_eq!(result, 99);
    }
}
