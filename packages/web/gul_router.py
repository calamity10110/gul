"""
GUL Router
URL routing for web applications.

Status: ✅ Implemented
Priority: High
"""

from typing import Dict, List, Callable, Optional, Any, Tuple
from dataclasses import dataclass
import re

__version__ = "0.1.0"
__all__ = ['Router', 'Route', 'match']

@dataclass
class Route:
    """Route definition"""
    method: str
    path: str
    handler: Callable
    name: Optional[str] = None
    regex: Optional[re.Pattern] = None
    param_names: List[str] = None
    
    def __post_init__(self):
        # Convert path variables to regex
        # /users/:id -> /users/(?P<id>[^/]+)
        if ':' in self.path:
            regex_str = '^' + re.sub(r':([a-zA-Z_][a-zA-Z0-9_]*)', r'(?P<\1>[^/]+)', self.path) + '$'
            self.regex = re.compile(regex_str)
            self.param_names = re.findall(r':([a-zA-Z_][a-zA-Z0-9_]*)', self.path)
        else:
            self.param_names = []

class Router:
    """
    URL router
    
    Example:
        router = Router()
        
        # Add routes
        @router.get("/")
        def home():
            return "Home"
        
        @router.post("/users")
        def create_user(data):
            return "User created"
        
        @router.get("/users/:id")
        def get_user(id):
            return f"User {id}"
        
        # Match route
        handler, params = router.match("GET", "/users/123")
        handler(params['id'])
    """
    
    def __init__(self):
        self.routes: List[Route] = []
        self._named_routes: Dict[str, Route] = {}
        self._trie = {}  # Could optimize with trie, but linear scan is fine for now
    
    def add(
        self,
        method: str,
        path: str,
        handler: Callable,
        name: Optional[str] = None
    ):
        """Add route"""
        route = Route(method.upper(), path, handler, name)
        self.routes.append(route)
        
        if name:
            self._named_routes[name] = route
    
    def get(self, path: str, name: Optional[str] = None):
        """Add GET route"""
        def decorator(func: Callable):
            self.add("GET", path, func, name)
            return func
        return decorator
    
    def post(self, path: str, name: Optional[str] = None):
        """Add POST route"""
        def decorator(func: Callable):
            self.add("POST", path, func, name)
            return func
        return decorator
    
    def put(self, path: str, name: Optional[str] = None):
        """Add PUT route"""
        def decorator(func: Callable):
            self.add("PUT", path, func, name)
            return func
        return decorator
    
    def delete(self, path: str, name: Optional[str] = None):
        """Add DELETE route"""
        def decorator(func: Callable):
            self.add("DELETE", path, func, name)
            return func
        return decorator
    
    def match(self, method: str, path: str) -> Tuple[Optional[Callable], Dict[str, str]]:
        """Match route"""
        method = method.upper()
        
        for route in self.routes:
            if route.method != method:
                continue
            
            # Static match
            if route.path == path:
                return route.handler, {}
            
            # Regex match
            if route.regex:
                match = route.regex.match(path)
                if match:
                    return route.handler, match.groupdict()
        
        return None, {}
    
    def url_for(self, name: str, **params) -> str:
        """Generate URL for named route"""
        if name not in self._named_routes:
            raise ValueError(f"Route not found: {name}")
        
        route = self._named_routes[name]
        path = route.path
        
        for key, value in params.items():
            path = path.replace(f':{key}', str(value))
        
        return path

def match(routes: List[Tuple[str, Callable]], path: str) -> Optional[Callable]:
    """Quick route matching"""
    for route_path, handler in routes:
        if route_path == path:
            return handler
    return None
