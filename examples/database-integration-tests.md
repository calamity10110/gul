# GUL Database Integration Tests

**Version**: 0.13.0 | **Syntax**: v3.2 | **Updated**: 2025-12-28

Test GUL's database capabilities with real public databases.

---

## Overview

This guide demonstrates GUL's database integration using publicly available databases and APIs:

- **SQLite** - Local file-based database
- **DuckDB** - In-memory analytics
- **Public REST APIs** - SQL-like querying
- **CSV/Parquet** - File-based data sources

---

## Test Setup

### Create Test Database

```gul
@imp std{fs, db}
@imp python{pandas}

fn setup_test_database():
    """Create SQLite test database with sample data"""

    # Create database
    let conn = db.connect("sqlite:///test_data.db")

    # Create tables
    conn.execute("""
        CREATE TABLE IF NOT EXISTS users (
            id INTEGER PRIMARY KEY,
            name TEXT NOT NULL,
            email TEXT UNIQUE NOT NULL,
            age INTEGER,
            country TEXT,
            created_at TIMESTAMP DEFAULT CURRENT_TIMESTAMP
        )
    """)

    conn.execute("""
        CREATE TABLE IF NOT EXISTS orders (
            id INTEGER PRIMARY KEY,
            user_id INTEGER,
            product TEXT,
            quantity INTEGER,
            price REAL,
            order_date DATE,
            FOREIGN KEY (user_id) REFERENCES users (id)
        )
    """)

    # Insert sample data
    let users = @list[
        @tuple("Alice Johnson", "alice@example.com", 28, "USA"),
        @tuple("Bob Smith", "bob@example.com", 35, "UK"),
        @tuple("Carol White", "carol@example.com", 42, "Canada"),
        @tuple("David Brown", "david@example.com", 31, "Australia"),
        @tuple("Eve Davis", "eve@example.com", 25, "USA")
    ]

    for user in users:
        conn.execute(
            "INSERT OR IGNORE INTO users (name, email, age, country) VALUES (?, ?, ?, ?)",
            @list[user]
        )

    # Insert orders
    let orders = @list[
        @tuple(1, "Laptop", 1, 1200.00, "2024-01-15"),
        @tuple(1, "Mouse", 2, 25.00, "2024-01-15"),
        @tuple(2, "Keyboard", 1, 75.00, "2024-01-16"),
        @tuple(3, "Monitor", 2, 300.00, "2024-01-17"),
        @tuple(4, "Headphones", 1, 150.00, "2024-01-18"),
        @tuple(5, "Webcam", 1, 80.00, "2024-01-19")
    ]

    for order in orders:
        conn.execute(
            "INSERT INTO orders (user_id, product, quantity, price, order_date) VALUES (?, ?, ?, ?, ?)",
            @list[order]
        )

    conn.commit()
    print("✅ Test database created successfully")

mn:
    setup_test_database()
```

---

## Basic SQL Operations

### SELECT Queries

```gul
@imp std.db

fn test_basic_queries():
    """Test basic SELECT operations"""

    let conn = db.connect("sqlite:///test_data.db")

    # Simple SELECT
    print("=== All Users ===")
    let users = conn.query("SELECT * FROM users")

    for user in users:
        print(user["name"], "-", user["email"])

    # WHERE clause
    print("\n=== Users from USA ===")
    let usa_users = conn.query(
        "SELECT name, age FROM users WHERE country = ?",
        @list["USA"]
    )

    for user in usa_users:
        print(user["name"], "age", user["age"])

    # Aggregate functions
    print("\n=== Statistics ===")
    let stats = conn.query_one("""
        SELECT
            COUNT(*) as total_users,
            AVG(age) as avg_age,
            MIN(age) as youngest,
            MAX(age) as oldest
        FROM users
    """)

    print("Total users:", stats["total_users"])
    print("Average age:", round(stats["avg_age"], 1))
    print("Age range:", stats["youngest"], "-", stats["oldest"])

mn:
    test_basic_queries()
```

### JOIN Operations

