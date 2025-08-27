use crate::vm::machine_state::MachineState;
use crate::vm::peripheral::Peripheral;

const NAME: &str = "ZF_OUT";

#[derive(Default)]
pub struct ZerFlagOutPeripheral {}

impl Peripheral for ZerFlagOutPeripheral {
    fn call(&self, called_name: String, state: &mut MachineState) {
        if called_name != NAME { return }

        state.push_to_main_bus(state.flag1 as u32);
    }

    fn late_call(&self, _: String, _: &mut MachineState) { /* pass */ }
}