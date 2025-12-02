import sqlite3
import os

def init_db():
    db_file = "voting.db"
    migration_file = "migrations/20240101000000_initial_setup.sql"
    
    # Read migration
    with open(migration_file, 'r') as f:
        sql_script = f.read()
        
    # Connect to DB (creates it)
    conn = sqlite3.connect(db_file)
    cursor = conn.cursor()
    
    # Execute SQL
    cursor.executescript(sql_script)
    
    conn.commit()
    conn.close()
    print(f"Database {db_file} initialized successfully.")

if __name__ == "__main__":
    init_db()
