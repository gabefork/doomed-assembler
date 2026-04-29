use std::{collections::HashMap, env, fs::{self, File}, io::{self, Write}};

//Special
pub const HALT: u32 = 0; //Done
//ALU - handled by execute stage and writeback
// RR = register + register
// RI = register + immediate
pub const ADD_RR: u32 = 1; //Done
pub const ADD_RI: u32 = 2; //Done
pub const SUB_RR: u32 = 3;
pub const SUB_RI: u32 = 4;
pub const MUL_RR: u32 = 5;
pub const MUL_RI: u32 = 6;
pub const DIV_RR: u32 = 7;
pub const DIV_RI: u32 = 8;
pub const MOD_RR: u32 = 9;
pub const MOD_RI: u32 = 10;
pub const AND_RR: u32 = 11;
pub const AND_RI: u32 = 12;
pub const OR_RR: u32 = 13;
pub const OR_RI: u32 = 14;
pub const NOT: u32 = 15;
pub const XOR_RR: u32 = 16;
pub const XOR_RI: u32 = 17;
pub const LS_RR: u32 = 18;
pub const LS_RI: u32 = 19;
pub const RS_RR: u32 = 20;
pub const RS_RI: u32 = 21;
pub const LSL_RR: u32 = 22;
pub const LSL_RI: u32 = 23;
pub const LSR_RR: u32 = 24;
pub const LSR_RI: u32 = 25;

//control flow - handled by excecute and writeback
//D = register direct
//I = register indirect 
//PC = PC relative
pub const JMP_D: u32 = 26; //Done
pub const JMP_I: u32 = 27;  //Done
pub const JMP_PC: u32 = 28;  //Done       //JMPS save lr for return
pub const CMP_RR: u32 = 29; //Done
pub const CMP_RI: u32 = 30;//Done
pub const JE_D: u32 = 31; //Done
pub const JE_I: u32 = 32; //Done
pub const JE_PC: u32 = 33; //Done
pub const JL_D: u32 = 34; //Done
pub const JL_I: u32 = 35; //Done
pub const JL_PC: u32 = 36; //Done
pub const JG_D: u32 = 37; //Done
pub const JG_I: u32 = 38; //Done
pub const JG_PC: u32 = 39; //Done
pub const RET: u32 = 40; //Done

//memory - handled by memory and writeback stages

pub const LDR_D: u32 = 41; //Done
pub const LDR_I: u32 = 42; //Done
pub const LDR_PC: u32 = 43; //Done
pub const STR_D: u32 = 44; //Done
pub const STR_I: u32 = 45; //Done
pub const STR_PC: u32 = 46; //Done
pub const GDR_D: u32 = 47; //Done
pub const GDR_I: u32 = 48; //Done
pub const GDR_PC: u32 = 49; //Done
pub const GTR_D: u32 = 50; //Done
pub const GTR_I: u32 = 51; //Done
pub const GTR_PC: u32 = 52; //Done
pub const PSH: u32 = 53; //Done
pub const POP: u32 = 55; //Done
pub const PSH_LR: u32 = 54;
pub const POP_LR: u32 = 56;
pub const FDR: u32 = 58; //frame load, pulls graphics memory into frame buffer //Done
pub const FTR: u32 = 59; //frame store, puts frame buffer into graphics memory //Done


