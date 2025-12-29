"""
GUL User Management
Complete user management system.

Status: ✅ Implemented
Priority: Critical
Phase: 3
"""

from typing import Dict, List, Optional
from dataclasses import dataclass, field
from datetime import datetime

__version__ = "0.1.0"
__all__ = ['UserManager', 'User', 'Role', 'Permission']

@dataclass
class Permission:
    """User permission"""
    name: str
    resource: str
    action: str  # "create", "read", "update", "delete"

@dataclass
class Role:
    """User role"""
    name: str
    permissions: List[Permission] = field(default_factory=list)
    description: str = ""

@dataclass
class User:
    """User account"""
    id: str
    email: str
    username: str
    status: str = "active"  # "active", "suspended", "deleted"
    roles: List[str] = field(default_factory=list)
    metadata: Dict = field(default_factory=dict)
    created_at: datetime = field(default_factory=datetime.utcnow)
    last_login: Optional[datetime] = None

class UserManager:
    """
    Complete user management system
    
    Example:
        manager = UserManager()
        
        # Create user
        user = manager.create_user("alice@example.com", "alice")
        
        # Assign role
        manager.assign_role(user.id, "editor")
        
        # Check permission
        if manager.has_permission(user.id, "post.create"):
            print("User can create posts")
        
        # Update user
        manager.update_user(user.id, status="suspended")
        
        # Search users
        users = manager.search_users(query="alice")
    """
    
    def __init__(self):
        self.users: Dict[str, User] = {}
        self.roles: Dict[str, Role] = {}
        self.email_index: Dict[str, str] = {}
        self.username_index: Dict[str, str] = {}
        self._init_default_roles()
    
    def _init_default_roles(self):
        """Initialize default roles"""
        # Admin role
        admin_perms = [
            Permission("all", "*", "*")
        ]
        self.roles["admin"] = Role("admin", admin_perms, "Administrator")
        
        # Editor role
        editor_perms = [
            Permission("content.create", "content", "create"),
            Permission("content.read", "content", "read"),
            Permission("content.update", "content", "update"),
            Permission("content.delete", "content", "delete")
        ]
        self.roles["editor"] = Role("editor", editor_perms, "Content Editor")
        
        # Viewer role
        viewer_perms = [
            Permission("content.read", "content", "read")
        ]
        self.roles["viewer"] = Role("viewer", viewer_perms, "Viewer")
    
    def create_user(
        self,
        email: str,
        username: str,
        roles: Optional[List[str]] = None
    ) -> User:
        """Create new user"""
        if email in self.email_index:
            raise ValueError(f"Email {email} already exists")
        
        if username in self.username_index:
            raise ValueError(f"Username {username} already exists")
        
        import uuid
        user_id = str(uuid.uuid4())
        
        user = User(
            id=user_id,
            email=email,
            username=username,
            roles=roles or ["viewer"]
        )
        
        self.users[user_id] = user
        self.email_index[email] = user_id
        self.username_index[username] = user_id
        
        return user
    
    def get_user(self, user_id: str) -> Optional[User]:
        """Get user by ID"""
        return self.users.get(user_id)
    
    def get_user_by_email(self, email: str) -> Optional[User]:
        """Get user by email"""
        user_id = self.email_index.get(email)
        return self.users.get(user_id) if user_id else None
    
    def get_user_by_username(self, username: str) -> Optional[User]:
        """Get user by username"""
        user_id = self.username_index.get(username)
        return self.users.get(user_id) if user_id else None
    
    def update_user(self, user_id: str, **kwargs) -> bool:
        """Update user"""
        user = self.get_user(user_id)
        if not user:
            return False
        
        for key, value in kwargs.items():
            if hasattr(user, key):
                setattr(user, key, value)
        
        return True
    
    def delete_user(self, user_id: str) -> bool:
        """Delete user (soft delete)"""
        return self.update_user(user_id, status="deleted")
    
    def assign_role(self, user_id: str, role_name: str) -> bool:
        """Assign role to user"""
        user = self.get_user(user_id)
        if not user or role_name not in self.roles:
            return False
        
        if role_name not in user.roles:
            user.roles.append(role_name)
        
        return True
    
    def remove_role(self, user_id: str, role_name: str) -> bool:
        """Remove role from user"""
        user = self.get_user(user_id)
        if not user:
            return False
        
        if role_name in user.roles:
            user.roles.remove(role_name)
        
        return True
    
    def has_permission(self, user_id: str, permission: str) -> bool:
        """Check if user has permission"""
        user = self.get_user(user_id)
        if not user or user.status != "active":
            return False
        
        # Check each role
        for role_name in user.roles:
            role = self.roles.get(role_name)
            if not role:
                continue
            
            for perm in role.permissions:
                # Wildcard match
                if perm.resource == "*" or perm.name == permission:
                    return True
        
        return False
    
    def list_users(
        self,
        status: Optional[str] = None,
        role: Optional[str] = None,
        page: int = 1,
        per_page: int = 20
    ) -> Dict:
        """List users with pagination"""
        users = list(self.users.values())
        
        # Filter by status
        if status:
            users = [u for u in users if u.status == status]
        
        # Filter by role
        if role:
            users = [u for u in users if role in u.roles]
        
        # Paginate
        start = (page - 1) * per_page
        end = start + per_page
        
        return {
            "items": users[start:end],
            "total": len(users),
            "page": page,
            "per_page": per_page
        }
    
    def search_users(self, query: str) -> List[User]:
        """Search users by email or username"""
        query_lower = query.lower()
        
        return [
            user for user in self.users.values()
            if query_lower in user.email.lower() or query_lower in user.username.lower()
        ]
    
    def update_last_login(self, user_id: str):
        """Update last login timestamp"""
        user = self.get_user(user_id)
        if user:
            user.last_login = datetime.utcnow()
    
    def get_user_stats(self) -> Dict:
        """Get user statistics"""
        users = list(self.users.values())
        
        return {
            "total": len(users),
            "active": len([u for u in users if u.status == "active"]),
            "suspended": len([u for u in users if u.status == "suspended"]),
            "deleted": len([u for u in users if u.status == "deleted"]),
            "by_role": {
                role: len([u for u in users if role in u.roles])
                for role in self.roles.keys()
            }
        }
