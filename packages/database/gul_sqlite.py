"""
GUL SQLite
SQLite database wrapper.

Status: ✅ Implemented
Priority: High
"""

import sqlite3
from typing import List, Dict, Any, Optional

__version__ = "0.1.0"
__all__ = ['Database', 'connect']

class Database:
    """
    SQLite Database Wrapper
    
    Example:
        db = Database("mydb.sqlite")
        db.execute("CREATE TABLE users (name TEXT)")
        db.insert("users", {"name": "Alice"})
        users = db.query("SELECT * FROM users")
    """
    
    def __init__(self, path: str = ":memory:"):
        self.path = path
        self.conn = sqlite3.connect(path, check_same_thread=False)
        self.conn.row_factory = sqlite3.Row
        
    def execute(self, sql: str, params: tuple = ()) -> sqlite3.Cursor:
        """Execute raw SQL"""
        with self.conn:
            return self.conn.execute(sql, params)
            
    def query(self, sql: str, params: tuple = ()) -> List[Dict[str, Any]]:
        """Query and return list of dicts"""
        cursor = self.conn.execute(sql, params)
        return [dict(row) for row in cursor.fetchall()]
        
    def query_one(self, sql: str, params: tuple = ()) -> Optional[Dict[str, Any]]:
        """Query single row"""
        cursor = self.conn.execute(sql, params)
        row = cursor.fetchone()
        return dict(row) if row else None
        
    def insert(self, table: str, data: Dict[str, Any]) -> int:
        """Insert dict into table"""
        keys = ", ".join(data.keys())
        placeholders = ", ".join(["?" for _ in data])
        values = tuple(data.values())
        
        sql = f"INSERT INTO {table} ({keys}) VALUES ({placeholders})"
        cursor = self.execute(sql, values)
        return cursor.lastrowid
        
    def update(self, table: str, data: Dict[str, Any], where: str, params: tuple = ()):
        """Update rows"""
        set_clause = ", ".join([f"{k} = ?" for k in data.keys()])
        values = tuple(data.values()) + params
        
        sql = f"UPDATE {table} SET {set_clause} WHERE {where}"
        self.execute(sql, values)
        
    def delete(self, table: str, where: str, params: tuple = ()):
        """Delete rows"""
        sql = f"DELETE FROM {table} WHERE {where}"
        self.execute(sql, params)
        
    def close(self):
        self.conn.close()

def connect(path: str = ":memory:") -> Database:
    return Database(path)
