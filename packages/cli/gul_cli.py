"""
GUL CLI Framework
Command-line interface framework with argument parsing.

Status: ✅ Implemented
Priority: High
"""

from typing import List, Optional, Dict, Any, Callable
from dataclasses import dataclass
from enum import Enum

__version__ = "0.1.0"
__all__ = ['CLI', 'Command', 'Argument', 'Flag']

class ArgumentType(Enum):
    STRING = "string"
    INTEGER = "int"
    FLOAT = "float"
    BOOLEAN = "bool"

@dataclass
class Argument:
    """Command argument"""
    name: str
    type: ArgumentType = ArgumentType.STRING
    required: bool = True
    default: Any = None
    help: str = ""

@dataclass
class Flag:
    """Command flag/option"""
    name: str
    short: Optional[str] = None
    type: ArgumentType = ArgumentType.BOOLEAN
    default: Any = None
    help: str = ""

class Command:
    """CLI command"""
    
    def __init__(self, name: str, description: str = ""):
        self.name = name
        self.description = description
        self.arguments: List[Argument] = []
        self.flags: List[Flag] = []
        self.handler: Optional[Callable] = None
        self.subcommands: Dict[str, 'Command'] = {}
    
    def add_argument(self, arg: Argument) -> 'Command':
        """Add argument"""
        self.arguments.append(arg)
        return self
    
    def add_flag(self, flag: Flag) -> 'Command':
        """Add flag"""
        self.flags.append(flag)
        return self
    
    def add_subcommand(self, cmd: 'Command') -> 'Command':
        """Add subcommand"""
        self.subcommands[cmd.name] = cmd
        return self
    
    def action(self, handler: Callable) -> 'Command':
        """Set command handler"""
        self.handler = handler
        return self

class CLI:
    """
    CLI framework
    
    Example:
        cli = CLI("myapp", "My Application", version="1.0.0")
        
        # Add command
        serve = Command("serve", "Start server")
        serve.add_flag(Flag("port", "p", ArgumentType.INTEGER, 8080, "Server port"))
        serve.add_flag(Flag("host", "h", ArgumentType.STRING, "localhost", "Host"))
        serve.action(lambda args: print(f"Starting server on {args['host']}:{args['port']}"))
        
        cli.add_command(serve)
        
        # Parse and run
        cli.run(["serve", "--port", "3000"])
    """
    
    def __init__(self, name: str, description: str = "", version: str = "0.1.0"):
        self.name = name
        self.description = description
        self.version = version
        self.commands: Dict[str, Command] = {}
        self.global_flags: List[Flag] = []
    
    def add_command(self, cmd: Command) -> 'CLI':
        """Add command"""
        self.commands[cmd.name] = cmd
        return self
    
    def add_global_flag(self, flag: Flag) -> 'CLI':
        """Add global flag"""
        self.global_flags.append(flag)
        return self
    
    def parse(self, args: List[str]) -> tuple:
        """Parse arguments"""
        if not args:
            return None, {}
        
        # Check for help/version
        if args[0] in ["-h", "--help", "help"]:
            self._print_help()
            return None, {}
        
        if args[0] in ["-v", "--version", "version"]:
            print(f"{self.name} version {self.version}")
            return None, {}
        
        # Get command
        cmd_name = args[0]
        if cmd_name not in self.commands:
            print(f"Unknown command: {cmd_name}")
            self._print_help()
            return None, {}
        
        command = self.commands[cmd_name]
        return command, self._parse_command(command, args[1:])
    
    def _parse_command(self, command: Command, args: List[str]) -> Dict[str, Any]:
        """Parse command arguments"""
        parsed = {}
        
        # Initialize with defaults
        for flag in command.flags:
            if flag.default is not None:
                parsed[flag.name] = flag.default
        
        i = 0
        positional_index = 0
        
        while i < len(args):
            arg = args[i]
            
            # Flag
            if arg.startswith("-"):
                flag_name = arg.lstrip("-")
                
                # Find flag
                flag = None
                for f in command.flags:
                    if f.name == flag_name or f.short == flag_name:
                        flag = f
                        break
                
                if not flag:
                    print(f"Unknown flag: {arg}")
                    i += 1
                    continue
                
                # Boolean flag
                if flag.type == ArgumentType.BOOLEAN:
                    parsed[flag.name] = True
                    i += 1
                    continue
                
                # Value flag
                if i + 1 >= len(args):
                    print(f"Missing value for flag: {arg}")
                    i += 1
                    continue
                
                value = args[i + 1]
                parsed[flag.name] = self._convert_value(value, flag.type)
                i += 2
            
            # Positional argument
            else:
                if positional_index < len(command.arguments):
                    arg_def = command.arguments[positional_index]
                    parsed[arg_def.name] = self._convert_value(arg, arg_def.type)
                    positional_index += 1
                i += 1
        
        # Check required arguments
        for arg_def in command.arguments:
            if arg_def.required and arg_def.name not in parsed:
                print(f"Missing required argument: {arg_def.name}")
                return {}
        
        return parsed
    
    def _convert_value(self, value: str, arg_type: ArgumentType) -> Any:
        """Convert string value to type"""
        if arg_type == ArgumentType.INTEGER:
            return int(value)
        elif arg_type == ArgumentType.FLOAT:
            return float(value)
        elif arg_type == ArgumentType.BOOLEAN:
            return value.lower() in ["true", "1", "yes"]
        else:
            return value
    
    def run(self, args: List[str]):
        """Parse and execute"""
        command, parsed_args = self.parse(args)
        
        if command and command.handler:
            command.handler(parsed_args)
    
    def _print_help(self):
        """Print help"""
        print(f"{self.name} - {self.description}")
        print(f"\nUsage: {self.name} [command] [options]")
        print("\nCommands:")
        
        for cmd in self.commands.values():
            print(f"  {cmd.name:<15} {cmd.description}")
        
        print("\nGlobal Options:")
        print("  -h, --help      Show help")
        print("  -v, --version   Show version")
