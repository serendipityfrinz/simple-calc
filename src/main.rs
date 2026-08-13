// Note: i used AI to fix the indention of the entire code.
use colored::*;
use inquire::{Select, CustomType};
use clearscreen::*;
use std::time::Duration;
use std::thread;
use clap::Parser;

// Please forgive me for such a simple argument i'm still learning clap TvT
#[derive(Parser)]
#[command(version, about, long_about = None)]
struct Args {
    /// Don't clear the terminal.
    #[arg(short, long)]
    noclear: bool,
}

fn main() {
    let args = Args::parse();

    if !args.noclear {
        clear().expect("Failed to clear screen");
        // Clear the terminal first so terminal is clean :)
    }

    loop {
        let mut _num1: f32 = 0.0;
        let mut _num2: f32 = 0.0;
        
        println!(
            "{} {} {}", 
            "===///".bold().green(), 
            "simple-calc".italic().bold().yellow(), 
            "///===".bold().green()
        );

        let options = vec![
            "1. Enter advanced mode".magenta().to_string(),
            "2. The four basic operations".green().to_string(),
            "3. Clear screen".yellow().to_string(),
            "4. Exit".red().to_string(),
        ];

        let a_options = vec![
            "1. Exponentiation".yellow().to_string(),
            "2. Square root".yellow().to_string(),
            "3. Rounding".yellow().to_string(),
        ];

        let f_options = vec![
            "2. Addition".green().to_string(),
            "3. Subtraction".green().to_string(),
            "4. Multiplication".green().to_string(),
            "5. Division".green().to_string(),
        ];
        
        let yn = vec![
            "No (default)".bright_red().to_string(),
            "Yes".bright_green().to_string(),
        ];

        let action = Select::new("What would you like to do?", options)
            .prompt()
            .unwrap();

        if action.contains("four basic operations"){
            let f_action = Select::new("Choose one from the four basic operations:", f_options)
                .prompt()
                .unwrap();
            _num1 = CustomType::<f32>::new("Enter the first number:")
                .prompt()
                .unwrap();
            _num2 = CustomType::<f32>::new("Enter the second number:")
                .prompt()
                .unwrap();
            if f_action.contains("Addition") {
                println!("{}: {}", "The answer is".cyan().bold(), _num1 + _num2);
            } else if f_action.contains("Subtraction") {
                println!("{}: {}", "The answer is".cyan().bold(), _num1 - _num2);
            } else if f_action.contains("Multiplication") {
                println!("{}: {}", "The answer is".cyan().bold(), _num1 * _num2);
            } else if f_action.contains("Division") {
                if _num2 == 0.0 {
                    println!("{}", "You can't divide by zero!".bold().red());
                } else {
                    println!("{}: {}", "The answer is".cyan().bold(), _num1 / _num2);
                }
            } else {
                unreachable!("You selected an option that isn't on the menu!");
            }   
        }

        if action.contains("advanced") {
            let a_action = Select::new("What would you like to do in advanced mode?", a_options)
                .prompt()
                .unwrap();

            if a_action.contains("Exponentiation") {
                let base = CustomType::<f32>::new("Enter the base:")
                    .prompt()
                    .unwrap();
                let exponent = CustomType::<f32>::new("Enter the exponent:")
                    .prompt()
                    .unwrap();
                println!("{}: {}", "The answer is".cyan().bold(), base.powf(exponent));
            } else if a_action.contains("Square root") {
                let radicand = CustomType::<f32>::new("Enter the radicand:")
                    .prompt()
                    .unwrap();
                let index = CustomType::<f32>::new("Enter the index (optional):")
                    .with_default(2.0)
                    .with_help_message("Enter to use the default (Square root)")
                    .prompt()
                    .unwrap();
                println!("{}: {}", "The root is".cyan().bold(), radicand.powf(1.0 / index));
            } else if a_action.contains("Rounding") {
                let round_num = CustomType::<f32>::new("Enter the number to round:")
                    .prompt()
                    .unwrap();
                let r_txt = format!(
                    "Do you want to control the rounding? [{}/{}]",
                    "Yes".bright_green(),
                    "No".bright_red()
                );
                let r_options = Select::new(&r_txt, yn)
                    .prompt()
                    .unwrap();

                if r_options.contains("Yes") {
                    let rr_options = Select::new(
                        "Select the type of round :)", 
                        vec![
                            "1. Rounding by decimal places".green(), 
                            "2. Always round up".green(), 
                            "3. Always round down".green()
                        ]
                    )
                    .prompt()
                    .unwrap();

                    if rr_options.contains("decimal") {
                        let dround = CustomType::<i32>::new("Enter the decimal places to use:")
                            .prompt()
                            .unwrap();
                        let multiplier = 10.0_f32.powi(dround);
                        println!(
                            "{} {}: {}", 
                            round_num.to_string().cyan().bold(), 
                            "rounded is".cyan().bold(), 
                            (round_num * multiplier).round() / multiplier
                        );
                    } else if rr_options.contains("round up") {
                        println!(
                            "{} {}: {}", 
                            round_num.to_string().cyan().bold(), 
                            "rounded is".cyan().bold(), 
                            round_num.ceil()
                        );  
                    } else if rr_options.contains("round down") {
                        println!(
                            "{} {}: {}", 
                            round_num.to_string().cyan().bold(), 
                            "rounded is".cyan().bold(), 
                            round_num.floor()
                        );
                    }
                } else {
                    println!(
                        "{} {}: {}", 
                        round_num.to_string().cyan().bold(), 
                        "rounded is".cyan().bold(),
                        round_num.round()
                    );
                }
            }
            continue;
        } else if action.contains("Exit") {
            println!("Bye!");
            break;
        } else if action.contains("Clear") {
            println!("{}", "Clearing screen!".yellow().bold()); // Just for fun lol
            thread::sleep(Duration::from_millis(500));
            clear().expect("Failed to clear screen.");
            continue;
        }
    }
}
