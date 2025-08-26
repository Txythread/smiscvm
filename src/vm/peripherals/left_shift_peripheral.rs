use crate::vm::machine_state::MachineState;
use crate::vm::peripheral::Peripheral;

const NAME: &str = "SUB_OUT";

#[derive(Default)]
pub struct SubPeripheral {}

impl Peripheral for SubPeripheral {
    fn call(&self, called_name: String, state: &mut MachineState) {
        if called_name != NAME { return }

        let result = state.alu_arg_1 - state.alu_arg_2;

        state.push_to_main_bus(result as u32);
    }

    fn late_call(&self, _: String, _: &mut MachineState) { /* pass */ }
}