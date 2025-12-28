# Data Analysis with GUL Tutorial

Learn to perform data analysis, statistical computing, and visualization with GUL.

## 📊 Data Analysis Basics

### Loading Data

```gul
import std.dataframe as df
import std.filesystem as fs

# Load CSV
data = df.read_csv("sales_data.csv")

# Load JSON
data = df.read_json("data.json")

# From database
import std.database
db = database.connect("postgresql://localhost/mydb")
data = df.from_query(db, "SELECT * FROM sales")
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
data_clean = data.drop_na()
data_filled = data.fill_na(0)

# Remove duplicates
data_unique = data.drop_duplicates()

# Filter rows
filtered = data.filter(fn(row): row["age"] > 18)

# Select columns
subset = data.select(["name", "age", "city"])
```

### Aggregation and Grouping

```gul
# Group by and aggregate
by_city = data.group_by("city").agg({
    "sales": "sum",
    "count": "count",
    "avg_age": fn(group): group["age"].mean()
})

# Multiple aggregations
summary = data.agg({
    "total_sales": fn(d): d["sales"].sum(),
    "avg_sales": fn(d): d["sales"].mean(),
    "max_sales": fn(d): d["sales"].max()
})
```

## 📈 Statistical Analysis

```gul
import std.stats

# Descriptive statistics
mean = stats.mean(data["sales"])
median = stats.median(data["sales"])
std_dev = stats.std_dev(data["sales"])

# Correlation
corr = stats.correlation(data["age"], data["sales"])

# Linear regression
model = stats.linear_regression(
    x=data["years_experience"],
    y=data["salary"]
)
print(f"Slope: {model.slope}, Intercept: {model.intercept}")
print(f"R²: {model.r_squared}")
```

## 📉 Visualization

```gul
import std.plot

# Line plot
plt = plot.line(
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
import std.dataframe as df
import std.stats
import std.plot

main:
    # Load data
    sales = df.read_csv("monthly_sales.csv")

    # Clean data
    sales_clean = sales.drop_na().drop_duplicates()

    # Analysis
    monthly_totals = sales_clean.group_by("month").agg({
        "total_sales": fn(g): g["amount"].sum(),
        "avg_sale": fn(g): g["amount"].mean()
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

**Last Updated**: 2025-12-28  
**Version**: 1.0.0  
**License**: MIT
