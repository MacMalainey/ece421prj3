use std::convert::Infallible;
use std::str::FromStr;

use terminal_menu::{TerminalMenuStruct, TerminalMenuItem};

type NoErr = Option<Infallible>;
type MResult<T> = Result<<T as FromMenu>::Output, <T as FromMenu>::Error>;

pub trait Prompt: ToMenu + FromMenu {
    fn prompt(params: &<Self as ToMenu>::Params) -> <Self as FromMenu>::Output {
        let mut previous_error = None;
        loop {
            let menu = terminal_menu::menu(Self::to_menu(params, previous_error));
            terminal_menu::run(&menu);
            let mut mmenu = terminal_menu::mut_menu(&menu);
            match Self::from_menu(&mut mmenu) {
                Ok(output) => return output,
                Err(err) => previous_error = Some(err)
            }
        }
    }
}
impl <T: ToMenu + FromMenu> Prompt for T {}

pub trait DefaultPrompt: Prompt
where
    <Self as ToMenu>::Params: Default
{
    fn prompt_default() -> <Self as FromMenu>::Output {
        Self::prompt(&<Self as ToMenu>::Params::default())
    }
}
impl <T: ToMenu + FromMenu> DefaultPrompt for T where <T as ToMenu>::Params: Default {}

pub trait ToMenu: FromMenu {
    type Params;
    fn to_menu(
        params: &Self::Params,
        prev_error: Option<<Self as FromMenu>::Error>
    ) -> Vec<TerminalMenuItem>;
}

pub trait FromMenu {
    type Output;
    type Error;
    fn from_menu(menu: &mut TerminalMenuStruct) -> MResult<Self>;
}

fn error_text(text: &str) -> String {
    use ansi_term::Color;

    Color::Red.bold().paint(text).to_string()
}

fn required_field(item: TerminalMenuItem, mark_missing: bool) -> TerminalMenuItem {
    use crossterm::style::Color;

    if mark_missing {
        item.colorize(Color::Red)
    } else {
        item
    }
}

pub struct ConnectionConfigMenu;

impl ToMenu for ConnectionConfigMenu {
    type Params = ();

    fn to_menu(_: &Self::Params, _: NoErr) -> Vec<TerminalMenuItem> {
        vec![
            terminal_menu::label("Connect to Local Database"),
            terminal_menu::string("Path", "localdev.db", false),
            terminal_menu::button("Done")
        ]
    }

}

impl FromMenu for ConnectionConfigMenu {
    type Output = String;
    type Error = Infallible;
    fn from_menu(menu: &mut TerminalMenuStruct) -> MResult<Self> {
        let path = String::from(menu.selection_value("Path"));

        Ok(path)
    }

}

pub struct ActionMenu;

impl ToMenu for ActionMenu {
    type Params = ();

    fn to_menu(_: &Self::Params, _: NoErr) -> Vec<TerminalMenuItem> {
        vec![
            terminal_menu::label("Select an action"),
            terminal_menu::button("Add User"),
            terminal_menu::button("Delete User"),
            terminal_menu::button("List Users"),
            terminal_menu::button("Add Record"),
            terminal_menu::button("Delete Record"),
            terminal_menu::button("List a User's Records"),
            terminal_menu::button("Exit")
        ]
    }

}

impl FromMenu for ActionMenu {
    type Output = crate::types::Actions;
    type Error = Infallible;
    fn from_menu(menu: &mut TerminalMenuStruct) -> MResult<Self> {
        use crate::types::Actions;
        let action = match menu.selected_item_name() {
            "Add User" => Actions::AddUser,
            "Delete User" => Actions::DeleteUser,
            "List Users" => Actions::ListUsers,
            "Add Record" => Actions::AddRecord,
            "Delete Record" => Actions::DeleteRecord,
            "List a User's Records" => Actions::ListUserRecords,
            "Exit" => Actions::Exit,
            val => panic!("Invalid Action value: {}", val)
        };

        Ok(action)
    }

}

pub struct NewUserMenu;

pub struct NewUserMenuError {
    pub username_empty: bool,
    pub password_empty: bool
}