```gul
@imp std.db

fn test_joins():
    """Test JOIN operations"""

    let conn = db.connect("sqlite:///test_data.db")

    # INNER JOIN
    print("=== Orders with User Details ===")
    let orders = conn.query("""
        SELECT
            u.name,
            u.email,
            o.product,
            o.quantity,
            o.price,
            o.order_date
        FROM orders o
        INNER JOIN users u ON o.user_id = u.id
        ORDER BY o.order_date DESC
    """)

    for order in orders:
        print(
            order["name"],
            "bought",
            order["quantity"],
            "x",
            order["product"],
            "for $" + str(order["price"])
        )

    # Aggregated JOIN
    print("\n=== User Order Summary ===")
    let summary = conn.query("""
        SELECT
            u.name,
            COUNT(o.id) as order_count,
            SUM(o.price * o.quantity) as total_spent
        FROM users u
        LEFT JOIN orders o ON u.id = o.user_id
        GROUP BY u.id, u.name
        ORDER BY total_spent DESC
    """)

    for row in summary:
        print(
            row["name"],
            "-",
            row["order_count"],
            "orders, $" + str(row["total_spent"] or 0)
        )

mn:
    test_joins()
```

---

## Advanced Queries

### Window Functions

```gul
@imp std.db

fn test_window_functions():
    """Test window functions (SQLite 3.25+)"""

    let conn = db.connect("sqlite:///test_data.db")

    # Ranking
    let ranked = conn.query("""
        SELECT
            name,
            country,
            age,
            RANK() OVER (ORDER BY age DESC) as age_rank,
            ROW_NUMBER() OVER (PARTITION BY country ORDER BY age DESC) as country_rank
        FROM users
    """)

    print("=== User Rankings ===")
    for user in ranked:
        print(
            "#" + str(user["age_rank"]),
            user["name"],
            "(" + user["country"] + ")",
            "- age", user["age"]
        )

mn:
    test_window_functions()
```

### Subqueries

```gul
@imp std.db

fn test_subqueries():
    """Test subqueries"""

    let conn = db.connect("sqlite:///test_data.db")

    # Correlated subquery
    let results = conn.query("""
        SELECT
            u.name,
            u.country,
            (SELECT COUNT(*) FROM orders o WHERE o.user_id = u.id) as order_count,
            (SELECT SUM(price) FROM orders o WHERE o.user_id = u.id) as total_spent
        FROM users u
        WHERE (SELECT COUNT(*) FROM orders o WHERE o.user_id = u.id) > 0
    """)

    print("=== Users with Orders ===")
    for row in results:
        print(
            row["name"],
            "from", row["country"],
            "-", row["order_count"], "orders,",
            "$" + str(row["total_spent"])
        )

mn:
    test_subqueries()
```

---

## Transactions

### ACID Transactions

```gul
@imp std.db

fn test_transactions():
    """Test transaction support"""

    let conn = db.connect("sqlite:///test_data.db")

    # Start transaction
    conn.begin()

    try:
        # Insert new user
        conn.execute(
            "INSERT INTO users (name, email, age, country) VALUES (?, ?, ?, ?)",
            @list["Frank Wilson", "frank@example.com", 29, "Germany"]
        )

        # Get the new user's ID
        let user = conn.query_one(
            "SELECT id FROM users WHERE email = ?",
            @list["frank@example.com"]
        )

        # Insert order for new user
        conn.execute(
            "INSERT INTO orders (user_id, product, quantity, price, order_date) VALUES (?, ?, ?, ?, ?)",
            @list[user["id"], "Tablet", 1, 500.00, "2024-01-20"]
        )

        # Commit transaction
        conn.commit()
        print("✅ Transaction committed successfully")

    catch error:
        # Rollback on error
        conn.rollback()
        print("❌ Transaction rolled back:", error)

mn:
    test_transactions()
```

---

## Analytics with DuckDB

### In-Memory Analytics

