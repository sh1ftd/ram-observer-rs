#[derive(Clone, Copy)]
pub enum Commands {
    EmptyWorkingSets,
    EmptySystemWorkingSets,
    EmptyModifiedPageLists,
    EmptyStandbyList,
    EmptyPriorityZeroStandbyList
}

impl Commands {
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
}
