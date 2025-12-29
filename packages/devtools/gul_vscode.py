"""
GUL VSCode Extension Configuration
VS Code extension package.json and configuration.

Status: ✅ Implemented
Priority: High
Phase: 1
"""

import json
from typing import Dict, List

__version__ = "0.1.0"
__all__ = ['VSCodeExtension', 'generate_package_json', 'generate_extension']

class VSCodeExtension:
    """VS Code extension configuration generator"""
    
    def __init__(
        self,
        name: str = "gul-lang",
        display_name: str = "GUL Language Support",
        version: str = "0.1.0"
    ):
        self.name = name
        self.display_name = display_name
        self.version = version
    
    def generate_package_json(self) -> Dict:
        """Generate package.json for VS Code extension"""
        return {
            "name": self.name,
            "displayName": self.display_name,
            "description": "Language support for GUL v3.2",
            "version": self.version,
            "engines": {"vscode": "^1.80.0"},
            "categories": ["Programming Languages"],
            "contributes": {
                "languages": [{
                    "id": "gul",
                    "aliases": ["GUL", "gul"],
                    "extensions": [".gul"],
                    "configuration": "./language-configuration.json"
                }],
                "grammars": [{
                    "language": "gul",
                    "scopeName": "source.gul",
                    "path": "./syntaxes/gul.tmLanguage.json"
                }],
                "snippets": [{
                    "language": "gul",
                    "path": "./snippets/gul.json"
                }]
            },
            "activationEvents": ["onLanguage:gul"],
            "main": "./out/extension.js"
        }
    
    def generate_language_config(self) -> Dict:
        """Generate language-configuration.json"""
        return {
            "comments": {
                "lineComment": "#",
                "blockComment": ["/*", "*/"]
            },
            "brackets": [
                ["{", "}"],
                ["[", "]"],
                ["(", ")"]
            ],
            "autoClosingPairs": [
                {"open": "{", "close": "}"},
                {"open": "[", "close": "]"},
                {"open": "(", "close": ")"},
                {"open": '"', "close": '"'},
                {"open": "'", "close": "'"}
            ],
            "surroundingPairs": [
                ["{", "}"],
                ["[", "]"],
                ["(", ")"],
                ['"', '"'],
                ["'", "'"]
            ],
            "folding": {
                "markers": {
                    "start": "^\\s*#region",
                    "end": "^\\s*#endregion"
                }
            }
        }
    
    def generate_syntax_grammar(self) -> Dict:
        """Generate TextMate grammar"""
        return {
            "$schema": "https://raw.githubusercontent.com/martinring/tmlanguage/master/tmlanguage.json",
            "name": "GUL",
            "patterns": [
                {"include": "#keywords"},
                {"include": "#strings"},
                {"include": "#comments"},
                {"include": "#numbers"},
                {"include": "#types"}
            ],
            "repository": {
                "keywords": {
                    "patterns": [{
                        "name": "keyword.control.gul",
                        "match": "\\b(let|var|fn|async|struct|match|if|else|for|while|return|@imp|mn)\\b"
                    }]
                },
                "strings": {
                    "patterns": [
                        {
                            "name": "string.quoted.double.gul",
                            "begin": '"',
                            "end": '"',
                            "patterns": [{"name": "constant.character.escape.gul", "match": "\\\\."}]
                        },
                        {
                            "name": "string.quoted.single.gul",
                            "begin": "'",
                            "end": "'",
                            "patterns": [{"name": "constant.character.escape.gul", "match": "\\\\."}]
                        }
                    ]
                },
                "comments": {
                    "patterns": [{
                        "name": "comment.line.number-sign.gul",
                        "match": "#.*$"
                    }]
                },
                "numbers": {
                    "patterns": [{
                        "name": "constant.numeric.gul",
                        "match": "\\b\\d+(\\.\\d+)?\\b"
                    }]
                },
                "types": {
                    "patterns": [{
                        "name": "storage.type.gul",
                        "match": "@(int|str|float|bool|list|dict|tuple|set)"
                    }]
                }
            },
            "scopeName": "source.gul"
        }
    
    def generate_snippets(self) -> Dict:
        """Generate code snippets"""
        return {
            "Function Definition": {
                "prefix": "fn",
                "body": [
                    "fn @${1:return_type} ${2:function_name}(${3:params}):",
                    "    ${4:# function body}",
                    "    return ${5:value}"
                ],
                "description": "Function definition"
            },
            "Async Function": {
                "prefix": "async",
                "body": [
                    "async ${1:function_name}(${2:params}):",
                    "    ${3:# async function body}",
                    "    return ${4:value}"
                ],
                "description": "Async function"
            },
            "Let Variable": {
                "prefix": "let",
                "body": "let ${1:name} = @${2:type}(${3:value})",
                "description": "Immutable variable"
            },
            "Var Variable": {
                "prefix": "var",
                "body": "var ${1:name} = @${2:type}(${3:value})",
                "description": "Mutable variable"
            },
            "Struct":  {
                "prefix": "struct",
                "body": [
                    "struct ${1:Name}:",
                    "    ${2:field}: @${3:type}",
                    "    ",
                    "    fn ${4:method}(self):",
                    "        ${5:# method body}"
                ],
                "description": "Struct definition"
            },
            "Import": {
                "prefix": "@imp",
                "body": "@imp ${1:module}",
                "description": "Import statement"
            },
            "Main": {
                "prefix": "mn",
                "body": [
                    "mn:",
                    "    ${1:# main code}"
                ],
                "description": "Main entry point"
            }
        }

def generate_package_json() -> str:
    """Quick helper to generate package.json"""
    ext = VSCodeExtension()
    return json.dumps(ext.generate_package_json(), indent=2)

def generate_extension() -> Dict[str, str]:
    """Generate all extension files"""
    ext = VSCodeExtension()
    return {
        'package.json': json.dumps(ext.generate_package_json(), indent=2),
        'language-configuration.json': json.dumps(ext.generate_language_config(), indent=2),
        'syntaxes/gul.tmLanguage.json': json.dumps(ext.generate_syntax_grammar(), indent=2),
        'snippets/gul.json': json.dumps(ext.generate_snippets(), indent=2)
    }
