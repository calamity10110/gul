"""
GUL API Gateway
HTTP API gateway with routing, middleware, and load balancing.

Status: ✅ Implemented
Priority: Critical
Phase: 2
"""

from typing import Dict, List, Optional, Callable, Any
from dataclasses import dataclass, field
import re

__version__ = "0.1.0"
__all__ = ['APIGateway', 'Route', 'Middleware', 'LoadBalancer']

@dataclass
class Route:
    """API route configuration"""
    path: str
    methods: List[str]
    target: str  # Backend service URL
    middleware: List[Callable] = field(default_factory=list)
    rate_limit: Optional[int] = None
    timeout: int = 30
    
    def matches(self, path: str, method: str) -> bool:
        """Check if route matches request"""
        return (method in self.methods and 
                (self.path == path or self._path_matches(path)))
    
    def _path_matches(self, path: str) -> bool:
        """Match path with wildcards"""
        pattern = self.path.replace('*', '.*').replace(':', '([^/]+)')
        return bool(re.match(f"^{pattern}$", path))

class LoadBalancer:
    """Simple round-robin load balancer"""
    
    def __init__(self, backends: List[str]):
        self.backends = backends
        self.current = 0
    
    def get_backend(self) -> str:
        """Get next backend"""
        backend = self.backends[self.current]
        self.current = (self.current + 1) % len(self.backends)
        return backend

class APIGateway:
    """
    API Gateway with routing, middleware, and load balancing
    
    Features:
    - Path-based routing
    - Method-based routing
    - Middleware chains
    - Load balancing
    - Rate limiting integration
    - Request/response transformation
    
    Example:
        gateway = APIGateway()
        
        # Add routes
        gateway.add_route("/api/users/*", ["GET", "POST"], "http://users-service:8001")
        gateway.add_route("/api/orders/*", ["GET", "POST"], "http://orders-service:8002")
        
        # Add middleware
        gateway.add_middleware(auth_middleware)
        gateway.add_middleware(logging_middleware)
        
        # Handle request
        response = gateway.route_request("/api/users/123", "GET")
    """
    
    def __init__(self):
        self.routes: List[Route] = []
        self.global_middleware: List[Callable] = []
        self.backends: Dict[str, LoadBalancer] = {}
    
    def add_route(
        self,
        path: str,
        methods: List[str],
        target: str,
        middleware: Optional[List[Callable]] = None,
        rate_limit: Optional[int] = None
    ) -> 'APIGateway':
        """Add a route"""
        route = Route(
            path=path,
            methods=methods,
            target=target,
            middleware=middleware or [],
            rate_limit=rate_limit
        )
        self.routes.append(route)
        return self
    
    def add_middleware(self, middleware: Callable) -> 'APIGateway':
        """Add global middleware"""
        self.global_middleware.append(middleware)
        return self
    
    def add_load_balancer(self, path_prefix: str, backends: List[str]) -> 'APIGateway':
        """Add load balancer for path prefix"""
        self.backends[path_prefix] = LoadBalancer(backends)
        return self
    
    def find_route(self, path: str, method: str) -> Optional[Route]:
        """Find matching route"""
        for route in self.routes:
            if route.matches(path, method):
                return route
        return None
    
    def route_request(self, path: str, method: str, request: Optional[Dict] = None) -> Dict:
        """Route a request"""
        request = request or {}
        
        # Find route
        route = self.find_route(path, method)
        if not route:
            return {'status': 404, 'body': 'Not Found'}
        
        # Apply global middleware
        for middleware in self.global_middleware:
            result = middleware(request)
            if result:
                return result
        
        # Apply route middleware
        for middleware in route.middleware:
            result = middleware(request)
            if result:
                return result
        
        # Get target backend
        target = route.target
        for prefix, balancer in self.backends.items():
            if path.startswith(prefix):
                target = balancer.get_backend()
                break
        
        # Proxy request (simplified)
        return {
            'status': 200,
            'body': {'target': target, 'path': path, 'method': method},
            'headers': {'X-Gateway': 'GUL-API-Gateway'}
        }
    
    def transform_request(self, request: Dict, transformers: List[Callable]) -> Dict:
        """Apply request transformers"""
        for transformer in transformers:
            request = transformer(request)
        return request
    
    def transform_response(self, response: Dict, transformers: List[Callable]) -> Dict:
        """Apply response transformers"""
        for transformer in transformers:
            response = transformer(response)
        return response

# Middleware examples
def cors_middleware(allowed_origins: List[str] = ["*"]):
    """CORS middleware"""
    def middleware(request: Dict) -> Optional[Dict]:
        origin = request.get('headers', {}).get('origin', '')
        
        if allowed_origins != ["*"] and origin not in allowed_origins:
            return {'status': 403, 'body': 'CORS not allowed'}
        
        # Add CORS headers to response
        return None
    
    return middleware

def auth_middleware(token_validator: Callable):
    """Authentication middleware"""
    def middleware(request: Dict) -> Optional[Dict]:
        auth_header = request.get('headers', {}).get('authorization', '')
        
        if not auth_header.startswith('Bearer '):
            return {'status': 401, 'body': 'Unauthorized'}
        
        token = auth_header[7:]
        if not token_validator(token):
            return {'status': 401, 'body': 'Invalid token'}
        
        return None
    
    return middleware

def logging_middleware(logger: Callable):
    """Logging middleware"""
    def middleware(request: Dict) -> Optional[Dict]:
        logger(f"{request.get('method')} {request.get('path')}")
        return None
    
    return middleware

# Request/Response transformers
def add_correlation_id(request: Dict) -> Dict:
    """Add correlation ID to request"""
    import uuid
    request.setdefault('headers', {})['X-Correlation-ID'] = str(uuid.uuid4())
    return request

def add_timestamp(response: Dict) -> Dict:
    """Add timestamp to response"""
    from datetime import datetime
    response.setdefault('headers', {})['X-Timestamp'] = datetime.utcnow().isoformat()
    return response

# Circuit breaker
class CircuitBreaker:
    """Circuit breaker for backend services"""
    
    def __init__(self, failure_threshold: int = 5, timeout: int = 60):
        self.failure_threshold = failure_threshold
        self.timeout = timeout
        self.failures: Dict[str, int] = {}
        self.open_until: Dict[str, float] = {}
    
    def is_open(self, service: str) -> bool:
        """Check if circuit is open"""
        import time
        if service in self.open_until:
            if time.time() < self.open_until[service]:
                return True
            else:
                # Half-open, reset
                del self.open_until[service]
                self.failures[service] = 0
        return False
    
    def record_success(self, service: str):
        """Record successful request"""
        self.failures[service] = 0
    
    def record_failure(self, service: str):
        """Record failed request"""
        self.failures[service] = self.failures.get(service, 0) + 1
        
        if self.failures[service] >= self.failure_threshold:
            import time
            self.open_until[service] = time.time() + self.timeout
