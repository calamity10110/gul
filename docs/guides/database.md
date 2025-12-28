# Database

**Version**: 0.13.0 | **Syntax**: v3.2 | **Updated**: 2025-12-28

---

# Database Module API Reference

The `std.database` module provides a unified interface for database operations across multiple database systems including PostgreSQL, MySQL, SQLite, MongoDB, and Redis.

## 📚 Supported Databases

- **SQL**: PostgreSQL, MySQL, SQLite, Microsoft SQL Server
- **NoSQL**: MongoDB, Redis, CouchDB
- **Time-Series**: InfluxDB, TimescaleDB
- **Graph**: Neo4j

## 🔌 Connections

### Connect to Database

```gul
import std.database

# PostgreSQL
db = database.connect("postgresql://user:password@localhost:5432/mydb")

# MySQL
db = database.connect("mysql://user:password@localhost:3306/mydb")

# SQLite
db = database.connect("sqlite:///path/to/database.db")
db_memory = database.connect("sqlite::memory:")

# MongoDB
db = database.connect("mongodb://localhost:27017/mydb")

# Redis
db = database.connect("redis://localhost:6379/0")
```

### Connection Pooling

```gul
pool = database.ConnectionPool(
    url="postgresql://user:password@localhost:5432/mydb",
    min_connections=5,
    max_connections=20,
    timeout=duration.seconds(30)
)

conn = pool.get_connection()
result = conn.query("SELECT * FROM users")
pool.release(conn)
```

## 📝 SQL Operations

### Query Execution

```gul
# Simple query
users = db.query("SELECT * FROM users")

# Query with parameters
user = db.query_one("SELECT * FROM users WHERE id = ?", [user_id])

# Named parameters
users = db.query(
    "SELECT * FROM users WHERE age > :age AND city = :city",
    {"age": 18, "city": "New York"}
)
```

### Insert, Update, Delete

```gul
# Insert
user_id = db.insert("users", {
    "name": "Alice",
    "email": "alice@example.com",
    "created_at": datetime.now()
})

# Update
db.update("users", {"age": 31}, where={"id": user_id})

# Delete
db.delete("users", where={"id": user_id})
```

## 🎯 Query Builder

```gul
query = db.table("users")
    .select("id", "name", "email")
    .where("age", ">", 18)
    .order_by("created_at", "DESC")
    .limit(10)

results = query.get()
```

## 🏗️ ORM

```gul
import std.database.orm

@orm.model
struct User:
    id: int @primary_key @auto_increment
    name: str @required
    email: str @unique
    created_at: datetime @default(datetime.now)

# CRUD
user = User.create(name="Alice", email="alice@example.com")
user = User.find(1)
users = User.all()
user.delete()
```

## 📊 Transactions

```gul
db.transaction(fn():
    db.insert("users", {"name": "Alice"})
    db.insert("profiles", {"user_id": user_id})
)
```

**Last Updated**: 2025-12-28  
**Version: 0.13.0  
**License**: MIT
