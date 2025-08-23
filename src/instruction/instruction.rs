use std::string::ToString;

const OUTPUT_MAP_STRING: [&str; 20] = ["PC_OUT", "PC_IN", "PC_INC", "MEM_ADDR_PTR_IN", "ALU_IN_A", "ALU_IN_B", "CAL_REG_A_IN", "CAL_REG_B_IN", "CAL_REG_A_OUT", "CAL_REG_B_OUT", "IMMEDIATE_OUT", "INSTR_IN", "MEM_OUT", "PLUS_OUT", "RESET_MICRO", "STDTRANS_IN", "STDTRANS_OUT", "STDTRANS_SEND", "ZF_IN", "ZF_OUT"]; // The left-most string in the list will end up in the LSb of the control 'word'

#[derive(Debug)]
pub struct Instruction {
    pub name: String,
    pub format: Vec<bool>,              // A 0 is a register, a 1 an immediate value
    pub op_code: u16,                   // This is only 8 bits rn, but the encoding would allow expanding to up to 9 bits
    pub stages: Vec<(u16, u64)>
}

impl Instruction {
    pub fn new(name: String, format: Vec<bool>, stages: Vec<(u16, u64)>) -> Instruction {
        Instruction { name, format, op_code: 0, stages }
    }

    pub fn new_with_op(name: String, format: Vec<bool>, op_code: u16, stages: Vec<(u16, u64)>) -> Instruction {
        Instruction { name, format, op_code, stages }
    }
}

pub fn micro_operation_at(idx: usize) -> String {
    if idx >= OUTPUT_MAP_STRING.len() {
        return "OUT OF RANGE".to_string();
    }
    OUTPUT_MAP_STRING[idx].to_string()
}


impl Clone for Instruction {
    fn clone(&self) -> Instruction {
        Instruction::new_with_op(self.name.clone(), self.format.clone(), self.op_code, self.stages.clone())
    }
}
