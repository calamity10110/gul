"""
GUL Scipy (Polyfill)
Scientific computing utilities (Stats/Optimize).

Status: ✅ Implemented
Priority: Medium
"""

import math

__version__ = "0.1.0"
__all__ = ['minimize', 'normal_pdf', 'normal_cdf']

def minimize(func, x0, learning_rate=0.01, iterations=100):
    """Simple Gradient Descent Minimizer (Numerical Gradient)"""
    x = x0
    h = 1e-5
    for _ in range(iterations):
        # Calc gradient needed for scalar x for simplicity
        # f'(x) ~ (f(x+h) - f(x-h)) / 2h
        grad = (func(x + h) - func(x - h)) / (2 * h)
        x -= learning_rate * grad
    return x

def normal_pdf(x, mu=0, sigma=1):
    """Normal Probability Density Function"""
    return (1 / (sigma * math.sqrt(2 * math.pi))) * math.exp(-0.5 * ((x - mu) / sigma)**2)

def normal_cdf(x, mu=0, sigma=1):
    """Normal Cumulative Distribution Function (Error function approximation)"""
    return 0.5 * (1 + math.erf((x - mu) / (sigma * math.sqrt(2))))
