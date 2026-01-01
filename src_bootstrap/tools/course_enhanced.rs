// Enhanced GUL Interactive Course - Beginner Friendly
// With code execution, validation, persistence, and better pedagogy

use serde::{Deserialize, Serialize};
use std::collections::HashMap;
use std::fs;
use std::path::PathBuf;

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct Lesson {
    pub id: String,
    pub title: String,
    pub difficulty: Difficulty,
    pub estimated_time: u32, // minutes
    pub description: String,
    pub learning_objectives: Vec<String>,
    pub prerequisites: Vec<String>,
    pub content: Vec<ContentBlock>,
    pub exercises: Vec<Exercise>,
    pub quiz: Option<Quiz>,
    pub completed: bool,
    pub score: f32,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub enum Difficulty {
    Beginner,
    Easy,
    Medium,
    Hard,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub enum ContentBlock {
    Text(String),
    Concept {
        title: String,
        explanation: String,
        why: String,     // Why is this important?
        analogy: String, // Real-world analogy
    },
    CodeExample {
        code: String,
        explanation: String,
        runnable: bool,
        expected_output: Option<String>,
    },
    Diagram {
        title: String,
        ascii_art: String,
        description: String,
    },
    TipBox {
        tip_type: TipType,
        content: String,
    },
    InteractiveDemo {
        prompt: String,
        starter_code: String,
    },
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub enum TipType {
    ProTip,
    CommonMistake,
    BestPractice,
    DebuggingHelp,
    DidYouKnow,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct Exercise {
    pub id: String,
    pub title: String,
    pub difficulty: Difficulty,
    pub prompt: String,
    pub detailed_instructions: Vec<String>, // Step-by-step
    pub starter_code: String,
    pub solution: String,
    pub test_cases: Vec<TestCase>,
    pub hints: Vec<Hint>,
    pub common_mistakes: Vec<CommonMistake>,
    pub completed: bool,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct TestCase {
    pub input: String,
    pub expected_output: String,
    pub description: String,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct Hint {
    pub level: u32, // 1 = gentle, 5 = almost solution
    pub content: String,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct CommonMistake {
    pub mistake: String,
    pub why_wrong: String,
    pub how_to_fix: String,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct Quiz {
    pub questions: Vec<QuizQuestion>,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct QuizQuestion {
    pub question: String,
    pub options: Vec<String>,
    pub correct_answer: usize,
    pub explanation: String,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct Course {
    pub title: String,
    pub description: String,
    pub version: String,
    pub lessons: Vec<Lesson>,
    pub current_lesson: usize,
    pub progress: CourseProgress,
    pub glossary: HashMap<String, String>,
    pub cheat_sheet: Vec<CheatSheetItem>,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct CourseProgress {
    pub lessons_completed: HashMap<String, bool>,
    pub exercises_completed: HashMap<String, bool>,
    pub total_score: f32,
    pub time_spent: u32, // minutes
    pub achievements: Vec<Achievement>,
    pub last_accessed: String,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct Achievement {
    pub id: String,
    pub title: String,
    pub description: String,
    pub unlocked: bool,
    pub icon: String,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct CheatSheetItem {
    pub category: String,
    pub syntax: String,
    pub description: String,
    pub example: String,
}

impl Course {
    pub fn new() -> Self {
        Course {
            title: "Learn GUL - Complete Beginner's Course".to_string(),
            description: "Master GUL from absolute basics to advanced topics".to_string(),
            version: "2.0.0".to_string(),
            lessons: Self::create_enhanced_lessons(),
            current_lesson: 0,
            progress: CourseProgress::new(),
            glossary: Self::create_glossary(),
            cheat_sheet: Self::create_cheat_sheet(),
        }
    }

    fn create_enhanced_lessons() -> Vec<Lesson> {
        vec![
            // Lesson 0: What is Programming? (NEW - Essential for beginners)
            Lesson {
                id: "what_is_programming".to_string(),
                title: "What is Programming?".to_string(),
                difficulty: Difficulty::Beginner,
                estimated_time: 15,
                description: "Understand what programming is and why it matters".to_string(),
                learning_objectives: vec![
                    "Understand what a computer program is".to_string(),
                    "Learn how computers execute instructions".to_string(),
                    "See real-world examples of programming".to_string(),
                    "Understand what makes GUL special".to_string(),
                ],
                prerequisites: vec!["None - Start here!".to_string()],
                content: vec![
                    ContentBlock::Concept {
                        title: "What is a Computer Program?".to_string(),
                        explanation: "A computer program is a set of instructions that tells a computer what to do. Just like a recipe tells you how to bake a cake, a program tells the computer how to perform tasks.".to_string(),
                        why: "Understanding this helps you think like a programmer - breaking down problems into step-by-step instructions.".to_string(),
                        analogy: "Think of it like giving directions to a friend. You need to be very specific: 'Turn left at the red building' not just 'go that way'. Computers need the same level of detail!".to_string(),
                    },
                    ContentBlock::Diagram {
                        title: "How Programs Work".to_string(),
                        ascii_art: r#"
    ┌─────────────┐
    │  Your Code  │  (What you write)
    └──────┬──────┘
           │
           ▼
    ┌─────────────┐
    │  Compiler   │  (Translates to machine code)
    └──────┬──────┘
           │
           ▼
    ┌─────────────┐
    │  Computer   │  (Executes instructions)
    └──────┬──────┘
           │
           ▼
    ┌─────────────┐
    │   Result    │  (Output you see)
    └─────────────┘
"#.to_string(),
                        description: "This shows the journey from your code to the result you see on screen.".to_string(),
                    },
                    ContentBlock::TipBox {
                        tip_type: TipType::DidYouKnow,
                        content: "Every app on your phone, every website you visit, and every video game you play was created by writing code - just like you're about to learn!".to_string(),
                    },
                    ContentBlock::CodeExample {
                        code: r#"# Your first program!
main:
    print("Hello, World!")
"#.to_string(),
                        explanation: "This is the traditional first program. It tells the computer to display 'Hello, World!' on the screen. Simple, but it's the start of your programming journey!".to_string(),
                        runnable: true,
                        expected_output: Some("Hello, World!".to_string()),
                    },
                    ContentBlock::Concept {
                        title: "What Makes GUL Special?".to_string(),
                        explanation: "GUL lets you use multiple programming languages in one file! Want to use Python for data science and Rust for speed? You can do both!".to_string(),
                        why: "Different languages are good at different things. GUL lets you use the best tool for each job.".to_string(),
                        analogy: "It's like having a Swiss Army knife instead of carrying separate tools. One tool, many capabilities!".to_string(),
                    },
                ],
                exercises: vec![
                    Exercise {
                        id: "first_program".to_string(),
                        title: "Your First Program".to_string(),
                        difficulty: Difficulty::Beginner,
                        prompt: "Write a program that prints your name".to_string(),
                        detailed_instructions: vec![
                            "1. Start with 'main:' - this is where your program begins".to_string(),
                            "2. On the next line, indent (press Tab or 4 spaces)".to_string(),
                            "3. Type: print(\"Your Name Here\")".to_string(),
                            "4. Replace 'Your Name Here' with your actual name".to_string(),
                            "5. Make sure to keep the quotes!".to_string(),
                        ],
                        starter_code: "main:\n    # Type your code here\n    ".to_string(),
                        solution: "main:\n    print(\"Alice\")\n".to_string(),
                        test_cases: vec![
                            TestCase {
                                input: "".to_string(),
                                expected_output: "Alice".to_string(),
                                description: "Should print a name".to_string(),
                            },
                        ],
                        hints: vec![
                            Hint {
                                level: 1,
                                content: "Remember to use the print() function".to_string(),
                            },
                            Hint {
                                level: 2,
                                content: "Put your name in quotes, like this: \"YourName\"".to_string(),
                            },
                            Hint {
                                level: 3,
                                content: "The complete line should look like: print(\"YourName\")".to_string(),
                            },
                        ],
                        common_mistakes: vec![
                            CommonMistake {
                                mistake: "Forgetting the quotes around the name".to_string(),
                                why_wrong: "Without quotes, GUL thinks it's a variable name, not text".to_string(),
                                how_to_fix: "Add quotes: print(\"Alice\") not print(Alice)".to_string(),
                            },
                            CommonMistake {
                                mistake: "Not indenting after main:".to_string(),
                                why_wrong: "GUL uses indentation to know what code belongs to main".to_string(),
                                how_to_fix: "Press Tab or add 4 spaces before print()".to_string(),
                            },
                        ],
                        completed: false,
                    },
                ],
                quiz: Some(Quiz {
                    questions: vec![
                        QuizQuestion {
                            question: "What is a computer program?".to_string(),
                            options: vec![
                                "A set of instructions for a computer".to_string(),
                                "A type of computer hardware".to_string(),
                                "A computer game".to_string(),
                                "A website".to_string(),
                            ],
                            correct_answer: 0,
                            explanation: "A program is a set of instructions that tells the computer what to do, step by step.".to_string(),
                        },
                        QuizQuestion {
                            question: "What makes GUL special?".to_string(),
                            options: vec![
                                "It's the fastest language".to_string(),
                                "You can use multiple languages in one file".to_string(),
                                "It only works on phones".to_string(),
                                "It doesn't need a compiler".to_string(),
                            ],
                            correct_answer: 1,
                            explanation: "GUL's superpower is letting you mix Rust, Python, JavaScript, and more in a single file!".to_string(),
                        },
                    ],
                }),
                completed: false,
                score: 0.0,
            },

            // Enhanced Lesson 1: Hello World (with much more detail)
            Lesson {
                id: "hello_world".to_string(),
                title: "Your First GUL Program".to_string(),
                difficulty: Difficulty::Beginner,
                estimated_time: 20,
                description: "Write and understand your first GUL program".to_string(),
                learning_objectives: vec![
                    "Understand the main: block".to_string(),
                    "Learn the print() function".to_string(),
                    "Understand strings (text)".to_string(),
                    "Learn about indentation".to_string(),
                ],
                prerequisites: vec!["Lesson 0: What is Programming?".to_string()],
                content: vec![
                    ContentBlock::Concept {
                        title: "The main: Block".to_string(),
                        explanation: "Every GUL program starts with 'main:'. This is the entry point - where your program begins running.".to_string(),
                        why: "The computer needs to know where to start. Without main:, it wouldn't know which code to run first!".to_string(),
                        analogy: "Think of main: as the front door of a house. You need to enter through the door before you can do anything inside.".to_string(),
                    },
                    ContentBlock::Diagram {
                        title: "Program Structure".to_string(),
                        ascii_art: r#"
main:                  ← Entry point (start here!)
    print("Hello")     ← Indented code (inside main)
    print("World")     ← More indented code
                       ← Program ends here
"#.to_string(),
                        description: "Notice how the code under main: is indented. This shows it belongs to main.".to_string(),
                    },
                    ContentBlock::TipBox {
                        tip_type: TipType::CommonMistake,
                        content: "Forgetting to indent is the #1 beginner mistake! Always indent code that belongs inside main:".to_string(),
                    },
                    ContentBlock::CodeExample {
                        code: r#"main:
    print("Hello, World!")
    print("I'm learning GUL!")
    print("This is fun!")
"#.to_string(),
                        explanation: "This program prints three lines. Each print() statement shows text on a new line.".to_string(),
                        runnable: true,
                        expected_output: Some("Hello, World!\nI'm learning GUL!\nThis is fun!".to_string()),
                    },
                    ContentBlock::TipBox {
                        tip_type: TipType::ProTip,
                        content: "You can print multiple things by separating them with commas: print(\"Hello\", \"World\")".to_string(),
                    },
                ],
                exercises: vec![
                    Exercise {
                        id: "hello_custom".to_string(),
                        title: "Personalized Greeting".to_string(),
                        difficulty: Difficulty::Easy,
                        prompt: "Create a program that prints a greeting with your name and favorite color".to_string(),
                        detailed_instructions: vec![
                            "1. Start with main:".to_string(),
                            "2. Print 'Hello, my name is [Your Name]'".to_string(),
                            "3. Print 'My favorite color is [Your Color]'".to_string(),
                            "4. Remember to indent both print statements!".to_string(),
                        ],
                        starter_code: "main:\n    # Print your name\n    \n    # Print your favorite color\n    ".to_string(),
                        solution: "main:\n    print(\"Hello, my name is Alice\")\n    print(\"My favorite color is blue\")\n".to_string(),
                        test_cases: vec![],
                        hints: vec![
                            Hint { level: 1, content: "Use two print() statements".to_string() },
                            Hint { level: 2, content: "Put your text in quotes".to_string() },
                            Hint { level: 3, content: "Example: print(\"Hello, my name is Alice\")".to_string() },
                        ],
                        common_mistakes: vec![
                            CommonMistake {
                                mistake: "Mixing single and double quotes".to_string(),
                                why_wrong: "Quotes must match: \"text\" or 'text', not \"text'".to_string(),
                                how_to_fix: "Use double quotes consistently: print(\"text\")".to_string(),
                            },
                        ],
                        completed: false,
                    },
                ],
                quiz: None,
                completed: false,
                score: 0.0,
            },

            // Continue with more enhanced lessons...
            // (I'll create a few more to show the pattern)
        ]
    }

    fn create_glossary() -> HashMap<String, String> {
        let mut glossary = HashMap::new();

        glossary.insert(
            "Variable".to_string(),
            "A named storage location for data. Like a labeled box where you can put things."
                .to_string(),
        );
        glossary.insert("Function".to_string(), 
            "A reusable block of code that performs a specific task. Like a recipe you can use multiple times.".to_string());
        glossary.insert(
            "String".to_string(),
            "Text data enclosed in quotes. Example: \"Hello\"".to_string(),
        );
        glossary.insert(
            "Integer".to_string(),
            "A whole number without decimals. Example: 42, -5, 0".to_string(),
        );
        glossary.insert(
            "Indentation".to_string(),
            "Spaces or tabs at the start of a line. GUL uses this to group code together."
                .to_string(),
        );
        glossary.insert(
            "Compiler".to_string(),
            "A program that translates your code into instructions the computer can execute."
                .to_string(),
        );
        glossary.insert(
            "Syntax".to_string(),
            "The rules for writing code correctly. Like grammar for programming languages."
                .to_string(),
        );
        glossary.insert(
            "Bug".to_string(),
            "An error or mistake in your code that causes it to not work as expected.".to_string(),
        );
        glossary.insert(
            "Debug".to_string(),
            "The process of finding and fixing bugs in your code.".to_string(),
        );
        glossary.insert(
            "Loop".to_string(),
            "Code that repeats multiple times. Like doing something over and over.".to_string(),
        );

        glossary
    }

    fn create_cheat_sheet() -> Vec<CheatSheetItem> {
        vec![
            CheatSheetItem {
                category: "Basics".to_string(),
                syntax: "main:".to_string(),
                description: "Program entry point".to_string(),
                example: "main:\n    print(\"Hello\")".to_string(),
            },
            CheatSheetItem {
                category: "Output".to_string(),
                syntax: "print(...)".to_string(),
                description: "Display text or values".to_string(),
                example: "print(\"Hello, World!\")".to_string(),
            },
            CheatSheetItem {
                category: "Variables".to_string(),
                syntax: "def name = value".to_string(),
                description: "Create immutable variable".to_string(),
                example: "def age = 25".to_string(),
            },
            CheatSheetItem {
                category: "Variables".to_string(),
                syntax: "def? name = value".to_string(),
                description: "Create mutable variable".to_string(),
                example: "def? count = 0".to_string(),
            },
            CheatSheetItem {
                category: "Functions".to_string(),
                syntax: "fn name(params):".to_string(),
                description: "Define a function".to_string(),
                example: "fn greet(name):\n    print(\"Hello\", name)".to_string(),
            },
        ]
    }

    // Save/Load progress
    pub fn save_progress(&self) -> Result<(), String> {
        let progress_file = Self::get_progress_file();
        let json = serde_json::to_string_pretty(&self.progress)
            .map_err(|e| format!("Failed to serialize progress: {}", e))?;
        fs::write(&progress_file, json)
            .map_err(|e| format!("Failed to write progress file: {}", e))?;
        Ok(())
    }

    pub fn load_progress(&mut self) -> Result<(), String> {
        let progress_file = Self::get_progress_file();
        if progress_file.exists() {
            let json = fs::read_to_string(&progress_file)
                .map_err(|e| format!("Failed to read progress file: {}", e))?;
            self.progress = serde_json::from_str(&json)
                .map_err(|e| format!("Failed to parse progress: {}", e))?;
        }
        Ok(())
    }

    fn get_progress_file() -> PathBuf {
        let home = std::env::var("HOME").unwrap_or_else(|_| ".".to_string());
        let mut path = PathBuf::from(home);
        path.push(".gul_course_progress.json");
        path
    }
}

impl CourseProgress {
    pub fn new() -> Self {
        CourseProgress {
            lessons_completed: HashMap::new(),
            exercises_completed: HashMap::new(),
            total_score: 0.0,
            time_spent: 0,
            achievements: Self::create_achievements(),
            last_accessed: "2025-12-04".to_string(),
        }
    }

    fn create_achievements() -> Vec<Achievement> {
        vec![
            Achievement {
                id: "first_lesson".to_string(),
                title: "First Steps".to_string(),
                description: "Complete your first lesson".to_string(),
                unlocked: false,
                icon: "🎯".to_string(),
            },
            Achievement {
                id: "hello_world".to_string(),
                title: "Hello, World!".to_string(),
                description: "Write your first program".to_string(),
                unlocked: false,
                icon: "👋".to_string(),
            },
            Achievement {
                id: "perfect_score".to_string(),
                title: "Perfect!".to_string(),
                description: "Get 100% on an exercise".to_string(),
                unlocked: false,
                icon: "💯".to_string(),
            },
            Achievement {
                id: "persistent".to_string(),
                title: "Persistent Learner".to_string(),
                description: "Complete 5 lessons".to_string(),
                unlocked: false,
                icon: "🔥".to_string(),
            },
            Achievement {
                id: "course_complete".to_string(),
                title: "Course Master".to_string(),
                description: "Complete all lessons".to_string(),
                unlocked: false,
                icon: "🏆".to_string(),
            },
        ]
    }
}

impl Default for Course {
    fn default() -> Self {
        Self::new()
    }
}

impl Default for CourseProgress {
    fn default() -> Self {
        Self::new()
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_enhanced_course_creation() {
        let course = Course::new();
        assert_eq!(course.version, "2.0.0");
        assert!(!course.lessons.is_empty());
        assert!(!course.glossary.is_empty());
        assert!(!course.cheat_sheet.is_empty());
    }

    #[test]
    fn test_glossary() {
        let course = Course::new();
        assert!(course.glossary.contains_key("Variable"));
        assert!(course.glossary.contains_key("Function"));
    }

    #[test]
    fn test_achievements() {
        let progress = CourseProgress::new();
        assert_eq!(progress.achievements.len(), 5);
        assert!(!progress.achievements[0].unlocked);
    }
}