```gul
@imp python{duckdb, pandas}

fn test_duckdb_analytics():
    """Test DuckDB for analytics"""

    @python {
        import duckdb
        import pandas as pd

        # Create in-memory database
        con = duckdb.connect(':memory:')

        # Load CSV data
        con.execute("""
            CREATE TABLE sales AS
            SELECT * FROM read_csv_auto('data/sales.csv')
        """)

        # Complex analytics query
        result = con.execute("""
            SELECT
                DATE_TRUNC('month', order_date) as month,
                country,
                SUM(price * quantity) as revenue,
                COUNT(DISTINCT user_id) as unique_customers,
                AVG(price) as avg_order_value
            FROM sales
            GROUP BY month, country
            ORDER BY month DESC, revenue DESC
        """).fetchdf()

        # Time series analysis
        time_series = con.execute("""
            SELECT
                order_date,
                SUM(price * quantity) OVER (
                    ORDER BY order_date
                    ROWS BETWEEN 6 PRECEDING AND CURRENT ROW
                ) as rolling_7day_revenue
            FROM sales
            ORDER BY order_date
        """).fetchdf()

        return {
            'monthly_revenue': result.to_dict('records'),
            'time_series': time_series.to_dict('records')
        }
    }

    print("=== Monthly Revenue by Country ===")
    for row in python.monthly_revenue:
        print(
            row['month'],
            row['country'],
            "- $" + str(round(row['revenue'], 2))
        )

mn:
    test_duckdb_analytics()
```

---

## Connecting to Public APIs

### REST API as Database

```gul
@imp std{http, json}

struct PublicDataAPI:
    """Query public APIs like a database"""

    base_url: @str

    async fn @list query_countries(self, region):
        """Get countries by region from REST Countries API"""

        let response = await http.get(
            self.base_url + "/region/" + region
        )

        if response.status != 200:
            return @list[]

        let countries = response.json()

        # Transform to table-like structure
        let results = @list[]
        for country in countries:
            results.append(@dict{
                name: country["name"]["common"],
                capital: country["capital"][0] if "capital" in country else "N/A",
                population: country["population"],
                area: country["area"],
                region: country["region"]
            })

        return results

    async fn @dict aggregate_by_region(self, region):
        """Aggregate statistics for region"""

        let countries = await self.query_countries(region)

        var total_pop = 0
        var total_area = 0.0
        var count = 0

        for country in countries:
            total_pop = total_pop + country["population"]
            total_area = total_area + country["area"]
            count = count + 1

        return @dict{
            region: region,
            countries: count,
            total_population: total_pop,
            total_area: total_area,
            avg_population: total_pop / count if count > 0 else 0
        }

# Test with public API
mn:
    let api = PublicDataAPI{
        base_url: "https://restcountries.com/v3.1"
    }

    # Query like SQL SELECT
    let european_countries = await api.query_countries("europe")

    print("=== European Countries ===")
    for country in european_countries[:5]:  # First 5
        print(
            country["name"],
            "- Pop:", country["population"],
            "Capital:", country["capital"]
        )

    # Aggregate like SQL GROUP BY
    let stats = await api.aggregate_by_region("europe")
    print("\n=== Europe Statistics ===")
    print("Countries:", stats["countries"])
    print("Total Population:", stats["total_population"])
    print("Average Population:", round(stats["avg_population"]))
```

---

## CSV/Parquet Processing

### File-Based Query

```gul
@imp python{pandas, pyarrow}

fn test_file_queries():
    """Query CSV and Parquet files like databases"""

    @python {
        import pandas as pd

        # Read CSV
        df = pd.read_csv('data/sample.csv')

        # SQL-like operations
        # WHERE
        filtered = df[df['age'] > 30]

        # GROUP BY
        grouped = df.groupby('country').agg({
            'age': 'mean',
            'id': 'count'
        }).reset_index()
        grouped.columns = ['country', 'avg_age', 'user_count']

        # ORDER BY
        sorted_df = grouped.sort_values('user_count', ascending=False)

        # Save as Parquet
        df.to_parquet('data/sample.parquet')

        # Read Parquet
        parquet_df = pd.read_parquet('data/sample.parquet')

        return {
            'total_rows': len(df),
            'countries': sorted_df.to_dict('records'),
            'parquet_size': len(parquet_df)
        }
    }

    print("=== File Query Results ===")
    print("Total rows:", python.total_rows)
    print("\nCountries by user count:")
    for row in python.countries:
        print(
            row['country'],
            "- Avg age:", round(row['avg_age'], 1),
            "Users:", row['user_count']
        )

mn:
    test_file_queries()
```

