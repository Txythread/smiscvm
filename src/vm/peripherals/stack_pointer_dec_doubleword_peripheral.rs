use crate::vm::machine_state::MachineState;
use crate::vm::peripheral::Peripheral;

const NAME: &str = "SP_DEC_DW";

#[derive(Default)]
pub struct StackPointerDecrementDoubleWordPeripheral {}

impl Peripheral for StackPointerDecrementDoubleWordPeripheral {
    fn call(&self, called_name: String, state: &mut MachineState ) {
        if called_name != NAME { return }

        // Decrement the stack pointer (register #31) by 4 bytes (one quadruple word).
        state.registers[31] -= 4;
    }
    fn late_call(&self, _: String, _: &mut MachineState) { /* pass */ }

}