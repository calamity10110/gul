"""
GUL Search Engine
Full-text search with indexing.

Status: ✅ Implemented
Phase: 5
"""

from typing import Dict, List, Set
from dataclasses import dataclass
import re

__version__ = "0.1.0"
__all__ = ['SearchEngine', 'SearchResult']

@dataclass
class SearchResult:
    id: str
    score: float
    data: Dict

class SearchEngine:
    """
    Full-text search engine
    
    Example:
        search = SearchEngine()
        
        search.index("doc1", {"title": "Hello World", "content": "..."})
        search.index("doc2", {"title": "Python Guide", "content": "..."})
        
        results = search.search("python")
    """
    
    def __init__(self):
        self.documents: Dict[str, Dict] = {}
        self.index: Dict[str, Set[str]] = {}
    
    def index(self, doc_id: str, document: Dict):
        self.documents[doc_id] = document
        
        # Index all text fields
        text = " ".join(str(v) for v in document.values())
        words = self._tokenize(text)
        
        for word in words:
            if word not in self.index:
                self.index[word] = set()
            self.index[word].add(doc_id)
    
    def search(self, query: str, limit: int = 10) -> List[SearchResult]:
        words = self._tokenize(query)
        
        # Find matching documents
        matches: Dict[str, int] = {}
        for word in words:
            doc_ids = self.index.get(word, set())
            for doc_id in doc_ids:
                matches[doc_id] = matches.get(doc_id, 0) + 1
        
        # Sort by score
        results = [
            SearchResult(id=doc_id, score=score, data=self.documents[doc_id])
            for doc_id, score in sorted(matches.items(), key=lambda x: x[1], reverse=True)
        ]
        
        return results[:limit]
    
    def delete(self, doc_id: str):
        if doc_id in self.documents:
            del self.documents[doc_id]
            
            for word_docs in self.index.values():
                word_docs.discard(doc_id)
    
    def _tokenize(self, text: str) -> List[str]:
        text = text.lower()
        words = re.findall(r'\w+', text)
        return [w for w in words if len(w) > 2]
