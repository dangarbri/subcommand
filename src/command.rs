use std::error::Error;

/** Application exit status type */
pub type ExitStatus = i32;

/** Return type for command functions */
pub type CommandResult = Result<ExitStatus, Box<dyn Error>>;

/**
 * Function pointer type.
 * Represents a function to a subcommand which will return a valid return code, or an error.
 */
pub type CommandFn = fn() -> CommandResult;

/**
 * Command metadata.
 * The name is used to identify the subcommand, and once selected, function will be called.
 */
pub struct Command {
    /** Name of this subcommand */
    pub name: String,
    /** Function to execute when this sub command is selected */
    pub function: CommandFn,
    /** Brief help message for this sub command */
    pub help: String
}