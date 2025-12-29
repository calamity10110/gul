"""
GUL Admin Dashboard
Admin dashboard with CRUD operations.

Status: ✅ Implemented
Priority: High
Phase: 3
"""

from typing import Dict, List, Optional, Any, Callable
from dataclasses import dataclass, field
from datetime import datetime

__version__ = "0.1.0"
__all__ = ['AdminDashboard', 'AdminUser', 'AuditLog', 'DashboardWidget']

@dataclass
class AdminUser:
    """Admin user"""
    id: str
    email: str
    role: str  # "superadmin", "admin", "moderator"
    permissions: List[str] = field(default_factory=list)
    created_at: datetime = field(default_factory=datetime.utcnow)

@dataclass
class AuditLog:
    """Audit log entry"""
    id: str
    admin_id: str
    action: str
    resource_type: str
    resource_id: str
    changes: Dict[str, Any]
    timestamp: datetime = field(default_factory=datetime.utcnow)

@dataclass
class DashboardWidget:
    """Dashboard widget"""
    id: str
    title: str
    type: str  # "stat", "chart", "table", "list"
    data_source: Callable
    config: Dict = field(default_factory=dict)

class AdminDashboard:
    """
    Admin dashboard with auth and audit logging
    
    Example:
        dashboard = AdminDashboard()
        
        # Create admin
        admin = dashboard.create_admin("admin@example.com", "superadmin")
        
        # Define widget
        def get_user_count():
            return {"value": 1250, "change": "+12%"}
        
        dashboard.add_widget(
            "users", "Total Users", "stat", get_user_count
        )
        
        # Perform admin action
        dashboard.update_resource(
            admin.id, "user", "user_123", {"status": "suspended"}
        )
        
        # Get dashboard data
        data = dashboard.get_dashboard_data()
    """
    
    def __init__(self):
        self.admins: Dict[str, AdminUser] = {}
        self.audit_logs: List[AuditLog] = []
        self.widgets: Dict[str, DashboardWidget] = {}
        self.resources: Dict[str, Dict[str, Any]] = {}
        self.log_counter = 0
    
    def create_admin(
        self,
        email: str,
        role: str = "admin",
        permissions: Optional[List[str]] = None
    ) -> AdminUser:
        """Create admin user"""
        import uuid
        admin_id = str(uuid.uuid4())
        
        admin = AdminUser(
            id=admin_id,
            email=email,
            role=role,
            permissions=permissions or self._get_default_permissions(role)
        )
        
        self.admins[admin_id] = admin
        self._log_action(admin_id, "create", "admin", admin_id, {"email": email})
        
        return admin
    
    def get_admin(self, admin_id: str) -> Optional[AdminUser]:
        """Get admin"""
        return self.admins.get(admin_id)
    
    def has_permission(self, admin_id: str, permission: str) -> bool:
        """Check admin permission"""
        admin = self.get_admin(admin_id)
        if not admin:
            return False
        
        return permission in admin.permissions or admin.role == "superadmin"
    
    def add_widget(
        self,
        widget_id: str,
        title: str,
        widget_type: str,
        data_source: Callable,
        config: Optional[Dict] = None
    ):
        """Add dashboard widget"""
        widget = DashboardWidget(
            id=widget_id,
            title=title,
            type=widget_type,
            data_source=data_source,
            config=config or {}
        )
        
        self.widgets[widget_id] = widget
    
    def get_dashboard_data(self) -> Dict[str, Any]:
        """Get all dashboard data"""
        data = {}
        
        for widget_id, widget in self.widgets.items():
            try:
                data[widget_id] = {
                    "title": widget.title,
                    "type": widget.type,
                    "data": widget.data_source(),
                    "config": widget.config
                }
            except Exception as e:
                data[widget_id] = {
                    "title": widget.title,
                    "error": str(e)
                }
        
        return data
    
    def list_resources(
        self,
        resource_type: str,
        page: int = 1,
        per_page: int = 20,
        filters: Optional[Dict] = None
    ) -> Dict:
        """List resources with pagination"""
        resources = list(self.resources.get(resource_type, {}).values())
        
        # Apply filters
        if filters:
            for key, value in filters.items():
                resources = [r for r in resources if r.get(key) == value]
        
        # Paginate
        start = (page - 1) * per_page
        end = start + per_page
        
        return {
            "items": resources[start:end],
            "total": len(resources),
            "page": page,
            "per_page": per_page
        }
    
    def get_resource(self, resource_type: str, resource_id: str) -> Optional[Any]:
        """Get single resource"""
        return self.resources.get(resource_type, {}).get(resource_id)
    
    def create_resource(
        self,
        admin_id: str,
        resource_type: str,
        data: Dict
    ) -> Dict:
        """Create resource"""
        if not self.has_permission(admin_id, f"{resource_type}.create"):
            raise PermissionError(f"No permission to create {resource_type}")
        
        import uuid
        resource_id = str(uuid.uuid4())
        data["id"] = resource_id
        data["created_at"] = datetime.utcnow().isoformat()
        
        if resource_type not in self.resources:
            self.resources[resource_type] = {}
        
        self.resources[resource_type][resource_id] = data
        
        self._log_action(admin_id, "create", resource_type, resource_id, data)
        
        return data
    
    def update_resource(
        self,
        admin_id: str,
        resource_type: str,
        resource_id: str,
        changes: Dict
    ) -> bool:
        """Update resource"""
        if not self.has_permission(admin_id, f"{resource_type}.update"):
            raise PermissionError(f"No permission to update {resource_type}")
        
        resource = self.get_resource(resource_type, resource_id)
        if not resource:
            return False
        
        resource.update(changes)
        resource["updated_at"] = datetime.utcnow().isoformat()
        
        self._log_action(admin_id, "update", resource_type, resource_id, changes)
        
        return True
    
    def delete_resource(
        self,
        admin_id: str,
        resource_type: str,
        resource_id: str
    ) -> bool:
        """Delete resource"""
        if not self.has_permission(admin_id, f"{resource_type}.delete"):
            raise PermissionError(f"No permission to delete {resource_type}")
        
        if resource_type not in self.resources or resource_id not in self.resources[resource_type]:
            return False
        
        del self.resources[resource_type][resource_id]
        
        self._log_action(admin_id, "delete", resource_type, resource_id, {})
        
        return True
    
    def get_audit_logs(
        self,
        admin_id: Optional[str] = None,
        resource_type: Optional[str] = None,
        limit: int = 100
    ) -> List[AuditLog]:
        """Get audit logs"""
        logs = self.audit_logs
        
        if admin_id:
            logs = [l for l in logs if l.admin_id == admin_id]
        
        if resource_type:
            logs = [l for l in logs if l.resource_type == resource_type]
        
        return logs[-limit:]
    
    def _log_action(
        self,
        admin_id: str,
        action: str,
        resource_type: str,
        resource_id: str,
        changes: Dict
    ):
        """Log admin action"""
        self.log_counter += 1
        log = AuditLog(
            id=f"log_{self.log_counter}",
            admin_id=admin_id,
            action=action,
            resource_type=resource_type,
            resource_id=resource_id,
            changes=changes
        )
        
        self.audit_logs.append(log)
    
    def _get_default_permissions(self, role: str) -> List[str]:
        """Get default permissions for role"""
        permissions_map = {
            "superadmin": ["*"],
            "admin": [
                "user.create", "user.read", "user.update", "user.delete",
                "content.create", "content.read", "content.update", "content.delete"
            ],
            "moderator": [
                "user.read", "user.update",
                "content.read", "content.update", "content.delete"
            ]
        }
        
        return permissions_map.get(role, [])
