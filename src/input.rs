use std::io::Write;

use inquire::*;

pub fn any_key(message: &str) {
    let _ = Text::new(message).prompt_skippable();
}

pub fn get_command(message: &str) -> String {
    let answer = Text::new(message).prompt();

    match answer {
        Ok(cmd) => {
            if cmd.len() > 3 {
                String::from(&cmd[..3])
            } else {
                cmd
            }
        }

        Err(_) => String::from(""),
    }
}

pub fn yesno(message: &str, default: bool) -> bool {
    let answer = Confirm::new(message)
        .with_default(default)
        //.with_help_message("This data is stored for good reasons")
        .prompt();

    match answer {
        Ok(v) => v,
        Err(_) => default,
    }
}

/* Input a value between 0.00 and 9.99 */
pub fn input_f32(message: &str, /*help_message: &str, */ min: f32, max: f32) -> f32 {
    let amount = CustomType::<f32>::new(message)
        .with_formatter(&|i| format!("{:.2}", i))
        .with_error_message("Please type a valid number")
        //.with_help_message(help_message)
        .prompt();

    match amount {
        Ok(v) => {
            if v < min {
                min
            } else if v > max {
                max
            } else {
                v
            }
        }
        Err(_) => min,
    }
}

/* Integer: unsigned, or returns -1 for blank/error */
pub fn input_i32(message: &str, /*help_message: &str,*/ min: i32, max: i32) -> i32 {
    //input(8).trim().parse().unwrap_or(-1)

    let amount = CustomType::<i32>::new(message)
        .with_formatter(&|i| format!("{:.2}", i))
        .with_error_message("Please type a valid number")
        //.with_help_message(help_message)
        .prompt();

    match amount {
        Ok(v) => {
            if v < min {
                min
            } else if v > max {
                max
            } else {
                v
            }
        }
        Err(_) => min,
    }
}
