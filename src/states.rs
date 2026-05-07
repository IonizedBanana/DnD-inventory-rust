// some enums to represent states

pub enum SaveState {
    Inventory,
    Notebook,
    Purse,
    Potion,
    None,
}

impl SaveState {
    pub fn next(&self) -> SaveState {
        match *self {
            SaveState::Inventory => SaveState::Notebook,
            SaveState::Notebook => SaveState::Purse,
            SaveState::Purse => SaveState::Potion,
            SaveState::Potion => SaveState::None,
            SaveState::None => SaveState::None,
        }
    }
}

pub enum ProgramState {
    None,
    InvMenu,
    NoteMenu,
    PurseMenu,
    PotionMenu,
}

// used to modularly print subtext about something
pub enum PrintState {
    Arrow,
    Comma,
}

impl PrintState {
    pub fn next(&self) -> PrintState {
        match *self {
            PrintState::Arrow => PrintState::Comma,
            PrintState::Comma => PrintState::Comma,
        }
    }

    // used to print either an arrow or a comma
    // for listing descriptors, makes it easy to 
    // print without hard coded logic
    pub fn format_print(&self) {
        match *self {
            PrintState::Arrow => {
                print!("-> ");
            }
            PrintState::Comma => {
                print!(", ");
            }
        }
    }
}