fn ret_op (op: &str) -> u32 {
    match op {
        //Special
        "HALT" => 0,
        //ALU - handled by execute stage and writeback
        // RR = register + register
        // RI = register + immediate
        "ADD_RR" => 1,
        "ADD_RI" => 2,
        "SUB_RR" => 3,
        "SUB_RI" => 4,
        "MUL_RR" => 5,
        "MUL_RI" => 6,
        "DIV_RR" => 7,
        "DIV_RI" => 8,
        "MOD_RR" => 9,
        "MOD_RI" => 10,
        "AND_RR" => 11,
        "AND_RI" => 12,
        "OR_RR" => 13,
        "OR_RI" => 14,
        "NOT" => 15,
        "XOR_RR" => 16,
        "XOR_RI" => 17,
        "LS_RR" => 18,
        "LS_RI" => 19,
        "RS_RR" => 20,
        "RS_RI" => 21,
        "LSL_RR" => 22,
        "LSL_RI" => 23,
        "LSR_RR" => 24,
        "LSR_RI" => 25,

        //control flow - handled by excecute and writeback
        //D = register direct
        //I = register indirect 
        //PC = PC relative
        "JMP_D" => 26,
        "JMP_I" => 27,
        "JMP_PC" => 28,
        "CMP_RR" => 29,
        "CMP_RI" => 30,
        "JE_D" => 31,
        "JE_I" => 32,
        "JE_PC" => 33,
        "JL_D" => 34,
        "JL_I" => 35,
        "JL_PC" => 36,
        "JG_D" => 37,
        "JG_I" => 38,
        "JG_PC" => 39,
        "RET" => 40,

        //memory - handled by memory and writeback stages

        "LDR_I" => 42,
        "LDR_D" => 41,
        "LDR_PC" => 43,
        "STR_D" => 44,
        "STR_I" => 45,
        "STR_PC" => 46,
        "GDR_D" => 47,
        "GDR_I" => 48,
        "GDR_PC" => 49,
        "GTR_D" => 50,
        "GTR_I" => 51,
        "GTR_PC" => 52,
        "PSH" => 53,
        "POP" => 55,
        "PSH_LR" => 54,
        "POP_LR" => 56,
        "FDR" => 58, //frame load, pulls graphics memory into frame buffer
        "FTR" => 59, //frame store, puts frame buffer into graphics memory
        _ => 0xDEADBEEF
    }
}

fn ret_reg(register: &str) -> u32 {
    if register == "PC" {
        return 32;
    }
    if register == "SP" {
        return 33;
    }
    if register == "LR" {
        return 34;
    } else {
        let mut reg_str = register.to_string();
        if register.starts_with("%") {
            reg_str.remove(0);
        }
        reg_str.remove(0);
        return reg_str.parse().unwrap();
    }
}

const TYPE_SHIFT: u32 = 31;
const OPCODE_SHIFT: u32 = 25;
const REG1_SHIFT: u32 = 20;
const REG2_SHIFT: u32 = 15;
const REG3_SHIFT: u32 = 10;
const IMMEDIATE1_SHIFT: u32 = 13;
const IMMEDIATE2_SHIFT: u32 = 8;
const POST_REG_SHIFT: u32 = 3;
const IMMEDIATE3_SHIFT: u32 = 3;

#[derive(Eq, PartialEq, Copy, Clone, Debug)]
enum InstructionType {
        ALU,
        Memory,
        Control,
        Halt
}

