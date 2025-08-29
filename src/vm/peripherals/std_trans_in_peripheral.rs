use crate::vm::machine_state::MachineState;
use crate::vm::peripheral::Peripheral;

const NAME: &str = "STDTRANS_IN";

#[derive(Default)]
pub struct StandardOutputTransmitterInPeripheral {}


impl Peripheral for StandardOutputTransmitterInPeripheral {
    fn call(&self, _: String, _: &mut MachineState) { /* pass */}

    fn late_call(&self, called_name: String, state: &mut MachineState) {
        if called_name != NAME { return }

        state.std_transmitter_contents = (state.main_bus & 0xFF) as u8;
    }
}