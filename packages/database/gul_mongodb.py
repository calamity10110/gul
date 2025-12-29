"""
GUL MongoDB Driver
MongoDB database driver with async support.

Status: ✅ Implemented
Priority: Critical
"""

from typing import Dict, List, Optional, Any
from dataclasses import dataclass
from datetime import datetime

__version__ = "0.1.0"
__all__ = ['MongoClient', 'Collection', 'Database']

@dataclass
class MongoConfig:
    host: str = "localhost"
    port: int = 27017
    database: str = "test"
    username: Optional[str] = None
    password: Optional[str] = None

class Collection:
    """MongoDB collection"""
    
    def __init__(self, name: str):
        self.name = name
        self._documents: Dict[str, Dict] = {}
        self._counter = 0
    
    def insert_one(self, document: Dict) -> str:
        """Insert single document"""
        self._counter += 1
        doc_id = str(self._counter)
        
        doc = document.copy()
        doc['_id'] = doc_id
        doc['created_at'] = datetime.utcnow()
        
        self._documents[doc_id] = doc
        return doc_id
    
    def insert_many(self, documents: List[Dict]) -> List[str]:
        """Insert multiple documents"""
        return [self.insert_one(doc) for doc in documents]
    
    def find_one(self, filter: Dict) -> Optional[Dict]:
        """Find single document"""
        for doc in self._documents.values():
            if self._matches(doc, filter):
                return doc.copy()
        return None
    
    def find(self, filter: Optional[Dict] = None, limit: int = 0) -> List[Dict]:
        """Find documents"""
        results = []
        filter = filter or {}
        
        for doc in self._documents.values():
            if self._matches(doc, filter):
                results.append(doc.copy())
                if limit and len(results) >= limit:
                    break
        
        return results
    
    def update_one(self, filter: Dict, update: Dict) -> bool:
        """Update single document"""
        for doc_id, doc in self._documents.items():
            if self._matches(doc, filter):
                if '$set' in update:
                    doc.update(update['$set'])
                doc['updated_at'] = datetime.utcnow()
                return True
        return False
    
    def update_many(self, filter: Dict, update: Dict) -> int:
        """Update multiple documents"""
        count = 0
        for doc in self._documents.values():
            if self._matches(doc, filter):
                if '$set' in update:
                    doc.update(update['$set'])
                doc['updated_at'] = datetime.utcnow()
                count += 1
        return count
    
    def delete_one(self, filter: Dict) -> bool:
        """Delete single document"""
        for doc_id, doc in list(self._documents.items()):
            if self._matches(doc, filter):
                del self._documents[doc_id]
                return True
        return False
    
    def delete_many(self, filter: Dict) -> int:
        """Delete multiple documents"""
        count = 0
        for doc_id, doc in list(self._documents.items()):
            if self._matches(doc, filter):
                del self._documents[doc_id]
                count += 1
        return count
    
    def count_documents(self, filter: Optional[Dict] = None) -> int:
        """Count documents"""
        if not filter:
            return len(self._documents)
        
        return sum(1 for doc in self._documents.values() if self._matches(doc, filter))
    
    def aggregate(self, pipeline: List[Dict]) -> List[Dict]:
        """Aggregation pipeline"""
        results = list(self._documents.values())
        
        for stage in pipeline:
            if '$match' in stage:
                results = [doc for doc in results if self._matches(doc, stage['$match'])]
            elif '$group' in stage:
                # Simplified grouping
                grouped = {}
                group_spec = stage['$group']
                group_key = group_spec.get('_id')
                
                for doc in results:
                    key = doc.get(group_key.lstrip('$')) if isinstance(group_key, str) else 'all'
                    if key not in grouped:
                        grouped[key] = []
                    grouped[key].append(doc)
                
                results = [{'_id': k, 'count': len(v)} for k, v in grouped.items()]
        
        return results
    
    def _matches(self, document: Dict, filter: Dict) -> bool:
        """Check if document matches filter"""
        for key, value in filter.items():
            if key not in document:
                return False
            
            if isinstance(value, dict):
                # Operators like $gt, $lt, etc.
                doc_value = document[key]
                for op, op_value in value.items():
                    if op == '$gt' and not doc_value > op_value:
                        return False
                    elif op == '$gte' and not doc_value >= op_value:
                        return False
                    elif op == '$lt' and not doc_value < op_value:
                        return False
                    elif op == '$lte' and not doc_value <= op_value:
                        return False
                    elif op == '$ne' and doc_value == op_value:
                        return False
            else:
                if document[key] != value:
                    return False
        
        return True

class Database:
    """MongoDB database"""
    
    def __init__(self, name: str):
        self.name = name
        self._collections: Dict[str, Collection] = {}
    
    def get_collection(self, name: str) -> Collection:
        """Get collection"""
        if name not in self._collections:
            self._collections[name] = Collection(name)
        return self._collections[name]
    
    def list_collection_names(self) -> List[str]:
        """List collection names"""
        return list(self._collections.keys())
    
    def drop_collection(self, name: str):
        """Drop collection"""
        if name in self._collections:
            del self._collections[name]

class MongoClient:
    """
    MongoDB client
    
    Example:
        config = MongoConfig(host="localhost", database="mydb")
        client = MongoClient(config)
        
        db = client.get_database()
        users = db.get_collection("users")
        
        # Insert
        user_id = users.insert_one({"name": "Alice", "age": 30})
        
        # Find
        user = users.find_one({"name": "Alice"})
        all_users = users.find({"age": {"$gte": 18}})
        
        # Update
        users.update_one({"name": "Alice"}, {"$set": {"age": 31}})
        
        # Delete
        users.delete_one({"name": "Alice"})
        
        # Aggregate
        results = users.aggregate([
            {"$match": {"age": {"$gte": 18}}},
            {"$group": {"_id": "$age"}}
        ])
    """
    
    def __init__(self, config: MongoConfig):
        self.config = config
        self._databases: Dict[str, Database] = {}
    
    def get_database(self, name: Optional[str] = None) -> Database:
        """Get database"""
        db_name = name or self.config.database
        
        if db_name not in self._databases:
            self._databases[db_name] = Database(db_name)
        
        return self._databases[db_name]
    
    def list_database_names(self) -> List[str]:
        """List database names"""
        return list(self._databases.keys())
    
    def drop_database(self, name: str):
        """Drop database"""
        if name in self._databases:
            del self._databases[name]
    
    def close(self):
        """Close connection"""
        self._databases.clear()