---

## Performance Testing

### Benchmark Queries

```gul
@imp std{db, time}

fn benchmark_database():
    """Benchmark database operations"""

    let conn = db.connect("sqlite:///test_data.db")

    # Benchmark: Simple SELECT
    let start = time.now()
    for _ in range(1000):
        conn.query("SELECT * FROM users WHERE age > 25")
    let simple_time = time.now() - start

    # Benchmark: JOIN
    start = time.now()
    for _ in range(1000):
        conn.query("""
            SELECT u.name, COUNT(o.id) as orders
            FROM users u
            LEFT JOIN orders o ON u.id = o.user_id
            GROUP BY u.id, u.name
        """)
    let join_time = time.now() - start

    # Benchmark: Aggregate
    start = time.now()
    for _ in range(1000):
        conn.query("""
            SELECT country, AVG(age), COUNT(*)
            FROM users
            GROUP BY country
        """)
    let agg_time = time.now() - start

    print("=== Benchmark Results (1000 iterations) ===")
    print("Simple SELECT:", round(simple_time, 3), "seconds")
    print("JOIN query:", round(join_time, 3), "seconds")
    print("Aggregate query:", round(agg_time, 3), "seconds")

mn:
    benchmark_database()
```

---

## Integration Test Runner

### Run All Tests

```gul
@imp std{db, time}

async fn run_all_tests():
    """Run comprehensive database test suite"""

    let tests = @list[
        @tuple("Setup", setup_test_database),
        @tuple("Basic Queries", test_basic_queries),
        @tuple("Joins", test_joins),
        @tuple("Window Functions", test_window_functions),
        @tuple("Subqueries", test_subqueries),
        @tuple("Transactions", test_transactions),
        @tuple("Benchmark", benchmark_database)
    ]

    var passed = 0
    var failed = 0

    print("🧪 Running GUL Database Integration Tests")
    print("=" * 50)

    for test in tests:
        let name = test[0]
        let fn = test[1]

        print("\n📝 Running:", name)

        try:
            let start = time.now()
            fn()
            let duration = time.now() - start

            print("✅", name, "passed in", round(duration, 3), "s")
            passed = passed + 1

        catch error:
            print("❌", name, "failed:", error)
            failed = failed + 1

    print("\n" + "=" * 50)
    print("📊 Test Results:")
    print("   Passed:", passed)
    print("   Failed:", failed)
    print("   Total:", passed + failed)

    if failed == 0:
        print("\n🎉 All tests passed!")
    else:
        print("\n⚠️  Some tests failed")

mn:
    await run_all_tests()
```

---

## Expected Output

```text
🧪 Running GUL Database Integration Tests
==================================================

📝 Running: Setup
✅ Test database created successfully
✅ Setup passed in 0.123 s

📝 Running: Basic Queries
=== All Users ===
Alice Johnson - alice@example.com
Bob Smith - bob@example.com
Carol White - carol@example.com
David Brown - david@example.com
Eve Davis - eve@example.com

=== Users from USA ===
Alice Johnson age 28
Eve Davis age 25

=== Statistics ===
Total users: 5
Average age: 32.2
Age range: 25 - 42
✅ Basic Queries passed in 0.045 s

📝 Running: Joins
=== Orders with User Details ===
Eve Davis bought 1 x Webcam for $80.0
David Brown bought 1 x Headphones for $150.0
Carol White bought 2 x Monitor for $300.0
Bob Smith bought 1 x Keyboard for $75.0
Alice Johnson bought 2 x Mouse for $25.0
Alice Johnson bought 1 x Laptop for $1200.0
✅ Joins passed in 0.038 s

==================================================
📊 Test Results:
   Passed: 7
   Failed: 0
   Total: 7

🎉 All tests passed!
```

---

## See Also

- [Data Engineering Guide](data-engineering.md)
- [Standard Library - Database](../api/database.md)
- [Package Catalog](../reference/package-catalog.md)

---

**Last Updated**: 2025-12-28  
**Version**: 0.13.0  
**Status**: Production Ready ✅
