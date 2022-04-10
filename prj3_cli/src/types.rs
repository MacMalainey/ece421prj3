/// Main actions user can perform
pub enum Actions {
    AddUser,
    DeleteUser,
    ListUsers,
    AddRecord,
    DeleteRecord,
    ListUserRecords,
    Exit
}

/// Return type for NewUserMenu, contains inputted username and unencrypted password
pub struct NewUser {
    pub username: String,
    pub password: String
}

/// Identical to Records<T>, used for List menus
pub struct PartialList<T> {
    pub items: Vec<T>,
    pub offset: i64,
    pub total: i64
}

/// Navigation returned from List menus
pub enum ListNav {
    Next,
    Prev
}
