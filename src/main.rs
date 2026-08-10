use colored::*;
use inquire::*;

fn main() {
    loop {
        let options = vec![
            "1. Addition".green().to_string(),
            "2. Subtraction".green().to_string(),
            "3. Multiplication".green().to_string(),
            "4. Division".green().to_string(),
            "5. Exit".red().to_string(),
        ];

        let action = Select::new("What would you like to do?", options)
            .prompt()
            .unwrap();

        if action.contains("Exit") {
            println!("Bye!");
            break;
        }
        
        let num1 = CustomType::<f32>::new("Enter the first number: ")
            .prompt()
            .unwrap();
        let num2 = CustomType::<f32>::new("Enter the second number: ")
            .prompt()
            .unwrap();

        if action.contains("Addition") {
            println!("{}: {}", "The answer is".cyan().bold(), num1 + num2);
        } else if action.contains("Subtraction") {
            println!("{}: {}", "The answer is".cyan().bold(), num1 - num2);
        } else if action.contains("Multiplication") {
            println!("{}: {}", "The answer is".cyan().bold(), num1 * num2);
        } else if action.contains("Division") {
            println!("{}: {}", "The answer is".cyan().bold(), num1 / num2);
        } else {
            println!("What? How did that happen?");
        }
    }
}
