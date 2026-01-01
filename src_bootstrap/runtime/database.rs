#![allow(dead_code)]
// Database interface (SQLite)
use rusqlite::{Connection, Row};
use std::collections::HashMap;

pub struct Database {
    conn: Connection,
}

impl Database {
    /// Create a new database connection
    pub fn new(path: &str) -> Result<Self, String> {
        let conn = Connection::open(path)
            .map_err(|e| format!("Failed to open database '{}': {}", path, e))?;

        Ok(Database { conn })
    }

    /// Create an in-memory database
    pub fn in_memory() -> Result<Self, String> {
        let conn = Connection::open_in_memory()
            .map_err(|e| format!("Failed to create in-memory database: {}", e))?;

        Ok(Database { conn })
    }

    /// Execute a SQL statement (INSERT, UPDATE, DELETE, CREATE, etc.)
    pub fn execute(&self, sql: &str, params: Vec<SqlValue>) -> Result<usize, String> {
        let params_refs: Vec<&dyn rusqlite::ToSql> =
            params.iter().map(|v| v as &dyn rusqlite::ToSql).collect();

        self.conn
            .execute(sql, params_refs.as_slice())
            .map_err(|e| format!("Failed to execute SQL: {}", e))
    }

    /// Query database and return rows
    pub fn query(
        &self,
        sql: &str,
        params: Vec<SqlValue>,
    ) -> Result<Vec<HashMap<String, SqlValue>>, String> {
        let params_refs: Vec<&dyn rusqlite::ToSql> =
            params.iter().map(|v| v as &dyn rusqlite::ToSql).collect();

        let mut stmt = self
            .conn
            .prepare(sql)
            .map_err(|e| format!("Failed to prepare statement: {}", e))?;

        let column_names: Vec<String> = stmt.column_names().iter().map(|s| s.to_string()).collect();

        let rows = stmt
            .query_map(params_refs.as_slice(), |row| {
                Ok(Self::row_to_hashmap(row, &column_names))
            })
            .map_err(|e| format!("Failed to query: {}", e))?;

        let mut result = Vec::new();
        for row in rows {
            result.push(row.map_err(|e| format!("Failed to process row: {}", e))?);
        }

        Ok(result)
    }

    /// Query a single row
    pub fn query_one(
        &self,
        sql: &str,
        params: Vec<SqlValue>,
    ) -> Result<Option<HashMap<String, SqlValue>>, String> {
        let mut results = self.query(sql, params)?;
        Ok(results.pop())
    }

    /// Begin a transaction
    pub fn begin_transaction(&mut self) -> Result<(), String> {
        self.execute("BEGIN TRANSACTION", vec![]).map(|_| ())
    }

    /// Commit a transaction
    pub fn commit(&mut self) -> Result<(), String> {
        self.execute("COMMIT", vec![]).map(|_| ())
    }

    /// Rollback a transaction
    pub fn rollback(&mut self) -> Result<(), String> {
        self.execute("ROLLBACK", vec![]).map(|_| ())
    }

    /// Get the last inserted row ID
    pub fn last_insert_id(&self) -> i64 {
        self.conn.last_insert_rowid()
    }

    /// Check if a table exists
    pub fn table_exists(&self, table_name: &str) -> Result<bool, String> {
        let sql = "SELECT name FROM sqlite_master WHERE type='table' AND name=?";
        let result = self.query_one(sql, vec![SqlValue::Text(table_name.to_string())])?;
        Ok(result.is_some())
    }

    /// Get all table names
    pub fn list_tables(&self) -> Result<Vec<String>, String> {
        let sql = "SELECT name FROM sqlite_master WHERE type='table' ORDER BY name";
        let rows = self.query(sql, vec![])?;

        Ok(rows
            .iter()
            .filter_map(|row| {
                if let Some(SqlValue::Text(name)) = row.get("name") {
                    Some(name.clone())
                } else {
                    None
                }
            })
            .collect())
    }

    fn row_to_hashmap(row: &Row, column_names: &[String]) -> HashMap<String, SqlValue> {
        let mut map = HashMap::new();

        for (i, name) in column_names.iter().enumerate() {
            let value = if let Ok(v) = row.get::<_, i64>(i) {
                SqlValue::Integer(v)
            } else if let Ok(v) = row.get::<_, f64>(i) {
                SqlValue::Real(v)
            } else if let Ok(v) = row.get::<_, String>(i) {
                SqlValue::Text(v)
            } else if let Ok(v) = row.get::<_, Vec<u8>>(i) {
                SqlValue::Blob(v)
            } else {
                SqlValue::Null
            };

            map.insert(name.clone(), value);
        }

        map
    }
}

#[derive(Debug, Clone)]
pub enum SqlValue {
    Null,
    Integer(i64),
    Real(f64),
    Text(String),
    Blob(Vec<u8>),
}

