pub enum UserActions {
    NONE,
    // COST,
    EDIT,
    EXIT,
    // FILE,
    FLUSH,
    // FORMAT,
    SAVE,
    HELP,
    COMMANDS,
    INPUT(String),
}

impl UserActions {
    pub fn description(&self) -> &'static str {
        match self {
            UserActions::NONE => "",
            // UserActions::COST => "\tcost - Display conversation costs.",
            UserActions::EDIT => {
                "\tedit - Edit the latest User message. Last Assistant reply will be lost."
            }
            UserActions::EXIT => "\texit - Exit the program.",
            // UserActions::FILE => "\tfile - Submit long text from a file to the chat.",
            UserActions::FLUSH => "\tflush - Start a new conversation.",
            // UserActions::FORMAT => "\tformat - Format multiline pasted text before sending to the chat.",
            UserActions::SAVE => "\tsave - Save the current conversation to a file.",
            UserActions::HELP => "\n\thelp - Display this help message.",
            UserActions::COMMANDS => "\tcommands - Display this list of commands.",
            UserActions::INPUT(_) => "",
        }
    }
}

pub enum Roles {
    SYSTEM,
    USER,
    ASSISTANT,
}

impl Roles {
    pub fn as_str(&self) -> &'static str {
        match self {
            Roles::SYSTEM => "system",
            Roles::USER => "user",
            Roles::ASSISTANT => "assistant",
        }
    }
}
