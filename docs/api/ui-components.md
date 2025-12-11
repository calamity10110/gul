# UI Components API Reference

GUL provides rich UI components for both Terminal UI (TUI) and Web UI applications.

## 🎨 TUI Components

### Basic Components

```gul
import std.ui.tui

# Text
text = tui.Text("Hello, GUL!", color="blue", bold=True)

# Button
button = tui.Button("Click Me", on_click=fn(): print("Clicked!"))

# Input
input = tui.Input(placeholder="Enter text...", on_change=fn(value): print(value))

# Label
label = tui.Label("Username:", color="green")
```

### Layout Components

```gul
# Box (container)
box = tui.Box([
    tui.Text("Title"),
    tui.Button("Action")
], border=True, padding=1)

# Row (horizontal layout)
row = tui.Row([
    tui.Text("Left"),
    tui.Text("Center"),
    tui.Text("Right")
])

# Column (vertical layout)
column = tui.Column([
    tui.Text("Top"),
    tui.Text("Middle"),
    tui.Text("Bottom")
])

# Grid
grid = tui.Grid([
    [tui.Text("A1"), tui.Text("A2")],
    [tui.Text("B1"), tui.Text("B2")]
])
```

### Interactive Components

```gul
# Checkbox
checkbox = tui.Checkbox("Accept terms", checked=False)

# Radio buttons
radio = tui.RadioGroup([
    "Option 1",
    "Option 2",
    "Option 3"
], selected=0)

# Select/Dropdown
select = tui.Select([
    "Choice A",
    "Choice B",
    "Choice C"
])

# Slider
slider = tui.Slider(min=0, max=100, value=50)

# Progress bar
progress = tui.ProgressBar(current=75, total=100)
```

### Data Display

```gul
# Table
table = tui.Table(
    headers=["Name", "Age", "City"],
    rows=[
        ["Alice", "25", "New York"],
        ["Bob", "30", "London"],
        ["Charlie", "35", "Tokyo"]
    ]
)

# List
list = tui.List([
    "Item 1",
    "Item 2",
    "Item 3"
], selectable=True)

# Tree view
tree = tui.Tree({
    "Root": {
        "Child 1": ["Leaf 1", "Leaf 2"],
        "Child 2": ["Leaf 3"]
    }
})
```

## 🌐 Web UI Components

### Basic Components

```gul
import std.ui.web

# Heading
h1 = web.H1("Welcome to GUL")
h2 = web.H2("Subtitle")

# Paragraph
p = web.P("This is a paragraph of text.")

# Button
button = web.Button(
    "Submit",
    onclick=fn(): handle_submit(),
    class="btn btn-primary"
)

# Input
input = web.Input(
    type="text",
    placeholder="Enter name...",
    value="",
    oninput=fn(e): handle_input(e.target.value)
)
```

### Forms

```gul
# Form
form = web.Form([
    web.Label("Name:"),
    web.Input(type="text", name="name"),

    web.Label("Email:"),
    web.Input(type="email", name="email"),

    web.Label("Password:"),
    web.Input(type="password", name="password"),

    web.Button("Sign Up", type="submit")
], onsubmit=handle_form_submit)

# Checkbox
checkbox = web.Checkbox("Remember me", checked=False)

# Radio buttons
radio_group = web.RadioGroup(
    name="plan",
    options=[
        ("free", "Free Plan"),
        ("pro", "Pro Plan"),
        ("enterprise", "Enterprise Plan")
    ]
)

# Select
select = web.Select(
    options=["Red", "Green", "Blue"],
    selected="Red"
)
```

### Layout Components

```gul
# Container/Div
container = web.Div([
    web.H1("Title"),
    web.P("Content")
], class="container")

# Flexbox
flex = web.Flex([
    web.Box("Item 1"),
    web.Box("Item 2"),
    web.Box("Item 3")
], direction="row", justify="space-between")

# Grid
grid = web.Grid([
    [web.Box("A1"), web.Box("A2"), web.Box("A3")],
    [web.Box("B1"), web.Box("B2"), web.Box("B3")]
], columns=3, gap="1rem")
```

### Data Display

```gul
# Table
table = web.Table(
    headers=["Name", "Email", "Role"],
    data=[
        ["Alice", "alice@example.com", "Admin"],
        ["Bob", "bob@example.com", "User"]
    ],
    sortable=True,
    hoverable=True
)

# Card
card = web.Card(
    title="Product Name",
    content="Product description goes here",
    footer=web.Button("Buy Now"),
    image="product.jpg"
)

# List
list = web.List([
    "First item",
    "Second item",
    "Third item"
], ordered=True)
```

### Interactive Components

```gul
# Modal
modal = web.Modal(
    title="Confirm Action",
    content=web.P("Are you sure?"),
    footer=[
        web.Button("Cancel", onclick=fn(): modal.close()),
        web.Button("Confirm", onclick=fn(): handle_confirm())
    ]
)

# Tabs
tabs = web.Tabs([
    ("home", "Home", web.Div("Home content")),
    ("profile", "Profile", web.Div("Profile content")),
    ("settings", "Settings", web.Div("Settings content"))
])

# Dropdown menu
dropdown = web.Dropdown(
    label="Actions",
    items=[
        ("edit", "Edit", fn(): handle_edit()),
        ("delete", "Delete", fn(): handle_delete())
    ]
)
```

### Charts & Visualization

```gul
# Line chart
line_chart = web.LineChart(
    data=[1, 3, 2, 5, 4, 6],
    labels=["Jan", "Feb", "Mar", "Apr", "May", "Jun"]
)

# Bar chart
bar_chart = web.BarChart(
    data=[10, 20, 15, 25, 30],
    labels=["A", "B", "C", "D", "E"]
)

# Pie chart
pie_chart = web.PieChart(
    data=[30, 25, 20, 15, 10],
    labels=["Red", "Blue", "Green", "Yellow", "Purple"]
)
```

## 🎯 Component Styling

### TUI Styling

```gul
# Colors
component.color = "red"
component.bg_color = "blue"

# Borders
component.border = True
component.border_style = "rounded"  # or "sharp", "double"

# Padding/Margin
component.padding = 2
component.margin = 1
```

### Web UI Styling

```gul
# CSS classes
button.class = "btn btn-primary btn-large"

# Inline styles
button.style = {
    "background-color": "#007bff",
    "color": "white",
    "padding": "10px 20px",
    "border-radius": "5px"
}
```

## 🔄 State Management

```gul
import std.ui.state

# Create state
count = state.State(0)

# Use in component
button = web.Button(
    f"Count: {count.value}",
    onclick=fn(): count.update(count.value + 1)
)

# Watch state changes
count.watch(fn(new_value):
    print(f"Count changed to: {new_value}")
)
```

## 📚 See Also

- [TUI Guide](../guides/tui.md)
- [Web UI Guide](../guides/webui.md)
- [Web Development Guide](../guides/web-development.md)

---

**Last Updated**: 2025-12-10  
**Version**: 1.0.0  
**License**: MIT
