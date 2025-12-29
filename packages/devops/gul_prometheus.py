"""
GUL Prometheus Integration
Metrics collection and exposition for Prometheus.

Status: ✅ Implemented
Priority: Critical
Phase: 1
"""

from typing import Dict, List, Optional, Callable
from dataclasses import dataclass, field
from collections import defaultdict
import time
from threading import Lock

__version__ = "0.1.0"
__all__ = ['Counter', 'Gauge', 'Histogram', 'Summary', 'Registry', 'exposition']

class Metric:
    """Base metric class"""
    
    def __init__(self, name: str, help_text: str, labels: Optional[List[str]] = None):
        self.name = name
        self.help_text = help_text
        self.labels = labels or []
        self._lock = Lock()

class Counter(Metric):
    """
    Counter metric - monotonically increasing value
    
    Example:
        requests = Counter('http_requests_total', 'Total HTTP requests', ['method', 'status'])
        requests.inc({'method': 'GET', 'status': '200'})
        requests.inc({'method': 'POST', 'status': '201'}, 5)
    """
    
    def __init__(self, name: str, help_text: str, labels: Optional[List[str]] = None):
        super().__init__(name, help_text, labels)
        self._values: Dict[tuple, float] = defaultdict(float)
    
    def inc(self, labels: Optional[Dict[str, str]] = None, amount: float = 1.0):
        """Increment counter"""
        if amount < 0:
            raise ValueError("Counter can only increase")
        
        label_key = self._make_key(labels)
        with self._lock:
            self._values[label_key] += amount
    
    def _make_key(self, labels: Optional[Dict[str, str]]) -> tuple:
        """Create hashable key from labels"""
        if not labels:
            return ()
        return tuple(sorted(labels.items()))
    
    def collect(self) -> List[str]:
        """Collect metric samples"""
        with self._lock:
            lines = [
                f"# HELP {self.name} {self.help_text}",
                f"# TYPE {self.name} counter"
            ]
            
            for label_key, value in self._values.items():
                if label_key:
                    label_str = ",".join(f'{k}="{v}"' for k, v in label_key)
                    lines.append(f"{self.name}{{{label_str}}} {value}")
                else:
                    lines.append(f"{self.name} {value}")
            
            return lines

class Gauge(Metric):
    """
    Gauge metric - value that can go up or down
    
    Example:
        memory = Gauge('memory_usage_bytes', 'Memory usage in bytes')
        memory.set(1024 * 1024)
        memory.inc(512)
        memory.dec(256)
    """
    
    def __init__(self, name: str, help_text: str, labels: Optional[List[str]] = None):
        super().__init__(name, help_text, labels)
        self._values: Dict[tuple, float] = defaultdict(float)
    
    def set(self, value: float, labels: Optional[Dict[str, str]] = None):
        """Set gauge to value"""
        label_key = self._make_key(labels)
        with self._lock:
            self._values[label_key] = value
    
    def inc(self, labels: Optional[Dict[str, str]] = None, amount: float = 1.0):
        """Increment gauge"""
        label_key = self._make_key(labels)
        with self._lock:
            self._values[label_key] += amount
    
    def dec(self, labels: Optional[Dict[str, str]] = None, amount: float = 1.0):
        """Decrement gauge"""
        label_key = self._make_key(labels)
        with self._lock:
            self._values[label_key] -= amount
    
    def _make_key(self, labels: Optional[Dict[str, str]]) -> tuple:
        if not labels:
            return ()
        return tuple(sorted(labels.items()))
    
    def collect(self) -> List[str]:
        """Collect metric samples"""
        with self._lock:
            lines = [
                f"# HELP {self.name} {self.help_text}",
                f"# TYPE {self.name} gauge"
            ]
            
            for label_key, value in self._values.items():
                if label_key:
                    label_str = ",".join(f'{k}="{v}"' for k, v in label_key)
                    lines.append(f"{self.name}{{{label_str}}} {value}")
                else:
                    lines.append(f"{self.name} {value}")
            
            return lines

