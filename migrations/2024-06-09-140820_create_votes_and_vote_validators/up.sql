CREATE TABLE votes (
    id INTEGER PRIMARY KEY AUTOINCREMENT,
    name TEXT NOT NULL,
    owner_id INTEGER NOT NULL,
    is_finished BOOLEAN NOT NULL,
    FOREIGN KEY (owner_id) REFERENCES users(id)
);

CREATE TABLE vote_validators (
    vote_id INTEGER NOT NULL,
    val_endpoint TEXT NOT NULL,
    FOREIGN KEY (vote_id) REFERENCES votes(id)
);