fn instr_fields_to_decimal(type_field: u32, opcode: u32, arg1: u32, arg2: u32, arg3: u32, instr_type: InstructionType) -> u32{
    if instr_type == InstructionType::ALU {
        if opcode == ADD_RI  || opcode == SUB_RI || opcode == MUL_RI || opcode == DIV_RI || opcode == AND_RI
        || opcode == OR_RI || opcode == XOR_RI || opcode == MOD_RI || opcode == XOR_RI || opcode == LSL_RI
        || opcode == LSR_RI || opcode == LS_RI || opcode == RS_RI {
            return (type_field << TYPE_SHIFT)
                | (opcode << OPCODE_SHIFT)
                | (arg1 << REG1_SHIFT)
                | (arg2 << IMMEDIATE2_SHIFT)
                | (arg3 << POST_REG_SHIFT);
        }

        else {
            return (type_field << TYPE_SHIFT)
                | (opcode << OPCODE_SHIFT)
                | (arg1 << REG1_SHIFT)
                | (arg2 << REG2_SHIFT)
                | (arg3 << REG3_SHIFT);
        }
        
    }
    else if instr_type == InstructionType::Control {
        if opcode == CMP_RI  { //if CMP specifically
            
                return (type_field << TYPE_SHIFT)
                | (opcode << OPCODE_SHIFT)
                | (arg1 << REG1_SHIFT)
                | (arg2 << IMMEDIATE2_SHIFT)
                | (arg3);
        }  
        else if opcode == CMP_RR {
                return (type_field << TYPE_SHIFT)
                | (opcode << OPCODE_SHIFT)
                | (arg1 << REG1_SHIFT)
                | (arg2 << REG2_SHIFT)
                | (arg3 << REG3_SHIFT);
        }
        else if opcode == JMP_D || opcode == JL_D || opcode == JE_D || opcode == JG_D { //expect immediate address for direct jump
            return (type_field << TYPE_SHIFT)
                | (opcode << OPCODE_SHIFT)
                | (arg1 << IMMEDIATE1_SHIFT)
                | (arg2 << IMMEDIATE2_SHIFT) //will be
                | (arg3 << POST_REG_SHIFT);
        }
        else { //some register jump
            return (type_field << TYPE_SHIFT)
                | (opcode << OPCODE_SHIFT)
                | (arg1 << REG1_SHIFT)
                | (arg2 << REG2_SHIFT)
                | (arg3 << REG3_SHIFT);

        }
        }
    else if instr_type == InstructionType::Memory {
    return (type_field << TYPE_SHIFT)
            | (opcode << OPCODE_SHIFT)
            | (arg1 << REG1_SHIFT)
            | (arg2 << REG2_SHIFT)
            | (arg3 << IMMEDIATE3_SHIFT);

    }
    else {
        return 0;
    }
}

