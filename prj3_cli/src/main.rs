use ansi_term::{Style, Colour};

use diesel::Connection;
use diesel::SqliteConnection;

use shared_types::queries::run_migrations;

use menus::*;

mod menus;

fn main() {
    println!("{}\n", Style::new().bold().paint("Project 3 Admin Console"));

    let config = ConnectionConfig::prompt();

    print!("Connecting to {} as {}... ",
        Colour::Cyan.underline().paint(&config.address),
        match config.method {
            ConnectionType::NetworkAPI => panic!("Networked API connections are not supported yet"),
            ConnectionType::DirectToDatabase => "local database"
        }
    );

    let conn = SqliteConnection::establish(&config.address).expect("Unable to connect to database");
    
    println!("{}", Style::new().bold().paint("Connected!"));

    print!("Running database migrations... ");

    run_migrations(&conn).expect("Database migration failed");

    println!("{}", Style::new().bold().paint("Finished!"));

    loop {
        let action = Actions::prompt();
        match action {
            Actions::AddUser => {},
            Actions::DeleteUser => {},
            Actions::ChangeUserPassword => {},
            Actions::AddRecord => {},
            Actions::CheckRecords => {},
            Actions::Exit => break
        }
    }

    println!("Exiting...");
}

fn add_user(conn: &SqliteConnection) {

}

fn delete_user(conn: &SqliteConnection) {
    
}

fn change_user_password(conn: &SqliteConnection) {
    
}

fn add_record(conn: &SqliteConnection) {
    
}

fn check_records(conn: &SqliteConnection) {
    
}
