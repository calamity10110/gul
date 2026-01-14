# Tui

**Version**: 0.14.0-dev | **Syntax**: v3.2 | **Updated**: 2026-01-08

---

# GUL TUI Framework

The GUL TUI (Terminal User Interface) framework provides:

````gul
use std::tui

mn:
    term = tui::Terminal::init()
    term.draw(|frame| {
        // ...
    })
```Contents

1. [Using the TUI IDE](#using-the-tui-ide)
2. [TUI Components Reference](#tui-components-reference)
3. [Developing TUI Applications](#developing-tui-applications)
4. [Advanced TUI Features](#advanced-tui-features)

---

## 🖥️ Using the TUI IDE

### Starting the IDE

```bash
# Navigate to your GUL project
cd my_project

# Start the TUI IDE
cargo run --example programming_deck

# Or if GUL is installed globally
gul ide --tui
````

## IDE Layout

```bash
┌─────────────────────────────────────────────────────────────┐
│ GUL IDE v0.11.0 - [Programming Deck]                       │
├──────────────┬────────────────────────────┬─────────────────┤
│  EXPLORER    │  EDITOR - main.mn          │    SYSTEM       │
│              │                            │                 │
│ ├── src      │  mn:                │ CPU Usage       │
│ │   ├── main │      print("Hello!")       │ [████░░░] 45%   │
│ │   └── utils│                            │                 │
│ ├── tests    │                            │ Memory          │
│ └── package  │                            │ [██████░] 60%   │
│              │                            │                 │
├──────────────┴────────────────────────────┴─────────────────┤
│  TERMINAL                                                   │
│  $ gul run main.mn                                          │
│  Hello!                                                     │
└─────────────────────────────────────────────────────────────┘
│ READY  |  Ln 2, Col 10  |  UTF-8  |  GUL                   │
└─────────────────────────────────────────────────────────────┘
```

### Keyboard Shortcuts

| Shortcut | Action            |
| -------- | ----------------- |
| `Ctrl+N` | New file          |
| `Ctrl+O` | Open file         |
| `Ctrl+S` | Save file         |
| `Ctrl+R` | Run current file  |
| `Ctrl+B` | Build project     |
| `Ctrl+T` | Run tests         |
| `Ctrl+F` | Find in file      |
| `Ctrl+H` | Find and replace  |
| `Ctrl+/` | Toggle comment    |
| `Ctrl+Q` | Quit IDE          |
| `F5`     | Debug             |
| `F9`     | Toggle breakpoint |

---

## 🎨 TUI Components Reference

### Text Component

```gul
ui.print(^&^[text{
    content: "Hello, World!",
    fg: "green",
    bg: "black",
    bold: true
}])
```

**Properties:**

- `content`: String to display
- `fg`: Foreground color (red, green, blue, yellow, cyan, magenta, white, black)
- `bg`: Background color
- `bold`: Boolean for bold text

### Button Component

```gul
ui.print(^&^[button{text: "Click Me!"}])
```

**Output:**

```bash
┌───────────┐
│ Click Me! │
└───────────┘
```

### Progress Bar

```gul
ui.print(^&^[progress{
    value: 75,
    max: 100,
    label: "Loading"
}])
```

**Output:**

```bash
Loading: [███████████████████████████░░░░░░░░░░░░░] 75%
```

### Slider

```gul
ui.print(^&^[slider{
    min: 0,
    max: 100,
    value: 50,
    label: "Volume"
}])
```

**Output:**

```bash
Volume: [====================|                    ] 50/100
```

### Table

```gul
ui.print(^&^[table{
    headers: ["ID", "Name", "Status"],
    rows: [
        ["1", "Alice", "Active"],
        ["2", "Bob", "Inactive"],
        ["3", "Charlie", "Active"]
    ]
}])
```

**Output:**

```bash
┌────┬─────────┬──────────┐
│ ID │ Name    │ Status   │
├────┼─────────┼──────────┤
│ 1  │ Alice   │ Active   │
│ 2  │ Bob     │ Inactive │
│ 3  │ Charlie │ Active   │
└────┴─────────┴──────────┘
```

### Canvas (ASCII Art)

```gul
canvas = ui.new_canvas(40, 10)
canvas.draw_rect(5, 1, 35, 8)
canvas.draw_line(5, 1, 35, 8)
ui.print(canvas)
```

### Tree View

```gul
ui.print(^&^[tree{
    nodes: [
        {name: "src", children: [
            {name: "main.mn"},
            {name: "utils.mn"}
        ]},
        {name: "tests", children: [
            {name: "test_api.mn"}
        ]}
    ]
}])
```

**Output:**

```bash
├── src
│   ├── main.mn
│   └── utils.mn
├── tests
│   └── test_api.mn
```

---

## 🛠️ Developing TUI Applications

### Basic TUI App Structure

