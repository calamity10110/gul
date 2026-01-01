// GUL Interactive TUI Course
// Terminal-based learning system for GUL programming language

use std::collections::HashMap;

#[derive(Debug, Clone)]
pub struct Lesson {
    pub id: String,
    pub title: String,
    pub description: String,
    pub content: String,
    pub code_example: String,
    pub exercise: Option<Exercise>,
    pub completed: bool,
}

#[derive(Debug, Clone)]
pub struct Exercise {
    pub prompt: String,
    pub starter_code: String,
    pub solution: String,
    pub hints: Vec<String>,
}

#[derive(Debug, Clone)]
pub struct Course {
    pub title: String,
    pub description: String,
    pub lessons: Vec<Lesson>,
    pub current_lesson: usize,
    pub progress: HashMap<String, bool>,
}

impl Course {
    pub fn new() -> Self {
        Course {
            title: "Learn GUL - Interactive Course".to_string(),
            description: "Master the GUL programming language through interactive lessons"
                .to_string(),
            lessons: Self::create_lessons(),
            current_lesson: 0,
            progress: HashMap::new(),
        }
    }

    fn create_lessons() -> Vec<Lesson> {
        vec![
            // Lesson 1: Introduction
            Lesson {
                id: "intro".to_string(),
                title: "Welcome to GUL".to_string(),
                description: "Introduction to GUL and your first program".to_string(),
                content: r#"
Welcome to GUL (GUL Universal Language)!

GUL is a universal programming language that allows you to:
- Write code in multiple languages in one file
- Compile to native, WASM, or embedded targets
- Use advanced features like reactive UI and GPU computing

Let's start with the classic "Hello, World!" program.

In GUL, every program starts with a 'main:' block.
The main block is where your program execution begins.
"#.to_string(),
                code_example: r#"main:
    print("Hello, World!")
"#.to_string(),
                exercise: Some(Exercise {
                    prompt: "Write a program that prints 'Hello, GUL!'".to_string(),
                    starter_code: "main:\n    # Your code here\n".to_string(),
                    solution: "main:\n    print(\"Hello, GUL!\")\n".to_string(),
                    hints: vec![
                        "Use the print() function".to_string(),
                        "Don't forget the quotes around the text".to_string(),
                    ],
                }),
                completed: false,
            },

            // Lesson 2: Variables and Types
            Lesson {
                id: "variables".to_string(),
                title: "Variables and Types".to_string(),
                description: "Learn about variables, types, and definitions".to_string(),
                content: r#"
Variables in GUL

GUL uses the 'def' keyword to define variables.
Variables are immutable by default (like Rust).
Use '?' after 'def' to make a variable mutable.

Syntax:
  def name = value        # Immutable
  def? name = value       # Mutable

GUL has type inference, so you don't need to specify types explicitly.
However, you can add type annotations if you want:
  def name: Type = value
"#.to_string(),
                code_example: r#"main:
    # Immutable variable
    def x = 42
    def name = "Alice"
    
    # Mutable variable
    def? count = 0
    count = count + 1
    
    # With type annotation
    def pi: f64 = 3.14159
    
    print(x)
    print(name)
    print(count)
"#.to_string(),
                exercise: Some(Exercise {
                    prompt: "Create a mutable variable 'age' with value 25, then increment it by 1".to_string(),
                    starter_code: "main:\n    # Define age\n    \n    # Increment age\n    \n    # Print age\n".to_string(),
                    solution: "main:\n    def? age = 25\n    age = age + 1\n    print(age)\n".to_string(),
                    hints: vec![
                        "Use 'def?' for mutable variables".to_string(),
                        "Increment with: age = age + 1".to_string(),
                    ],
                }),
                completed: false,
            },

            // Lesson 3: Functions
            Lesson {
                id: "functions".to_string(),
                title: "Functions".to_string(),
                description: "Define and call functions".to_string(),
                content: r#"
Functions in GUL

Functions are defined using the 'fn' keyword.
Syntax:
  fn function_name(param1, param2):
      # function body
      return result

Functions can have:
- Parameters with optional type annotations
- Return types (inferred or explicit)
- Multiple return values

Example with type annotations:
  fn add(a: i32, b: i32) -> i32:
      return a + b
"#.to_string(),
                code_example: r#"# Define a function
fn greet(name):
    return "Hello, " + name + "!"

fn add(a, b):
    return a + b

fn multiply(x: i32, y: i32) -> i32:
    return x * y

main:
    def message = greet("World")
    print(message)
    
    def sum = add(5, 3)
    print(sum)
    
    def product = multiply(4, 7)
    print(product)
"#.to_string(),
                exercise: Some(Exercise {
                    prompt: "Create a function 'square' that takes a number and returns its square".to_string(),
                    starter_code: "fn square(n):\n    # Your code here\n\nmain:\n    def result = square(5)\n    print(result)\n".to_string(),
                    solution: "fn square(n):\n    return n * n\n\nmain:\n    def result = square(5)\n    print(result)\n".to_string(),
                    hints: vec![
                        "Multiply n by itself".to_string(),
                        "Use the return keyword".to_string(),
                    ],
                }),
                completed: false,
            },

            // Lesson 4: Control Flow
            Lesson {
                id: "control_flow".to_string(),
                title: "Control Flow".to_string(),
                description: "If statements, loops, and conditionals".to_string(),
                content: r#"
Control Flow in GUL

If Statements:
  if condition:
      # code
  elif other_condition:
      # code
  else:
      # code

Loops:
  # For loop
  for item in collection:
      # code
  
  # While loop
  while condition:
      # code
  
  # Loop (infinite)
  loop:
      # code
      break  # exit loop

GUL uses indentation (like Python) to define code blocks.
"#.to_string(),
                code_example: r#"main:
    # If statement
    def age = 18
    if age >= 18:
        print("Adult")
    else:
        print("Minor")
    
    # For loop
    for i in [1, 2, 3, 4, 5]:
        print(i)
    
    # While loop
    def? count = 0
    while count < 3:
        print(count)
        count = count + 1
"#.to_string(),
                exercise: Some(Exercise {
                    prompt: "Write a function that prints numbers from 1 to n using a for loop".to_string(),
                    starter_code: "fn print_numbers(n):\n    # Your code here\n\nmain:\n    print_numbers(5)\n".to_string(),
                    solution: "fn print_numbers(n):\n    for i in range(1, n + 1):\n        print(i)\n\nmain:\n    print_numbers(5)\n".to_string(),
                    hints: vec![
                        "Use a for loop with range()".to_string(),
                        "range(1, n + 1) gives numbers from 1 to n".to_string(),
                    ],
                }),
                completed: false,
            },

            // Lesson 5: Multi-Language Integration
            Lesson {
                id: "multi_language".to_string(),
                title: "Multi-Language Integration".to_string(),
                description: "Use Rust, Python, JavaScript, and SQL in one file".to_string(),
                content: r#"
Multi-Language Integration

GUL's superpower is the ability to mix multiple languages!

Use language blocks with @ annotations:
  @rust    - Rust code
  @python  - Python code
  @js      - JavaScript code
  @sql     - SQL queries
  @c       - C code

Each block is compiled/executed in its native environment.
Data can be shared between languages with zero-copy when possible.
"#.to_string(),
                code_example: r#"main:
    # Rust for performance
    @rust
    fn fibonacci(n: u64) -> u64 {
        match n {
            0 => 0,
            1 => 1,
            _ => fibonacci(n-1) + fibonacci(n-2)
        }
    }
    
    # Python for data science
    @python
    import numpy as np
    data = np.array([1, 2, 3, 4, 5])
    mean = np.mean(data)
    
    # JavaScript for web
    @js
    console.log("Hello from JS!");
    
    # Use the results
    def fib_10 = fibonacci(10)
    print(fib_10)
"#.to_string(),
                exercise: Some(Exercise {
                    prompt: "Create a Rust function that doubles a number, then call it from main".to_string(),
                    starter_code: "main:\n    @rust\n    # Define your function here\n    \n    # Call it here\n".to_string(),
                    solution: "main:\n    @rust\n    fn double(n: i32) -> i32 {\n        n * 2\n    }\n    \n    def result = double(21)\n    print(result)\n".to_string(),
                    hints: vec![
                        "Use Rust syntax inside @rust block".to_string(),
                        "Call the function after the @rust block".to_string(),
                    ],
                }),
                completed: false,
            },

            // Lesson 6: Async/Await
            Lesson {
                id: "async".to_string(),
                title: "Async/Await".to_string(),
                description: "Asynchronous programming in GUL".to_string(),
                content: r#"
Async/Await in GUL

GUL has built-in async support powered by Tokio.

Async Functions:
  asy function_name(params):
      # async code
      return result

Await expressions:
  def result = await async_function()

Use async for:
- Network requests
- File I/O
- Database queries
- Concurrent operations

The runtime automatically handles task scheduling.
"#.to_string(),
                code_example: r#"# Async function
asy fetch_data(url):
    # Simulated async operation
    await sleep(1000)  # Sleep 1 second
    return "Data from " + url

asy process_multiple():
    def data1 = await fetch_data("api.example.com/1")
    def data2 = await fetch_data("api.example.com/2")
    return [data1, data2]

main:
    def results = await process_multiple()
    print(results)
"#.to_string(),
                exercise: Some(Exercise {
                    prompt: "Create an async function that waits 500ms then returns 'Done'".to_string(),
                    starter_code: "asy wait_and_return():\n    # Your code here\n\nmain:\n    def result = await wait_and_return()\n    print(result)\n".to_string(),
                    solution: "asy wait_and_return():\n    await sleep(500)\n    return \"Done\"\n\nmain:\n    def result = await wait_and_return()\n    print(result)\n".to_string(),
                    hints: vec![
                        "Use await sleep(500)".to_string(),
                        "Return a string".to_string(),
                    ],
                }),
                completed: false,
            },

            // Lesson 7: Data Structures
            Lesson {
                id: "data_structures".to_string(),
                title: "Data Structures".to_string(),
                description: "Lists, dictionaries, and custom types".to_string(),
                content: r#"
Data Structures in GUL

Lists (Arrays):
  def numbers = [1, 2, 3, 4, 5]
  def first = numbers[0]
  numbers.append(6)

Dictionaries (Hash Maps):
  def person = {
      "name": "Alice",
      "age": 30,
      "city": "NYC"
  }
  def name = person["name"]

Tuples:
  def point = (10, 20)
  def (x, y) = point

Sets:
  def unique = {1, 2, 3, 2, 1}  # {1, 2, 3}
"#.to_string(),
                code_example: r#"main:
    # List
    def? fruits = ["apple", "banana", "orange"]
    fruits.append("grape")
    print(fruits)
    
    # Dictionary
    def user = {
        "name": "Bob",
        "age": 25,
        "email": "bob@example.com"
    }
    print(user["name"])
    
    # Tuple
    def coordinates = (100, 200)
    def (x, y) = coordinates
    print(x, y)
    
    # Iterate over list
    for fruit in fruits:
        print(fruit)
"#.to_string(),
                exercise: Some(Exercise {
                    prompt: "Create a dictionary with your favorite book's title and author, then print both".to_string(),
                    starter_code: "main:\n    # Create dictionary\n    \n    # Print title and author\n".to_string(),
                    solution: "main:\n    def book = {\n        \"title\": \"1984\",\n        \"author\": \"George Orwell\"\n    }\n    print(book[\"title\"])\n    print(book[\"author\"])\n".to_string(),
                    hints: vec![
                        "Use curly braces {} for dictionaries".to_string(),
                        "Access values with book[\"key\"]".to_string(),
                    ],
                }),
                completed: false,
            },

            // Lesson 8: Error Handling
            Lesson {
                id: "error_handling".to_string(),
                title: "Error Handling".to_string(),
                description: "Handle errors gracefully".to_string(),
                content: r#"
Error Handling in GUL

Try-Catch blocks:
  try:
      # code that might fail
  catch error:
      # handle error

Result type (like Rust):
  fn divide(a, b) -> Result:
      if b == 0:
          return Err("Division by zero")
      return Ok(a / b)

Option type:
  fn find_user(id) -> Option:
      if user_exists(id):
          return Some(user)
      return None

Use pattern matching to handle Results and Options.
"#.to_string(),
                code_example: r#"fn safe_divide(a, b):
    try:
        if b == 0:
            return Err("Cannot divide by zero")
        return Ok(a / b)
    catch error:
        return Err(error)

main:
    def result = safe_divide(10, 2)
    match result:
        Ok(value):
            print("Result:", value)
        Err(error):
            print("Error:", error)
    
    def result2 = safe_divide(10, 0)
    match result2:
        Ok(value):
            print("Result:", value)
        Err(error):
            print("Error:", error)
"#.to_string(),
                exercise: Some(Exercise {
                    prompt: "Write a function that safely accesses a list element by index".to_string(),
                    starter_code: "fn safe_get(list, index):\n    # Your code here\n\nmain:\n    def numbers = [1, 2, 3]\n    def result = safe_get(numbers, 1)\n    print(result)\n".to_string(),
                    solution: "fn safe_get(list, index):\n    if index < 0 or index >= len(list):\n        return None\n    return Some(list[index])\n\nmain:\n    def numbers = [1, 2, 3]\n    def result = safe_get(numbers, 1)\n    print(result)\n".to_string(),
                    hints: vec![
                        "Check if index is valid".to_string(),
                        "Return None for invalid index".to_string(),
                        "Return Some(value) for valid index".to_string(),
                    ],
                }),
                completed: false,
            },

            // Lesson 9: Modules and Imports
            Lesson {
                id: "modules".to_string(),
                title: "Modules and Imports".to_string(),
                description: "Organize code with modules and imports".to_string(),
                content: r#"
Modules and Imports

Import GUL modules:
  imp std.io
  imp std.http
  imp std.math

Import from other languages:
  imp [python: numpy]
  imp [rust: tokio]
  imp [js: express]

Use imported functions:
  std.io.read_file("data.txt")
  std.http.get("https://api.example.com")

Create your own modules by organizing code in separate files.
"#.to_string(),
                code_example: r#"# Import standard library
imp std.io
imp std.math

# Import from Python
imp [python: numpy as np]

main:
    # Use standard library
    def content = std.io.read_file("example.txt")
    def sqrt = std.math.sqrt(16)
    
    # Use Python numpy
    @python
    arr = np.array([1, 2, 3, 4, 5])
    mean = np.mean(arr)
    
    print(sqrt)
    print(mean)
"#.to_string(),
                exercise: Some(Exercise {
                    prompt: "Import std.math and calculate the square root of 25".to_string(),
                    starter_code: "# Import here\n\nmain:\n    # Calculate sqrt(25)\n    \n    # Print result\n".to_string(),
                    solution: "imp std.math\n\nmain:\n    def result = std.math.sqrt(25)\n    print(result)\n".to_string(),
                    hints: vec![
                        "Use 'imp std.math'".to_string(),
                        "Call std.math.sqrt(25)".to_string(),
                    ],
                }),
                completed: false,
            },

            // Lesson 10: Final Project
            Lesson {
                id: "final_project".to_string(),
                title: "Final Project".to_string(),
                description: "Build a complete application".to_string(),
                content: r#"
Final Project: Todo List Application

Now that you've learned the basics, let's build a complete application!

Your task:
1. Create a todo list manager
2. Use functions for add, remove, and list operations
3. Use data structures (list/dictionary)
4. Handle errors gracefully
5. (Bonus) Add async file saving

This project combines everything you've learned:
- Variables and functions
- Control flow
- Data structures
- Error handling
- (Optional) Multi-language features
"#.to_string(),
                code_example: r#"# Todo List Application

def? todos = []

fn add_todo(task):
    todos.append({
        "task": task,
        "completed": false
    })
    return Ok("Task added")

fn complete_todo(index):
    if index < 0 or index >= len(todos):
        return Err("Invalid index")
    todos[index]["completed"] = true
    return Ok("Task completed")

fn list_todos():
    for i, todo in enumerate(todos):
        def status = "✓" if todo["completed"] else " "
        print(f"[{status}] {i}: {todo['task']}")

main:
    add_todo("Learn GUL")
    add_todo("Build a project")
    add_todo("Share with friends")
    
    list_todos()
    
    complete_todo(0)
    print("\nAfter completing first task:")
    list_todos()
"#.to_string(),
                exercise: Some(Exercise {
                    prompt: "Extend the todo app with a remove_todo function".to_string(),
                    starter_code: "def? todos = []\n\nfn remove_todo(index):\n    # Your code here\n\nmain:\n    # Test your function\n".to_string(),
                    solution: "def? todos = []\n\nfn remove_todo(index):\n    if index < 0 or index >= len(todos):\n        return Err(\"Invalid index\")\n    todos.remove(index)\n    return Ok(\"Task removed\")\n\nmain:\n    todos.append({\"task\": \"Test\", \"completed\": false})\n    remove_todo(0)\n    print(len(todos))  # Should be 0\n".to_string(),
                    hints: vec![
                        "Check if index is valid".to_string(),
                        "Use list.remove(index)".to_string(),
                        "Return Ok or Err".to_string(),
                    ],
                }),
                completed: false,
            },
        ]
    }

