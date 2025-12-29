"""
GUL OpenAPI
OpenAPI (Swagger) documentation generator.

Status: ✅ Implemented
Priority: Medium
"""

from typing import Dict, List, Optional, Any, Callable, Type
from dataclasses import dataclass, field
import json
import inspect

__version__ = "0.1.0"
__all__ = ['OpenAPI', 'Operation', 'Parameter', 'Response', 'api', 'schema']

@dataclass
class Parameter:
    name: str
    in_: str  # query, path, header, cookie
    type: str = "string"
    required: bool = True
    description: Optional[str] = None

@dataclass
class Response:
    status: int
    description: str
    schema: Optional[Dict] = None

@dataclass
class Operation:
    method: str
    path: str
    summary: Optional[str] = None
    description: Optional[str] = None
    operation_id: Optional[str] = None
    parameters: List[Parameter] = field(default_factory=list)
    responses: Dict[int, Response] = field(default_factory=dict)
    tags: List[str] = field(default_factory=list)

class OpenAPI:
    """
    OpenAPI generator
    
    Example:
        api = OpenAPI(title="My API", version="1.0.0")
        
        @api.get("/users", tags=["Users"])
        def get_users():
            '''Get all users'''
            pass
            
        print(api.to_json())
    """
    
    def __init__(
        self,
        title: str = "API",
        version: str = "1.0.0",
        description: Optional[str] = None
    ):
        self.title = title
        self.version = version
        self.description = description
        self.paths: Dict[str, Dict[str, Operation]] = {}
        self.schemas: Dict[str, Dict] = {}
    
    def add_operation(self, operation: Operation):
        """Add operation"""
        if operation.path not in self.paths:
            self.paths[operation.path] = {}
        
        self.paths[operation.path][operation.method.lower()] = operation
    
    def route(self, path: str, method: str, **kwargs):
        """Route decorator"""
        def decorator(func):
            # Parse docstring
            summary = func.__name__
            description = func.__doc__ or ""
            
            # Create operation
            op = Operation(
                method=method,
                path=path,
                summary=kwargs.get("summary", summary),
                description=kwargs.get("description", description),
                tags=kwargs.get("tags", []),
                operation_id=f"{method.lower()}_{path.replace('/', '_').strip('_')}"
            )
            
            # Add parameters based on type hints (simplified)
            sig = inspect.signature(func)
            for name, param in sig.parameters.items():
                if name == "self":
                    continue
                
                param_in = "query"
                if f"{{{name}}}" in path:
                    param_in = "path"
                
                op.parameters.append(Parameter(
                    name=name,
                    in_=param_in,
                    type="string"  # Simplification
                ))
            
            self.add_operation(op)
            return func
        return decorator
    
    def get(self, path: str, **kwargs):
        return self.route(path, "get", **kwargs)
    
    def post(self, path: str, **kwargs):
        return self.route(path, "post", **kwargs)
    
    def put(self, path: str, **kwargs):
        return self.route(path, "put", **kwargs)
    
    def delete(self, path: str, **kwargs):
        return self.route(path, "delete", **kwargs)
    
    def dump(self) -> Dict:
        """Dump to dict"""
        paths_data = {}
        
        for path, operations in self.paths.items():
            paths_data[path] = {}
            for method, op in operations.items():
                paths_data[path][method] = {
                    "summary": op.summary,
                    "description": op.description,
                    "operationId": op.operation_id,
                    "tags": op.tags,
                    "parameters": [
                        {
                            "name": p.name,
                            "in": p.in_,
                            "required": p.required,
                            "schema": {"type": p.type}
                        }
                        for p in op.parameters
                    ],
                    "responses": {
                        str(status): {"description": resp.description}
                        for status, resp in op.responses.items()
                    }
                }
                
                # Default response if none
                if not paths_data[path][method]["responses"]:
                    paths_data[path][method]["responses"]["200"] = {
                        "description": "Successful operation"
                    }
        
        return {
            "openapi": "3.0.0",
            "info": {
                "title": self.title,
                "version": self.version,
                "description": self.description
            },
            "paths": paths_data,
            "components": {
                "schemas": self.schemas
            }
        }
    
    def to_json(self) -> str:
        """Export to JSON"""
        return json.dumps(self.dump(), indent=2)

# Global instance
_api = OpenAPI()

def api() -> OpenAPI:
    return _api

def schema(model: Type):
    """Register schema"""
    _api.schemas[model.__name__] = {
        "type": "object",
        "properties": {
            # Inspect fields
            "id": {"type": "integer"}
        }
    }
    return model