fn main() {
    let args: Vec<String> = env::args().collect(); 
    let path = args.get(1).expect("Format: doomed-assembler <file>");
    let assembly_string = fs::read_to_string(path).expect("File not found");
    let mut assembly: Vec<Vec<&str>> = Vec::new();
    let mut symbol_table: HashMap<&str, u32> = HashMap::new();
    let mut to_remove: Vec<usize> = Vec::new();
    let mut words: Vec<u32> = Vec::new();

    // Split lines in file
    for line in assembly_string.lines() {
        let split_line : Vec<&str> = line.split(" ").collect();
        assembly.push(split_line);
    }

    // Find symbols
    let mut cur_address: u32 = 0;
    for (i, line) in assembly.iter().enumerate() {
        if line.len() == 1 {
            if line[0].starts_with(".") {
                symbol_table.insert(line[0], cur_address);
                to_remove.push(i);
            }
        }
        cur_address += 1;
    }

    // Remove symbols from main assembly
    for &index in to_remove.iter().rev() {
        assembly.remove(index);
    }

    // Parse instructions
    for line in assembly {
        let mut opcode = line[0].to_string();
        if line[0] == "HALT" {
            let out = instr_fields_to_decimal(0, 0, 0, 0, 0, InstructionType::Halt);
            words.push(out);
        }
        // Check ALU operations
        else if line[0] == "ADD" || line[0] == "SUB" || line[0] == "MUL" || line[0] == "DIV" || line[0] == "MOD" || line[0] == "AND" || line[0] == "OR" || line[0] == "NOT" || line[0] == "XOR" || line[0] == "LS" || line[0] == "RS" || line[0] == "LSL" || line[0] == "LSR" || line[0] == "GDD" || line[0] == "GUB" || line[0] == "GUL" || line[0] == "GIV" {
            // Register-Register
            if line[2].starts_with("R") {
                opcode.push_str("_RR");
                let tf = (line[0] == "GDD" || line[0] == "GUB" || line[0] == "GUL" || line[0] == "GIV") as u32;
                let out = instr_fields_to_decimal(tf, ret_op(&opcode), ret_reg(line[1]), ret_reg(line[2]), ret_reg(line[3]), InstructionType::ALU);
                words.push(out);
            } else {
                // Register-Immediate
                opcode.push_str("_RI");
                let tf = (line[0] == "GDD" || line[0] == "GUB" || line[0] == "GUL" || line[0] == "GIV") as u32;
                let out = instr_fields_to_decimal(tf, ret_op(&opcode), ret_reg(line[1]), line[2].parse().unwrap(), ret_reg(line[3]), InstructionType::ALU);
                words.push(out);
            }
        }
        // Check control operations
        else if line[0] == "JMP" || line[0] == "JE" || line[0] == "JL" || line[0] == "JG" {
            if line[1].starts_with("R") {
                // Register direct
                let tf = (line.len() >= 3) as u32;
                let offset: u32;
                if tf == 1 {
                    offset = line[3].parse().unwrap();
                } else {
                    offset = 0;
                }
                if line[1] == "PC" {
                    opcode.push_str("_PC");
                    let out = instr_fields_to_decimal(tf, ret_op(&opcode), 0, offset, 0, InstructionType::Control);
                    words.push(out);
                }
                opcode.push_str("_I");
                let out = instr_fields_to_decimal(tf, ret_op(&opcode), ret_reg(line[1]), offset, 0, InstructionType::Control);
                words.push(out);
            } else {
                // Immediate
                opcode.push_str("_D");
                let out = instr_fields_to_decimal(0, ret_op(&opcode), symbol_table[line[1]], 0, 0, InstructionType::Control);
                words.push(out);
            }
        }
        else if line[0] == "CMP" {
            if line[2].starts_with("R") {
                opcode.push_str("_RR");
                let out = instr_fields_to_decimal(0, ret_reg(&opcode), ret_reg(line[1]), ret_reg(line[2]), 0, InstructionType::Control);
                words.push(out);
            } else {
                opcode.push_str("_RI");
                let out = instr_fields_to_decimal(0, ret_reg(&opcode), ret_reg(line[1]), line[2].parse().unwrap(), 0, InstructionType::Control);
                words.push(out);
            }
        }
        else if line[0] == "RET" {
            words.push(instr_fields_to_decimal(0, RET, 0, 0, 0, InstructionType::Control));
        }
        // Check memory operations
        else if line[0] == "LDR" || line[0] == "STR" || line[0] == "GDR" || line[0] == "GTR" {
            let tf = (line.len() > 3) as u32;
            let mut offset: u32 = 0;
            if tf == 1 {offset = line[3].parse().unwrap();}
            if line[1] == "PC" {
                opcode.push_str("_PC");
                let out = instr_fields_to_decimal(tf, ret_op(&opcode), 0, ret_reg(line[2]), offset, InstructionType::Memory);
                words.push(out);
            } else if line[1].starts_with("%") {
                opcode.push_str("_I");
                let out = instr_fields_to_decimal(tf, ret_op(&opcode), ret_reg(line[1]), ret_reg(line[2]), offset, InstructionType::Memory);
                words.push(out);
            }
            else {
                opcode.push_str("_D");
                let out = instr_fields_to_decimal(tf, ret_op(&opcode), ret_reg(line[1]), ret_reg(line[2]), offset, InstructionType::Memory);
                words.push(out);
            }
        }
        else if line[0] == "PUSH" || line[0] == "POP" {
            if line[2] == "LR" {
                opcode.push_str("_LR");
                let out = instr_fields_to_decimal(0, ret_op(&opcode), ret_reg(line[1]), 0, 0, InstructionType::Memory);
                words.push(out);
            } else {
                let out = instr_fields_to_decimal(0, ret_op(&opcode), ret_reg(line[1]), ret_reg(line[2]), 0, InstructionType::Memory);
                words.push(out);
            }
        }
        else if line[0] == "FDR" || line[0] == "FTR"{
            words.push(instr_fields_to_decimal(0, ret_op(line[0]), 0, 0, 0, InstructionType::Control));
        }
    }

    // Save to file
    let mut save_path = path.to_owned();
    save_path.push_str(".bin");
    let file: std::result::Result<File, io::Error> = File::create(save_path);
    match file {
        Result::Ok(mut f) => {
            for word in words {
                let _ = f.write_all(&word.to_le_bytes());
            }
        }    
        Result::Err(_x) => {
            return;
        }
    }    
}
