use std::io::{self, Write};

pub fn confirm_execution() -> bool {
    println!("\n🌿 Mellow Security Guard");
    println!("━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━");
    println!("⚠️  Potential security risk detected in the code.");
    println!("   AI-generated or modified scripts may contain harmful commands.");
    print!("\n👉 Do you want to proceed with execution? (y/N): ");

    io::stdout().flush().expect("Failed to flush stdout");

    let mut input = String::new();
    io::stdin()
        .read_line(&mut input)
        .expect("Failed to read user input");

    match input.trim().to_lowercase().as_str() {
        "y" | "yes" => true,
        _ => false,
    }
}
