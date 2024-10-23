use crate::error::{BaseError, BaseResult};
use crate::settings::Settings;
use clap::{ArgMatches, Command};

/// A trait that defines the basic behavior for commands.
pub trait BaseCommand {
    /// Configure the command.
    ///
    /// # Returns
    /// A `Command` instance with the command's configuration.
    fn configure(&self) -> Command {
        Command::new(self.name()).about(self.about())
    }

    /// Handle the command execution.
    ///
    /// # Arguments
    /// * `matches` - The command-line arguments.
    /// * `settings` - The settings for the command.
    ///
    /// # Returns
    /// A `BaseResult` indicating the success or failure of the command execution.
    fn handle(&self, matches: &ArgMatches, settings: &Settings) -> BaseResult<()>;

    /// Get the name of the command.
    ///
    /// # Returns
    /// A string slice representing the name of the command.
    fn name(&self) -> &'static str;

    /// Get the about message for the command.
    ///
    /// # Returns
    /// A string slice representing the about message of the command.
    fn about(&self) -> &'static str;
}

/// Configure all commands.
///
/// # Arguments
/// * `root_name` - The name of the root command.
/// * `root_about` - The about message for the root command.
/// * `commands` - A vector of boxed commands to configure.
///
/// # Returns
/// A `Command` instance with all subcommands configured.
pub fn configure_all(
    root_name: &'static str,
    root_about: &'static str,
    commands: Vec<Box<dyn BaseCommand>>,
) -> Command {
    let mut root_command = Command::new(root_name).about(root_about);
    for cmd in commands.iter() {
        root_command = root_command.subcommand(cmd.configure());
    }
    root_command.arg_required_else_help(true)
}

/// Handle the execution of all commands.
///
/// # Arguments
/// * `matches` - The command-line arguments.
/// * `commands` - A vector of boxed commands to handle.
/// * `settings` - The settings for the commands.
///
/// # Returns
/// A `BaseResult` indicating the success or failure of the command execution.
pub fn handle_all(
    matches: &ArgMatches,
    commands: Vec<Box<dyn BaseCommand>>,
    settings: &Settings,
) -> BaseResult<()> {
    if let Some((cmd_name, sub_matches)) = matches.subcommand() {
        for cmd in commands.iter() {
            if cmd_name == cmd.name() {
                return cmd.handle(sub_matches, settings);
            }
        }
        Err(BaseError::command_error(&format!(
            "Unknown command: {}",
            cmd_name
        )))
    } else {
        Ok(())
    }
}
