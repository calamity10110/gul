"""
GUL Input Validation
Comprehensive input validation and sanitization.

Status: ✅ Implemented
Priority: Critical
Phase: 1
"""

from typing import Any, Optional, Dict, List, Callable, Union
import re
from dataclasses import dataclass

__version__ = "0.1.0"
__all__ = ['Validator', 'ValidationError', 'Schema', 'sanitize']

class ValidationError(Exception):
    """Exception raised when validation fails"""
    def __init__(self, field: str, message: str):
        self.field = field
        self.message = message
        super().__init__(f"{field}: {message}")

@dataclass
class FieldValidator:
    """Validator for a single field"""
    required: bool = False
    type_check: Optional[type] = None
    min_length: Optional[int] = None
    max_length: Optional[int] = None
    min_value: Optional[Union[int, float]] = None
    max_value: Optional[Union[int, float]] = None
    pattern: Optional[str] = None
    choices: Optional[List] = None
    custom: Optional[Callable] = None

class Schema:
    """
    Schema for validating dictionaries
    
    Example:
        schema = Schema({
            'email': FieldValidator(
                required=True,
                pattern=r'^[a-zA-Z0-9._%+-]+@[a-zA-Z0-9.-]+\.[a-zA-Z]{2,}$'
            ),
            'age': FieldValidator(
                required=True,
                type_check=int,
                min_value=0,
                max_value=150
            )
        })
        
        schema.validate({'email': 'test@example.com', 'age': 25})
    """
    
    def __init__(self, fields: Dict[str, FieldValidator]):
        self.fields = fields
    
    def validate(self, data: Dict[str, Any]) -> Dict[str, Any]:
        """
        Validate data against schema
        
        Args:
            data: Dictionary to validate
        
        Returns:
            Validated data
        
        Raises:
            ValidationError: If validation fails
        """
        validated = {}
        
        # Check required fields
        for field_name, validator in self.fields.items():
            if validator.required and field_name not in data:
                raise ValidationError(field_name, "Field is required")
            
            if field_name not in data:
                continue
            
            value = data[field_name]
            validated[field_name] = self._validate_field(field_name, value, validator)
        
        return validated
    
    def _validate_field(self, name: str, value: Any, validator: FieldValidator) -> Any:
        """Validate a single field"""
        
        # Type check
        if validator.type_check and not isinstance(value, validator.type_check):
            raise ValidationError(name, f"Must be of type {validator.type_check.__name__}")
        
        # String validations
        if isinstance(value, str):
            # Min length
            if validator.min_length and len(value) < validator.min_length:
                raise ValidationError(name, f"Must be at least {validator.min_length} characters")
            
            # Max length
            if validator.max_length and len(value) > validator.max_length:
                raise ValidationError(name, f"Must be at most {validator.max_length} characters")
            
            # Pattern
            if validator.pattern and not re.match(validator.pattern, value):
                raise ValidationError(name, "Invalid format")
        
        # Numeric validations
        if isinstance(value, (int, float)):
            # Min value
            if validator.min_value is not None and value < validator.min_value:
                raise ValidationError(name, f"Must be at least {validator.min_value}")
            
            # Max value
            if validator.max_value is not None and value > validator.max_value:
                raise ValidationError(name, f"Must be at most {validator.max_value}")
        
        # Choices
        if validator.choices and value not in validator.choices:
            raise ValidationError(name, f"Must be one of: {', '.join(map(str, validator.choices))}")
        
        # Custom validator
        if validator.custom:
            if not validator.custom(value):
                raise ValidationError(name, "Custom validation failed")
        
        return value

