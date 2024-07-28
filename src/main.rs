use std::{ io::{ self, stdin, Write }, process };

fn get_command() -> Result<u8, String> {
    let mut option: String = String::new();

    println!("Enter command 1|2|3 :");
    print!("> ");
    io::stdout().lock().flush().unwrap();
    stdin().read_line(&mut option).expect("can't get value");

    let option: u8 = match option.trim().parse() {
        Ok(num) => num,
        Err(_) => {
            return Err("Invalid input ".to_string());
        }
    };

    let option = match option {
        1 | 2 | 3 => option,
        _ => {
            return Err("Invalid input".to_string());
        }
    };
    Ok(option)
}

fn main() {
    print!("\x1b[2J\x1b[1;1H");
    println!("Welcome to Temperature Converter cli");

    println!("1. Fahrenheit to Celsius");
    println!("2. Celsius to Fahrenheit");
    println!("3. Exit");

    loop {
        match get_command() {
            Ok(option) => {
                handle_command(option);
                // break;
            }
            Err(err) => println!("\x1b[31mError : {err}\x1b[0m\n"),
        }
    }
}

fn handle_command(command: u8) {
    if command == 3 {
        println!("Exiting... good luck\n");
        process::exit(0);
    }

    loop {
        match get_number() {
            Ok(number) => {
                let (output, init_unit, out_unit) = match command {
                    1 => ((number - 32.0) * (5.0 / 9.0), "Fahrenheit", "Celsius"),
                    2 => (number * (5.0 / 9.0) + 32.0, "Celsius", "Fahrenheit"),
                    _ => (1.0, "", ""),
                };

                println!("\x1b[35m{number} {init_unit} is a {output:.3} {out_unit} !\x1b[0m\n");
                break;
            }
            Err(err) => println!("\x1b[31mError : {err}\x1b[0m\n"),
        }
    }
}

fn get_number() -> Result<f64, String> {
    let mut number: String = String::new();

    println!("Please enter a number:");
    print!("> ");
    io::stdout().lock().flush().unwrap();

    stdin().read_line(&mut number).expect("can't get value");

    let number: f64 = match number.trim().parse() {
        Ok(num) => num,
        Err(_) => {
            return Err("Invalid input only number valid ".to_string());
        }
    };

    let number: f64 = number.into();

    Ok(number)
}
