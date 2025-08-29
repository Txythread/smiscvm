use crate::vm::machine_state::MachineState;
use crate::vm::peripheral::Peripheral;

const NAME: &str = "STDTRANS_SEND";

#[derive(Default)]
pub struct StandardOutputTransmitterSendPeripheral {}


impl Peripheral for StandardOutputTransmitterSendPeripheral {
    fn call(&self, called_name: String, state: &mut MachineState) {
        if called_name != NAME { return }

        state.stdout.push(state.std_transmitter_contents as char);

    }

    fn late_call(&self, _: String, _: &mut MachineState) { /* pass */ }
}