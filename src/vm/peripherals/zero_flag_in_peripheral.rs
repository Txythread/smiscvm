use crate::vm::machine_state::MachineState;
use crate::vm::peripheral::Peripheral;

const NAME: &str = "ZF_IN";

#[derive(Default)]
pub struct ZeroFlagInPeripheral {}

impl Peripheral for ZeroFlagInPeripheral {
    fn call(&self, _: String, _: &mut MachineState) { /* pass */ }

    fn late_call(&self, called_name: String, state: &mut MachineState) {
        if called_name != NAME { return }

        state.flag1 = state.main_bus == 0;
    }
}