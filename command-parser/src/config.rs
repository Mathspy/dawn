use std::{
    borrow::Cow,
    collections::{HashMap, HashSet},
};

use crate::{CaseSensitivity, CommandBuilder};

/// Configuration for a [`Parser`].
///
/// [`Parser`]: struct.Parser.html
#[derive(Clone, Debug, Default)]
pub struct Config<'a> {
    commands: HashSet<CaseSensitivity>,
    prefixes: HashMap<Cow<'a, str>, Cow<'a, str>>,
}

impl<'a> Config<'a> {
    /// Creates a fresh default configuration with no commands or prefixes.
    pub fn new() -> Self {
        Self::default()
    }

    /// Returns an immutable reference to the commands.
    pub fn commands(&self) -> &HashSet<CaseSensitivity> {
        &self.commands
    }

    /// Returns a mutable reference to the commands.
    ///
    /// Use the [`add_command`] and [`remove_command`] methods for an easier way
    /// to manage commands.
    ///
    /// [`add_command`]: #method.add_command
    /// [`remove_command`]: #method.remove_command
    pub fn commands_mut(&mut self) -> &mut HashSet<CaseSensitivity> {
        &mut self.commands
    }

    /// Returns an immutable reference to the prefixes.
    ///
    /// Use the [`add_prefix`] and [`remove_prefix`] methods for an easier way
    /// to manage prefixes.
    ///
    /// [`add_prefix`]: #method.add_prefix
    /// [`remove_prefix`]: #method.remove_prefix
    pub fn prefixes(&self) -> &HashMap<Cow<'_, str>, Cow<'_, str>> {
        &self.prefixes
    }

    /// Returns a mutable reference to the prefixes.
    pub fn prefixes_mut(&mut self) -> &mut HashMap<Cow<'a, str>, Cow<'a, str>> {
        &mut self.prefixes
    }

    /// Returns a [`CommandBuilder`] which can be used to add a command to the list of commands.
    ///
    /// # Examples
    ///
    /// ```rust
    /// use dawn_command_parser::Config;
    ///
    /// let mut config = Config::new();
    /// config.command("ping").add();
    /// assert_eq!(1, config.commands().len());
    /// ```
    ///
    /// [`CommandBuilder`]: struct.CommandBuilder.html
    pub fn command<'b>(&'b mut self, name: impl Into<String>) -> CommandBuilder<'b, 'a> {
        CommandBuilder {
            name: name.into(),
            case_sensitive: false,
            config: self,
        }
    }

    pub(crate) fn add_command(&mut self, name: String, case_sensitive: bool) -> bool {
        let command = if case_sensitive {
            CaseSensitivity::Sensitive(name)
        } else {
            CaseSensitivity::Insensitive(name.into())
        };
        self.commands.insert(command)
    }

    /// Removes a command from the list of commands.
    ///
    /// Any commands that would match the command provided are removed.
    ///
    /// # Examples
    ///
    /// ```rust
    /// use dawn_command_parser::Config;
    ///
    /// let mut config = Config::new();
    /// config.command("ping").case_sensitive().add();
    /// config.command("PING").add();
    /// assert_eq!(2, config.commands().len());
    ///
    /// // Now remove it and verify that there are no commands.
    /// config.remove_command("ping");
    /// assert!(config.commands().is_empty());
    /// ```
    pub fn remove_command(&mut self, command: impl AsRef<str>) {
        self.commands.retain(|c| c != command.as_ref());
    }

    /// Adds a prefix to the list of prefixes.
    ///
    /// # Examples
    ///
    /// ```rust
    /// use dawn_command_parser::Config;
    ///
    /// let mut config = Config::new();
    /// config.add_prefix("!");
    /// assert_eq!(1, config.prefixes().len());
    /// ```
    pub fn add_prefix(&mut self, prefix: impl Into<Cow<'a, str>>) {
        self.prefixes.insert(prefix.into(), Cow::Borrowed(""));
    }

    /// Removes a prefix from the list of prefixes.
    ///
    /// # Examples
    ///
    /// ```rust
    /// use dawn_command_parser::Config;
    ///
    /// let mut config = Config::new();
    /// config.add_prefix("!");
    /// config.add_prefix("~");
    /// assert_eq!(2, config.prefixes().len());
    ///
    /// // Now remove one and verify that there is only 1 prefix.
    /// config.remove_prefix("!");
    /// assert_eq!(1, config.prefixes().len());
    /// ```
    pub fn remove_prefix(&mut self, prefix: impl Into<Cow<'a, str>>) -> Option<Cow<'a, str>> {
        self.prefixes.remove(&prefix.into())
    }
}

#[cfg(test)]
mod tests {
    use super::Config;

    #[test]
    fn test_getters() {
        let mut config = Config::new();
        assert!(config.commands().is_empty());
        assert!(config.commands_mut().is_empty());
        assert!(config.prefixes().is_empty());
        assert!(config.prefixes_mut().is_empty());
    }
}
