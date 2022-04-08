use ansi_term::{Style, Colour};

use diesel::Connection;
use diesel::SqliteConnection;

use shared_types::queries::*;
use shared_types::models::*;

use types::*;
use menus::*;

mod types;
mod menus;

fn print_info(input: String) {
    println!("{}", Colour::Blue.paint(input))
}

fn print_err(input: String) {
    eprintln!("{}", Colour::Red.bold().paint(input))
}

fn main() {
    println!("{}\n", Style::new().bold().paint("Project 3 Admin Console"));

    let path = ConnectionConfigMenu::prompt_default();

    print!("Connecting to local database at {}... ",
        Colour::Cyan.underline().paint(&path),
    );

    let conn = SqliteConnection::establish(&path).expect("Unable to connect to database");
    
    println!("{}", Style::new().bold().paint("Connected!"));

    print!("Running database migrations... ");

    run_migrations(&conn).expect("Database migration failed");

    println!("{}", Style::new().bold().paint("Finished!"));

    loop {
        let action = ActionMenu::prompt_default();
        match action {
            Actions::AddUser => add_user(&conn),
            Actions::DeleteUser => delete_user(&conn),
            Actions::ListUsers => list_users(&conn),
            Actions::AddRecord => add_record(&conn),
            Actions::DeleteRecord => delete_record(&conn),
            Actions::ListUserRecords => list_user_records(&conn),
            Actions::Exit => break
        }

        std::thread::sleep(std::time::Duration::from_secs(2));
    }

    // TODO: Maybe perform check to ensure no operations are currently waiting for the database and wait here while alerting the user
    println!("{}", Style::new().bold().paint("Exiting..."));
}

fn add_user(conn: &SqliteConnection) {
    let new_user = NewUserMenu::prompt_default();
    
    if let Some(NewUser { username, password }) = new_user {
        match UserModel::generate_new(username, password) {
            Ok(model) => match users::add(conn, model) {
                Ok(()) => print_info(format!("User added successfully")),
                Err(err) => print_err(format!("{:?}", err))
            },
            Err(err) => print_err(format!("{:?}", err))
        };
        
    }
}

fn delete_user(conn: &SqliteConnection) {
    let id = UserIdMenu::prompt_default();

    if let Some(id) = id {
        print_info(format!("Deleting user: {}", id));

        match users::delete(conn, &id) {
            Ok(()) => print_info(format!("User deleted successfully")),
            Err(err) => print_err(format!("{:?}", err))
        }
    }
}

fn list_users(conn: &SqliteConnection) {

    while let Some(id) = UserIdMenu::prompt_default() {
        let mut offset = 0;
        let step = 10;
        loop {
            let list = match users::find_users(conn, &id) {
                Ok((users, total)) => PartialList {
                    items: users,
                    offset,
                    total
                },
                Err(err) => {
                    print_err(format!("{:?}", err));
                    break;
                }
            };

            match ListUserMenu::prompt(&list) {
                Some(ListNav::Next) => offset += step,
                Some(ListNav::Prev) => offset -= step,
                None => break
            }
        }
    }

}

fn add_record(conn: &SqliteConnection) {
    use shared_types::types::UserAuthToken;

    if let Some(record) = UserIdMenu::prompt_default().and_then(|id| MatchRecordMenu::prompt_default().map(|record| (UserAuthToken::from(id), record))) {
        match match_records::add(conn, MatchRecordModel::from(record)) {
            Ok(()) => print_info(format!("Record added successfully")),
            Err(err) => print_err(format!("{:?}", err))
        }
    }
}

fn delete_record(conn: &SqliteConnection) {
    if let Some(id) = MatchRecordIdMenu::prompt_default() {
        match match_records::delete(conn, id) {
            Ok(()) => print_info(format!("Record deleted successfully")),
            Err(err) => print_err(format!("{:?}", err))
        }
    }
}

fn list_user_records(conn: &SqliteConnection) {
    use shared_types::types::MatchQuerySortBy;

    if let Some(id) = UserIdMenu::prompt_default() {
        let mut offset = 0;
        let step = 10;
        loop {
            let records = match match_records::find_by_user(
                conn,
                &id,
                None,
                MatchQuerySortBy::StartTime,
                false,
                None,
                None,
                step,
                offset
            ) {
                Ok((records, total)) => PartialList {
                    items: records.into_iter()
                        .map(|model| (model.get_id().unwrap(), model.as_record()))
                        .collect(),
                    offset,
                    total
                },
                Err(err) => {
                    eprintln!("{:?}", err);
                    break;
                }
            };

            match ListRecordsMenu::prompt(&records) {
                Some(ListNav::Next) => offset += step,
                Some(ListNav::Prev) => offset -= step,
                None => break
            }
        }
    }
}
