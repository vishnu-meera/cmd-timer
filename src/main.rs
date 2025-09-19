use std::env;
use std::process::{Command, Stdio};
use std::time::Instant;
// use std::path::Path;

fn main() {
    let args: Vec<String> = env::args().collect();
    
    if args.len() < 2 {
        print_usage();
        std::process::exit(1);
    }

    match args[1].as_str() {
        "enable" => enable_timer(),
        "disable" => disable_timer(),
        "run" => {
            if args.len() < 3 {
                println!("Usage: timer run <command> [args...]");
                std::process::exit(1);
            }
            run_with_timing(&args[2..]);
        }
        "--help" | "-h" | "help" => {
            print_usage();
        }
        _ => {
            // Always time any command passed to timer
            run_with_timing(&args[1..]);
        }
    }
}

fn print_usage() {
    println!("Command Timer Utility");
    println!("Usage:");
    println!("  timer <command> [args...]       - Time any command");
    println!("  timer enable                    - Enable automatic timing");
    println!("  timer disable                   - Disable automatic timing");
    println!("  timer run <command> [args...]   - Time a specific command");
    println!("  timer help                      - Show this help");
    println!("");
    println!("Examples:");
    println!("  timer node script.js            - Times the node command");
    println!("  timer git status                - Times the git command");
    println!("  timer cargo build               - Times the cargo command");
    println!("  timer powershell -Command Get-Process");
    println!("  timer python -c \"print('hello')\"");
    println!("");
    println!("The timer will automatically start when the command begins");
    println!("and log the duration when it completes.");
}

fn enable_timer() {
    println!("🕐 Timer enabled");
    println!("Now you can run: timer <your-command>");
    println!("Example: timer node script.js");
    
    if let Err(e) = std::fs::write(".timer_enabled", "") {
        eprintln!("Warning: Could not create timer marker: {}", e);
    }
}

fn disable_timer() {
    let start = std::time::Instant::now();
    
    println!("⏹️  Timer disabled");
    let after_print = start.elapsed();
    
    let file_result = std::fs::remove_file(".timer_enabled");
    let after_file = start.elapsed();
    
    match file_result {
        Ok(_) => println!("Timer file removed successfully"),
        Err(_) => println!("Timer was not enabled or already disabled")
    }
    let after_final_print = start.elapsed();
    
    // Debug timing (remove this later)
    println!("Debug timings:");
    println!("  Print: {:?}", after_print);
    println!("  File op: {:?}", after_file - after_print);
    println!("  Final print: {:?}", after_final_print - after_file);
    println!("  Total: {:?}", after_final_print);
}

// fn is_timer_enabled() -> bool {
//     Path::new(".timer_enabled").exists()
// }

fn run_with_timing(command_args: &[String]) {
    if command_args.is_empty() {
        println!("No command specified");
        return;
    }

    let program = &command_args[0];
    let args = &command_args[1..];
    
    println!("⏱️  Executing: {}", command_args.join(" "));
    
    let start_time = Instant::now();
    
    // Execute the command with inherited stdio for real-time output
    let result = Command::new(program)
        .args(args)
        .stdin(Stdio::inherit())
        .stdout(Stdio::inherit())
        .stderr(Stdio::inherit())
        .status();
    
    let elapsed = start_time.elapsed();
    
    match result {
        Ok(status) => {
            println!(); // Add a newline for separation
            if status.success() {
                println!("✅ Command completed successfully");
            } else {
                println!("❌ Command failed with exit code: {:?}", status.code());
            }
            print_elapsed_time(elapsed);
            
            // Exit with the same code as the child process
            if let Some(code) = status.code() {
                std::process::exit(code);
            }
        }
        Err(e) => {
            println!("❌ Failed to execute '{}': {}", program, e);
            print_elapsed_time(elapsed);
            std::process::exit(1);
        }
    }
}

fn print_elapsed_time(elapsed: std::time::Duration) {
    let total_ms = elapsed.as_millis();
    let seconds = elapsed.as_secs();
    let ms = elapsed.subsec_millis();
    
    if seconds > 0 {
        println!("⏱️  Time taken: {}.{:03}s ({} ms)", seconds, ms, total_ms);
    } else {
        println!("⏱️  Time taken: {} ms", total_ms);
    }
}