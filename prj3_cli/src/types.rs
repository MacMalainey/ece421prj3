pub enum Actions {
    AddUser,
    DeleteUser,
    ListUsers,
    AddRecord,
    DeleteRecord,
    ListUserRecords,
    Exit
}

pub struct NewUser {
    pub username: String,
    pub password: String
}

pub struct PartialList<T> {
    pub items: Vec<T>,
    pub offset: i64,
    pub total: i64
}

pub enum ListNav {
    Next,
    Prev
}
