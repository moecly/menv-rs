use std::io::Write;

use colored::Colorize;

#[derive(Debug)]
pub struct Common;

#[expect(dead_code)]
impl Common {
    pub fn print_separator() {
        println!("{}", "═".repeat(50).cyan());
    }

    pub fn print_title(title: &str) {
        println!("\n{}", "═".repeat(50).cyan());
        println!("  {}", title.bright_white().bold());
        println!("{}", "═".repeat(50).cyan());
        println!();
    }

    pub fn print_emoji_title(emoji: &str, title: &str) {
        println!("\n{}", "═".repeat(50).cyan());
        println!("  {} {}", emoji, title.bright_white().bold());
        println!("{}", "═".repeat(50).cyan());
        println!();
    }

    pub fn print_success(message: &str) {
        println!("  ✅ {}", message.green());
    }

    pub fn print_error(message: &str) {
        println!("  ❌ {}", message.red());
    }

    pub fn print_progress(name: &str) {
        print!("  {} {} ... ", "⏳".yellow(), name);
        std::io::stdout().flush().unwrap();
    }

    pub fn print_progress_done(name: &str) {
        println!("\r  ✅ {}                         ", name.green());
    }

    pub fn print_progress_failed(name: &str, error: &str) {
        println!("\r  ❌ {} - {}", name.red(), error.red());
    }

    pub fn print_summary(success: usize, failed: usize, total: usize) {
        println!("\n{}", "─".repeat(50).dimmed());
        println!(
            "  {} {}: {}/{}",
            "✨".bright_yellow(),
            "Complete".green().bold(),
            success,
            total
        );
        if failed > 0 {
            println!("  {} {}: {}", "⚠️".yellow(), "Failed".red().bold(), failed);
        }
        println!("{}", "═".repeat(50).cyan());
        println!();
    }
}
