"""
GUL RBAC (Role-Based Access Control)
Advanced authorization with permissions.

Status: ✅ Implemented  
Phase: 4
"""

from typing import Dict, List, Set
from dataclasses import dataclass, field

__version__ = "0.1.0"
__all__ = ['RBAC', 'AuthorizationManager']

@dataclass
class Permission:
    resource: str
    action: str
    conditions: Dict = field(default_factory=dict)

class RBAC:
    """
    Role-Based Access Control
    
    Example:
        rbac = RBAC()
        rbac.create_role("editor", ["post:create", "post:edit", "post:delete"])
        rbac.assign_role("user_123", "editor")
        
        if rbac.can("user_123", "post:create"):
            # Allow action
            pass
    """
    
    def __init__(self):
        self.roles: Dict[str, Set[str]] = {}
        self.user_roles: Dict[str, Set[str]] = {}
    
    def create_role(self, role: str, permissions: List[str]):
        self.roles[role] = set(permissions)
    
    def assign_role(self, user_id: str, role: str):
        if user_id not in self.user_roles:
            self.user_roles[user_id] = set()
        self.user_roles[user_id].add(role)
    
    def can(self, user_id: str, permission: str) -> bool:
        user_roles = self.user_roles.get(user_id, set())
        for role in user_roles:
            if permission in self.roles.get(role, set()):
                return True
        return False

class AuthorizationManager:
    """Complete authorization with policies"""
    def __init__(self):
        self.rbac = RBAC()
        self.policies: Dict[str, callable] = {}
    
    def add_policy(self, name: str, evaluator: callable):
        self.policies[name] = evaluator
    
    def authorize(self, user_id: str, action: str, resource: Dict = None) -> bool:
        if not self.rbac.can(user_id, action):
            return False
        
        for policy in self.policies.values():
            if not policy(user_id, action, resource):
                return False
        
        return True
