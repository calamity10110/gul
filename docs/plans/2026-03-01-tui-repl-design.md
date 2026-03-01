# TUI REPL Design

**Date**: 2026-03-01
**Version**: 0.14.0-dev
**Status**: Approved
**Backend**: Bootstrap interpreter (`src_bootstrap`)

## Summary

Build an interactive TUI-based REPL for GUL using `ratatui` (already a dependency). The REPL provides a split-pane interface with output history, input editing, a variables/functions sidebar, and a project banner. It uses the existing bootstrap interpreter pipeline for evaluation, maintaining persistent state between inputs.

## Layout

```
+-----------------------------------------------------+
|  GUL REPL | project: <dir> | file: (none)        [?]|
+------------------------------------+----------------+
| Output Pane (scrollable)           | Variables      |
|                                    |                |
| > const x = 42                     | x: int = 42   |
| > x + 8                           | y: int = 84   |
| 50                                 | greet: fn(1)  |
| > @fn greet(name)(res):            |                |
| .     res = "Hello, " + name       |----------------|
| > greet("World")                   | Functions      |
| "Hello, World"                     |                |
|                                    | greet(name)    |
+------------------------------------+ double(x)     |
| gul> _                             |                |
+------------------------------------+----------------+
```

### Components

- **Banner** (top row): Project name (working directory basename), loaded file name (or "(none)"), help indicator `[?]`.
- **Output pane** (left, top ~80%): Scrollable history. User inputs prefixed with `>`, continuation lines with `.`, evaluation results displayed plain. Errors displayed in red.
- **Input pane** (left, bottom ~20%): Current input line with `gul>` prompt. Supports multi-line editing.
- **Sidebar** (right, ~25% width): Two sections — Variables (name, type, short value) and Functions (name with parameter list). Auto-refreshes after each evaluation from interpreter state.

## Interaction Model

### Input

- **Enter**: Evaluate current input. If line ends with `:` (block start), enter multi-line mode.
- **Multi-line**: Continue with indented lines. Empty line or Ctrl-D submits the block.
- **Up/Down**: Cycle through input history (persisted to `~/.gul_history`).
- **Ctrl-C**: Cancel current input.
- **Ctrl-D** (empty line): Exit REPL.
- **Ctrl-L**: Clear output pane.
- **Tab**: Reserved for future auto-completion (no-op for v1).

### Evaluation

Each input goes through the existing bootstrap pipeline: Lexer -> Parser -> Interpreter. The interpreter maintains persistent state — variables and functions survive between evaluations.

- Expressions: result value is printed
- Statements (assignments, function defs): no output unless they produce side effects (e.g., `print()`)
- Errors: displayed in red with source position

### Special Commands

Prefixed with `:` to distinguish from GUL code:

| Command | Action |
|---------|--------|
| `:help` | Show keybindings and commands |
| `:clear` | Clear output history |
| `:load file.mn` | Load and execute a GUL file (updates banner) |
| `:reset` | Clear all state (variables, functions) |
| `:quit` | Exit REPL |

## Architecture

### Module Structure

```
src_bootstrap/
  repl/
    mod.rs          -- public API: start_repl()
    ui.rs           -- ratatui layout, rendering
    input.rs        -- input handling, multi-line, history
    state.rs        -- REPL state (history, variables, output buffer)
    commands.rs     -- special command parsing and execution
```

### Data Flow

```
User keystroke
  -> input.rs (edit buffer, history navigation)
  -> on Enter: commands.rs checks for ":" prefix
     -> if special command: execute directly
     -> if GUL code: feed through Lexer -> Parser -> Interpreter
  -> state.rs (append to output, update variable list)
  -> ui.rs (re-render all panes)
```

### Key Dependencies

- `ratatui` — already in Cargo.toml for TUI rendering
- `crossterm` — already a ratatui dependency, provides raw terminal input
- Bootstrap interpreter — existing `src_bootstrap` eval pipeline
- No new external dependencies needed

## Testing Strategy

- **Unit tests**: Input parser (multi-line detection, command parsing), state management (variable tracking)
- **Integration tests**: Feed GUL code through REPL state, verify output buffer contents
- **Manual testing**: Interactive TUI testing for rendering and keybindings

## Deferred (Not v1)

- Tab auto-completion
- Syntax highlighting in input pane
- Variable value inspection (click to expand)
- Debugger integration
- Persistent sessions (save/restore state)
