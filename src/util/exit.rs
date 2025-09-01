use colorize::AnsiColor;
use std::process;

pub fn exit(message: String, exit_code: ExitCode) {
    exit_with_variant(message, exit_code, 1);
}

pub fn exit_with_variant(message: String, exit_code: ExitCode, variant: u8) {
    let mut message = message.red();

    let mut variant = variant;


    // Check if the variant is acceptable, if not change it to one and attach a warning
    if variant < 1 || variant > 9 {
        let warning = format!("Program was supposed to exit with variant {} of the exit code, but this is out of range (1...9), choosing one instead.", variant).yellow();
        message += warning.as_str();
        variant = 1;
    }

    // Print the message (and the warning from the variant if it's been attached by the previous if statement
    eprintln!("{}", message);

    let exit_code_suffix = exit_code.get_code();            // This might represent 05 from 105 for bad code
    let exit_code_prefix = variant * 100;                   // This might represent  1 from 105, which is actually 100 decimal
    let exit_code = exit_code_prefix + exit_code_suffix;    // This might represent the entire 105 code

    process::exit(exit_code as i32);
}


pub enum ExitCode {
    BadArgument,                // A CLI argument is not as expected
    Internal                    // Internal malfunction with no further explanation
}

impl ExitCode {
    pub fn get_code(&self) -> u8 {
        match self {
            ExitCode::BadArgument => 0, // This will be formated as x00 where x is non-zero
            ExitCode::Internal => 99,
        }
    }
}