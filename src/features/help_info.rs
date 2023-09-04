use crate::models::enums::UserActions;

pub fn help_info() {
    /*
    Prints the available commands
    */
    println!("You can use the following commands:");
    let commands: [UserActions; 9] = [
        UserActions::COST,
        UserActions::EDIT,
        UserActions::EXIT,
        UserActions::FILE,
        UserActions::FLUSH,
        UserActions::FORMAT,
        UserActions::SAVE,
        UserActions::HELP,
        UserActions::COMMANDS,
    ];
    for command in commands {
        println!("{}", command.description());
    }
}
