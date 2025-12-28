# Math Science

**Version**: 0.13.0 | **Syntax**: v3.2 | **Updated**: 2025-12-28

---

# Math & Science Module API Reference

The `std.math` and `std.science` modules provide comprehensive mathematical and scientific computing capabilities.

## 🔢 Basic Math Operations

```gul
import std.math

# Constants
pi = math.PI
e = math.E
tau = math.TAU

# Basic functions
abs_val = math.abs(-5)              # 5
ceil_val = math.ceil(4.2)           # 5
floor_val = math.floor(4.8)         # 4
round_val = math.round(4.5)         # 5
sqrt_val = math.sqrt(16)            # 4.0
power = math.pow(2, 8)              # 256
```

## 📐 Trigonometry

```gul
# Angles in radians
sin_val = math.sin(math.PI / 2)     # 1.0
cos_val = math.cos(0)               # 1.0
tan_val = math.tan(math.PI / 4)     # 1.0

# Inverse trig
asin_val = math.asin(1)
acos_val = math.acos(0)
atan_val = math.atan(1)
atan2_val = math.atan2(y, x)

# Degrees/radians conversion
degrees = math.degrees(math.PI)     # 180
radians = math.radians(180)         # π
```

## 📊 Statistics

```gul
import std.stats

data = vec[1, 2, 3, 4, 5, 6, 7, 8, 9, 10]

# Central tendency
mean = stats.mean(data)             # 5.5
median = stats.median(data)         # 5.5
mode = stats.mode(data)

# Dispersion
variance = stats.variance(data)
std_dev = stats.std_dev(data)
range_val = stats.range(data)

# Distribution
percentile_75 = stats.percentile(data, 75)
quartiles = stats.quartiles(data)
```

## 🔬 Scientific Computing

```gul
import std.science

# Linear algebra
matrix = science.Matrix([
    [1, 2, 3],
    [4, 5, 6],
    [7, 8, 9]
])

# Matrix operations
determinant = matrix.det()
inverse = matrix.inverse()
transpose = matrix.transpose()
eigvals = matrix.eigenvalues()

# Vector operations
v1 = science.Vector([1, 2, 3])
v2 = science.Vector([4, 5, 6])

dot_product = v1.dot(v2)
cross_product = v1.cross(v2)
magnitude = v1.magnitude()
normalized = v1.normalize()
```

## 📈 Data Analysis

```gul
import std.dataframe

# Create DataFrame
df = dataframe.DataFrame({
    "name": ["Alice", "Bob", "Charlie"],
    "age": [25, 30, 35],
    "salary": [50000, 60000, 70000]
})

# Basic operations
print(df.head())
print(df.describe())

# Filtering
young_people = df.filter(fn(row): row["age"] < 30)

# Aggregation
avg_salary = df["salary"].mean()
total_salary = df["salary"].sum()

# Grouping
by_age = df.group_by("age").agg({
    "salary": "mean"
})
```

## 🧮 Complex Numbers

```gul
import std.complex

z1 = complex.Complex(3, 4)          # 3 + 4i
z2 = complex.Complex(1, 2)          # 1 + 2i

# Operations
sum = z1 + z2
product = z1 * z2
magnitude = z1.abs()
phase = z1.phase()
conjugate = z1.conjugate()
```

## 🎲 Random Numbers

```gul
import std.random

# Random integers
rand_int = random.randint(1, 100)

# Random floats
rand_float = random.random()        # 0.0 to 1.0
rand_uniform = random.uniform(1.0, 10.0)

# Random from distribution
normal = random.gauss(mean=0, std=1)

# Random choice
choice = random.choice([1, 2, 3, 4, 5])
sample = random.sample([1, 2, 3, 4, 5], k=3)

# Shuffle
items = [1, 2, 3, 4, 5]
random.shuffle(items)
```

## 📚 See Also

- [Scientific Computing Tutorial](../tutorials/scientific-computing.md)
- [Data Analysis Tutorial](../tutorials/data-analysis.md)

---

**Last Updated**: 2025-12-28  
**Version: 0.13.0  
**License**: MIT
