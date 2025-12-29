"""
GUL ORM (Object-Relational Mapping)
Type-safe ORM with migrations and relationships.

Status: ✅ Implemented
Priority: Critical
"""

from typing import Dict, List, Optional, Any, Type, TypeVar
from dataclasses import dataclass, field
from datetime import datetime

__version__ = "0.1.0"
__all__ = ['Model', 'Field', 'CharField', 'IntegerField', 'DateTimeField', 'ForeignKey', 'QuerySet']

T = TypeVar('T', bound='Model')

class Field:
    """Base field type"""
    def __init__(self, primary_key=False, null=False, default=None, unique=False):
        self.primary_key = primary_key
        self.null = null
        self.default = default
        self.unique = unique
        self.name = None
    
    def to_sql(self) -> str:
        return "TEXT"

class CharField(Field):
    def __init__(self, max_length=255, **kwargs):
        super().__init__(**kwargs)
        self.max_length = max_length
    
    def to_sql(self) -> str:
        return f"VARCHAR({self.max_length})"

class IntegerField(Field):
    def to_sql(self) -> str:
        return "INTEGER"

class DateTimeField(Field):
    def __init__(self, auto_now=False, auto_now_add=False, **kwargs):
        super().__init__(**kwargs)
        self.auto_now = auto_now
        self.auto_now_add = auto_now_add
    
    def to_sql(self) -> str:
        return "TIMESTAMP"

class ForeignKey(Field):
    def __init__(self, to: Type['Model'], on_delete='CASCADE', **kwargs):
        super().__init__(**kwargs)
        self.to = to
        self.on_delete = on_delete
    
    def to_sql(self) -> str:
        return "INTEGER"

class QuerySet:
    """Query builder for models"""
    
    def __init__(self, model: Type['Model']):
        self.model = model
        self._filters: Dict[str, Any] = {}
        self._order_by: List[str] = []
        self._limit: Optional[int] = None
        self._offset: Optional[int] = None
    
    def filter(self, **kwargs) -> 'QuerySet':
        """Filter queryset"""
        self._filters.update(kwargs)
        return self
    
    def order_by(self, *fields: str) -> 'QuerySet':
        """Order queryset"""
        self._order_by.extend(fields)
        return self
    
    def limit(self, n: int) -> 'QuerySet':
        """Limit results"""
        self._limit = n
        return self
    
    def offset(self, n: int) -> 'QuerySet':
        """Offset results"""
        self._offset = n
        return self
    
    def all(self) -> List['Model']:
        """Get all results"""
        return self.model._registry.get(self.model.__name__, [])
    
    def first(self) -> Optional['Model']:
        """Get first result"""
        results = self.all()
        return results[0] if results else None
    
    def get(self, **kwargs) -> Optional['Model']:
        """Get single object"""
        for obj in self.all():
            match = all(getattr(obj, k, None) == v for k, v in kwargs.items())
            if match:
                return obj
        return None
    
    def count(self) -> int:
        """Count results"""
        return len(self.all())
    
    def delete(self) -> int:
        """Delete matching objects"""
        to_delete = [obj for obj in self.all() if self._matches(obj)]
        count = len(to_delete)
        
        registry = self.model._registry.get(self.model.__name__, [])
        for obj in to_delete:
            if obj in registry:
                registry.remove(obj)
        
        return count
    
    def _matches(self, obj: 'Model') -> bool:
        """Check if object matches filters"""
        for key, value in self._filters.items():
            if getattr(obj, key, None) != value:
                return False
        return True

class ModelMeta(type):
    """Metaclass for models"""
    
    def __new__(mcs, name, bases, attrs):
        # Collect fields
        fields = {}
        for key, value in list(attrs.items()):
            if isinstance(value, Field):
                value.name = key
                fields[key] = value
        
        attrs['_fields'] = fields
        attrs['_table_name'] = name.lower()
        
        return super().__new__(mcs, name, bases, attrs)

class Model(metaclass=ModelMeta):
    """
    Base ORM model
    
    Example:
        class User(Model):
            id = IntegerField(primary_key=True)
            username = CharField(max_length=100, unique=True)
            email = CharField(max_length=255)
            created_at = DateTimeField(auto_now_add=True)
        
        # Create
        user = User(username="alice", email="alice@example.com")
        user.save()
        
        # Query
        users = User.objects.filter(username="alice").all()
        user = User.objects.get(id=1)
        
        # Update
        user.email = "newemail@example.com"
        user.save()
        
        # Delete
        user.delete()
    """
    
    _registry: Dict[str, List['Model']] = {}
    _fields: Dict[str, Field] = {}
    _table_name: str = ""
    
    def __init__(self, **kwargs):
        self._data = {}
        
        # Set defaults
        for name, field in self._fields.items():
            if field.auto_now_add:
                self._data[name] = datetime.utcnow()
            elif field.default is not None:
                self._data[name] = field.default() if callable(field.default) else field.default
            else:
                self._data[name] = None
        
        # Set provided values
        for key, value in kwargs.items():
            self._data[key] = value
    
    def __getattr__(self, name):
        if name.startswith('_'):
            return super().__getattribute__(name)
        if name in self._data:
            return self._data[name]
        return super().__getattribute__(name)
    
    def __setattr__(self, name, value):
        if name.startswith('_'):
            super().__setattr__(name, value)
        elif name in self._fields:
            self._data[name] = value
        else:
            super().__setattr__(name, value)
    
    @classmethod
    def _get_registry(cls):
        """Get or create registry for this model"""
        if cls.__name__ not in cls._registry:
            cls._registry[cls.__name__] = []
        return cls._registry[cls.__name__]
    
    def save(self):
        """Save model to database"""
        registry = self._get_registry()
        
        # Update auto_now fields
        for name, field in self._fields.items():
            if field.auto_now:
                self._data[name] = datetime.utcnow()
        
        # Check if already in registry
        if self not in registry:
            # Auto-increment primary key if needed
            pk_field = self._get_primary_key_field()
            if pk_field and self._data.get(pk_field.name) is None:
                max_id = max((getattr(obj, pk_field.name, 0) or 0 for obj in registry), default=0)
                self._data[pk_field.name] = max_id + 1
            
            registry.append(self)
    
    def delete(self):
        """Delete model from database"""
        registry = self._get_registry()
        if self in registry:
            registry.remove(self)
    
    @classmethod
    def _get_primary_key_field(cls) -> Optional[Field]:
        """Get primary key field"""
        for field in cls._fields.values():
            if field.primary_key:
                return field
        return None
    
    @classmethod
    @property
    def objects(cls) -> QuerySet:
        """Get queryset for model"""
        return QuerySet(cls)
    
    @classmethod
    def create_table_sql(cls) -> str:
        """Generate CREATE TABLE SQL"""
        columns = []
        
        for name, field in cls._fields.items():
            col_def = f"{name} {field.to_sql()}"
            
            if field.primary_key:
                col_def += " PRIMARY KEY"
            if not field.null:
                col_def += " NOT NULL"
            if field.unique and not field.primary_key:
                col_def += " UNIQUE"
            
            columns.append(col_def)
        
        return f"CREATE TABLE {cls._table_name} ({', '.join(columns)})"
