# Scientific Computing with GUL Tutorial

Perform advanced scientific computations, simulations, and numerical analysis with GUL.

## 🧮 Numerical Computing

### Linear Algebra

```gul
import std.science.linalg as la

# Create matrices
A = la.Matrix([
    [1, 2, 3],
    [4, 5, 6],
    [7, 8, 9]
])

B = la.Matrix([
    [9, 8, 7],
    [6, 5, 4],
    [3, 2, 1]
])

# Matrix operations
C = A + B
D = A * B
det = A.determinant()
inv = A.inverse()
eigen = A.eigenvalues()

# Solve linear system Ax = b
b = la.Vector([1, 2, 3])
x = la.solve(A, b)
```

### Differential Equations

```gul
import std.science.ode

# Define ODE: dy/dt = -k*y
fn exponential_decay(t, y, k):
    return -k * y

# Solve ODE
solution = ode.solve_ivp(
    exponential_decay,
    t_span=(0, 10),
    y0=[1.0],
    args=[0.5],
    method="RK45"
)

print(solution.t)  # Time points
print(solution.y)  # Solution values
```

### Integration and Differentiation

```gul
import std.science.calculus as calc

# Numerical integration
result = calc.integrate(
    fn(x): x**2,
    lower=0,
    upper=10
)

# Numerical differentiation
derivative = calc.differentiate(
    fn(x): x**3,
    x=2.0
)

# Gradient
gradient = calc.gradient(
    fn(x, y): x**2 + y**2,
    point=[1.0, 1.0]
)
```

## 🔬 Data Analysis

### Statistical Tests

```gul
import std.science.stats as stats

data1 = [1.2, 2.3, 1.9, 2.1, 1.8]
data2 = [2.1, 3.2, 2.8, 3.0, 2.9]

# T-test
t_statistic, p_value = stats.t_test(data1, data2)
print(f"t = {t_statistic}, p = {p_value}")

# Correlation
correlation = stats.pearson_correlation(data1, data2)

# Chi-square test
chi2, p = stats.chi_square_test(observed, expected)
```

### Machine Learning

```gul
import std.science.ml as ml

# Linear regression
X = [[1], [2], [3], [4], [5]]
y = [2, 4, 6, 8, 10]

model = ml.LinearRegression()
model.fit(X, y)

predictions = model.predict([[6], [7]])
print(predictions)  # [12, 14]

# Clustering
data = [[1, 2], [1.5, 1.8], [5, 8], [8, 8], [1, 0.6], [9, 11]]
kmeans = ml.KMeans(n_clusters=2)
labels = kmeans.fit_predict(data)
```

## 📊 Simulations

### Monte Carlo Simulation

```gul
import std.random
import std.math

fn estimate_pi(n_samples: int): float:
    inside_circle = 0

    for i in range(n_samples):
        x = random.random()
        y = random.random()

        if x**2 + y**2 <= 1:
            inside_circle += 1

    return 4.0 * inside_circle / n_samples

main:
    pi_estimate = estimate_pi(1_000_000)
    print(f"π ≈ {pi_estimate}")
```

### Physics Simulation

```gul
import std.science.physics

# Projectile motion
projectile = physics.Projectile(
    velocity=50,  # m/s
    angle=45,     # degrees
    height=0      # m
)

# Simulate trajectory
trajectory = projectile.simulate(dt=0.01)

for point in trajectory:
    print(f"t={point.t}, x={point.x}, y={point.y}")
```

## 🎯 Complete Example: Population Model

```gul
import std.science.ode
import std.plot

# Logistic growth model
fn logistic_growth(t, y, r, K):
    P = y[0]
    dP_dt = r * P * (1 - P / K)
    return [dP_dt]

main:
    # Parameters
    growth_rate = 0.1
    carrying_capacity = 1000
    initial_pop = 10

    # Solve ODE
    solution = ode.solve_ivp(
        logistic_growth,
        t_span=(0, 100),
        y0=[initial_pop],
        args=[growth_rate, carrying_capacity],
        t_eval=range(0, 101)
    )

    # Plot results
    plot.line(
        x=solution.t,
        y=solution.y[0],
        title="Population Growth",
        xlabel="Time",
        ylabel="Population"
    ).save("population.png")

    print(f"Final population: {solution.y[0][-1]:.0f}")
```

## 🔬 Advanced Topics

### Optimization

```gul
import std.science.optimize

# Minimize function
fn rosenbrock(x):
    return (1 - x[0])**2 + 100*(x[1] - x[0]**2)**2

result = optimize.minimize(
    rosenbrock,
    x0=[0, 0],
    method="BFGS"
)

print(f"Minimum at: {result.x}")
print(f"Function value: {result.fun}")
```

### Fast Fourier Transform

```gul
import std.science.fft

# Generate signal
t = linspace(0, 1, 1000)
signal = sin(2 * pi * 50 * t) + 0.5 * sin(2 * pi * 120 * t)

# Compute FFT
frequencies = fft.fft(signal)
power_spectrum = abs(frequencies)**2

# Find dominant frequencies
peaks = fft.find_peaks(power_spectrum)
```

---

**Last Updated**: 2025-12-28  
**Version**: 1.0.0  
**License**: MIT
