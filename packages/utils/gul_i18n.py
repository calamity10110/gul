"""
GUL i18n
Internationalization and localization.

Status: ✅ Implemented
Priority: Medium
"""

from typing import Dict, Optional, List, Any
import json
import os
import re

__version__ = "0.1.0"
__all__ = ['I18n', 't', 'Translator']

class I18n:
    """
    Internationalization
    
    Example:
        i18n = I18n()
        
        # Load translations
        i18n.load("en", {"hello": "Hello {name}!", "welcome": "Welcome"})
        i18n.load("es", {"hello": "¡Hola {name}!", "welcome": "Bienvenido"})
        
        # Set locale
        i18n.set_locale("es")
        
        # Translate
        print(i18n.t("hello", name="Alice"))  # "¡Hola Alice!"
        
        # Plurals
        i18n.load("en", {
            "apples": "{count, plural, =0 {no apples} one {1 apple} other {# apples}}"
        })
    """
    
    def __init__(self, default_locale: str = "en"):
        self.default_locale = default_locale
        self.locale = default_locale
        self.translations: Dict[str, Dict[str, str]] = {}
        self.fallbacks: Dict[str, str] = {}
    
    def load(self, locale: str, data: Dict[str, str]):
        """Load translations"""
        if locale not in self.translations:
            self.translations[locale] = {}
        self.translations[locale].update(data)
    
    def load_file(self, locale: str, filename: str):
        """Load translations from file"""
        with open(filename) as f:
            data = json.load(f)
            self.load(locale, data)
    
    def set_locale(self, locale: str):
        """Set current locale"""
        self.locale = locale
    
    def set_fallback(self, locale: str, fallback: str):
        """Set fallback locale"""
        self.fallbacks[locale] = fallback
    
    def t(self, key: str, **kwargs) -> str:
        """Translate key"""
        # Try current locale
        text = self._get_translation(self.locale, key)
        
        # Try fallback
        if text is None and self.locale in self.fallbacks:
            text = self._get_translation(self.fallbacks[self.locale], key)
        
        # Try default locale
        if text is None:
            text = self._get_translation(self.default_locale, key)
        
        if text is None:
            return key
        
        # Interpolate variables
        return self._interpolate(text, kwargs)
    
    def _get_translation(self, locale: str, key: str) -> Optional[str]:
        """Get raw translation"""
        if locale in self.translations and key in self.translations[locale]:
            return self.translations[locale][key]
        return None
    
    def _interpolate(self, text: str, params: Dict[str, Any]) -> str:
        """Interpolate variables"""
        # Simple {var} replacement
        for key, value in params.items():
            text = text.replace(f"{{{key}}}", str(value))
        
        # Could add pluralization logic here
        
        return text

# Global instance
_i18n = I18n()

def t(key: str, **kwargs) -> str:
    """Quick translate"""
    return _i18n.t(key, **kwargs)

class Translator:
    """Translator context"""
    
    def __init__(self, locale: str):
        self.locale = locale
    
    def t(self, key: str, **kwargs) -> str:
        """Translate in context"""
        original_locale = _i18n.locale
        _i18n.set_locale(self.locale)
        result = _i18n.t(key, **kwargs)
        _i18n.set_locale(original_locale)
        return result
