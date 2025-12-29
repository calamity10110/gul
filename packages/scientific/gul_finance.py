"""
GUL Finance
Financial calculations (TVM).

Status: ✅ Implemented
Priority: Low
"""

__version__ = "0.1.0"
__all__ = ['fv', 'pv', 'pmt', 'npv']

def fv(rate, nper, pmt, pv, type=0):
    """Future Value"""
    if rate == 0:
        return -1 * (pv + pmt * nper)
    temp = (1 + rate)**nper
    if type == 1:
        return -1 * (pv * temp + pmt * (1 + rate) * (temp - 1) / rate)
    return -1 * (pv * temp + pmt * (temp - 1) / rate)

def pv(rate, nper, pmt, fv=0, type=0):
    """Present Value"""
    if rate == 0:
        return -1 * (fv + pmt * nper)
    temp = (1 + rate)**nper
    if type == 1:
        return -1 * (fv + pmt * (1 + rate) * (temp - 1) / rate) / temp
    return -1 * (fv + pmt * (temp - 1) / rate) / temp

def pmt(rate, nper, pv, fv=0, type=0):
    """Payment"""
    if rate == 0:
        return -1 * (fv + pv) / nper
    temp = (1 + rate)**nper
    if type == 1:
        return -1 * (fv + pv * temp) * rate / ((1 + rate) * (temp - 1))
    return -1 * (fv + pv * temp) * rate / (temp - 1)

def npv(rate, values):
    """Net Present Value"""
    return sum(v / (1 + rate)**(t+1) for t, v in enumerate(values))
