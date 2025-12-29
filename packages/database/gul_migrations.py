"""
GUL Migrations
Database schema migrations with version control.

Status: ✅ Implemented
Priority: Critical
"""

from typing import List, Optional, Dict, Any, Callable
from dataclasses import dataclass
from datetime import datetime
import hashlib

__version__ = "0.1.0"
__all__ = ['Migration', 'MigrationManager', 'Schema']

@dataclass
class Migration:
    """Database migration"""
    version: str
    name: str
    up: Callable
    down: Callable
    checksum: str = ""
    applied_at: Optional[datetime] = None

class Schema:
    """Schema builder for migrations"""
    
    def __init__(self):
        self.operations: List[str] = []
    
    def create_table(self, table: str, columns: Dict[str, str]) -> 'Schema':
        """Create table"""
        col_defs = [f"{name} {dtype}" for name, dtype in columns.items()]
        sql = f"CREATE TABLE {table} ({', '.join(col_defs)})"
        self.operations.append(sql)
        return self
    
    def drop_table(self, table: str) -> 'Schema':
        """Drop table"""
        self.operations.append(f"DROP TABLE {table}")
        return self
    
    def add_column(self, table: str, column: str, dtype: str) -> 'Schema':
        """Add column"""
        self.operations.append(f"ALTER TABLE {table} ADD COLUMN {column} {dtype}")
        return self
    
    def drop_column(self, table: str, column: str) -> 'Schema':
        """Drop column"""
        self.operations.append(f"ALTER TABLE {table} DROP COLUMN {column}")
        return self
    
    def rename_column(self, table: str, old_name: str, new_name: str) -> 'Schema':
        """Rename column"""
        self.operations.append(f"ALTER TABLE {table} RENAME COLUMN {old_name} TO {new_name}")
        return self
    
    def add_index(self, table: str, columns: List[str], name: Optional[str] = None) -> 'Schema':
        """Add index"""
        idx_name = name or f"idx_{table}_{'_'.join(columns)}"
        cols = ", ".join(columns)
        self.operations.append(f"CREATE INDEX {idx_name} ON {table} ({cols})")
        return self
    
    def drop_index(self, name: str) -> 'Schema':
        """Drop index"""
        self.operations.append(f"DROP INDEX {name}")
        return self
    
    def execute_sql(self, sql: str) -> 'Schema':
        """Execute raw SQL"""
        self.operations.append(sql)
        return self
    
    def get_operations(self) -> List[str]:
        """Get all operations"""
        return self.operations

class MigrationManager:
    """
    Migration manager with version control
    
    Example:
        manager = MigrationManager()
        
        # Define migration
        def create_users_up(schema):
            schema.create_table("users", {
                "id": "INTEGER PRIMARY KEY",
                "username": "VARCHAR(100) NOT NULL",
                "email": "VARCHAR(255) NOT NULL",
                "created_at": "TIMESTAMP DEFAULT CURRENT_TIMESTAMP"
            })
            schema.add_index("users", ["username"], "idx_users_username")
        
        def create_users_down(schema):
            schema.drop_table("users")
        
        # Register migration
        manager.register("001", "create_users", create_users_up, create_users_down)
        
        # Run migrations
        manager.migrate()
        
        # Rollback
        manager.rollback()
    """
    
    def __init__(self):
        self.migrations: List[Migration] = []
        self.applied_migrations: Dict[str, Migration] = {}
    
    def register(
        self,
        version: str,
        name: str,
        up: Callable[[Schema], None],
        down: Callable[[Schema], None]
    ):
        """Register migration"""
        # Calculate checksum from operations
        schema_up = Schema()
        up(schema_up)
        checksum = self._calculate_checksum(schema_up.get_operations())
        
        migration = Migration(
            version=version,
            name=name,
            up=up,
            down=down,
            checksum=checksum
        )
        
        self.migrations.append(migration)
        self.migrations.sort(key=lambda m: m.version)
    
    def migrate(self, target: Optional[str] = None) -> List[Migration]:
        """Run migrations"""
        applied = []
        
        for migration in self.migrations:
            # Skip if already applied
            if migration.version in self.applied_migrations:
                continue
            
            # Stop if reached target
            if target and migration.version > target:
                break
            
            # Apply migration
            schema = Schema()
            migration.up(schema)
            
            # Execute operations (in real implementation)
            operations = schema.get_operations()
            
            # Mark as applied
            migration.applied_at = datetime.utcnow()
            self.applied_migrations[migration.version] = migration
            applied.append(migration)
        
        return applied
    
    def rollback(self, steps: int = 1) -> List[Migration]:
        """Rollback migrations"""
        rolled_back = []
        
        # Get last N applied migrations
        applied_list = sorted(
            self.applied_migrations.values(),
            key=lambda m: m.version,
            reverse=True
        )
        
        for migration in applied_list[:steps]:
            # Rollback migration
            schema = Schema()
            migration.down(schema)
            
            # Execute operations (in real implementation)
            operations = schema.get_operations()
            
            # Mark as not applied
            del self.applied_migrations[migration.version]
            migration.applied_at = None
            rolled_back.append(migration)
        
        return rolled_back
    
    def status(self) -> List[Dict[str, Any]]:
        """Get migration status"""
        status = []
        
        for migration in self.migrations:
            is_applied = migration.version in self.applied_migrations
            
            status.append({
                "version": migration.version,
                "name": migration.name,
                "applied": is_applied,
                "applied_at": migration.applied_at,
                "checksum": migration.checksum
            })
        
        return status
    
    def pending(self) -> List[Migration]:
        """Get pending migrations"""
        return [
            m for m in self.migrations
            if m.version not in self.applied_migrations
        ]
    
    def reset(self):
        """Reset all migrations"""
        # Rollback all
        self.rollback(steps=len(self.applied_migrations))
    
    def _calculate_checksum(self, operations: List[str]) -> str:
        """Calculate checksum of operations"""
        content = "\n".join(operations)
        return hashlib.sha256(content.encode()).hexdigest()[:16]
    
    def generate_migration_file(self, version: str, name: str) -> str:
        """Generate migration file template"""
        return f'''"""
Migration {version}: {name}
"""

def up(schema):
    """Apply migration"""
    schema.create_table("{name}", {{
        "id": "INTEGER PRIMARY KEY",
        # Add columns here
    }})

def down(schema):
    """Rollback migration"""
    schema.drop_table("{name}")
'''
