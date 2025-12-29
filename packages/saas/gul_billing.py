"""
GUL Billing & Subscriptions
Subscription management and billing.

Status: ✅ Implemented
Priority: Critical
Phase: 3
"""

from typing import Dict, List, Optional
from dataclasses import dataclass, field
from datetime import datetime, timedelta
from enum import Enum

__version__ = "0.1.0"
__all__ = ['BillingManager', 'Subscription', 'Plan', 'Invoice', 'SubscriptionStatus']

class SubscriptionStatus(Enum):
    """Subscription status"""
    ACTIVE = "active"
    TRIAL = "trial"
    PAST_DUE = "past_due"
    CANCELED = "canceled"
    EXPIRED = "expired"

@dataclass
class Plan:
    """Subscription plan"""
    id: str
    name: str
    price: float
    interval: str  # "month" or "year"
    features: Dict[str, Any] = field(default_factory=dict)
    limits: Dict[str, int] = field(default_factory=dict)
    trial_days: int = 0

@dataclass
class Subscription:
    """User subscription"""
    id: str
    user_id: str
    plan_id: str
    status: SubscriptionStatus
    current_period_start: datetime
    current_period_end: datetime
    cancel_at_period_end: bool = False
    trial_end: Optional[datetime] = None
    metadata: Dict = field(default_factory=dict)

@dataclass
class Invoice:
    """Billing invoice"""
    id: str
    subscription_id: str
    amount: float
    status: str  # "draft", "open", "paid", "void"
    created_at: datetime
    due_date: datetime
    paid_at: Optional[datetime] = None
    items: List[Dict] = field(default_factory=list)

class BillingManager:
    """
    Subscription and billing manager
    
    Example:
        billing = BillingManager()
        
        # Create plan
        plan = Plan("pro", "Pro Plan", 29.99, "month", trial_days=14)
        billing.add_plan(plan)
        
        # Subscribe user
        sub = billing.subscribe("user_123", "pro")
        
        # Check subscription
        if billing.is_active("user_123"):
            print("Active subscription")
        
        # Cancel subscription
        billing.cancel("user_123", at_period_end=True)
    """
    
    def __init__(self):
        self.plans: Dict[str, Plan] = {}
        self.subscriptions: Dict[str, Subscription] = {}  # user_id -> subscription
        self.invoices: Dict[str, List[Invoice]] = {}
        self.invoice_counter = 0
        self.sub_counter = 0
    
    def add_plan(self, plan: Plan):
        """Add subscription plan"""
        self.plans[plan.id] = plan
    
    def get_plan(self, plan_id: str) -> Optional[Plan]:
        """Get plan"""
        return self.plans.get(plan_id)
    
    def subscribe(
        self,
        user_id: str,
        plan_id: str,
        trial: bool = True
    ) -> Optional[Subscription]:
        """Create subscription"""
        plan = self.get_plan(plan_id)
        if not plan:
            return None
        
        now = datetime.utcnow()
        trial_end = None
        status = SubscriptionStatus.ACTIVE
        
        if trial and plan.trial_days > 0:
            trial_end = now + timedelta(days=plan.trial_days)
            status = SubscriptionStatus.TRIAL
        
        period_end = now + timedelta(days=30 if plan.interval == "month" else 365)
        
        self.sub_counter += 1
        subscription = Subscription(
            id=f"sub_{self.sub_counter}",
            user_id=user_id,
            plan_id=plan_id,
            status=status,
            current_period_start=now,
            current_period_end=period_end,
            trial_end=trial_end
        )
        
        self.subscriptions[user_id] = subscription
        
        # Create first invoice (if not trial)
        if not trial or plan.trial_days == 0:
            self._create_invoice(subscription, plan)
        
        return subscription
    
    def get_subscription(self, user_id: str) -> Optional[Subscription]:
        """Get user subscription"""
        return self.subscriptions.get(user_id)
    
    def is_active(self, user_id: str) -> bool:
        """Check if subscription is active"""
        sub = self.get_subscription(user_id)
        if not sub:
            return False
        
        return sub.status in [SubscriptionStatus.ACTIVE, SubscriptionStatus.TRIAL]
    
    def cancel(self, user_id: str, at_period_end: bool = True):
        """Cancel subscription"""
        sub = self.get_subscription(user_id)
        if not sub:
            return False
        
        if at_period_end:
            sub.cancel_at_period_end = True
        else:
            sub.status = SubscriptionStatus.CANCELED
        
        return True
    
    def reactivate(self, user_id: str):
        """Reactivate canceled subscription"""
        sub = self.get_subscription(user_id)
        if not sub:
            return False
        
        sub.cancel_at_period_end = False
        sub.status = SubscriptionStatus.ACTIVE
        return True
    
    def change_plan(self, user_id: str, new_plan_id: str):
        """Change subscription plan"""
        sub = self.get_subscription(user_id)
        if not sub:
            return False
        
        new_plan = self.get_plan(new_plan_id)
        if not new_plan:
            return False
        
        sub.plan_id = new_plan_id
        
        # Prorate and create invoice
        self._create_invoice(sub, new_plan)
        
        return True
    
    def renew(self, user_id: str):
        """Renew subscription"""
        sub = self.get_subscription(user_id)
        if not sub:
            return False
        
        plan = self.get_plan(sub.plan_id)
        if not plan:
            return False
        
        # Update period
        now = datetime.utcnow()
        sub.current_period_start = now
        sub.current_period_end = now + timedelta(
            days=30 if plan.interval == "month" else 365
        )
        
        # End trial if active
        if sub.status == SubscriptionStatus.TRIAL:
            sub.status = SubscriptionStatus.ACTIVE
        
        # Create invoice
        self._create_invoice(sub, plan)
        
        return True
    
    def _create_invoice(self, subscription: Subscription, plan: Plan) -> Invoice:
        """Create invoice"""
        self.invoice_counter += 1
        now = datetime.utcnow()
        
        invoice = Invoice(
            id=f"inv_{self.invoice_counter}",
            subscription_id=subscription.id,
            amount=plan.price,
            status="open",
            created_at=now,
            due_date=now + timedelta(days=7),
            items=[{
                "description": f"{plan.name} subscription",
                "amount": plan.price
            }]
        )
        
        if subscription.user_id not in self.invoices:
            self.invoices[subscription.user_id] = []
        
        self.invoices[subscription.user_id].append(invoice)
        
        return invoice
    
    def get_invoices(self, user_id: str) -> List[Invoice]:
        """Get user invoices"""
        return self.invoices.get(user_id, [])
    
    def pay_invoice(self, invoice_id: str) -> bool:
        """Mark invoice as paid"""
        for invoices in self.invoices.values():
            for invoice in invoices:
                if invoice.id == invoice_id:
                    invoice.status = "paid"
                    invoice.paid_at = datetime.utcnow()
                    return True
        
        return False
    
    def get_usage(self, user_id: str) -> Dict[str, int]:
        """Get usage stats (placeholder)"""
        return {
            "api_calls": 0,
            "storage_mb": 0,
            "users": 0
        }
