use crate::vm::machine_state::MachineState;
use crate::vm::peripheral::Peripheral;

const NAME: &str = "NAND_OUT";

#[derive(Default)]
pub struct NandPeripheral {}


impl Peripheral for NandPeripheral {
    fn call(&self, called_name: String, state: &mut MachineState) {
        if called_name != NAME { return }

        let result = (state.alu_arg_1 & state.alu_arg_2) ^ 0x7f_ff_ff_ffu32 as i32;

        state.push_to_main_bus(result as u32); // |= to simulate behaviour in case multiple
    }

    fn late_call(&self, _: String, _: &mut MachineState) { /* pass */ }
}