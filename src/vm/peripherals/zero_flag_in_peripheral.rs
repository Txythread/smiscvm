use crate::vm::machine_state::MachineState;
use crate::vm::peripheral::Peripheral;

const NAME: &str = "IMMEDIATE_OUT";

#[derive(Default)]
pub struct ImmediateOutPeripheral {}

impl Peripheral for ImmediateOutPeripheral {
    fn call(&self, called_name: String, state: &mut MachineState) {
        if called_name != NAME { return }

        // The value as it's encoded in the instruction, with the first bit being the sign and all other bits being the value
        let encoded_value = state.current_instruction & 0x00_00_1F_FF;

        let sign =                      encoded_value & 0x00_00_10_00 > 0;
        let value_without_sign =        encoded_value & 0x00_00_0F_FF;

        let mut value = value_without_sign;

        if sign {
            // Turn the value negative
            value =                                  value | 0xFF_FF_F0_00;
        }

        state.push_to_main_bus(value);
    }

    fn late_call(&self, _: String, _: &mut MachineState) { /* pass */ }
}