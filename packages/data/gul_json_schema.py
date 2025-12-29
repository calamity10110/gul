"""
GUL JSON Schema Validator
JSON schema validation.

Status: ✅ Implemented
Priority: High
"""

from typing import Dict, List, Optional, Any, Union

__version__ = "0.1.0"
__all__ = ['JSONSchema', 'validate', 'ValidationError']

class ValidationError(Exception):
    """Schema validation error"""
    pass

class JSONSchema:
    """
    JSON Schema validator
    
    Example:
        schema = JSONSchema({
            "type": "object",
            "properties": {
                "name": {"type": "string"},
                "age": {"type": "integer", "minimum": 0},
                "email": {"type": "string", "format": "email"}
            },
            "required": ["name", "email"]
        })
        
        data = {"name": "Alice", "age": 30, "email": "alice@example.com"}
        schema.validate(data)  # OK
        
        data = {"name": "Bob"}  # Missing email
        schema.validate(data)  # Raises ValidationError
    """
    
    def __init__(self, schema: Dict):
        self.schema = schema
    
    def validate(self, data: Any, path: str = "$") -> bool:
        """Validate data against schema"""
        return self._validate(data, self.schema, path)
    
    def _validate(self, data: Any, schema: Dict, path: str) -> bool:
        """Internal validation"""
        # Type validation
        if "type" in schema:
            if not self._validate_type(data, schema["type"], path):
                return False
        
        # Type-specific validation
        data_type = schema.get("type")
        
        if data_type == "object":
            return self._validate_object(data, schema, path)
        elif data_type == "array":
            return self._validate_array(data, schema, path)
        elif data_type == "string":
            return self._validate_string(data, schema, path)
        elif data_type == "integer" or data_type == "number":
            return self._validate_number(data, schema, path)
        
        return True
    
    def _validate_type(self, data: Any, expected_type: Union[str, List[str]], path: str) -> bool:
        """Validate type"""
        if isinstance(expected_type, list):
            return any(self._check_type(data, t) for t in expected_type)
        
        if not self._check_type(data, expected_type):
            raise ValidationError(f"{path}: Expected {expected_type}, got {type(data).__name__}")
        
        return True
    
    def _check_type(self, data: Any, type_name: str) -> bool:
        """Check if data matches type"""
        type_map = {
            "null": lambda x: x is None,
            "boolean": lambda x: isinstance(x, bool),
            "integer": lambda x: isinstance(x, int) and not isinstance(x, bool),
            "number": lambda x: isinstance(x, (int, float)) and not isinstance(x, bool),
            "string": lambda x: isinstance(x, str),
            "array": lambda x: isinstance(x, list),
            "object": lambda x: isinstance(x, dict)
        }
        
        checker = type_map.get(type_name)
        return checker(data) if checker else False
    
    def _validate_object(self, data: Dict, schema: Dict, path: str) -> bool:
        """Validate object"""
        if not isinstance(data, dict):
            raise ValidationError(f"{path}: Expected object")
        
        # Required properties
        required = schema.get("required", [])
        for prop in required:
            if prop not in data:
                raise ValidationError(f"{path}: Missing required property '{prop}'")
        
        # Validate properties
        properties = schema.get("properties", {})
        for key, value in data.items():
            if key in properties:
                self._validate(value, properties[key], f"{path}.{key}")
        
        # Additional properties
        if "additionalProperties" in schema:
            if schema["additionalProperties"] is False:
                extra = set(data.keys()) - set(properties.keys())
                if extra:
                    raise ValidationError(f"{path}: Additional properties not allowed: {extra}")
        
        return True
    
    def _validate_array(self, data: List, schema: Dict, path: str) -> bool:
        """Validate array"""
        if not isinstance(data, list):
            raise ValidationError(f"{path}: Expected array")
        
        # Min/max items
        if "minItems" in schema and len(data) < schema["minItems"]:
            raise ValidationError(f"{path}: Array too short (min {schema['minItems']})")
        
        if "maxItems" in schema and len(data) > schema["maxItems"]:
            raise ValidationError(f"{path}: Array too long (max {schema['maxItems']})")
        
        # Validate items
        if "items" in schema:
            item_schema = schema["items"]
            for i, item in enumerate(data):
                self._validate(item, item_schema, f"{path}[{i}]")
        
        # Unique items
        if schema.get("uniqueItems"):
            if len(data) != len(set(str(x) for x in data)):
                raise ValidationError(f"{path}: Array items must be unique")
        
        return True
    
    def _validate_string(self, data: str, schema: Dict, path: str) -> bool:
        """Validate string"""
        if not isinstance(data, str):
            raise ValidationError(f"{path}: Expected string")
        
        # Min/max length
        if "minLength" in schema and len(data) < schema["minLength"]:
            raise ValidationError(f"{path}: String too short (min {schema['minLength']})")
        
        if "maxLength" in schema and len(data) > schema["maxLength"]:
            raise ValidationError(f"{path}: String too long (max {schema['maxLength']})")
        
        # Pattern
        if "pattern" in schema:
            import re
            if not re.match(schema["pattern"], data):
                raise ValidationError(f"{path}: String doesn't match pattern")
        
        # Format
        if "format" in schema:
            self._validate_format(data, schema["format"], path)
        
        # Enum
        if "enum" in schema and data not in schema["enum"]:
            raise ValidationError(f"{path}: Value must be one of {schema['enum']}")
        
        return True
    
    def _validate_number(self, data: Union[int, float], schema: Dict, path: str) -> bool:
        """Validate number"""
        if not isinstance(data, (int, float)) or isinstance(data, bool):
            raise ValidationError(f"{path}: Expected number")
        
        # Min/max
        if "minimum" in schema and data < schema["minimum"]:
            raise ValidationError(f"{path}: Number too small (min {schema['minimum']})")
        
        if "maximum" in schema and data > schema["maximum"]:
            raise ValidationError(f"{path}: Number too large (max {schema['maximum']})")
        
        # Multiple of
        if "multipleOf" in schema and data % schema["multipleOf"] != 0:
            raise ValidationError(f"{path}: Number must be multiple of {schema['multipleOf']}")
        
        return True
    
    def _validate_format(self, data: str, format_name: str, path: str):
        """Validate string format"""
        import re
        
        formats = {
            "email": r'^[a-zA-Z0-9._%+-]+@[a-zA-Z0-9.-]+\.[a-zA-Z]{2,}$',
            "uri": r'^https?://',
            "date": r'^\d{4}-\d{2}-\d{2}$',
            "time": r'^\d{2}:\d{2}:\d{2}$',
            "ipv4": r'^\d{1,3}\.\d{1,3}\.\d{1,3}\.\d{1,3}$'
        }
        
        pattern = formats.get(format_name)
        if pattern and not re.match(pattern, data):
            raise ValidationError(f"{path}: Invalid {format_name} format")

def validate(data: Any, schema: Dict) -> bool:
    """Quick validation"""
    validator = JSONSchema(schema)
    return validator.validate(data)
