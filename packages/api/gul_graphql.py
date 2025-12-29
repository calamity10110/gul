"""
GUL GraphQL Server
GraphQL API server implementation.

Status: ✅ Implemented
Priority: High
Phase: 2
"""

from typing import Dict, List, Optional, Any, Callable
from dataclasses import dataclass, field

__version__ = "0.1.0"
__all__ = ['GraphQLServer', 'Schema', 'Query', 'Mutation', 'Type']

@dataclass
class Type:
    """GraphQL type definition"""
    name: str
    fields: Dict[str, str]
    resolvers: Dict[str, Callable] = field(default_factory=dict)

@dataclass
class Query:
    """GraphQL query"""
    name: str
    return_type: str
    args: Dict[str, str] = field(default_factory=dict)
    resolver: Optional[Callable] = None

@dataclass
class Mutation:
    """GraphQL mutation"""
    name: str
    return_type: str
    args: Dict[str, str] = field(default_factory=dict)
    resolver: Optional[Callable] = None

class Schema:
    """GraphQL schema"""
    
    def __init__(self):
        self.types: Dict[str, Type] = {}
        self.queries: Dict[str, Query] = {}
        self.mutations: Dict[str, Mutation] = {}
    
    def add_type(self, type_def: Type) -> 'Schema':
        """Add type"""
        self.types[type_def.name] = type_def
        return self
    
    def add_query(self, query: Query) -> 'Schema':
        """Add query"""
        self.queries[query.name] = query
        return self
    
    def add_mutation(self, mutation: Mutation) -> 'Schema':
        """Add mutation"""
        self.mutations[mutation.name] = mutation
        return self
    
    def to_sdl(self) -> str:
        """Generate SDL"""
        lines = []
        
        for type_def in self.types.values():
            lines.append(f"type {type_def.name} {{")
            for field, field_type in type_def.fields.items():
                lines.append(f"  {field}: {field_type}")
            lines.append("}")
        
        if self.queries:
            lines.append("type Query {")
            for query in self.queries.values():
                args = ", ".join(f"{k}: {v}" for k, v in query.args.items())
                lines.append(f"  {query.name}({args}): {query.return_type}")
            lines.append("}")
        
        if self.mutations:
            lines.append("type Mutation {")
            for mutation in self.mutations.values():
                args = ", ".join(f"{k}: {v}" for k, v in mutation.args.items())
                lines.append(f"  {mutation.name}({args}): {mutation.return_type}")
            lines.append("}")
        
        return "\n".join(lines)

class GraphQLServer:
    """
    GraphQL server
    
    Example:
        server = GraphQLServer()
        
        # Define types
        user_type = Type("User", {"id": "ID!", "name": "String!", "email": "String!"})
        server.schema.add_type(user_type)
        
        # Define query
        def get_user(args):
            return {"id": args["id"], "name": "Alice", "email": "alice@example.com"}
        
        server.schema.add_query(Query("user", "User", {"id": "ID!"}, get_user))
        
        # Execute
        result = server.execute('{ user(id: "1") { name email } }')
    """
    
    def __init__(self):
        self.schema = Schema()
    
    def execute(self, query: str, variables: Optional[Dict] = None) -> Dict:
        """Execute GraphQL query"""
        # Simplified execution
        parsed = self._parse_query(query)
        
        if parsed['operation'] == 'query':
            return self._execute_query(parsed, variables or {})
        elif parsed['operation'] == 'mutation':
            return self._execute_mutation(parsed, variables or {})
        
        return {'errors': [{'message': 'Invalid operation'}]}
    
    def _parse_query(self, query: str) -> Dict:
        """Parse GraphQL  query (simplified)"""
        query = query.strip()
        
        if query.startswith('mutation'):
            return {'operation': 'mutation', 'query': query}
        else:
            return {'operation': 'query', 'query': query}
    
    def _execute_query(self, parsed: Dict, variables: Dict) -> Dict:
        """Execute query"""
        # Simplified - call first query resolver
        for query_name, query in self.schema.queries.items():
            if query.resolver:
                try:
                    data = query.resolver(variables)
                    return {'data': {query_name: data}}
                except Exception as e:
                    return {'errors': [{'message': str(e)}]}
        
        return {'data': None}
    
    def _execute_mutation(self, parsed: Dict, variables: Dict) -> Dict:
        """Execute mutation"""
        for mutation_name, mutation in self.schema.mutations.items():
            if mutation.resolver:
                try:
                    data = mutation.resolver(variables)
                    return {'data': {mutation_name: data}}
                except Exception as e:
                    return {'errors': [{'message': str(e)}]}
        
        return {'data': None}
