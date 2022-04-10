-- Your SQL goes here
CREATE TABLE match_records (
    id INTEGER PRIMARY KEY,
    user_id TEXT,
    finished_at DATETIME NOT NULL,
    game_id INTEGER NOT NULL,
    cpu_level INTEGER NOT NULL,
    moves INTEGER NOT NULL,
    result INTEGER NOT NULL,
    CHECK (
        (game_id == 1 OR game_id == 2) AND
        (cpu_level == 3 OR cpu_level == 6 OR cpu_level == 9) AND
        (result <= 1 OR result >= -1)
    ),
    FOREIGN KEY(user_id) REFERENCES users(user_id)
);