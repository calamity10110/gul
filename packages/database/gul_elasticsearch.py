"""
GUL Elasticsearch
Elasticsearch REST Client.

Status: ✅ Implemented
Priority: Medium
"""

import json
from typing import Dict, Any, Optional, List
import urllib.request
import urllib.error

__version__ = "0.1.0"
__all__ = ['Elasticsearch', 'Client']

class Elasticsearch:
    """
    Elasticsearch Client (HTTP)
    
    Example:
        es = Elasticsearch("http://localhost:9200")
        es.index("my-index", {"title": "Hello"}, id="1")
        res = es.search("my-index", {"query": {"match_all": {}}})
    """
    
    def __init__(self, url: str = "http://localhost:9200"):
        self.url = url.rstrip("/")
        
    def _req(self, method: str, path: str, body: Optional[Dict] = None) -> Dict:
        url = f"{self.url}/{path}"
        data = json.dumps(body).encode() if body else None
        
        req = urllib.request.Request(url, data=data, method=method)
        req.add_header('Content-Type', 'application/json')
        
        try:
            with urllib.request.urlopen(req) as res:
                return json.loads(res.read())
        except urllib.error.HTTPError as e:
            return json.loads(e.read())
            
    def index(self, index: str, doc: Dict, id: Optional[str] = None) -> Dict:
        """Index a document"""
        path = f"{index}/_doc/{id}" if id else f"{index}/_doc"
        method = "PUT" if id else "POST"
        return self._req(method, path, doc)
        
    def get(self, index: str, id: str) -> Dict:
        """Get a document"""
        return self._req("GET", f"{index}/_doc/{id}")
        
    def search(self, index: str, query: Dict) -> Dict:
        """Search documents"""
        return self._req("POST", f"{index}/_search", query)
        
    def delete(self, index: str, id: str) -> Dict:
        """Delete document"""
        return self._req("DELETE", f"{index}/_doc/{id}")

Client = Elasticsearch