impl rusqlite::ToSql for SqlValue {
    fn to_sql(&self) -> rusqlite::Result<rusqlite::types::ToSqlOutput<'_>> {
        match self {
            SqlValue::Null => Ok(rusqlite::types::ToSqlOutput::Owned(
                rusqlite::types::Value::Null,
            )),
            SqlValue::Integer(i) => Ok(rusqlite::types::ToSqlOutput::Owned(
                rusqlite::types::Value::Integer(*i),
            )),
            SqlValue::Real(r) => Ok(rusqlite::types::ToSqlOutput::Owned(
                rusqlite::types::Value::Real(*r),
            )),
            SqlValue::Text(s) => Ok(rusqlite::types::ToSqlOutput::Owned(
                rusqlite::types::Value::Text(s.clone()),
            )),
            SqlValue::Blob(b) => Ok(rusqlite::types::ToSqlOutput::Owned(
                rusqlite::types::Value::Blob(b.clone()),
            )),
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_database_creation() {
        let db = Database::in_memory().unwrap();
        assert!(db.list_tables().unwrap().is_empty());
    }

    #[test]
    fn test_create_table() {
        let db = Database::in_memory().unwrap();

        let sql = "CREATE TABLE users (id INTEGER PRIMARY KEY, name TEXT, age INTEGER)";
        db.execute(sql, vec![]).unwrap();

        assert!(db.table_exists("users").unwrap());
    }

    #[test]
    fn test_insert_and_query() {
        let db = Database::in_memory().unwrap();

        db.execute(
            "CREATE TABLE users (id INTEGER PRIMARY KEY, name TEXT, age INTEGER)",
            vec![],
        )
        .unwrap();

        db.execute(
            "INSERT INTO users (name, age) VALUES (?, ?)",
            vec![SqlValue::Text("Alice".to_string()), SqlValue::Integer(30)],
        )
        .unwrap();

        let rows = db.query("SELECT * FROM users", vec![]).unwrap();
        assert_eq!(rows.len(), 1);

        if let Some(SqlValue::Text(name)) = rows[0].get("name") {
            assert_eq!(name, "Alice");
        } else {
            panic!("Expected name to be Text");
        }
    }

    #[test]
    fn test_query_one() {
        let db = Database::in_memory().unwrap();

        db.execute(
            "CREATE TABLE users (id INTEGER PRIMARY KEY, name TEXT)",
            vec![],
        )
        .unwrap();
        db.execute(
            "INSERT INTO users (name) VALUES (?)",
            vec![SqlValue::Text("Bob".to_string())],
        )
        .unwrap();

        let row = db
            .query_one(
                "SELECT * FROM users WHERE name = ?",
                vec![SqlValue::Text("Bob".to_string())],
            )
            .unwrap();
        assert!(row.is_some());
    }

    #[test]
    fn test_last_insert_id() {
        let db = Database::in_memory().unwrap();

        db.execute(
            "CREATE TABLE users (id INTEGER PRIMARY KEY, name TEXT)",
            vec![],
        )
        .unwrap();
        db.execute(
            "INSERT INTO users (name) VALUES (?)",
            vec![SqlValue::Text("Charlie".to_string())],
        )
        .unwrap();

        let id = db.last_insert_id();
        assert_eq!(id, 1);
    }

    #[test]
    fn test_transaction() {
        let mut db = Database::in_memory().unwrap();

        db.execute(
            "CREATE TABLE users (id INTEGER PRIMARY KEY, name TEXT)",
            vec![],
        )
        .unwrap();

        db.begin_transaction().unwrap();
        db.execute(
            "INSERT INTO users (name) VALUES (?)",
            vec![SqlValue::Text("Dave".to_string())],
        )
        .unwrap();
        db.commit().unwrap();

        let rows = db.query("SELECT * FROM users", vec![]).unwrap();
        assert_eq!(rows.len(), 1);
    }

    #[test]
    fn test_rollback() {
        let mut db = Database::in_memory().unwrap();

        db.execute(
            "CREATE TABLE users (id INTEGER PRIMARY KEY, name TEXT)",
            vec![],
        )
        .unwrap();

        db.begin_transaction().unwrap();
        db.execute(
            "INSERT INTO users (name) VALUES (?)",
            vec![SqlValue::Text("Eve".to_string())],
        )
        .unwrap();
        db.rollback().unwrap();

        let rows = db.query("SELECT * FROM users", vec![]).unwrap();
        assert_eq!(rows.len(), 0);
    }

    #[test]
    fn test_list_tables() {
        let db = Database::in_memory().unwrap();

        db.execute("CREATE TABLE users (id INTEGER)", vec![])
            .unwrap();
        db.execute("CREATE TABLE posts (id INTEGER)", vec![])
            .unwrap();

        let tables = db.list_tables().unwrap();
        assert_eq!(tables.len(), 2);
        assert!(tables.contains(&"users".to_string()));
        assert!(tables.contains(&"posts".to_string()));
    }
}