impl ToMenu for NewUserMenu {
    type Params = ();

    fn to_menu(_: &Self::Params, prev_err: Option<NewUserMenuError>) -> Vec<TerminalMenuItem> {

        let NewUserMenuError { username_empty, password_empty } = prev_err.unwrap_or(NewUserMenuError { username_empty: false, password_empty: false });

        let mut menu = vec![
            terminal_menu::label("Register New User"),
            required_field(terminal_menu::string("Username", "", false), username_empty),
            required_field(terminal_menu::string("Password", "", false), password_empty),
            terminal_menu::button("Confirm"),
            terminal_menu::button("Cancel")
        ];

        if username_empty || password_empty {
            menu.push(terminal_menu::label(error_text("Username and password must not be empty!")))
        }

        menu

    }

}

impl FromMenu for NewUserMenu {
    type Output = Option<crate::types::NewUser>;
    type Error = NewUserMenuError;
    fn from_menu(menu: &mut TerminalMenuStruct) -> MResult<Self> {
        use crate::types::NewUser;
        match menu.selected_item_name() {
            "Confirm" => {
                let username = String::from(menu.selection_value("Username"));
                let password = String::from(menu.selection_value("Password"));

                let username_empty = username.trim().is_empty();
                let password_empty = password.trim().is_empty();

                if username_empty || password_empty {
                    Err(NewUserMenuError { username_empty, password_empty })
                } else {
                    Ok(Some(NewUser { username, password }))
                }
            },
            "Cancel" => Ok(None),
            val => panic!("Invalid Action value: {}", val)
        }
    }
}

pub struct UserIdMenu;

impl ToMenu for UserIdMenu {
    type Params = ();
    fn to_menu(_: &(), prev_err: Option<()>) -> Vec<TerminalMenuItem> {
        let mut menu = vec![
            required_field(terminal_menu::string("Username", "", false), prev_err.is_some()),
            terminal_menu::button("Confirm"),
            terminal_menu::button("Cancel")
        ];

        if prev_err.is_some() {
            menu.push(terminal_menu::label(error_text("Username must not be empty!")))
        }

        menu
    }
}

impl FromMenu for UserIdMenu {
    type Output = Option<String>;
    type Error = ();
    fn from_menu(menu: &mut TerminalMenuStruct) -> MResult<Self> {
        match menu.selected_item_name() {
            "Confirm" => {
                let input = String::from(menu.selection_value("Username"));
                if input.trim().is_empty() {
                    Err(())
                } else {
                    Ok(Some(input))
                }
            },
            "Cancel" => Ok(None),
            val => panic!("Invalid Action value: {}", val)
        }
    }
}

pub struct MatchRecordMenu;

impl ToMenu for MatchRecordMenu {
    type Params = ();

    fn to_menu(_: &Self::Params, _: NoErr) -> Vec<TerminalMenuItem> {
        use chrono::{Local, Datelike, Timelike};

        let localtime = Local::now();

        vec![
            terminal_menu::label("New Record"),
            terminal_menu::submenu("Finished At", vec![
                terminal_menu::numeric(
                    "Year", localtime.year() as f64, Some(1.0), Some(2021.0), Some(2022.0)
                ),
                terminal_menu::numeric(
                    "Month", localtime.month() as f64, Some(1.0), Some(1.0), Some(12.0)
                ),
                terminal_menu::numeric(
                    "Day", localtime.day() as f64, Some(1.0), Some(1.0), Some(31.0)
                ),
                terminal_menu::numeric(
                    "Hour", localtime.hour() as f64, Some(1.0), Some(0.0), Some(23.0)
                ),
                terminal_menu::numeric(
                    "Minute", localtime.minute() as f64, Some(1.0), Some(0.0), Some(59.0)
                ),
                terminal_menu::numeric(
                    "Second", localtime.second() as f64, Some(1.0), Some(0.0), Some(59.0)
                ),
                terminal_menu::label("Note: Date and time will be parsed using the local timezone"),
                terminal_menu::back_button("Back")
            ]),
            terminal_menu::list(
                "Game", vec!["Connect4", "OttoToot"]
            ),
            terminal_menu::list(
                "CPU Level", vec!["Easy", "Medium", "Hard"]
            ),
            terminal_menu::numeric("Moves", 4.0, Some(1.0), Some(4.0), Some(49.0)),
            terminal_menu::list(
                "Result", vec!["Win", "Loss", "Tie"]
            ),
            terminal_menu::button("Submit"),
            terminal_menu::button("Cancel")
        ]
    }

}

