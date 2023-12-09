# Dispatch

Dispatch is a sub-command dispatcher.

You've probably seen CLI programs that have many subcommands.
For example with `cargo` itself has sub commands like build, check, clean, etc.

Dispatch is a library that lets you easily add subcommands like this in your programs.

## Usage

``` rust
// Define a function for your subcommand which returns the type Result<i32, Box<dyn Error>>
fn subcommand() -> CommandResult {
    Ok(0)
}

// Create the dispatcher
let mut dispatcher = Dispatcher::new();

// Register your subcommands
dispatcher.register(Command { name: "subcommand", help: "My first subcommand", function: subcommand});

// Print help message to console
dispatcher.print_help();

// Execute sub command by name
dispatcher.run("subcommand")
```

## More information

All subcommands return the type `CommandResult` which maps to `Result<i32, Box<dyn Error>>`.
This allows your subcommands to readily use rust's built-in `?` error checking syntax.
The i32 is intended to be the return status or exit code of your subcommand.
To me, this is what `main` would return as the program's exit status, though you could use it however you like.

You are still responsible for checking the input arguments and executing the subcommands as needed.