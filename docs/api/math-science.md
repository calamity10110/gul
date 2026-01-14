# Math Science

**Version**: 0.14.0-dev | **Syntax**: v3.2 | **Updated**: 2026-01-08

---

# Math & Science Module API Reference

The `std.math` and `std.science` modules provide comprehensive mathematical and scientific computing capabilities.

## 🔢 Basic Math Operations

```gul
@imp std.math

# Constants
let pi = math.PI
let e = math.E
let tau = math.TAU

# Basic functions
let abs_val = math.abs(-5)              # 5
let ceil_val = math.ceil(4.2)           # 5
let floor_val = math.floor(4.8)         # 4
let round_val = math.round(4.5)         # 5
let sqrt_val = math.sqrt(16)            # 4.0
let power = math.pow(2, 8)              # 256
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
@imp std.stats

let data = @list[1, 2, 3, 4, 5, 6, 7, 8, 9, 10]

# Central tendency
let mean = stats.mean(data)             # 5.5
let median = stats.median(data)         # 5.5
let mode = stats.mode(data)

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

## 🧠 Autograd (Automatic Differentiation)

GUL provides native automatic differentiation support for machine learning and scientific optimization. These are available as global built-in functions.

```gul
# Initialize tape
autograd_begin()

# Create tracked variables
let x = make_var(2.0)
let y = make_var(3.0)

# Build computational graph
# z = x^2 + x*y
let x2 = var_mul(x, x)
let xy = var_mul(x, y)
let z = var_add(x2, xy)

print(f"Forward value: {var_val(z)}") # 10.0

# Compute gradients
backward(z)

print(f"dz/dx: {var_grad(x)}") # 7.0 (2x + y = 2*2 + 3)
print(f"dz/dy: {var_grad(y)}") # 2.0 (x = 2)

# Cleanup
autograd_end()
```

### Autograd API

| Function | Description |
| -------- | ----------- |
| `autograd_begin()` | Starts the autograd tape. |
| `autograd_end()` | Clears the autograd tape and releases memory. |
| `make_var(val)` | Creates a new variable tracked by the tape. |
| `var_val(v)` | Retrieves the current value of a tracked variable. |
| `var_grad(v)` | Retrieves the gradient of a tracked variable after `backward()`. |
| `var_add(a, b)` | Tracks addition. |
| `var_mul(a, b)` | Tracks multiplication. |
| `var_sin(v)` | Tracks sine function. |
| `backward(v)` | Computes gradients via backpropagation starting from `v`. |

## 📚 See Also

- [Scientific Computing Tutorial](../tutorials/scientific-computing.md)
- [Data Analysis Tutorial](../tutorials/data-analysis.md)

---

**Last Updated**: 2026-01-08  
**Version**: 0.14.0-dev  
**License**: MIT
