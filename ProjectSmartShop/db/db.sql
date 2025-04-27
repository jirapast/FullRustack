-- Create demo_items table
CREATE TABLE IF NOT EXISTS demo_items (
    id INTEGER PRIMARY KEY AUTOINCREMENT,
    name TEXT NOT NULL
);

INSERT INTO demo_items (name) VALUES ("alice");
INSERT INTO demo_items (name) VALUES ("bob");