    pub fn get_current_lesson(&self) -> Option<&Lesson> {
        self.lessons.get(self.current_lesson)
    }

    pub fn next_lesson(&mut self) -> bool {
        if self.current_lesson < self.lessons.len() - 1 {
            self.current_lesson += 1;
            true
        } else {
            false
        }
    }

    pub fn previous_lesson(&mut self) -> bool {
        if self.current_lesson > 0 {
            self.current_lesson -= 1;
            true
        } else {
            false
        }
    }

    pub fn complete_lesson(&mut self, lesson_id: &str) {
        self.progress.insert(lesson_id.to_string(), true);
        if let Some(lesson) = self.lessons.iter_mut().find(|l| l.id == lesson_id) {
            lesson.completed = true;
        }
    }

    pub fn get_progress_percentage(&self) -> f32 {
        let completed = self.progress.values().filter(|&&v| v).count();
        (completed as f32 / self.lessons.len() as f32) * 100.0
    }
}

impl Default for Course {
    fn default() -> Self {
        Self::new()
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_course_creation() {
        let course = Course::new();
        assert_eq!(course.title, "Learn GUL - Interactive Course");
        assert!(!course.lessons.is_empty());
        assert_eq!(course.current_lesson, 0);
    }

    #[test]
    fn test_lesson_navigation() {
        let mut course = Course::new();
        assert_eq!(course.current_lesson, 0);

        assert!(course.next_lesson());
        assert_eq!(course.current_lesson, 1);

        assert!(course.previous_lesson());
        assert_eq!(course.current_lesson, 0);
    }

    #[test]
    fn test_lesson_completion() {
        let mut course = Course::new();
        let lesson_id = course.lessons[0].id.clone();

        course.complete_lesson(&lesson_id);
        assert!(course.progress.get(&lesson_id).unwrap_or(&false));
    }

    #[test]
    fn test_progress_percentage() {
        let mut course = Course::new();
        assert_eq!(course.get_progress_percentage(), 0.0);

        course.complete_lesson(&course.lessons[0].id.clone());
        assert!(course.get_progress_percentage() > 0.0);
    }
}
