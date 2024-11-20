/// Represents available RAM management commands that can be executed via RAMMap64.exe
/// Each variant corresponds to a specific memory clearing operation
#[allow(clippy::enum_variant_names)]
#[derive(Clone, Copy)]
pub enum Commands {
    EmptyWorkingSets,
    EmptySystemWorkingSets,
    EmptyModifiedPageLists,
    EmptyStandbyList,
    EmptyPriorityZeroStandbyList,
}

impl Commands {
    /// Maps keyboard characters to Commands for hotkey functionality
    /// The tuple contains (hotkey_char, corresponding_command)
    pub const ACTION_MAP: [(char, Commands); 5] = [
        ('1', Commands::EmptyWorkingSets),
        ('2', Commands::EmptySystemWorkingSets),
        ('3', Commands::EmptyModifiedPageLists),
        ('4', Commands::EmptyStandbyList),
        ('5', Commands::EmptyPriorityZeroStandbyList),
    ];

    /// Returns the command-line parameter for RAMMap64.exe corresponding to this command
    ///
    /// # Returns
    /// * A string slice containing the RAMMap parameter
    pub fn parameter(&self) -> &str {
        match self {
            Self::EmptyWorkingSets => "-Ew",
            Self::EmptySystemWorkingSets => "-Es",
            Self::EmptyModifiedPageLists => "-Em",
            Self::EmptyStandbyList => "-Et",
            Self::EmptyPriorityZeroStandbyList => "-E0",
        }
    }

    /// Returns a human-readable name for the command
    ///
    /// # Returns
    /// * A string slice containing the display name
    pub fn display_name(&self) -> &str {
        match self {
            Self::EmptyWorkingSets => "Empty Working Sets",
            Self::EmptySystemWorkingSets => "Empty System Working Sets",
            Self::EmptyModifiedPageLists => "Empty Modified Page Lists",
            Self::EmptyStandbyList => "Empty Standby List",
            Self::EmptyPriorityZeroStandbyList => "Empty Priority 0 Standby List",
        }
    }

    /// Retrieves a command by its index in the ACTION_MAP
    ///
    /// # Arguments
    /// * `index` - The index to look up
    ///
    /// # Returns
    /// * `Some(Commands)` if index is valid
    /// * `None` if index is out of bounds
    pub fn from_index(index: usize) -> Option<Self> {
        Self::ACTION_MAP.get(index).map(|(_, cmd)| *cmd)
    }

    /// Retrieves a command by its associated hotkey character
    ///
    /// # Arguments
    /// * `c` - The character to look up
    ///
    /// # Returns
    /// * `Some(Commands)` if character matches a hotkey
    /// * `None` if no matching hotkey is found
    pub fn from_char(c: char) -> Option<Self> {
        Self::ACTION_MAP
            .iter()
            .find(|(key, _)| *key == c)
            .map(|(_, cmd)| *cmd)
    }
}
