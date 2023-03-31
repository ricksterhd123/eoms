pub const DB_SCHEMA: &str = r"
    CREATE TABLE IF NOT EXISTS orders (
        id INTEGER PRIMARY KEY AUTOINCREMENT, 
        ref VARCHAR(255)
    );
";
