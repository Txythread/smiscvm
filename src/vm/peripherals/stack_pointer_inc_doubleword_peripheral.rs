use crate::vm::machine_state::MachineState;
use crate::vm::peripheral::Peripheral;

const NAME: &str = "SP_INC_DW";

#[derive(Default)]
pub struct StackPointerIncrementDoubleWordPeripheral {}

impl Peripheral for StackPointerIncrementDoubleWordPeripheral {
    fn call(&self, called_name: String, state: &mut MachineState ) {
        if called_name != NAME { return }

        // Increment the stack pointer (register #31) by 4 bytes (one quadruple word).
        state.registers[31] += 4;

    }
    fn late_call(&self, called_name: String, state: &mut MachineState) { /* pass */ }

}