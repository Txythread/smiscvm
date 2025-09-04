# smiscvm - Stupid Mixed Instruction Set Computer Virtual Machine
A VM whose inner workings mimic the inner workings of a CPU.#


## Installation

1. Make sure you have [`smiscasm`](https://github.com/Txythread/smiscasm) installed
2. Download smiscvm using `git clone https://github.com/Txythread/smiscvm`
3. "cd" into the directory (`cd smiscasm`)
4. Execute the production scirpt (`./production.sh`)
5. This might ask you to enter your password to move files into /usr/local/bin.
If you don't want this, cancel the program and execute `sudo mv target/release/smiscvm /usr/local/bin` manually


## Basic Usage
First, assemble your code as documented in smiscasm's documentation. Then, invoke smiscvm with the name of the resulting .o file as an argument: `smiscvm my_file.o`.

## TUI
`smiscvm` shows information about the state of the machine while running. It first displays all regular registers (0-31), the memory address register (*"ma"*), the current instruction register (*"in"*) and the program counter (*"pc"*).  
It then shows the flags (zero flag (*"ZF"*) and privilegded mode (*"PM"*) and the ALU's resgisters. This is then followed by the clock (up or down), the micro-op-counter (*"Step"*), the average IPC (*"øIPC") and the contents of the main bus (*"mBus"*). Additional stdout messages from the program are displayed below all of the previously mentioned information.

## Simulation speed
The (theoretical max.) clock speed can be set using the `-hz` (or `--hertz`) flag.

## Adding new micro operations
You can add a new micro operation by:
1. appending the name to the "OUTPUT_MAP_STRING" variable found in instruction.rs. **Place at the end of the list** or previously written test cases will fail. **Do this in both: *smiscasm* and *smiscvm*.**
2. Adding a "peripheral" that executes the micro operation to *src/vm/peripherals/*. Make sure to attach this file to src/vm/peripherals/mod.rs (don't forget "pub").
3. Editing *src/vm/machine.rs::set_up_peripherals* to set up the peripheral whenever this functions is being executed.
4. Recompiling smiscasm and smiscvm (follow order).
