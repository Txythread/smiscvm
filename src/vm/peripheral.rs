use crate::vm::machine_state::MachineState;

pub trait Peripheral {
    fn call(&self, called_name: String, state: &mut MachineState);
    fn late_call(&self, called_name: String, state: &mut MachineState);
}