class Histogram(Metric):
    """
    Histogram metric - observations in buckets
    
    Example:
        latency = Histogram('http_request_duration_seconds', 'HTTP request latency')
        latency.observe(0.25)
        latency.observe(0.5)
    """
    
    DEFAULT_BUCKETS = [0.005, 0.01, 0.025, 0.05, 0.075, 0.1, 0.25, 0.5, 0.75, 1.0, 2.5, 5.0, 7.5, 10.0]
    
    def __init__(
        self,
        name: str,
        help_text: str,
        labels: Optional[List[str]] = None,
        buckets: Optional[List[float]] = None
    ):
        super().__init__(name, help_text, labels)
        self.buckets = sorted(buckets or self.DEFAULT_BUCKETS)
        self._sums: Dict[tuple, float] = defaultdict(float)
        self._counts: Dict[tuple, int] = defaultdict(int)
        self._buckets: Dict[tuple, Dict[float, int]] = defaultdict(
            lambda: {b: 0 for b in self.buckets}
        )
    
    def observe(self, value: float, labels: Optional[Dict[str, str]] = None):
        """Record an observation"""
        label_key = self._make_key(labels)
        
        with self._lock:
            self._sums[label_key] += value
            self._counts[label_key] += 1
            
            for bucket in self.buckets:
                if value <= bucket:
                    self._buckets[label_key][bucket] += 1
    
    def _make_key(self, labels: Optional[Dict[str, str]]) -> tuple:
        if not labels:
            return ()
        return tuple(sorted(labels.items()))
    
    def collect(self) -> List[str]:
        """Collect metric samples"""
        with self._lock:
            lines = [
                f"# HELP {self.name} {self.help_text}",
                f"# TYPE {self.name} histogram"
            ]
            
            for label_key in set(list(self._sums.keys()) + list(self._counts.keys())):
                base_labels = dict(label_key) if label_key else {}
                
                # Bucket counts
                cumulative = 0
                for bucket in self.buckets:
                    cumulative += self._buckets[label_key].get(bucket, 0)
                    bucket_labels = {**base_labels, 'le': str(bucket)}
                    label_str = ",".join(f'{k}="{v}"' for k, v in sorted(bucket_labels.items()))
                    lines.append(f"{self.name}_bucket{{{label_str}}} {cumulative}")
                
                # +Inf bucket
                inf_labels = {**base_labels, 'le': '+Inf'}
                label_str = ",".join(f'{k}="{v}"' for k, v in sorted(inf_labels.items()))
                lines.append(f"{self.name}_bucket{{{label_str}}} {self._counts[label_key]}")
                
                # Sum and count
                if label_key:
                    label_str = ",".join(f'{k}="{v}"' for k, v in label_key)
                    lines.append(f"{self.name}_sum{{{label_str}}} {self._sums[label_key]}")
                    lines.append(f"{self.name}_count{{{label_str}}} {self._counts[label_key]}")
                else:
                    lines.append(f"{self.name}_sum {self._sums[label_key]}")
                    lines.append(f"{self.name}_count {self._counts[label_key]}")
            
            return lines

class Registry:
    """
    Metrics registry
    
    Example:
        registry = Registry()
        
        requests = Counter('requests_total', 'Total requests')
        registry.register(requests)
        
        requests.inc()
        
        print(registry.exposition())
    """
    
    def __init__(self):
        self._metrics: List[Metric] = []
        self._lock = Lock()
    
    def register(self, metric: Metric) -> Metric:
        """Register a metric"""
        with self._lock:
            self._metrics.append(metric)
        return metric
    
    def unregister(self, metric: Metric):
        """Unregister a metric"""
        with self._lock:
            self._metrics.remove(metric)
    
    def exposition(self) -> str:
        """Generate Prometheus exposition format"""
        with self._lock:
            lines = []
            for metric in self._metrics:
                lines.extend(metric.collect())
                lines.append("")
            return "\n".join(lines)

# Default registry
default_registry = Registry()

def exposition() -> str:
    """Get exposition from default registry"""
    return default_registry.exposition()

# Decorators
def track_duration(histogram: Histogram, labels: Optional[Dict[str, str]] = None):
    """
    Decorator to track function duration
    
    Example:
        latency = Histogram('func_duration_seconds', 'Function duration')
        
        @track_duration(latency)
        def my_function():
            time.sleep(1)
    """
    def decorator(func: Callable) -> Callable:
        def wrapper(*args, **kwargs):
            start = time.time()
            try:
                return func(*args, **kwargs)
            finally:
                duration = time.time() - start
                histogram.observe(duration, labels)
        return wrapper
    return decorator

def count_calls(counter: Counter, labels: Optional[Dict[str, str]] = None):
    """
    Decorator to count function calls
    
    Example:
        calls = Counter('function_calls_total', 'Total function calls')
        
        @count_calls(calls)
        def my_function():
            pass
    """
    def decorator(func: Callable) -> Callable:
        def wrapper(*args, **kwargs):
            counter.inc(labels)
            return func(*args, **kwargs)
        return wrapper
    return decorator

# Common metrics
class CommonMetrics:
    """Pre-defined common metrics"""
    
    @staticmethod
    def http_metrics(registry: Optional[Registry] = None) -> Dict[str, Metric]:
        """HTTP server metrics"""
        reg = registry or default_registry
        
        return {
            'requests': reg.register(Counter(
                'http_requests_total',
                'Total HTTP requests',
                ['method', 'status', 'path']
            )),
            'duration': reg.register(Histogram(
                'http_request_duration_seconds',
                'HTTP request duration'
            )),
            'in_progress': reg.register(Gauge(
                'http_requests_in_progress',
                'HTTP requests in progress'
            ))
        }
    
    @staticmethod
    def system_metrics(registry: Optional[Registry] = None) -> Dict[str, Metric]:
        """System metrics"""
        reg = registry or default_registry
        
        return {
            'cpu': reg.register(Gauge('system_cpu_usage', 'CPU usage percentage')),
            'memory': reg.register(Gauge('system_memory_bytes', 'Memory usage in bytes')),
            'disk': reg.register(Gauge('system_disk_bytes', 'Disk usage in bytes'))
        }
