use std::process::Command;
use crate::vm::machine_state::MachineState;
use crate::vm::peripheral::Peripheral;

const NAME: &str = "STDTRANS_IN";

#[derive(Default)]
pub struct StandardOutputTransmitterInPeripheral {}


impl Peripheral for StandardOutputTransmitterInPeripheral {
    fn call(&self, _: String, _: &mut MachineState) { /* pass */}

    fn late_call(&self, called_name: String, state: &mut MachineState) {
        if called_name != NAME { return }

        // Look up which encoding to use
        if state.config.legacy_encoding{
            // Use legacy (ASCII) encoding.
            state.std_transmitter_contents = (state.main_bus & 0xFF) as u8;
        }else{
            // Use new (STC) encoding
            // Convert using smisc-connect

            let stc_input_value = (state.main_bus & 0xFF).to_string();
            let command_result = Command::new("smisc-connect")
                .arg("--stc-to-string")
                .arg(stc_input_value.clone())
                .output();

            if let Some(result) = command_result.ok() {
                let mut result = result.stdout;

                if result.len() > 1 {
                    // Remove the last char as it's a newline
                    result.remove(result.len() - 1);

                    // Get the first (and only) character
                    let character = result[0];

                    state.std_transmitter_contents = character;
                }
            }else {
                state.std_transmitter_contents = ' ' as u8;

                // Notify the user.
                state.stdout = "Could not use smisc-connect for conversion. Use legacy mode or make sure smisc-connect is installed.".to_string();
            }


        }

    }
}