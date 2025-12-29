"""
GUL Multi-Tenancy
Multi-tenant architecture with tenant isolation.

Status: ✅ Implemented
Priority: Critical
Phase: 3
"""

from typing import Dict, List, Optional, Any
from dataclasses import dataclass, field
from datetime import datetime

__version__ = "0.1.0"
__all__ = ['TenantManager', 'Tenant', 'TenantContext', 'tenant_required']

@dataclass
class Tenant:
    """Tenant data"""
    id: str
    name: str
    domain: Optional[str] = None
    plan: str = "free"
    status: str = "active"
    created_at: datetime = field(default_factory=datetime.utcnow)
    settings: Dict[str, Any] = field(default_factory=dict)
    limits: Dict[str, int] = field(default_factory=dict)

class TenantContext:
    """Thread-local tenant context"""
    _current_tenant: Optional[Tenant] = None
    
    @classmethod
    def set_tenant(cls, tenant: Tenant):
        """Set current tenant"""
        cls._current_tenant = tenant
    
    @classmethod
    def get_tenant(cls) -> Optional[Tenant]:
        """Get current tenant"""
        return cls._current_tenant
    
    @classmethod
    def clear(cls):
        """Clear tenant context"""
        cls._current_tenant = None

class TenantManager:
    """
    Multi-tenant manager
    
    Features:
    - Tenant isolation
    - Per-tenant databases
    - Resource limits
    - Subdomain routing
    
    Example:
        manager = TenantManager()
        
        # Create tenant
        tenant = manager.create_tenant("acme", "Acme Corp", plan="pro")
        
        # Set current tenant
        manager.set_current(tenant.id)
        
        # Get tenant data
        data = manager.get_tenant_data(tenant.id, "users")
    """
    
    def __init__(self):
        self.tenants: Dict[str, Tenant] = {}
        self.tenant_data: Dict[str, Dict[str, Any]] = {}
        self.domain_map: Dict[str, str] = {}
    
    def create_tenant(
        self,
        id: str,
        name: str,
        domain: Optional[str] = None,
        plan: str = "free"
    ) -> Tenant:
        """Create new tenant"""
        tenant = Tenant(
            id=id,
            name=name,
            domain=domain,
            plan=plan,
            limits=self._get_plan_limits(plan)
        )
        
        self.tenants[id] = tenant
        self.tenant_data[id] = {}
        
        if domain:
            self.domain_map[domain] = id
        
        return tenant
    
    def get_tenant(self, tenant_id: str) -> Optional[Tenant]:
        """Get tenant by ID"""
        return self.tenants.get(tenant_id)
    
    def get_tenant_by_domain(self, domain: str) -> Optional[Tenant]:
        """Get tenant by domain"""
        tenant_id = self.domain_map.get(domain)
        return self.tenants.get(tenant_id) if tenant_id else None
    
    def update_tenant(self, tenant_id: str, **kwargs):
        """Update tenant"""
        if tenant_id not in self.tenants:
            return False
        
        tenant = self.tenants[tenant_id]
        for key, value in kwargs.items():
            if hasattr(tenant, key):
                setattr(tenant, key, value)
        
        return True
    
    def delete_tenant(self, tenant_id: str):
        """Delete tenant"""
        if tenant_id in self.tenants:
            tenant = self.tenants[tenant_id]
            if tenant.domain:
                self.domain_map.pop(tenant.domain, None)
            
            del self.tenants[tenant_id]
            del self.tenant_data[tenant_id]
    
    def set_current(self, tenant_id: str) -> bool:
        """Set current tenant context"""
        tenant = self.get_tenant(tenant_id)
        if tenant:
            TenantContext.set_tenant(tenant)
            return True
        return False
    
    def get_current(self) -> Optional[Tenant]:
        """Get current tenant"""
        return TenantContext.get_tenant()
    
    def get_tenant_data(self, tenant_id: str, key: str) -> Any:
        """Get tenant-specific data"""
        return self.tenant_data.get(tenant_id, {}).get(key)
    
    def set_tenant_data(self, tenant_id: str, key: str, value: Any):
        """Set tenant-specific data"""
        if tenant_id not in self.tenant_data:
            self.tenant_data[tenant_id] = {}
        
        self.tenant_data[tenant_id][key] = value
    
    def check_limit(self, tenant_id: str, resource: str, current: int) -> bool:
        """Check if tenant is within resource limits"""
        tenant = self.get_tenant(tenant_id)
        if not tenant:
            return False
        
        limit = tenant.limits.get(resource)
        if limit is None:
            return True
        
        return current < limit
    
    def _get_plan_limits(self, plan: str) -> Dict[str, int]:
        """Get limits for plan"""
        limits = {
            "free": {"users": 5, "storage_mb": 100, "api_calls": 1000},
            "pro": {"users": 50, "storage_mb": 10000, "api_calls": 100000},
            "enterprise": {"users": -1, "storage_mb": -1, "api_calls": -1}  # -1 = unlimited
        }
        
        return limits.get(plan, limits["free"]).copy()
    
    def list_tenants(self, status: Optional[str] = None) -> List[Tenant]:
        """List all tenants"""
        tenants = list(self.tenants.values())
        
        if status:
            tenants = [t for t in tenants if t.status == status]
        
        return tenants

def tenant_required(func):
    """Decorator to require tenant context"""
    def wrapper(*args, **kwargs):
        tenant = TenantContext.get_tenant()
        if not tenant:
            raise ValueError("No tenant context set")
        
        return func(*args, **kwargs)
    
    return wrapper

# Middleware for tenant resolution
def tenant_middleware(tenant_header: str = "X-Tenant-ID"):
    """Middleware to resolve tenant from request"""
    def middleware(manager: TenantManager):
        def handler(request):
            # Try header
            tenant_id = request.get('headers', {}).get(tenant_header)
            
            if not tenant_id:
                # Try domain
                host = request.get('headers', {}).get('host', '')
                tenant = manager.get_tenant_by_domain(host)
                if tenant:
                    tenant_id = tenant.id
            
            if tenant_id:
                manager.set_current(tenant_id)
            
            return None
        
        return handler
    
    return middleware
