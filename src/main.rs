use colored::*;
use inquire::{Select, CustomType};
use clearscreen::*;
use std::time::Duration;
use std::thread;

fn main() {
    loop {
        let mut _num1: f32 = 0.0;
        let mut _num2: f32 = 0.0;
        
        println!("{} {} {}", "===///".bold().green(), 
            "simple-calc".italic().bold().yellow(), 
            "///===".bold().green());

        let options = vec![
            "1. Enter advanced mode".magenta().to_string(),
            "2. Addition".green().to_string(),
            "3. Subtraction".green().to_string(),
            "4. Multiplication".green().to_string(),
            "5. Division".green().to_string(),
            "6. Rounding".green().to_string(),
            "7. Clear screen".yellow().to_string(),
            "8. Exit".red().to_string(),
        ];

        let a_options = vec![
            "1. Exponentiation".yellow().to_string(),
            "2. Square root".yellow().to_string(),
        ];
        
        // This vector variable is for next update (0.1.8)
        let _yn = vec![
            "Yes".green().to_string(),
            "No".green().to_string(),
        ];

        let action = Select::new("What would you like to do?", options)
            .prompt()
            .unwrap();
        
        /* Since rounding is "special" and doesn't explicitly need 2 numbers, 
           i'm gonna drag it to the top. */

        if action.contains("Rounding") {
            let round_num = CustomType::<f32>::new("Enter the number to round:")
                .prompt()
                .unwrap();
            println!("{} {}: {}", round_num
                .to_string()
                .cyan()
                .bold(), 
                "rounded is:"
                .cyan()
                .bold(), 
                round_num.round());
            continue;
        }
        if action.contains("advanced"){
            let a_action = Select::new("What would you like to do in advanced mode?", a_options)
                .prompt()
                .unwrap();
            if a_action.contains("Exponentiation"){
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
        } else {
            _num1 = CustomType::<f32>::new("Enter the first number:")
                .prompt()
                .unwrap();
            _num2 = CustomType::<f32>::new("Enter the second number:")
                .prompt()
                .unwrap();
        }

        if action.contains("Addition") {
            println!("{}: {}", "The answer is".cyan().bold(), _num1 + _num2);
        } else if action.contains("Subtraction") {
            println!("{}: {}", "The answer is".cyan().bold(), _num1 - _num2);
        } else if action.contains("Multiplication") {
            println!("{}: {}", "The answer is".cyan().bold(), _num1 * _num2);
        } else if action.contains("Division") {
            if _num2 == 0.0 {
                println!("{}", "You can't divide by zero!".bold().red());
            } else {
                println!("{}: {}", "The answer is".cyan().bold(), _num1 / _num2);
            }
        } else {
            unreachable!("You selected an option that isn't on the menu!");
        }
    }
}