class Validator:
    """
    Utility class for common validations
    
    Provides methods for validating:
    - Email addresses
    - URLs
    - Phone numbers
    - Credit cards
    - IP addresses
    - UUIDs
    """
    
    @staticmethod
    def email(value: str) -> bool:
        """Validate email address"""
        pattern = r'^[a-zA-Z0-9._%+-]+@[a-zA-Z0-9.-]+\.[a-zA-Z]{2,}$'
        return bool(re.match(pattern, value))
    
    @staticmethod
    def url(value: str) -> bool:
        """Validate URL"""
        pattern = r'^https?://[a-zA-Z0-9.-]+\.[a-zA-Z]{2,}(/.*)?$'
        return bool(re.match(pattern, value))
    
    @staticmethod
    def phone(value: str) -> bool:
        """Validate phone number (basic)"""
        # Remove common separators
        cleaned = re.sub(r'[-\s().+]', '', value)
        return cleaned.isdigit() and 10 <= len(cleaned) <= 15
    
    @staticmethod
    def credit_card(value: str) -> bool:
        """Validate credit card using Luhn algorithm"""
        # Remove spaces and dashes
        cleaned = re.sub(r'[-\s]', '', value)
        
        if not cleaned.isdigit() or len(cleaned) < 13 or len(cleaned) > 19:
            return False
        
        # Luhn algorithm
        def luhn_checksum(card_number):
            def digits_of(n):
                return [int(d) for d in str(n)]
            
            digits = digits_of(card_number)
            odd_digits = digits[-1::-2]
            even_digits = digits[-2::-2]
            checksum = sum(odd_digits)
            for d in even_digits:
                checksum += sum(digits_of(d * 2))
            return checksum % 10
        
        return luhn_checksum(cleaned) == 0
    
    @staticmethod
    def ipv4(value: str) -> bool:
        """Validate IPv4 address"""
        pattern = r'^(?:(?:25[0-5]|2[0-4][0-9]|[01]?[0-9][0-9]?)\.){3}(?:25[0-5]|2[0-4][0-9]|[01]?[0-9][0-9]?)$'
        return bool(re.match(pattern, value))
    
    @staticmethod
    def uuid(value: str) -> bool:
        """Validate UUID"""
        pattern = r'^[0-9a-f]{8}-[0-9a-f]{4}-[0-9a-f]{4}-[0-9a-f]{4}-[0-9a-f]{12}$'
        return bool(re.match(pattern, value.lower()))
    
    @staticmethod
    def username(value: str) -> bool:
        """Validate username (alphanumeric, underscore, hyphen)"""
        pattern = r'^[a-zA-Z0-9_-]{3,20}$'
        return bool(re.match(pattern, value))
    
    @staticmethod
    def password_strength(value: str) -> Dict[str, bool]:
        """
        Check password strength
        
        Returns:
            Dictionary with strength indicators
        """
        return {
            'min_length': len(value) >= 8,
            'has_uppercase': bool(re.search(r'[A-Z]', value)),
            'has_lowercase': bool(re.search(r'[a-z]', value)),
            'has_digit': bool(re.search(r'\d', value)),
            'has_special': bool(re.search(r'[!@#$%^&*(),.?":{}|<>]', value))
        }
    
    @staticmethod
    def is_strong_password(value: str) -> bool:
        """Check if password meets strength requirements"""
        strength = Validator.password_strength(value)
        return all(strength.values())

class Sanitizer:
    """Input sanitization utilities"""
    
    @staticmethod
    def html_escape(value: str) -> str:
        """Escape HTML special characters"""
        return (value
            .replace('&', '&amp;')
            .replace('<', '&lt;')
            .replace('>', '&gt;')
            .replace('"', '&quot;')
            .replace("'", '&#x27;'))
    
    @staticmethod
    def strip_tags(value: str) -> str:
        """Remove HTML tags"""
        return re.sub(r'<[^>]+>', '', value)
    
    @staticmethod
    def normalize_whitespace(value: str) -> str:
        """Normalize whitespace"""
        return ' '.join(value.split())
    
    @staticmethod
    def alphanumeric_only(value: str) -> str:
        """Keep only alphanumeric characters"""
        return re.sub(r'[^a-zA-Z0-9]', '', value)
    
    @staticmethod
    def sql_escape(value: str) -> str:
        """Basic SQL escaping (use parameterized queries instead!)"""
        return value.replace("'", "''")
    
    @staticmethod
    def truncate(value: str, max_length: int, suffix: str = '...') -> str:
        """Truncate string to max length"""
        if len(value) <= max_length:
            return value
        return value[:max_length - len(suffix)] + suffix

def sanitize(value: str, html: bool = True, normalize: bool = True) -> str:
    """
    Quick sanitization helper
    
    Args:
        value: String to sanitize
        html: Escape HTML characters
        normalize: Normalize whitespace
    
    Returns:
        Sanitized string
    """
    result = value
    
    if html:
        result = Sanitizer.html_escape(result)
    
    if normalize:
        result = Sanitizer.normalize_whitespace(result)
    
    return result

# Common schemas
class CommonSchemas:
    """Pre-defined schemas for common use cases"""
    
    @staticmethod
    def user_registration():
        """Schema for user registration"""
        return Schema({
            'username': FieldValidator(
                required=True,
                type_check=str,
                min_length=3,
                max_length=20,
                custom=Validator.username
            ),
            'email': FieldValidator(
                required=True,
                type_check=str,
                custom=Validator.email
            ),
            'password': FieldValidator(
                required=True,
                type_check=str,
                min_length=8,
                custom=Validator.is_strong_password
            )
        })
    
    @staticmethod
    def login():
        """Schema for login"""
        return Schema({
            'email': FieldValidator(
                required=True,
                type_check=str,
                custom=Validator.email
            ),
            'password': FieldValidator(
                required=True,
                type_check=str,
                min_length=1
            )
        })
    
    @staticmethod
    def api_key():
        """Schema for API key"""
        return Schema({
            'key': FieldValidator(
                required=True,
                type_check=str,
                pattern=r'^[a-zA-Z0-9]{32,64}$'
            )
        })
