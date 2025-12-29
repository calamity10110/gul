"""
GUL Pagination
Data pagination utilities.

Status: ✅ Implemented
Priority: Medium
"""

from typing import List, Dict, Any, Optional, TypeVar, Generic
from dataclasses import dataclass
from math import ceil

__version__ = "0.1.0"
__all__ = ['Paginator', 'Page', 'paginate']

T = TypeVar('T')

@dataclass
class Page(Generic[T]):
    """Pagination result"""
    items: List[T]
    page: int
    per_page: int
    total: int
    total_pages: int
    has_next: bool
    has_prev: bool
    
    def to_dict(self) -> Dict:
        """Convert to dictionary"""
        return {
            'items': self.items,
            'page': self.page,
            'per_page': self.per_page,
            'total': self.total,
            'total_pages': self.total_pages,
            'has_next': self.has_next,
            'has_prev': self.has_prev
        }

class Paginator:
    """
    Paginator
    
    Example:
        items = list(range(100))
        
        paginator = Paginator(items, per_page=10)
        
        # Get page
        page = paginator.get_page(1)
        print(page.items)  # [0, 1, 2, ..., 9]
        print(page.has_next)  # True
        
        # Iterate pages
        for page in paginator:
            process(page.items)
    """
    
    def __init__(self, items: List[Any], per_page: int = 20):
        self.items = items
        self.per_page = per_page
        self.total = len(items)
        self.total_pages = ceil(self.total / per_page) if per_page > 0 else 0
    
    def get_page(self, page: int) -> Page:
        """Get specific page"""
        if page < 1:
            page = 1
        if page > self.total_pages:
            page = self.total_pages
        
        start = (page - 1) * self.per_page
        end = start + self.per_page
        
        items = self.items[start:end]
        
        return Page(
            items=items,
            page=page,
            per_page=self.per_page,
            total=self.total,
            total_pages=self.total_pages,
            has_next=page < self.total_pages,
            has_prev=page > 1
        )
    
    def __iter__(self):
        """Iterate all pages"""
        for page_num in range(1, self.total_pages + 1):
            yield self.get_page(page_num)

def paginate(items: List[Any], page: int = 1, per_page: int = 20) -> Page:
    """Quick pagination"""
    paginator = Paginator(items, per_page)
    return paginator.get_page(page)
