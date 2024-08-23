use std::io;

fn main() {
    loop {
        println!("Enter temperature in Fahrenheit (or type 'exit' to quit): ");
        let mut temp_in_f: String = String::new();
        io::stdin()
            .read_line(&mut temp_in_f)
            .expect("Failed to read line");

        // Exit condition
        if temp_in_f.trim().eq_ignore_ascii_case("exit") {
            break;
        }

        // Parse input and handle errors
        match temp_in_f.trim().parse::<f64>() {
            Ok(temp_in_f) => {
                let temp_in_c: f64 = fahrenheit_to_celsius(temp_in_f);
                // :.2 chooses decimal places to display result to
                println!("{:.2} °F is {:.2} °C", temp_in_f, temp_in_c);
            }
            Err(_) => {
                println!("Invalid input. Please enter a valid number.");
            }
        }
    }
}

// Function to convert Fahrenheit to Celsius
fn fahrenheit_to_celsius(fahrenheit: f64) -> f64 {
    (fahrenheit - 32.0) * 5.0 / 9.0
}

// Defines a config to run unit tests
#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_conversion() {
        assert_eq!(fahrenheit_to_celsius(32.0), 0.0);
        assert_eq!(fahrenheit_to_celsius(100.0), 37.77777777777778);
        assert_eq!(fahrenheit_to_celsius(-40.0), -40.0);
        assert_eq!(fahrenheit_to_celsius(212.0), 100.0);
        assert_eq!(fahrenheit_to_celsius(0.0), -17.77777777777778);
    }
}
