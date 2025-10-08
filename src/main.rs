use clap::Parser;
use std::env;
use std::fmt::Debug;
use std::thread::sleep;
use std::time::Duration;
use std::fs;
use std::path::PathBuf;
use crossterm::execute;
use crossterm::terminal::{EnterAlternateScreen, LeaveAlternateScreen};
use crossterm::cursor::{Hide, Show};
use ctrlc;
use crate::vm::machine::Machine;
use crate::vm::machine_state::ScreenPrintingInfo;
use crate::help::help::print_help;
use crate::util::exit::{exit, ExitCode};

mod instruction;
mod vm;
mod help;
mod util;


#[derive(Debug, PartialEq, Clone, Parser)]
pub struct ArgumentList{
    pub file: Option<String>,

    #[clap(short, long)]
    pub help: bool,                             // -h or --help
                
    #[clap(long)]
    pub hertz: Option<Option<u16>>,             // -hz or --hertz

    #[clap(long)]
    pub legacy_encoding: bool,                  // --legacy-encoding
}

impl ArgumentList{
    pub fn new() -> ArgumentList{
        ArgumentList{file: None, help: false, hertz: None, legacy_encoding: false}
    }

    /// Checks whether the current amount of data is enough (0) or the file name is missing (1)
    pub fn needs_input_file(&self) -> bool{
        let is_ok = self.help || self.file.is_some();
        !is_ok
    }
}

fn main() {
    // Generate a reasonable argument list
    let args = ArgumentList::parse();

    if args.help {  print_help(args); return;  }

    // Load the file
    let path = expand_path(&args.file.clone().unwrap()).unwrap();
    let input_file = fs::read(path.clone());


    if input_file.is_err() {
        exit(format!("Input file not found: {}", path.to_str().unwrap().to_string()), ExitCode::BadArgument);
    }


    let input_file = input_file.unwrap();

    // Check the file is in the same dir

    // The amount of dirs between PWD and file
    let amount_of_subdirs = path.to_str().unwrap().split('/').count() - 1;

    if amount_of_subdirs > 0 {
        exit(format!("Input file must be directly beneath the working directory, but there are {} directories in between.", amount_of_subdirs), ExitCode::BadArgument);
    }


    let mut machine = Machine::new(args.clone());

    // Initialize peripherals
    machine.set_up_peripherals();

    // Load the program
    machine.state.push_to_memory(input_file, 0);

    // Set the instruction to 'add x0, 0', which should do nothing but load the next instruction in practice.
    machine.state.current_instruction = 0x50_00_00_00;

    // Enter the alternate screen so the original screen contents won't be lost
    // P.S: the same for the cursor with Hide/Show
    execute!(std::io::stdout(), EnterAlternateScreen, Hide).unwrap();

    // Change Ctrl+C behaviour to return to the original screen before exiting
    ctrlc::set_handler(move || {
        execute!(std::io::stdout(), LeaveAlternateScreen, Show).unwrap();
        std::process::exit(0);
    })
        .expect("Error setting Ctrl-C handler");


    // The screen printing info required to print by machine.state.print()
    // This is for detecting when an entire redraw is required
    let mut screen_info: Option<ScreenPrintingInfo> = None;

    loop {
        machine.state.print(false, 0, &mut screen_info);

        let main_bus_contents = machine.simulate_clock_pulse();

        if let Some(hertz) = args.hertz {
            sleep(Duration::from_secs_f32(0.5f32 / (hertz.unwrap() as f32)));
        }

        machine.state.print(true, main_bus_contents, &mut screen_info);

        if let Some(hertz) = args.hertz {
            sleep(Duration::from_secs_f32(0.5f32 / (hertz.unwrap() as f32)));
        }
    }
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


