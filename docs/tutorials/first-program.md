# Your First GUL Program

This tutorial walks you through creating your first complete GUL program, explaining every concept along the way.

## 📝 Project: Todo List Manager

We'll build a command-line todo list manager that demonstrates core GUL features.

## 🎯 Step 1: Project Setup

Create a new directory:

```bash
mkdir todo-app
cd todo-app
```

Initialize a GUL project:

```bash
gul init
```

This creates:

- `gul.toml` - Project configuration
- `src/` - Source code directory
- `README.md` - Project documentation

## 📄 Step 2: Define Data Structure

Create `src/todo.mn`:

```gul
# Define a Todo struct
struct Todo:
    id: int
    title: str
    completed: bool
    created_at: datetime

    fn new(id: int, title: str): Todo:
        return Todo {
            id: id,
            title: title,
            completed: False,
            created_at: datetime.now()
        }

    fn toggle(mut self):
        self.completed = !self.completed

    fn display(self): str:
        status = "✓" if self.completed else " "
        return f"[{status}] {self.id}. {self.title}"
```

**Concepts:**

- **struct**: Defines a custom data type
- **fn**: Method definition
- **mut self**: Allows modifying the instance
- **datetime.now()**: Gets current timestamp

## 🗂️ Step 3: Todo Manager

Create `src/manager.mn`:

```gul
import std.filesystem as fs

struct TodoManager:
    todos: vec[Todo]
    next_id: int
    filename: str

    fn new(filename: str): TodoManager:
        return TodoManager {
            todos: vec[],
            next_id: 1,
            filename: filename
        }

    fn add_todo(mut self, title: str):
        todo = Todo.new(self.next_id, title)
        self.todos.push(todo)
        self.next_id += 1
        self.save()
        print(f"Added: {title}")

    fn list_todos(self):
        if len(self.todos) == 0:
            print("No todos yet!")
            return

        print("\nYour Todos:")
        print("-" * 40)
        for todo in self.todos:
            print(todo.display())
        print("-" * 40)

    fn toggle_todo(mut self, id: int):
        for mut todo in self.todos:
            if todo.id == id:
                todo.toggle()
                self.save()
                print(f"Toggled: {todo.title}")
                return

        print(f"Todo {id} not found")

    fn delete_todo(mut self, id: int):
        for i in range(len(self.todos)):
            if self.todos[i].id == id:
                title = self.todos[i].title
                self.todos.remove(i)
                self.save()
                print(f"Deleted: {title}")
                return

        print(f"Todo {id} not found")

    fn save(self):
        # Serialize todos to JSON
        data = {
            "next_id": self.next_id,
            "todos": [
                {
                    "id": todo.id,
                    "title": todo.title,
                    "completed": todo.completed,
                    "created_at": todo.created_at.to_string()
                }
                for todo in self.todos
            ]
        }

        json_str = json.dumps(data)
        fs.write_text(self.filename, json_str)

    fn load(mut self):
        if !fs.exists(self.filename):
            return

        json_str = fs.read_text(self.filename)
        data = json.loads(json_str)

        self.next_id = data["next_id"]
        self.todos = [
            Todo {
                id: item["id"],
                title: item["title"],
                completed: item["completed"],
                created_at: datetime.parse(item["created_at"])
            }
            for item in data["todos"]
        ]
```

**Concepts:**

- **vec**: Dynamic array
- **List comprehension**: `[... for item in items]`
- **JSON serialization**: Convert structs to/from JSON
- **File I/O**: Reading and writing files

## 🎮 Step 4: Command-Line Interface

Create `src/main.mn`:

```gul
import std.filesystem as fs

fn show_help():
    print("""
    Todo List Manager

    Commands:
        add <title>      Add a new todo
        list             List all todos
        toggle <id>      Toggle todo completion
        delete <id>      Delete a todo
        help             Show this help
        quit             Exit the program
    """)

main:
    print("Welcome to GUL Todo Manager!")
    print("Type 'help' for commands\n")

    manager = TodoManager.new("todos.json")
    manager.load()

    while True:
        input_line = input("> ")
        parts = input_line.split()

        if len(parts) == 0:
            continue

        command = parts[0]

        match command:
            "add":
                if len(parts) < 2:
                    print("Usage: add <title>")
                else:
                    title = " ".join(parts[1:])
                    manager.add_todo(title)

            "list":
                manager.list_todos()

            "toggle":
                if len(parts) < 2:
                    print("Usage: toggle <id>")
                else:
                    id = int(parts[1])
                    manager.toggle_todo(id)

            "delete":
                if len(parts) < 2:
                    print("Usage: delete <id>")
                else:
                    id = int(parts[1])
                    manager.delete_todo(id)

            "help":
                show_help()

            "quit":
                print("Goodbye!")
                break

            _:
                print(f"Unknown command: {command}")
                print("Type 'help' for available commands")
```

**Concepts:**

- **main** block: Program entry point
- **while True**: Infinite loop
- **input()**: Read user input
- **match**: Pattern matching
- **String methods**: split(), join()

## 🏃 Step 5: Run Your Program

```bash
gul run src/main.mn
```

Try these commands:

```
> add Buy groceries
Added: Buy groceries

> add Write code
Added: Write code

> list
Your Todos:
----------------------------------------
[ ] 1. Buy groceries
[ ] 2. Write code
----------------------------------------

> toggle 1
Toggled: Buy groceries

> list
Your Todos:
----------------------------------------
[✓] 1. Buy groceries
[ ] 2. Write code
----------------------------------------

> delete 1
Deleted: Buy groceries

> quit
Goodbye!
```

## 🎨 Step 6: Add Color (Optional)

Enhance with colors:

```gul
import std.terminal

fn display_colored(self): str:
    if self.completed:
        color = terminal.green
        status = "✓"
    else:
        color = terminal.yellow
        status = " "

    return color(f"[{status}] {self.id}. {self.title}")
```

## 🚀 Step 7: Build for Production

```bash
# Build optimized binary
gul build --release

# Run the binary
./target/release/todo-app
```

## 🎓 What You've Learned

✅ **Structs**: Custom data types with methods  
✅ **Vectors**: Dynamic arrays  
✅ **File I/O**: Reading and writing files  
✅ **JSON**: Serialization and deserialization  
✅ **Pattern Matching**: Clean conditional logic  
✅ **User Input**: Interactive CLI applications  
✅ **Error Handling**: Graceful error management

## 🔍 Improvements to Try

1. **Add due dates**: Include deadline tracking
2. **Priority levels**: Add high/medium/low priorities
3. **Categories**: Organize todos by category
4. **Search**: Find todos by keyword
5. **Export**: Export to CSV or Markdown
6. **Web UI**: Build a web interface

## 📚 Next Steps

- **Web Server**: Build a [web-based todo app](web-server.md)
- **Database**: Add [database persistence](data-analysis.md)
- **Testing**: Learn to write [tests](../guides/testing-deployment.md)
- **Deployment**: [Deploy your application](../guides/testing-deployment.md)

## 💡 Key Takeaways

- GUL makes it easy to build real applications quickly
- Strong typing catches errors early
- File I/O and JSON are built-in
- Pattern matching simplifies complex conditions
- The standard library is comprehensive

---

**Congratulations!** 🎉 You've built your first complete GUL application!

---

**Last Updated**: 2025-12-10  
**Version**: 1.0.0  
**License**: MIT
