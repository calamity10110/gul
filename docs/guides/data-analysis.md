# Data Analysis

**Version**: 0.14.0-dev | **Syntax**: v3.2 | **Updated**: 2026-01-08

---

# Data Analysis with GUL Tutorial

Learn to perform data analysis, statistical computing, and visualization with GUL.

## 📊 Data Analysis Basics

### Loading Data

```gul
@imp std.dataframe as df
@imp std.filesystem as fs

# Load CSV
let data = df.read_csv("sales_data.csv")

# Load JSON
let data = df.read_json("data.json")

# From database
@imp std.database
let db = database.connect("postgresql://localhost/mydb")
let data = df.from_query(db, "SELECT * FROM sales")
```

### Data Exploration

```gul
# View first rows
print(data.head(10))

# Summary statistics
print(data.describe())

# Data info
print(data.info())

# Column names
print(data.columns)

# Shape
print(f"Rows: {data.rows}, Columns: {data.cols}")
```

### Data Cleaning

```gul
# Handle missing values
let data_clean = data.drop_na()
let data_filled = data.fill_na(0)

# Remove duplicates
let data_unique = data.drop_duplicates()

# Filter rows
let filtered = data.filter((row) => row["age"] > 18)

# Select columns
let subset = data.select(@list["name", "age", "city"])
```

### Aggregation and Grouping

```gul
# Group by and aggregate
let by_city = data.group_by("city").agg(@dict{
    sales: "sum",
    count: "count",
    avg_age: (group) => group["age"].mean()
})

# Multiple aggregations
let summary = data.agg(@dict{
    total_sales: (d) => d["sales"].sum(),
    avg_sales: (d) => d["sales"].mean(),
    max_sales: (d) => d["sales"].max()
})
```

## 📈 Statistical Analysis

```gul
@imp std.stats

# Descriptive statistics
let mean = stats.mean(data["sales"])
let median = stats.median(data["sales"])
let std_dev = stats.std_dev(data["sales"])

# Correlation
let corr = stats.correlation(data["age"], data["sales"])

# Linear regression
let model = stats.linear_regression(
    x=data["years_experience"],
    y=data["salary"]
)
print(f"Slope: {model.slope}, Intercept: {model.intercept}")
print(f"R²: {model.r_squared}")
```

## 📉 Visualization

```gul
@imp std.plot

# Line plot
let plt = plot.line(
    x=data["date"],
    y=data["sales"],
    title="Sales Over Time"
)
plt.save("sales_trend.png")

# Bar chart
plot.bar(
    labels=data["product"],
    values=data["quantity"]
).save("products.png")

# Scatter plot
plot.scatter(
    x=data["age"],
    y=data["income"],
    title="Age vs Income"
).save("scatter.png")

# Histogram
plot.histogram(
    data=data["age"],
    bins=20
).save("age_distribution.png")
```

## 🎯 Complete Example

```gul
@imp std.dataframe as df
@imp std.stats
@imp std.plot

mn:
    # Load data
    let sales = df.read_csv("monthly_sales.csv")

    # Clean data
    let sales_clean = sales.drop_na().drop_duplicates()

    # Analysis
    let monthly_totals = sales_clean.group_by("month").agg(@dict{
        total_sales: (g) => g["amount"].sum(),
        avg_sale: (g) => g["amount"].mean()
    })

    # Statistics
    print(f"Total Revenue: ${monthly_totals['total_sales'].sum()}")
    print(f"Average Monthly: ${monthly_totals['total_sales'].mean()}")

    # Visualization
    plot.line(
        x=monthly_totals["month"],
        y=monthly_totals["total_sales"],
        title="Monthly Sales Trends"
    ).save("report.png")

    # Export results
    monthly_totals.to_csv("sales_summary.csv")
```

---

**Last Updated**: 2026-01-08  
**Version**: 0.14.0-dev  
**License**: MIT
