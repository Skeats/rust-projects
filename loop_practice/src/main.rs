use std::io;

fn main() {
    println!("---- Kiki's Loop Practice ----");

    loop {
        println!("Please enter a number corresponding to the elements below:");
        println!("(1) Convert Farenheit to Celsius");
        println!("(2) Convert Celsius to Farenheit");
        println!("(3) Generate Fibonacci numbers");

        let mut response = String::new();

        io::stdin()
            .read_line(&mut response)
            .expect("Failed to read line");

        let response: u32 = match response.trim().parse() {
            Ok(num) => num,
            Err(_) => {
                println!("Please enter a number");
                continue;
            }
        };

        match response {
            1 => convert_to_celsius(),
            2 => convert_to_farenheit(),
            3 => generate_fibonacci(),
            _ => {
                println!("Please enter a number between 1 and 3");
                continue;
            }
        }

        return;
    }
}

fn convert_to_celsius() {
    loop {
        println!("Please enter a Farenheit value:");
        let mut farenheit = String::new();

        io::stdin()
            .read_line(&mut farenheit)
            .expect("Failed to read line");

        let farenheit: f64 = match farenheit.trim().parse() {
            Ok(num) => num,
            Err(_) => {
                println!("Please enter a number");
                continue;
            }
        };

        let celsius = (farenheit - 32.0) * (5.0 / 9.0);
        println!("{farenheit}F in Celsius is {celsius:.2}C");
        return;
    }
}

fn convert_to_farenheit() {
    loop {
        println!("Please enter a Celsius value:");
        let mut celsius = String::new();

        io::stdin()
            .read_line(&mut celsius)
            .expect("Failed to read line");

        let celsius: f64 = match celsius.trim().parse() {
            Ok(num) => num,
            Err(_) => {
                println!("Please enter a number");
                continue;
            }
        };

        let farenheit = (celsius * (9.0 / 5.0)) + 32.0;
        println!("{celsius}C in Farenheit is {farenheit:.2}F");
        return;
    }
}

fn generate_fibonacci() {
    loop {
        println!("How many fibonacci numbers do you want to generate?");
        let mut fib_count = String::new();

        io::stdin()
            .read_line(&mut fib_count)
            .expect("Failed to read line");

        let fib_count: usize = match fib_count.trim().parse() {
            Ok(num) => {
                if num == 0 {
                    println!("Please enter a number greater than 0");
                    continue;
                } else if num > 186 {
                    println!("Please enter a number less than 186");
                    continue;
                } else {
                    num
                }
            }
            Err(_) => {
                println!("Please enter a number");
                continue;
            }
        };

        let mut numbers: Vec<u128> = Vec::new();
        numbers.push(0);
        numbers.push(1);

        // Handle cases where fib_count < 3
        if fib_count == 1 {
            println!("Fibonacci numbers: {}", numbers[0]);
            return;
        }

        if fib_count == 2 {
            println!("Fibonacci numbers: {}, {}", numbers[0], numbers[1]);
            return;
        }

        for i in 2..fib_count {
            numbers.push(numbers[i - 1] + numbers[i - 2]);
        }

        print!("Fibonacci numbers: ");

        for i in 0..numbers.len() {
            if i == numbers.len() - 1 {
                println!("{}", numbers[i]);
            } else {
                print!("{}, ", numbers[i]);
            }
        }

        return;
    }
}
