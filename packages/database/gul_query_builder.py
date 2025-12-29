"""
GUL Query Builder
Type-safe SQL query builder.

Status: ✅ Implemented
Priority: Critical
"""

from typing import List, Optional, Any, Tuple, Dict
from enum import Enum

__version__ = "0.1.0"
__all__ = ['Query', 'JoinType', 'OrderDirection']

class JoinType(Enum):
    INNER = "INNER JOIN"
    LEFT = "LEFT JOIN"
    RIGHT = "RIGHT JOIN"
    FULL = "FULL OUTER JOIN"

class OrderDirection(Enum):
    ASC = "ASC"
    DESC = "DESC"

class Query:
    """
    SQL query builder
    
    Example:
        # SELECT
        query = Query().select("users", ["id", "name", "email"])
        query.where("age", ">", 18)
        query.order_by("name", OrderDirection.ASC)
        query.limit(10)
        
        sql, params = query.build()
        # SELECT id, name, email FROM users WHERE age > ? ORDER BY name ASC LIMIT 10
        
        # INSERT
        query = Query().insert("users", {
            "name": "Alice",
            "email": "alice@example.com",
            "age": 30
        })
        
        # UPDATE
        query = Query().update("users", {"email": "newemail@example.com"})
        query.where("id", "=", 123)
        
        # DELETE
        query = Query().delete("users")
        query.where("status", "=", "inactive")
        
        # JOIN
        query = Query().select("users", ["users.name", "orders.total"])
        query.join("orders", "users.id", "orders.user_id", JoinType.LEFT)
        query.where("orders.status", "=", "completed")
    """
    
    def __init__(self):
        self._type: Optional[str] = None
        self._table: Optional[str] = None
        self._columns: List[str] = []
        self._values: Dict[str, Any] = {}
        self._where_clauses: List[Tuple[str, str, Any]] = []
        self._joins: List[Tuple[JoinType, str, str, str]] = []
        self._group_by: List[str] = []
        self._having: List[Tuple[str, str, Any]] = []
        self._order_by: List[Tuple[str, OrderDirection]] = []
        self._limit_value: Optional[int] = None
        self._offset_value: Optional[int] = None
        self._params: List[Any] = []
    
    def select(self, table: str, columns: Optional[List[str]] = None) -> 'Query':
        """SELECT query"""
        self._type = "SELECT"
        self._table = table
        self._columns = columns or ["*"]
        return self
    
    def insert(self, table: str, values: Dict[str, Any]) -> 'Query':
        """INSERT query"""
        self._type = "INSERT"
        self._table = table
        self._values = values
        return self
    
    def update(self, table: str, values: Dict[str, Any]) -> 'Query':
        """UPDATE query"""
        self._type = "UPDATE"
        self._table = table
        self._values = values
        return self
    
    def delete(self, table: str) -> 'Query':
        """DELETE query"""
        self._type = "DELETE"
        self._table = table
        return self
    
    def where(self, column: str, operator: str, value: Any) -> 'Query':
        """WHERE clause"""
        self._where_clauses.append((column, operator, value))
        return self
    
    def where_in(self, column: str, values: List[Any]) -> 'Query':
        """WHERE IN clause"""
        placeholders = ", ".join("?" * len(values))
        self._where_clauses.append((column, "IN", f"({placeholders})"))
        self._params.extend(values)
        return self
    
    def where_null(self, column: str, is_null: bool = True) -> 'Query':
        """WHERE NULL clause"""
        op = "IS NULL" if is_null else "IS NOT NULL"
        self._where_clauses.append((column, op, None))
        return self
    
    def join(self, table: str, left_col: str, right_col: str, join_type: JoinType = JoinType.INNER) -> 'Query':
        """JOIN clause"""
        self._joins.append((join_type, table, left_col, right_col))
        return self
    
    def group_by(self, *columns: str) -> 'Query':
        """GROUP BY clause"""
        self._group_by.extend(columns)
        return self
    
    def having(self, column: str, operator: str, value: Any) -> 'Query':
        """HAVING clause"""
        self._having.append((column, operator, value))
        return self
    
    def order_by(self, column: str, direction: OrderDirection = OrderDirection.ASC) -> 'Query':
        """ORDER BY clause"""
        self._order_by.append((column, direction))
        return self
    
    def limit(self, n: int) -> 'Query':
        """LIMIT clause"""
        self._limit_value = n
        return self
    
    def offset(self, n: int) -> 'Query':
        """OFFSET clause"""
        self._offset_value = n
        return self
    
    def build(self) -> Tuple[str, List[Any]]:
        """Build SQL query"""
        self._params = []
        
        if self._type == "SELECT":
            sql = self._build_select()
        elif self._type == "INSERT":
            sql = self._build_insert()
        elif self._type == "UPDATE":
            sql = self._build_update()
        elif self._type == "DELETE":
            sql = self._build_delete()
        else:
            raise ValueError("Query type not set")
        
        return sql, self._params
    
    def _build_select(self) -> str:
        """Build SELECT query"""
        cols = ", ".join(self._columns)
        sql = f"SELECT {cols} FROM {self._table}"
        
        # JOINs
        for join_type, table, left_col, right_col in self._joins:
            sql += f" {join_type.value} {table} ON {left_col} = {right_col}"
        
        # WHERE
        sql += self._build_where()
        
        # GROUP BY
        if self._group_by:
            sql += " GROUP BY " + ", ".join(self._group_by)
        
        # HAVING
        if self._having:
            having_parts = []
            for col, op, val in self._having:
                having_parts.append(f"{col} {op} ?")
                self._params.append(val)
            sql += " HAVING " + " AND ".join(having_parts)
        
        # ORDER BY
        if self._order_by:
            order_parts = [f"{col} {direction.value}" for col, direction in self._order_by]
            sql += " ORDER BY " + ", ".join(order_parts)
        
        # LIMIT/OFFSET
        if self._limit_value:
            sql += f" LIMIT {self._limit_value}"
        if self._offset_value:
            sql += f" OFFSET {self._offset_value}"
        
        return sql
    
    def _build_insert(self) -> str:
        """Build INSERT query"""
        columns = list(self._values.keys())
        placeholders = ", ".join("?" * len(columns))
        
        sql = f"INSERT INTO {self._table} ({', '.join(columns)}) VALUES ({placeholders})"
        self._params.extend(self._values.values())
        
        return sql
    
    def _build_update(self) -> str:
        """Build UPDATE query"""
        set_parts = [f"{col} = ?" for col in self._values.keys()]
        sql = f"UPDATE {self._table} SET {', '.join(set_parts)}"
        self._params.extend(self._values.values())
        
        sql += self._build_where()
        
        return sql
    
    def _build_delete(self) -> str:
        """Build DELETE query"""
        sql = f"DELETE FROM {self._table}"
        sql += self._build_where()
        return sql
    
    def _build_where(self) -> str:
        """Build WHERE clause"""
        if not self._where_clauses:
            return ""
        
        where_parts = []
        for col, op, val in self._where_clauses:
            if val is None:
                where_parts.append(f"{col} {op}")
            elif op == "IN":
                where_parts.append(f"{col} {op} {val}")
            else:
                where_parts.append(f"{col} {op} ?")
                self._params.append(val)
        
        return " WHERE " + " AND ".join(where_parts)
    
    def __str__(self) -> str:
        """String representation"""
        sql, params = self.build()
        return f"{sql} -- params: {params}"
