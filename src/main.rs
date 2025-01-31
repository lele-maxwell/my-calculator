
use std::fs::OpenOptions;
use std::io::Write;

fn main() {
    println!("A simple calculator to add, subtract, multiply, and divide two numbers");
    println!("operators: +, -, *, /, ^, %, !");
    println!("sample: num1 operator num2 ");
/* 
    let args: Vec<String> = env::args().collect();
    if args.len() != 4 {
        println!("Usage: <num1> <operator> <num2>");
        return;
    }

    let num1 = args[1].parse::<f64>().unwrap();
    let num2 = args[3].parse::<f64>().unwrap();
    let operator = args[2].chars().next().unwrap();
*/


let mut num1: String = String::new();
let mut num2:String= String::new();

println!("Enter the first number: ");
std::io::stdin().read_line(&mut num1).unwrap();
let num1: f64 = num1.trim().parse().unwrap();


let mut operator: String = String::new();
println!("Enter the operator:[+] [-] [*] [/] [!] [^] [%]");
std::io::stdin().read_line(&mut operator).unwrap();
let operator: char = operator.trim().chars().next().unwrap();


println!("Enter the second number: ");
std::io::stdin().read_line(&mut num2).unwrap();
let num2: f64 = num2.trim().parse().unwrap();




    let result = match operator {
        '+' => add(num1, num2),
        '-' => subtract(num1, num2),
        '*' => multiply(num1, num2),
        '/' => divide(num1, num2),
        '^' => pow(num1, num2),
        '%' => modulus(num1, num2),
        '!' => factorial(num1),
        _ => {
            println!("Invalid operator");
            return;
        }
    };

    println!("Result: {}", result);
    log(num1, &operator.to_string(), num2, result);
}

fn add(num1: f64, num2: f64) -> f64 {
    num1 + num2
}

fn subtract(num1: f64, num2: f64) -> f64 {
    num1 - num2
}

fn multiply(num1: f64, num2: f64) -> f64 {
    num1 * num2
}

fn divide(num1: f64, num2: f64) -> f64 {
    if num2 == 0.0 {
        println!("Error: Division by zero");
        
    }
    num1 / num2
}

fn pow(num1: f64, num2: f64) -> f64 {
    num1.powf(num2)
}

fn modulus(num1: f64, num2: f64) -> f64 {
    num1 % num2
}

fn factorial(num1: f64) -> f64 {
    if num1 == 0.0 {
        return 1.0;
    } else if num1 < 0.0 {
        return 0.0;
    } else if num1 == 1.0 {
        return 1.0;
    }
    num1 * factorial(num1 - 1.0)
    
}

fn log(num1: f64, operator: &str, num2: f64, result: f64) {
    let log_entry = format!("{} {} {} = {}\n", num1, operator.trim(), num2, result);
    let mut file = OpenOptions::new()
        .append(true)
        .create(true)
        .open("history.txt")
        .expect("Unable to open file");
    file.write_all(log_entry.as_bytes()).expect("Unable to write data");
}