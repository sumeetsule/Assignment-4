use clap::{Arg, Command};
mod tests;

fn add(a: f64, b: f64) -> f64 {
    a + b
}

fn sub(a: f64, b: f64) -> f64 {
    a - b
}

fn mul(a: f64, b: f64) -> f64 {
    a * b
}

fn div(a: f64, b: f64) -> Result<f64, &'static str> {
    if b == 0.0 {
        return Err("Second operand cannot be 0 for division");
    }
    Ok(a / b)
}

fn main() {
    let matches = Command::new("cli-calc")
        .version("0.0.1")
        .about("This is cli-calc with Rust which performs basic arithmetic operations \nThe developer is Sumeet Sule")
        .arg(
            Arg::new("operation")
                .short('o')
                .long("operation")
                .help("This calc can perform: add, sub, mul, div")
                .required(true),
        )
        .arg(
            Arg::new("operand1")
                .short('a')
                .long("operand1")
                .help("First Operand")
                .required(true),
        )
        .arg(
            Arg::new("operand2")
                .short('b')
                .long("operand2")
                .help("Second Operand")
                .required(true),
        )
        .get_matches();

    // let a: f64 = matches.get_one::<String>("operand1")
    //     .unwrap()
    //     .parse()
    //     .expect("Operand1 must be a number");

    // let b: f64 = matches.get_one::<String>("operand2")
    //     .unwrap()
    //     .parse()
    //     .expect("Operand2 must be a number");


    let a:f64 = match matches.get_one::<String>("operand1") {
        Some(op) => match op.parse() {
            Ok(val) => val,
            Err(_) => {
                eprintln!("Operand 1 must be a number");
                std::process::exit(1);
            }
        },
        None => {
            eprintln!("Operand 1 is required");
            std::process::exit(1);
        }
    };

    let b:f64 = match matches.get_one::<String>("operand2") {
        Some(op) => match op.parse() {
            Ok(val) => val,
            Err(_) => {
                eprintln!("Operand 2 must be a number");
                std::process::exit(1);
            }
        },
        None => {
            eprintln!("Operand 2 is required");
            std::process::exit(1);
        }
    };
    
    let operation = matches.get_one::<String>("operation")
        .expect("Operation is required");
    

        let result = match operation.as_str() {
            "add" | "+" => Ok(add(a, b)),
            "sub" | "-" => Ok(sub(a, b)),
            "mul" | "*" => Ok(mul(a, b)),
            "div" | "/" => div(a, b),
            _ => Err("Invalid operation: Try 'add' or '+', 'sub' or '-', 'mul' or '*','div' or '/' "),
        };
    
        match result {
            Ok(res) => println!("Result: {}", res),
            Err(e) => eprintln!("Error: {}", e),
        }
}

