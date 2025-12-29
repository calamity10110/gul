"""
GUL REST Framework
RESTful API framework with routing and serialization.

Status: ✅ Implemented
Priority: High
Phase: 2
"""

from typing import Dict, List, Optional, Any, Callable
from dataclasses import dataclass

__version__ = "0.1.0"
__all__ = ['RESTRouter', 'Resource', 'Serializer']

class Serializer:
    """Data serializer"""
    
    def __init__(self, fields: List[str]):
        self.fields = fields
    
    def serialize(self, obj: Any) -> Dict:
        """Serialize object"""
        if isinstance(obj, dict):
            return {k: v for k, v in obj.items() if k in self.fields}
        
        return {field: getattr(obj, field, None) for field in self.fields}
    
    def serialize_many(self, objects: List[Any]) -> List[Dict]:
        """Serialize list"""
        return [self.serialize(obj) for obj in objects]

class Resource:
    """REST resource"""
    
    def __init__(self, name: str, serializer: Optional[Serializer] = None):
        self.name = name
        self.serializer = serializer
        self.data_store: Dict[str, Any] = {}
    
    def list(self) -> Dict:
        """GET /resource"""
        items = list(self.data_store.values())
        if self.serializer:
            items = self.serializer.serialize_many(items)
        return {'status': 200, 'data': items}
    
    def create(self, data: Dict) -> Dict:
        """POST /resource"""
        import uuid
        id = str(uuid.uuid4())
        data['id'] = id
        self.data_store[id] = data
        return {'status': 201, 'data': data}
    
    def get(self, id: str) -> Dict:
        """GET /resource/:id"""
        item = self.data_store.get(id)
        if not item:
            return {'status': 404, 'error': 'Not found'}
        
        if self.serializer:
            item = self.serializer.serialize(item)
        return {'status': 200, 'data': item}
    
    def update(self, id: str, data: Dict) -> Dict:
        """PUT /resource/:id"""
        if id not in self.data_store:
            return {'status': 404, 'error': 'Not found'}
        
        self.data_store[id].update(data)
        return {'status': 200, 'data': self.data_store[id]}
    
    def delete(self, id: str) -> Dict:
        """DELETE /resource/:id"""
        if id not in self.data_store:
            return {'status': 404, 'error': 'Not found'}
        
        del self.data_store[id]
        return {'status': 204, 'data': None}

class RESTRouter:
    """
    REST API router
    
    Example:
        router = RESTRouter()
        
        # Add resource
        users = Resource("users", Serializer(["id", "name", "email"]))
        router.add_resource("/api/users", users)
        
        # Handle requests
        response = router.route("POST", "/api/users", {"name": "Alice"})
    """
    
    def __init__(self):
        self.resources: Dict[str, Resource] = {}
    
    def add_resource(self, path: str, resource: Resource) -> 'RESTRouter':
        """Add REST resource"""
        self.resources[path] = resource
        return self
    
    def route(self, method: str, path: str, data: Optional[Dict] = None) -> Dict:
        """Route HTTP request"""
        # Find resource
        resource_path, resource_id = self._parse_path(path)
        resource = self.resources.get(resource_path)
        
        if not resource:
            return {'status': 404, 'error': 'Resource not found'}
        
        # Route to method
        if method == 'GET':
            if resource_id:
                return resource.get(resource_id)
            return resource.list()
        
        elif method == 'POST':
            return resource.create(data or {})
        
        elif method == 'PUT' and resource_id:
            return resource.update(resource_id, data or {})
        
        elif method == 'DELETE' and resource_id:
            return resource.delete(resource_id)
        
        return {'status': 405, 'error': 'Method not allowed'}
    
    def _parse_path(self, path: str) -> tuple:
        """Parse path into resource and ID"""
        parts = path.rstrip('/').split('/')
        
        if len(parts) > 0 and parts[-1] and not parts[-1].startswith('api'):
            resource_id = parts[-1]
            resource_path = '/'.join(parts[:-1])
            return resource_path, resource_id
        
        return path, None
