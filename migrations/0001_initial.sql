CREATE TABLE rocket_thruster (
    id INTEGER PRIMARY KEY AUTOINCREMENT,
    name TEXT NOT NULL,
    manufacturer TEXT NOT NULL,
    min_consumption_in_liter_per_second REAL NOT NULL,
    max_consumption_in_liter_per_second REAL NOT NULL,
    fuel_type TEXT NOT NULL
);
