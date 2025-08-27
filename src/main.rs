use std::env;
use std::fmt::{Debug, Formatter};
use std::process::exit;
use std::thread::sleep;
use std::fs;
use std::path::PathBuf;
use std::time::Duration;
use colorize;
use colorize::AnsiColor;
use crossterm::execute;
use crossterm::terminal::{EnterAlternateScreen, LeaveAlternateScreen};
use crossterm::cursor::{Hide, Show};
use crate::vm::machine::Machine;
use crate::help::help::print_help;

mod instruction;
mod vm;
mod help;


pub struct ArgumentList{
    pub file: Option<String>,
    pub help: bool,                             // -h or --help
    pub hertz: Option<u16>,                   // -hz or --hertz
}

impl ArgumentList{
    pub fn new() -> ArgumentList{
        ArgumentList{file: None, help: false, hertz: None}
    }

    /// Checks whether the current amount of data is enough (0) or the file name is missing (1)
    pub fn needs_input_file(&self) -> bool{
        let is_ok = self.help || self.file.is_some();
        !is_ok
    }
}

impl Debug for ArgumentList{
    fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
        f.debug_struct("ArgumentList")
            .field("file", &self.file)
            .field("help", &self.help)
            .field("hertz", &self.hertz)
            .finish()
    }
}


impl PartialEq for ArgumentList{
    fn eq(&self, other: &Self) -> bool{
        self.file == other.file && self.help == other.help && self.hertz == other.hertz
    }
}


fn main() {
    // Retrieve arguments from the terminal first
    let cli_args: Vec<String> = env::args().collect();

    // Generate a reasonable argument list
    let args = get_arguments_from_list(cli_args);

    if args.help {  print_help(args); return;  }

    // Load the file
    let path = expand_path(&args.file.clone().unwrap()).unwrap();
    let input_file = fs::read(path.clone());


    if input_file.is_err() {
        let msg = format!("Input file not found: {}", path.to_str().unwrap().to_string()).red().to_string();
        eprintln!("{}", msg);
        exit(100);
    }

    let input_file = input_file.unwrap();

    // Check the file is in the same dir

    // The amount of dirs between PWD and file
    let amount_of_subdirs = path.to_str().unwrap().split('/').count() - 1;

    if amount_of_subdirs > 0 {
        let msg = format!("Input file must be directly beneath the working directory, but there are {} directories in between.", amount_of_subdirs).red().to_string();
        eprintln!("{}", msg);
        exit(100);
    }


    let mut machine = Machine::new();

    // Initialize peripherals
    machine.set_up_peripherals();

    // Load the program
    machine.state.push_to_memory(input_file, 0);

    // Set the instruction to 'add x0, 0', which should do nothing but load the next instruction in practice.
    machine.state.current_instruction = 0x50_00_00_00;

    execute!(std::io::stdout(), EnterAlternateScreen).unwrap();

    execute!(std::io::stdout(), Hide).unwrap();
    loop {
        machine.simulate_clock_pulse();

        machine.state.print(false);

        if let Some(hertz) = args.hertz {
            sleep(Duration::from_secs_f32(0.5f32 / (hertz as f32)));
        }

        machine.state.print(true);

        if let Some(hertz) = args.hertz {
            sleep(Duration::from_secs_f32(0.5f32 / (hertz as f32)));
        }

        if machine.state.current_instruction == 0 && machine.state.program_counter != 0{
            break;
        }
    }

    execute!(std::io::stdout(), LeaveAlternateScreen, Show).unwrap();

}


pub fn expand_path(path_str: &str) -> Option<PathBuf> {
    let expanded = if path_str.starts_with("~/") {
        let home = env::var("HOME").ok()?;
        PathBuf::from(home).join(&path_str[2..])
    } else if path_str.starts_with("$PWD/") {
        let pwd = env::var("PWD").ok()?;
        PathBuf::from(pwd).join(&path_str[5..])
    } else {
        PathBuf::from(path_str)
    };

    Some(expanded)
}

fn get_arguments_from_list(args: Vec<String>) -> ArgumentList {
    // Remove the first argument as it's just the name of the bin
    let mut args = args;
    args.remove(0);

    // Make space for the result
    let mut result = ArgumentList::new();

    // Sort the arguments
    // The first out-of-context (not belonging or being connected to a flag (-)) is the input file
    let mut current_flag: Option<String> = None;

    for arg in args {
        if let Some(arg_first_char) = arg.chars().nth(0){
            // Check if this argument is necessary for the last flag
            if let Some(flag) = current_flag.clone(){
                let value = arg.clone();

                // Add it if it is not a call for help
                if value == "--help" || value == "-h" {
                    result.help = true;
                }

                match flag.as_str() {
                    "-hz" | "--hertz" => {
                        if let Some(value) = value.parse::<u16>().ok() {
                            result.hertz = Some(value);
                        }else{
                            let msg = format!("Positive integer (lower than 2^16) expected as a value for hertz, but {} was found.", value).red();
                            eprintln!("{}", msg);
                            exit(100);
                        }
                    }

                    _=>{
                        let error = format!("Unknown flag {}.", flag).red().to_string();
                        eprintln!("{}", error);
                        exit(100)
                    }
                }

                current_flag = None;
                continue;
            }

            // Check if the argument is a flag
            if arg_first_char == '-' {
                // This is a flag
                // Therefore, look if the next argument also needs to be checked or the argument can be added right away

                match arg.as_str() {
                    "-h" | "--help" => {
                        result.help = true;
                    }


                    _=>{
                        current_flag = Some(arg);
                    }
                }
                continue;
            }

            // The argument is not a flag, nor is it used after a flag, ...
            // ... so it has to be the name of the file
            if result.file.is_some(){
                let error = format!("\"{}\" and \"{:?}\" can't both be input files.", result.file.clone().unwrap(), arg).red().to_string();
                eprintln!("{}", error);
                exit(100);
            }

            // Isn't yet written, so add the file name
            result.file = Some(arg);
        }
    }

    if current_flag.is_some() && !result.help {
        let error = "All flags that act like parameters must have their second part provided.".red().to_string();
        eprintln!("{}", error);
        exit(100);
    }

    if current_flag.is_some() {
        // Set the argument anyway as help is requested.
        match current_flag.clone().unwrap().as_str() {
            "-hz" | "--hertz" => {
                result.hertz = Some(0);
            }

            _=>{
                let error = format!("Unknown flag {}.", current_flag.unwrap()).red().to_string();
                eprintln!("{}", error);
                exit(100)
            }
        }
    }

    if result.needs_input_file(){
        let error = "No input files provided.".red().to_string();
        eprintln!("{}", error);
        exit(100);
    }

    result
}