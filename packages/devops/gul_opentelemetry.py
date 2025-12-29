"""
GUL OpenTelemetry Integration
Distributed tracing and observability.

Status: ✅ Implemented
Priority: Critical
Phase: 1
"""

from typing import Dict, Optional, List, Callable, Any
from dataclasses import dataclass, field
from datetime import datetime
import time
import random

__version__ = "0.1.0"
__all__ = ['Tracer', 'Span', 'SpanContext', 'trace']

@dataclass
class SpanContext:
    """Span context for distributed tracing"""
    trace_id: str
    span_id: str
    parent_id: Optional[str] = None
    
    @staticmethod
    def generate_trace_id() -> str:
        """Generate trace ID"""
        return f"{random.getrandbits(128):032x}"
    
    @staticmethod
    def generate_span_id() -> str:
        """Generate span ID"""
        return f"{random.getrandbits(64):016x}"

class Span:
    """Trace span"""
    
    def __init__(
        self,
        name: str,
        context: SpanContext,
        kind: str = "internal"
    ):
        self.name = name
        self.context = context
        self.kind = kind
        self.start_time = time.time()
        self.end_time: Optional[float] = None
        self.attributes: Dict[str, Any] = {}
        self.events: List[Dict] = []
        self.status: str = "ok"
        self.error: Optional[str] = None
    
    def set_attribute(self, key: str, value: Any) -> 'Span':
        """Set span attribute"""
        self.attributes[key] = value
        return self
    
    def add_event(self, name: str, attributes: Optional[Dict] = None) -> 'Span':
        """Addspan event"""
        self.events.append({
            'name': name,
            'timestamp': time.time(),
            'attributes': attributes or {}
        })
        return self
    
    def set_status(self, status: str, error: Optional[str] = None) -> 'Span':
        """Set span status"""
        self.status = status
        if error:
            self.error = error
        return self
    
    def end(self):
        """End span"""
        self.end_time = time.time()
    
    def duration_ms(self) -> float:
        """Get duration in milliseconds"""
        end = self.end_time or time.time()
        return (end - self.start_time) * 1000
    
    def to_dict(self) -> Dict:
        """Convert to dictionary"""
        return {
            'name': self.name,
            'trace_id': self.context.trace_id,
            'span_id': self.context.span_id,
            'parent_id': self.context.parent_id,
            'kind': self.kind,
            'start_time': self.start_time,
            'end_time': self.end_time,
            'duration_ms': self.duration_ms() if self.end_time else None,
            'attributes': self.attributes,
            'events': self.events,
            'status': self.status,
            'error': self.error
        }

class Tracer:
    """
    OpenTelemetry-style tracer
    
    Example:
        tracer = Tracer("my-service")
        
        with tracer.start_span("operation") as span:
            span.set_attribute("user_id", "123")
            do_work()
            span.add_event("checkpoint")
    """
    
    def __init__(self, service_name: str):
        self.service_name = service_name
        self.spans: List[Span] = []
        self.active_span: Optional[Span] = None
    
    def start_span(
        self,
        name: str,
        kind: str = "internal",
        parent: Optional[Span] = None
    ) -> Span:
        """Start a new span"""
        if parent:
            context = SpanContext(
                trace_id=parent.context.trace_id,
                span_id=SpanContext.generate_span_id(),
                parent_id=parent.context.span_id
            )
        else:
            context = SpanContext(
                trace_id=SpanContext.generate_trace_id(),
                span_id=SpanContext.generate_span_id()
            )
        
        span = Span(name, context, kind)
        span.set_attribute("service.name", self.service_name)
        self.spans.append(span)
        self.active_span = span
        return span
    
    def get_active_span(self) -> Optional[Span]:
        """Get currently active span"""
        return self.active_span
    
    def export_spans(self) -> List[Dict]:
        """Export all spans"""
        return [span.to_dict() for span in self.spans]
    
    def clear_spans(self):
        """Clear collected spans"""
        self.spans = []
        self.active_span = None

class SpanManager:
    """Context manager for spans"""
    
    def __init__(self, span: Span):
        self.span = span
    
    def __enter__(self) -> Span:
        return self.span
    
    def __exit__(self, exc_type, exc_val, exc_tb):
        if exc_type:
            self.span.set_status("error", str(exc_val))
        self.span.end()
        return False

# Extend Tracer with context manager support
def _start_span_cm(self, name: str, kind: str = "internal", parent: Optional[Span] = None):
    """Start span with context manager"""
    span = self.start_span(name, kind, parent)
    return SpanManager(span)

Tracer.start_span_as_current = _start_span_cm

def trace(operation_name: str, tracer: Optional[Tracer] = None):
    """
    Decorator for tracing functions
    
    Example:
        tracer = Tracer("my-service")
        
        @trace("process_data", tracer)
        def process_data(data):
            return data * 2
    """
    def decorator(func: Callable) -> Callable:
        def wrapper(*args, **kwargs):
            if tracer:
                with tracer.start_span_as_current(operation_name) as span:
                    span.set_attribute("function", func.__name__)
                    try:
                        result = func(*args, **kwargs)
                        span.set_status("ok")
                        return result
                    except Exception as e:
                        span.set_status("error", str(e))
                        raise
            else:
                return func(*args, **kwargs)
        return wrapper
    return decorator

# HTTP propagation helpers
class W3CTracePropagator:
    """W3C Trace Context propagation"""
    
    @staticmethod
    def inject(span: Span) -> Dict[str, str]:
        """Inject trace context into headers"""
        traceparent = f"00-{span.context.trace_id}-{span.context.span_id}-01"
        return {'traceparent': traceparent}
    
    @staticmethod
    def extract(headers: Dict[str, str]) -> Optional[SpanContext]:
        """Extract trace context from headers"""
        traceparent = headers.get('traceparent')
        if not traceparent:
            return None
        
        parts = traceparent.split('-')
        if len(parts) != 4:
            return None
        
        return SpanContext(
            trace_id=parts[1],
            span_id=parts[2]
        )

# Exporter interface
class ConsoleExporter:
    """Export spans to console"""
    
    @staticmethod
    def export(spans: List[Dict]):
        """Export spans"""
        for span in spans:
            print(f"Span: {span['name']}")
            print(f"  Trace ID: {span['trace_id']}")
            print(f"  Span ID: {span['span_id']}")
            if span['parent_id']:
                print(f"  Parent ID: {span['parent_id']}")
            print(f"  Duration: {span.get('duration_ms', 'N/A')}ms")
            print(f"  Status: {span['status']}")
            if span['attributes']:
                print(f"  Attributes: {span['attributes']}")
            if span['events']:
                print(f"  Events: {len(span['events'])}")
            print()

# Quick helpers
def create_tracer(service_name: str) -> Tracer:
    """Create a tracer"""
    return Tracer(service_name)

def trace_http_request(
    tracer: Tracer,
    method: str,
    url: str,
    status_code: Optional[int] = None
) -> Span:
    """Create HTTP request span"""
    span = tracer.start_span(f"{method} {url}", kind="client")
    span.set_attribute("http.method", method)
    span.set_attribute("http.url", url)
    if status_code:
        span.set_attribute("http.status_code", status_code)
    return span
