"""
GUL Validation
Data validation library.

Status: ✅ Implemented
Priority: High
"""

from typing import Any, Dict, List, Optional, Callable, Union
import re

__version__ = "0.1.0"
__all__ = ['Validator', 'Rule', 'validate']

class ValidationError(Exception):
    pass

class Rule:
    """Validation rule"""
    def __init__(self, check: Callable, message: str):
        self.check = check
        self.message = message

class Validator:
    """
    Data validator
    
    Example:
        v = Validator()
        
        rules = {
            'username': [
                v.required(),
                v.min_length(3),
                v.max_length(20)
            ],
            'email': [
                v.required(),
                v.email()
            ],
            'age': [
                v.integer(),
                v.min(18)
            ]
        }
        
        errors = v.validate(data, rules)
    """
    
    def validate(self, data: Dict[str, Any], schema: Dict[str, List[Rule]]) -> Dict[str, List[str]]:
        """Validate data against schema"""
        errors = {}
        
        for field, rules in schema.items():
            value = data.get(field)
            field_errors = []
            
            for rule in rules:
                if not rule.check(value):
                    field_errors.append(rule.message.format(field=field, value=value))
                    # Stop on first error for this field if it's 'required'
                    if rule.message == "{field} is required":
                        break
            
            if field_errors:
                errors[field] = field_errors
        
        return errors
    
    # Rules
    
    def required(self) -> Rule:
        return Rule(
            lambda v: v is not None and v != "",
            "{field} is required"
        )
    
    def min_length(self, length: int) -> Rule:
        return Rule(
            lambda v: v is None or len(str(v)) >= length,
            f"{{field}} must be at least {length} characters"
        )
    
    def max_length(self, length: int) -> Rule:
        return Rule(
            lambda v: v is None or len(str(v)) <= length,
            f"{{field}} must be at most {length} characters"
        )
    
    def min(self, value: Union[int, float]) -> Rule:
        return Rule(
            lambda v: v is None or (isinstance(v, (int, float)) and v >= value),
            f"{{field}} must be at least {value}"
        )
    
    def max(self, value: Union[int, float]) -> Rule:
        return Rule(
            lambda v: v is None or (isinstance(v, (int, float)) and v <= value),
            f"{{field}} must be at most {value}"
        )
    
    def email(self) -> Rule:
        pattern = r'^[a-zA-Z0-9._%+-]+@[a-zA-Z0-9.-]+\.[a-zA-Z]{2,}$'
        return Rule(
            lambda v: v is None or bool(re.match(pattern, str(v))),
            "{field} must be a valid email"
        )
    
    def url(self) -> Rule:
        pattern = r'^https?://'
        return Rule(
            lambda v: v is None or bool(re.match(pattern, str(v))),
            "{field} must be a valid URL"
        )
    
    def integer(self) -> Rule:
        return Rule(
            lambda v: v is None or isinstance(v, int),
            "{field} must be an integer"
        )
    
    def boolean(self) -> Rule:
        return Rule(
            lambda v: v is None or isinstance(v, bool),
            "{field} must be a boolean"
        )
    
    def one_of(self, values: List[Any]) -> Rule:
        return Rule(
            lambda v: v is None or v in values,
            f"{{field}} must be one of {values}"
        )
    
    def matches(self, pattern: str) -> Rule:
        return Rule(
            lambda v: v is None or bool(re.match(pattern, str(v))),
            "{field} has invalid format"
        )
    
    def custom(self, func: Callable, message: str) -> Rule:
        """Custom validation rule"""
        return Rule(func, message)

def validate(data: Dict[str, Any], schema: Dict[str, List[Rule]]) -> Dict[str, List[str]]:
    """Quick validation"""
    validator = Validator()
    return validator.validate(data, schema)