```gul
@imp ui

mn:
    # Initialize UI runtime
    runtime = ui.Runtime.new()
    runtime.clear()

    # Create header
    runtime.print(^&^[text{
        content: "My TUI App",
        fg: "cyan",
        bold: true
    }])

    # Create interactive menu
    choice = runtime.menu([
        "Option 1",
        "Option 2",
        "Option 3",
        "Exit"
    ])

    # Handle user input
    @if choice == 0:
        handle_option_1()
    @elif choice == 1:
        handle_option_2()
    @elif choice == 2:
        handle_option_3()
    @else:
        runtime.quit()
```

### Input Handling

```gul
# Text input
name = ui.input("Enter your name: ")

# Password input (hidden)
password = ui.input_password("Enter password: ")

# Number input
age = ui.input_number("Enter your age: ", min=0, max=150)

# Confirmation
confirmed = ui.confirm("Are you sure?")  # Returns true/false
```

### Real-time Updates

```gul
@imp ui, time

mn:
    runtime = ui.Runtime.new()

    ?count = 0
    @while true:
        runtime.clear()
        runtime.print(^÷^[text{
            content: f"Counter: {?count}",
            fg: "green"
        }])

        ?count = ?count + 1
        time.sleep(1)  # Update every second
```

### Multi-Panel Layout

```gul
# Create a dashboard with multiple panels
mn create_dashboard():
    runtime = ui.Runtime.new()

    # Left panel: File tree
    left_panel = ui.VBox([
        ui.Text("Files", fg="cyan", bold=true),
        ui.Tree(get_file_tree())
    ])

    # Center panel: Content
    center_panel = ui.VBox([
        ui.Text("Content", fg="cyan", bold=true),
        ui.Text(get_content())
    ])

    # Right panel: Stats
    right_panel = ui.VBox([
        ui.Text("Stats", fg="cyan", bold=true),
        ui.Progress(value=cpu_usage(), max=100, label="CPU"),
        ui.Progress(value=mem_usage(), max=100, label="Memory")
    ])

    # Combine panels
    main_layout = ui.HBox([left_panel, center_panel, right_panel])
    runtime.print(main_layout)
```

---

## 🚀 Advanced TUI Features

### Custom Themes

```gul
# Define a custom theme
theme = ui.Theme({
    primary: "cyan",
    secondary: "magenta",
    success: "green",
    error: "red",
    warning: "yellow",
    background: "black",
    foreground: "white"
})

# Apply theme
ui.set_theme(theme)
```

### Event Handling

```gul
# Listen for keyboard events
@while true:
    event = ui.wait_for_event()

    @if event.type == "key":
        @if event.key == "q":
            break
        @elif event.key == "r":
            refresh_display()
        @elif event.key == "h":
            show_help()
```

### Animations

```gul
# Spinner animation
spinner = ui.Spinner(["⠋", "⠙", "⠹", "⠸", "⠼", "⠴", "⠦", "⠧", "⠇", "⠏"])

@while loading:
    ui.print(spinner.next())
    time.sleep(0.1)
```

### Modal Dialogs

```gul
# Show a modal dialog
result = ui.dialog({
    title: "Confirm Action",
    message: "Are you sure you want to delete this file?",
    buttons: ["Yes", "No", "Cancel"]
})

@if result == 0:  # Yes
    delete_file()
```

---

## 📦 TUI Component Library

### Installing Additional Components

```bash
gul add ui-charts    # Add charting components
gul add ui-forms     # Add form components
gul add ui-widgets   # Add widget library
```

### Using Charts

```gul
@imp ui.charts

data = [10, 20, 15, 30, 25, 40]
labels = ["Jan", "Feb", "Mar", "Apr", "May", "Jun"]

# Bar chart
ui.print(^&^[bar_chart{
    data: data,
    labels: labels,
    title: "Monthly Sales"
}])

# Line chart
ui.print(^&^[line_chart{
    data: data,
    labels: labels,
    title: "Trend Analysis"
}])
```

---

## 🎓 Best Practices

1. **Keep UI Updates Efficient**: Only redraw changed components
2. **Handle Terminal Resize**: Listen for resize events and adjust layout
3. **Use Color Wisely**: Ensure good contrast for readability
4. **Provide Keyboard Shortcuts**: Make your app accessible
5. **Test on Different Terminals**: Verify compatibility across terminal emulators
6. **Handle Errors Gracefully**: Show user-friendly error messages
7. **Save State**: Persist user preferences and session data

---

## 🔧 Troubleshooting

### TUI Not Displaying Correctly

```bash
# Check terminal capabilities
echo $TERM

# Set to xterm-256color if needed
export TERM=xterm-256color
```

### Colors Not Showing

```gul
# Check if terminal supports colors
@if ui.supports_color():
    ui.enable_color()
@else:
    ui.disable_color()
```

### Performance Issues

```gul
# Use buffered rendering
runtime.enable_buffering()
runtime.print(component1)
runtime.print(component2)
runtime.flush()  # Render all at once
```

---

## 📚 Additional Resources

- [TUI Examples](examples/tui/)
- [Component API Reference](docs/api/ui.md)
- [TUI Design Patterns](docs/patterns/tui.md)

**Build amazing terminal applications with GUL!** 🚀
