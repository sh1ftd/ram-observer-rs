#[derive(Clone, Copy)]
pub enum Commands {
    EmptyWorkingSets,
    EmptySystemWorkingSets,
    EmptyModifiedPageLists,
    EmptyStandbyList,
    EmptyPriorityZeroStandbyList
}

impl Commands {
    pub const ACTION_MAP: [(char, Commands); 5] = [
        ('1', Commands::EmptyWorkingSets),
        ('2', Commands::EmptySystemWorkingSets),
        ('3', Commands::EmptyModifiedPageLists),
        ('4', Commands::EmptyStandbyList),
        ('5', Commands::EmptyPriorityZeroStandbyList),
    ];

    pub fn parameter(&self) -> &str {
        match self {
            Self::EmptyWorkingSets => "-Ew",
            Self::EmptySystemWorkingSets => "-Es",
            Self::EmptyModifiedPageLists => "-Em",
            Self::EmptyStandbyList => "-Et",
            Self::EmptyPriorityZeroStandbyList => "-E0",
        }
    }

    pub fn display_name(&self) -> &str {
        match self {
            Self::EmptyWorkingSets => "Empty Working Sets",
            Self::EmptySystemWorkingSets => "Empty System Working Sets",
            Self::EmptyModifiedPageLists => "Empty Modified Page Lists",
            Self::EmptyStandbyList => "Empty Standby List",
            Self::EmptyPriorityZeroStandbyList => "Empty Priority 0 Standby List",
        }
    }

    pub fn from_index(index: usize) -> Option<Self> {
        Self::ACTION_MAP.get(index).map(|(_, cmd)| *cmd)
    }

    pub fn from_char(c: char) -> Option<Self> {
        Self::ACTION_MAP.iter()
            .find(|(key, _)| *key == c)
            .map(|(_, cmd)| *cmd)
    }
}
