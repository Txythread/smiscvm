const PROGRAM: &'static [u8] = include_bytes!("../../../test_code/general-test/test.o");

#[cfg(test)]
mod tests {
    use crate::vm::machine::Machine;
    use crate::vm::test::general_test::PROGRAM;

    #[test]
    fn general_test() {
        // Perform a test where most instructions are tested.
        // The program is an assembled version of test_code/general_test/test.s
        // The program should store 8 at memory address 0x3000 if all instructions work as expected

        let mut machine = Machine::new();
        machine.set_up_peripherals();

        machine.state.push_to_memory(PROGRAM.to_vec(), 0x00_00_00_00);

        println!("Program 0...100: {:?}", machine.state.memory);

        // For the machine to start executing, an instruction needs to be loaded first
        // In this case: add x0, 0
        // This won't actually be executed tho; it just loads the next instruction
        machine.state.current_instruction = 0x50_00_00_00;

        // The machine gets 10k clock cycles to halt.
        for _ in 0..10_000{
            _ = machine.simulate_clock_pulse();

            if machine.state.halted {
                // The machine won't do anything anyway, so just exit the loop early
                break;
            }
        }


        // Get memory address 0x3000, where the program's output should be stored
        let result = machine.state.memory[0x3000];

        // Check if the result is 8 as it should be
        assert_eq!(result, 8);
    }
}