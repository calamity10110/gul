"""
GUL Template Engine
Template rendering engine.

Status: ✅ Implemented
Priority: Medium
"""

from typing import Dict, Any, Callable, Optional
import re

__version__ = "0.1.0"
__all__ = ['Template', 'render']

class Template:
    """
    Template engine
    
    Example:
        template = Template("Hello {{name}}! You have {{count}} messages.")
        result = template.render(name="Alice", count=5)
        # "Hello Alice! You have 5 messages."
        
        # Conditionals
        template = Template("{{#if logged_in}}Welcome back!{{/if}}")
        
        # Loops
        template = Template("{{#each items}}- {{name}}\n{{/each}}")
    """
    
    def __init__(self, content: str):
        self.content = content
        self.filters: Dict[str, Callable] = {
            'upper': str.upper,
            'lower': str.lower,
            'title': str.title,
            'trim': str.strip
        }
    
    def render(self, **context) -> str:
        """Render template with context"""
        result = self.content
        
        # Process blocks (if, each)
        result = self._process_blocks(result, context)
        
        # Process variables
        result = self._process_variables(result, context)
        
        return result
    
    def _process_variables(self, content: str, context: Dict) -> str:
        """Process {{variable}} syntax"""
        def replace_var(match):
            var_expr = match.group(1).strip()
            
            # Check for filters: {{name|upper}}
            if '|' in var_expr:
                var_name, filter_name = var_expr.split('|', 1)
                var_name = var_name.strip()
                filter_name = filter_name.strip()
                
                value = self._get_value(var_name, context)
                
                if filter_name in self.filters:
                    value = self.filters[filter_name](str(value))
                
                return str(value)
            
            value = self._get_value(var_expr, context)
            return str(value) if value is not None else ''
        
        return re.sub(r'\{\{([^}]+)\}\}', replace_var, content)
    
    def _process_blocks(self, content: str, context: Dict) -> str:
        """Process {{#if}}, {{#each}} blocks"""
        # Process if blocks
        content = self._process_if_blocks(content, context)
        
        # Process each blocks
        content = self._process_each_blocks(content, context)
        
        return content
    
    def _process_if_blocks(self, content: str, context: Dict) -> str:
        """Process {{#if condition}}...{{/if}}"""
        pattern = r'\{\{#if\s+([^}]+)\}\}(.*?)\{\{/if\}\}'
        
        def replace_if(match):
            condition = match.group(1).strip()
            block_content = match.group(2)
            
            value = self._get_value(condition, context)
            
            if value:
                return block_content
            
            return ''
        
        return re.sub(pattern, replace_if, content, flags=re.DOTALL)
    
    def _process_each_blocks(self, content: str, context: Dict) -> str:
        """Process {{#each items}}...{{/each}}"""
        pattern = r'\{\{#each\s+([^}]+)\}\}(.*?)\{\{/each\}\}'
        
        def replace_each(match):
            var_name = match.group(1).strip()
            block_content = match.group(2)
            
            items = self._get_value(var_name, context)
            
            if not items or not isinstance(items, (list, tuple)):
                return ''
            
            results = []
            for item in items:
                if isinstance(item, dict):
                    item_result = self._process_variables(block_content, item)
                else:
                    item_result = self._process_variables(block_content, {'item': item})
                results.append(item_result)
            
            return ''.join(results)
        
        return re.sub(pattern, replace_each, content, flags=re.DOTALL)
    
    def _get_value(self, path: str, context: Dict) -> Any:
        """Get value from context by path (e.g., 'user.name')"""
        parts = path.split('.')
        value = context
        
        for part in parts:
            if isinstance(value, dict):
                value = value.get(part)
            else:
                return None
        
        return value
    
    def add_filter(self, name: str, func: Callable):
        """Add custom filter"""
        self.filters[name] = func

def render(template: str, **context) -> str:
    """Quick template render"""
    tmpl = Template(template)
    return tmpl.render(**context)
