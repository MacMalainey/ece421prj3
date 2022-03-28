use terminal_menu::{TerminalMenuStruct, TerminalMenuItem};

pub trait Prompt: ToMenu + FromMenu + Sized {
    fn prompt() -> Self {
        let menu = terminal_menu::menu(Self::to_menu());
        terminal_menu::run(&menu);
        let mut mmenu = terminal_menu::mut_menu(&menu);
        Self::from_menu(&mut mmenu)
    }
}

pub trait ToMenu {
    fn to_menu() -> Vec<TerminalMenuItem>;
}

pub trait FromMenu {
    fn from_menu(menu: &mut TerminalMenuStruct) -> Self;
}

pub enum ConnectionType {
    DirectToDatabase,
    NetworkAPI
}

pub struct ConnectionConfig {
    pub address: String,
    pub method: ConnectionType
}

impl Prompt for ConnectionConfig {}

impl ToMenu for ConnectionConfig {

    fn to_menu() -> Vec<TerminalMenuItem> {
        vec![
            terminal_menu::list(
                "Connect to",
                vec![
                    "Local Database",
                    "API Server"
                ]
            ),
            terminal_menu::string("Address", "localhost:8000", false),
            terminal_menu::button("Done")
        ]
    }

}

impl FromMenu for ConnectionConfig {

    fn from_menu(menu: &mut TerminalMenuStruct) -> Self {
        let method = match menu.selection_value("Connect to") {
            "Local Database" => ConnectionType::DirectToDatabase,
            "API Server" => ConnectionType::NetworkAPI,
            _ => panic!("Menu returned unknown connection type")
        };
        let address = String::from(menu.selection_value("Address"));

        ConnectionConfig {
            method,
            address
        }
    }

}

pub enum Actions {
    AddUser,
    DeleteUser,
    ChangeUserPassword,
    AddRecord,
    CheckRecords,
    Exit
}

impl Prompt for Actions {}

impl ToMenu for Actions {

    fn to_menu() -> Vec<TerminalMenuItem> {
        vec![
            terminal_menu::label("Select an action"),
            terminal_menu::button("Add User"),
            terminal_menu::button("Delete User"),
            terminal_menu::button("Change User Password"),
            terminal_menu::button("Add Record"),
            terminal_menu::button("Check Records"),
            terminal_menu::button("Exit")
        ]
    }

}

impl FromMenu for Actions {

    fn from_menu(menu: &mut TerminalMenuStruct) -> Self {
        match menu.selected_item_name() {
            "Add User" => Actions::AddUser,
            "Delete User" => Actions::DeleteUser,
            "Change User Password" => Actions::ChangeUserPassword,
            "Add Record" => Actions::AddRecord,
            "Check Records" => Actions::CheckRecords,
            "Exit" => Actions::Exit,
            val => panic!("Invalid Action value: {}", val)
        }
    }

}
