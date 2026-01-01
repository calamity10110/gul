// TUI Course Interface
// Interactive terminal-based course viewer

use super::course::{Course, Lesson};
use std::io::{self, Write};

pub struct CourseTUI {
    course: Course,
    show_hints: bool,
}

impl CourseTUI {
    pub fn new() -> Self {
        CourseTUI {
            course: Course::new(),
            show_hints: false,
        }
    }

    pub fn run(&mut self) -> io::Result<()> {
        self.show_welcome();

        loop {
            self.clear_screen();
            self.show_header();

            if let Some(lesson) = self.course.get_current_lesson() {
                self.show_lesson(lesson);

                let choice = self.get_user_input(
                    "\nOptions: [N]ext, [P]revious, [H]ints, [E]xercise, [Q]uit: ",
                )?;

                match choice.to_lowercase().as_str() {
                    "n" | "next" => {
                        if !self.course.next_lesson() {
                            println!("\n🎉 Congratulations! You've completed all lessons!");
                            self.show_completion_stats();
                            break;
                        }
                    }
                    "p" | "prev" | "previous" => {
                        self.course.previous_lesson();
                    }
                    "h" | "hints" => {
                        self.show_hints = !self.show_hints;
                    }
                    "e" | "exercise" => {
                        self.show_exercise(lesson);
                    }
                    "q" | "quit" => {
                        println!("\nThanks for learning GUL! Keep coding! 🚀");
                        break;
                    }
                    _ => {
                        println!("Invalid option. Press Enter to continue...");
                        let _ = self.get_user_input("")?;
                    }
                }
            } else {
                break;
            }
        }

        Ok(())
    }

    fn show_welcome(&self) {
        self.clear_screen();
        println!("{}", "=".repeat(70));
        println!("{:^70}", "🎓 Welcome to GUL Interactive Course 🎓");
        println!("{}", "=".repeat(70));
        println!();
        println!("  Learn GUL through interactive lessons and hands-on exercises!");
        println!();
        println!("  📚 {} lessons covering:", self.course.lessons.len());
        println!("     • Basics (variables, functions, control flow)");
        println!("     • Multi-language integration");
        println!("     • Async/await programming");
        println!("     • Data structures and error handling");
        println!("     • Real-world project");
        println!();
        println!("{}", "=".repeat(70));
        println!("\nPress Enter to start...");
        let _ = io::stdin().read_line(&mut String::new());
    }

    fn show_header(&self) {
        let progress = self.course.get_progress_percentage();
        println!("{}", "=".repeat(70));
        println!("{:^70}", self.course.title);
        println!("{}", "-".repeat(70));
        println!(
            "  Progress: [{:50}] {:.0}%",
            "█".repeat((progress / 2.0) as usize),
            progress
        );
        println!(
            "  Lesson {}/{}: {}",
            self.course.current_lesson + 1,
            self.course.lessons.len(),
            self.course
                .get_current_lesson()
                .map(|l| l.title.as_str())
                .unwrap_or("")
        );
        println!("{}", "=".repeat(70));
        println!();
    }

    fn show_lesson(&self, lesson: &Lesson) {
        // Show lesson content
        println!("📖 {}", lesson.title);
        println!("{}", "-".repeat(70));
        println!("{}", lesson.description);
        println!();

        // Show lesson content
        for line in lesson.content.lines() {
            println!("{}", line);
        }
        println!();

        // Show code example
        if !lesson.code_example.is_empty() {
            println!("💻 Code Example:");
            println!("{}", "─".repeat(70));
            for line in lesson.code_example.lines() {
                println!("  {}", line);
            }
            println!("{}", "─".repeat(70));
        }
    }

    fn show_exercise(&self, lesson: &Lesson) {
        if let Some(exercise) = &lesson.exercise {
            self.clear_screen();
            println!("{}", "=".repeat(70));
            println!("{:^70}", "✏️  Exercise");
            println!("{}", "=".repeat(70));
            println!();
            println!("{}", exercise.prompt);
            println!();

            if self.show_hints {
                println!("💡 Hints:");
                for (i, hint) in exercise.hints.iter().enumerate() {
                    println!("  {}. {}", i + 1, hint);
                }
                println!();
            }

            println!("Starter Code:");
            println!("{}", "─".repeat(70));
            for line in exercise.starter_code.lines() {
                println!("  {}", line);
            }
            println!("{}", "─".repeat(70));
            println!();

            let show_solution = self
                .get_user_input("Show solution? (y/n): ")
                .unwrap_or_default()
                .to_lowercase();

            if show_solution == "y" || show_solution == "yes" {
                println!();
                println!("✅ Solution:");
                println!("{}", "─".repeat(70));
                for line in exercise.solution.lines() {
                    println!("  {}", line);
                }
                println!("{}", "─".repeat(70));
            }

            println!("\nPress Enter to continue...");
            let _ = io::stdin().read_line(&mut String::new());
        } else {
            println!("\nNo exercise for this lesson.");
            println!("Press Enter to continue...");
            let _ = io::stdin().read_line(&mut String::new());
        }
    }

    fn show_completion_stats(&self) {
        println!();
        println!("{}", "=".repeat(70));
        println!("{:^70}", "🏆 Course Completed! 🏆");
        println!("{}", "=".repeat(70));
        println!();
        println!(
            "  You've completed all {} lessons!",
            self.course.lessons.len()
        );
        println!("  Progress: 100%");
        println!();
        println!("  What's next?");
        println!("  • Build your own GUL projects");
        println!("  • Explore the GUL documentation");
        println!("  • Join the GUL community");
        println!("  • Contribute to GUL development");
        println!();
        println!("  Resources:");
        println!("  • Website: https://gul-lang.org");
        println!("  • Docs: https://docs.gul-lang.org");
        println!("  • GitHub: https://github.com/gul-lang/gul");
        println!();
        println!("{}", "=".repeat(70));
    }

    fn clear_screen(&self) {
        print!("\x1B[2J\x1B[1;1H");
        io::stdout().flush().unwrap();
    }

    fn get_user_input(&self, prompt: &str) -> io::Result<String> {
        print!("{}", prompt);
        io::stdout().flush()?;

        let mut input = String::new();
        io::stdin().read_line(&mut input)?;
        Ok(input.trim().to_string())
    }
}

impl Default for CourseTUI {
    fn default() -> Self {
        Self::new()
    }
}

// CLI command to run the course
pub fn run_course() {
    println!("Starting GUL Interactive Course...\n");

    let mut tui = CourseTUI::new();
    if let Err(e) = tui.run() {
        eprintln!("Error running course: {}", e);
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_course_tui_creation() {
        let tui = CourseTUI::new();
        assert_eq!(tui.course.current_lesson, 0);
        assert!(!tui.show_hints);
    }
}