impl FromMenu for MatchRecordMenu {
    type Output = Option<shared_types::types::MatchRecord>;
    type Error = Infallible;
    fn from_menu(menu: &mut TerminalMenuStruct) -> MResult<Self> {
        use chrono::{Utc, Local, TimeZone};
        use chrono::naive::{NaiveDateTime, NaiveDate, NaiveTime};

        use shared_types::types::{MatchRecord, GameType, CpuLevel, MatchResult};

        match menu.selected_item_name() {
            "Submit" => {
                let finished_at = {
                    let datetime_menu = menu.get_submenu("Finished At");
                    Local.from_local_datetime(
                        &NaiveDateTime::new(
                            NaiveDate::from_ymd(
                                datetime_menu.numeric_value("Year") as i32,
                                datetime_menu.numeric_value("Month") as u32,
                                datetime_menu.numeric_value("Day") as u32,
                            ),
                            NaiveTime::from_hms(
                                datetime_menu.numeric_value("Hour") as u32,
                                datetime_menu.numeric_value("Minute") as u32,
                                datetime_menu.numeric_value("Second") as u32,
                            ),
                        )
                    ).unwrap().with_timezone(&Utc)
                };

                let game_id = match menu.selection_value("Game") {
                    "Connect4" => GameType::Connect4,
                    "OttoToot" => GameType::OttoToot,
                    val => panic!("Invalid Game value: {}", val)
                };

                let cpu_level = match menu.selection_value("CPU Level") {
                    "Easy" => CpuLevel::Easy,
                    "Medium" => CpuLevel::Medium,
                    "Hard" => CpuLevel::Hard,
                    val => panic!("Invalid CPU Level value: {}", val)
                };

                let result = match menu.selection_value("Result") {
                    "Win" => MatchResult::Win,
                    "Loss" => MatchResult::Loss,
                    "Tie" => MatchResult::Tie,
                    val => panic!("Invalid Result value: {}", val)
                };

                let moves = menu.numeric_value("Moves") as i32;

                Ok(Some(MatchRecord {
                    user_id: None, // Will get filled in later
                    finished_at,
                    game_id,
                    cpu_level,
                    moves,
                    result
                }))
            },
            "Cancel" => Ok(None),
            val => panic!("Invalid Action value: {}", val)
        }
    }
}

pub struct MatchRecordIdMenu;

impl ToMenu for MatchRecordIdMenu {
    type Params = ();

    fn to_menu(_: &Self::Params, prev_err: Option<()>) -> Vec<TerminalMenuItem> {
        let mut menu = vec![
            required_field(terminal_menu::string("ID", "", false), prev_err.is_some()),
            terminal_menu::button("Confirm"),
            terminal_menu::button("Cancel")
        ];

        if prev_err.is_some() {
            menu.push(terminal_menu::label(error_text("Invalid ID (must be positive integer)")))
        }

        menu
    }
}

impl FromMenu for MatchRecordIdMenu {
    type Output = Option<i32>;
    type Error = ();

    fn from_menu(menu: &mut TerminalMenuStruct) -> MResult<Self> {
        match menu.selected_item_name() {
            "Confirm" => {
                match i32::from_str(menu.selection_value("ID")) {
                    Ok(val) => {
                        if val >= 0 {
                            Ok(Some(val))
                        } else {
                            Err(())
                        }
                    },
                    Err(_) => {
                        Err(())
                    }
                }
            },
            "Cancel" => Ok(None),
            val => panic!("Invalid Action value: {}", val)
        }
    }
}

pub struct ListRecordsMenu;

