# GUL Interactive TUI Course

**Version:** v0.13.0  
**Status:** ✅ Complete  
**Lessons:** 10  
**Difficulty:** Beginner to Intermediate

---

## Overview

The GUL Interactive TUI Course is a terminal-based learning system that teaches the GUL programming language through interactive lessons, code examples, and hands-on exercises.

---

## Features

### 📚 Comprehensive Curriculum

- **10 Lessons** covering basics to advanced topics
- **Progressive Learning** from simple to complex concepts
- **Hands-on Exercises** with starter code and solutions
- **Real Code Examples** that you can run

### 🎯 Interactive Learning

- **Terminal-based Interface** - Learn directly in your terminal
- **Progress Tracking** - See your completion percentage
- **Hint System** - Get help when you need it
- **Solution Viewing** - Check your work against solutions

### 💡 Smart Design

- **Self-paced** - Learn at your own speed
- **Navigation** - Easy forward/backward movement
- **Clear Examples** - Well-commented code samples
- **Practical Exercises** - Apply what you learn immediately

---

## Course Outline

### Lesson 1: Welcome to GUL

**Topics:**

- Introduction to GUL
- Your first "Hello, World!" program
- Understanding the `main:` block

**Exercise:** Write a program that prints "Hello, GUL!"

---

### Lesson 2: Variables and Types

**Topics:**

- Defining variables with `def`
- Immutable vs mutable variables (`def?`)
- Type inference and annotations
- Basic data types

**Exercise:** Create and manipulate mutable variables

---

### Lesson 3: Functions

**Topics:**

- Defining functions with `fn`
- Parameters and return values
- Type annotations for functions
- Calling functions

**Exercise:** Create a function that squares a number

---

### Lesson 4: Control Flow

**Topics:**

- If/elif/else statements
- For loops and while loops
- Loop control (break, continue)
- Indentation-based syntax

**Exercise:** Print numbers from 1 to n using a loop

---

### Lesson 5: Multi-Language Integration

**Topics:**

- Using `@rust`, `@python`, `@js`, `@sql` blocks
- Mixing multiple languages in one file
- Zero-copy data sharing
- Language-specific features

**Exercise:** Create a Rust function and call it from GUL

---

### Lesson 6: Async/Await

**Topics:**

- Async functions with `asy`
- Await expressions
- Tokio-powered async runtime
- Concurrent operations

**Exercise:** Create an async function with delays

---

### Lesson 7: Data Structures

**Topics:**

- Lists (arrays)
- Dictionaries (hash maps)
- Tuples
- Sets
- Iteration and manipulation

**Exercise:** Create and use a dictionary

---

### Lesson 8: Error Handling

**Topics:**

- Try-catch blocks
- Result type (Ok/Err)
- Option type (Some/None)
- Pattern matching for errors

**Exercise:** Write a safe list access function

---

### Lesson 9: Modules and Imports

**Topics:**

- Importing GUL standard library
- Importing from other languages
- Using imported functions
- Module organization

**Exercise:** Import and use std.math

---

### Lesson 10: Final Project

**Topics:**

- Building a complete application
- Todo list manager
- Combining all concepts
- Best practices

**Exercise:** Extend the todo app with new features

---

## How to Use

### Starting the Course

```bash
# From the GUL project directory
cargo run --bin gul -- course

# Or if GUL is installed
gul course
```

### Navigation

While in the course, you can use these commands:

- **N** or **Next** - Go to the next lesson
- **P** or **Previous** - Go to the previous lesson
- **H** or **Hints** - Toggle hint display
- **E** or **Exercise** - View and practice the exercise
- **Q** or **Quit** - Exit the course

### Viewing Exercises

1. Press **E** to view the current lesson's exercise
2. Read the prompt and starter code
3. Toggle **H** for hints if needed
4. Type **y** to view the solution when ready

### Progress Tracking

Your progress is displayed at the top of each screen:

```
Progress: [██████████████████████████                        ] 50%
```

---

## Course Structure

### Code Organization

```
src/tools/
├── course.rs       # Course data and lesson definitions
└── course_tui.rs   # TUI interface and navigation
```

### Lesson Structure

Each lesson includes:

- **ID** - Unique identifier
- **Title** - Lesson name
- **Description** - Brief overview
- **Content** - Detailed explanation
- **Code Example** - Working code demonstration
- **Exercise** - Hands-on practice (optional)
  - Prompt
  - Starter code
  - Solution
  - Hints

---

## Learning Path

### Beginner Track (Lessons 1-4)

Start here if you're new to programming or GUL:

1. Welcome to GUL
2. Variables and Types
3. Functions
4. Control Flow

### Intermediate Track (Lessons 5-7)

Continue with GUL-specific features: 5. Multi-Language Integration 6. Async/Await 7. Data Structures

### Advanced Track (Lessons 8-10)

Master advanced concepts: 8. Error Handling 9. Modules and Imports 10. Final Project

---

## Example Session

