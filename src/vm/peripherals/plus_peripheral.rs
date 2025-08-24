use crate::vm::machine_state::MachineState;
use crate::vm::peripheral::Peripheral;

const NAME: &str = "PLUS_OUT";

#[derive(Default)]
pub struct PlusPeripheral {}


impl Peripheral for PlusPeripheral {
    fn call(&self, called_name: String, state: &mut MachineState) {
        if called_name != NAME { return }

        let result = state.alu_arg_1 + state.alu_arg_2;

        state.push_to_main_bus(result as u32); // |= to simulate behaviour in case multiple
    }

    fn late_call(&self, _: String, _: &mut MachineState) { /* pass */ }
}