impl ToMenu for ListRecordsMenu {
    type Params = crate::types::PartialList<(i32, shared_types::types::MatchRecord)>;

    fn to_menu(list: &crate::types::PartialList<(i32, shared_types::types::MatchRecord)>, _: NoErr) -> Vec<TerminalMenuItem> {
        let mut menu = Vec::with_capacity(list.items.len() + 4);
        menu.push(
            terminal_menu::label(
                format!(
                    "{:5} | {:8} | {:8} | {:8} | {:<5} | {:8} | {:8}",
                    "ID",
                    "Date",
                    "Time",
                    "Game",
                    "Moves",
                    "Opponent",
                    "Result"
                )
            )
        );

        menu.extend(list.items.iter().map(
            |(id, record)| {
                use shared_types::types::{GameType, CpuLevel, MatchResult};

                terminal_menu::label(
                    format!(
                        "{:<5} | {:8} | {:8} | {:8} | {:<5} | {:8} | {:8}",
                        id,
                        record.finished_at.with_timezone(&chrono::Local).format("%d/%m/%y"),
                        record.finished_at.with_timezone(&chrono::Local).format("%H:%M:%S"),
                        match record.game_id {
                            GameType::OttoToot => "OttoToot",
                            GameType::Connect4 => "Connect4"
                        },
                        record.moves,
                        match record.cpu_level {
                            CpuLevel::Easy => "Easy",
                            CpuLevel::Medium => "Medium",
                            CpuLevel::Hard => "Hard"
                        },
                        match record.result {
                            MatchResult::Win => "Win",
                            MatchResult::Loss => "Loss",
                            MatchResult::Tie => "Tie"
                        }
                    )
                )
            }
        ));

        let last_index = list.offset + list.items.len() as i64;

        menu.push(
            terminal_menu::label(format!(
                "Showing {} to {} of {}",
                list.offset + 1,
                last_index,
                list.total
            ))
        );

        if last_index < list.total {
            menu.push(terminal_menu::button("Next"));
        }

        if list.offset > 0 {
            menu.push(terminal_menu::button("Prev"));
        }

        menu.push(terminal_menu::button("Done"));

        menu
    }
}

impl FromMenu for ListRecordsMenu {
    type Output = Option<crate::types::ListNav>;
    type Error = Infallible;

    fn from_menu(menu: &mut TerminalMenuStruct) -> MResult<Self> {
        use crate::types::ListNav;
        match menu.selected_item_name() {
            "Next" => Ok(Some(ListNav::Next)),
            "Prev" => Ok(Some(ListNav::Prev)),
            "Done" => Ok(None),
            val => panic!("Invalid Records List Action value: {}", val)
        }
    }
}

pub struct ListUserMenu;

impl ToMenu for ListUserMenu {
    type Params = crate::types::PartialList<String>;

    fn to_menu(list: &crate::types::PartialList<String>, _: NoErr) -> Vec<TerminalMenuItem> {
        let mut menu = Vec::with_capacity(list.items.len() + 4);

        menu.extend(list.items.iter().map(
            |id| terminal_menu::label(id)
        ));

        let last_index = list.offset + list.items.len() as i64;

        menu.push(
            terminal_menu::label(format!(
                "Showing {} to {} of {}",
                list.offset + 1,
                last_index,
                list.total
            ))
        );

        if last_index < list.total {
            menu.push(terminal_menu::button("Next"));
        }

        if list.offset > 0 {
            menu.push(terminal_menu::button("Prev"));
        }

        menu.push(terminal_menu::button("Done"));

        menu
    }
}

impl FromMenu for ListUserMenu {
    type Output = Option<crate::types::ListNav>;
    type Error = Infallible;

    fn from_menu(menu: &mut TerminalMenuStruct) -> MResult<Self> {
        use crate::types::ListNav;
        match menu.selected_item_name() {
            "Next" => Ok(Some(ListNav::Next)),
            "Prev" => Ok(Some(ListNav::Prev)),
            "Done" => Ok(None),
            val => panic!("Invalid Records List Action value: {}", val)
        }
    }
}