```
==================================================================
           🎓 Learn GUL - Interactive Course 🎓
==================================================================

  Progress: [██████████                                        ] 20%
  Lesson 2/10: Variables and Types

==================================================================

📖 Variables and Types
──────────────────────────────────────────────────────────────────
Learn about variables, types, and definitions

Variables in GUL

GUL uses the 'def' keyword to define variables.
Variables are immutable by default (like Rust).
Use '?' after 'def' to make a variable mutable.

💻 Code Example:
──────────────────────────────────────────────────────────────────
  main:
      # Immutable variable
      def x = 42
      def name = "Alice"

      # Mutable variable
      def? count = 0
      count = count + 1
──────────────────────────────────────────────────────────────────

Options: [N]ext, [P]revious, [H]ints, [E]xercise, Play[G]round, [Q]uit:
```

---

## Tips for Success

### 1. Take Your Time

- Don't rush through lessons
- Excercise with the code examples
- Experiment in playground
- Try variations of the exercises

### 2. Practice Actively

- Type out the code examples yourself
- Modify examples to see what happens
- Complete all exercises

### 3. Use Hints Wisely

- Try solving exercises first
- Use hints if you're stuck
- View solutions only after attempting

### 4. Build on Knowledge

- Each lesson builds on previous ones
- Review earlier lessons if needed
- Connect concepts together

### 5. Experiment

- Try breaking the code to learn
- Combine concepts from different lessons
- Create your own examples

---

## Extending the Course

### Adding New Lessons

To add a new lesson, edit `src/tools/course.rs`:

```rust
Lesson {
    id: "your_lesson_id".to_string(),
    title: "Your Lesson Title".to_string(),
    description: "Brief description".to_string(),
    content: r#"
Detailed lesson content here
"#.to_string(),
    code_example: r#"
# Your code example
main:
    print("Example")
"#.to_string(),
    exercise: Some(Exercise {
        prompt: "Exercise prompt".to_string(),
        starter_code: "# Starter code\n".to_string(),
        solution: "# Solution\n".to_string(),
        hints: vec!["Hint 1".to_string()],
    }),
    completed: false,
}
```

### Customizing the TUI

The TUI interface can be customized in `src/tools/course_tui.rs`:

- Modify colors and formatting
- Add new navigation options
- Enhance progress visualization
- Add interactive code execution

---

## Testing

Run the course tests:

```bash
cargo test tools::course
```

Expected output:

```
running 5 tests
test tools::course::tests::test_course_creation ... ok
test tools::course::tests::test_lesson_completion ... ok
test tools::course::tests::test_lesson_navigation ... ok
test tools::course::tests::test_progress_percentage ... ok
test tools::course_tui::tests::test_course_tui_creation ... ok

test result: ok. 5 passed; 0 failed
```

---

## Future Enhancements

### Planned Features

- [ ] Save progress to disk
- [ ] Code execution within the course
- [ ] Interactive code editor
- [ ] Quiz mode with scoring
- [ ] Certificate generation
- [ ] Multi-language course content
- [ ] Video tutorials integration
- [ ] Community-contributed lessons

### Advanced Features

- [ ] AI-powered hints
- [ ] Adaptive difficulty
- [ ] Peer learning integration
- [ ] Real-time collaboration
- [ ] Gamification elements

---

## Troubleshooting

### Course Won't Start

```bash
# Rebuild the project
cargo build --release

# Try running directly
cargo run --bin gul -- course
```

### Display Issues

- Ensure your terminal supports UTF-8
- Use a terminal with at least 70 columns width
- Try a different terminal emulator

### Navigation Problems

- Press Enter after typing commands
- Commands are case-insensitive
- Use single letters (n, p, h, e, q)

---

## Contributing

Want to improve the course? Contributions are welcome!

### How to Contribute

1. Fork the repository
2. Add or improve lessons
3. Test your changes
4. Submit a pull request

### Guidelines

- Keep lessons focused and concise
- Provide clear, working code examples
- Include exercises with solutions
- Add helpful hints
- Test all code examples

---

## Resources

### Documentation

- **GUL Docs:** https://docs.mn-lang.org
- **API Reference:** https://api.mn-lang.org
- **Examples:** https://github.com/gul-lang/gul/tree/main/examples

### Community

- **Discord:** https://discord.gg/gul
- **GitHub:** https://github.com/gul-lang/gul
- **Forum:** https://community.mn-lang.org

### Support

- **Issues:** https://github.com/gul-lang/gul/issues
- **Discussions:** https://github.com/gul-lang/gul/discussions

---

## License

The GUL Interactive Course is part of the GUL project and is released under the MIT License.

---

## Acknowledgments

Special thanks to:

- The GUL community for feedback
- Contributors who improved lessons
- Everyone learning GUL!

---

**Created:** 2025-12-02 16:29:51 PST  
**Version:** v0.13.0  
**Status:** ✅ Production Ready  
**Lessons:** 10  
**Tests:** 5/5 passing

---

_Happy Learning! 🚀_
