/* CPU object struct. */
pub struct cpu {
    program_counter: u8,
    stack_pointer: u8,
    instruction_pointer: u8,
    accumulator: u8,
    register1: u8,
    register2: u8,
    register3: u8,
    register4: u8,
    sign_flag: bool,
    zero_flag: bool,
    parity_flag: bool,
    carry_flag: bool,
    auxiliary_carry: bool,
}

pub enum instructions {
    NOP,
    MOV,
    ADD,
    SUB,
    JMP
}

/* CPU implementation. */
impl cpu {
    /* Constructs a new CPU struct with values. */
    pub fn new(pc: u8, sp: u8, ip: u8, ac: u8, r1: u8, r2: u8, r3: u8, r4: u8, sf: bool, zf: bool, pf: bool, cf: bool, acf: bool) -> cpu {
        cpu {
            program_counter: pc,
            stack_pointer: sp,
            instruction_pointer: ip,
            accumulator: ac,
            register1: r1,
            register2: r2,
            register3: r3,
            register4: r4,
            sign_flag: sf,
            zero_flag: zf,
            parity_flag: pf,
            carry_flag: cf,
            auxiliary_carry: acf,
        }
    }

    /* Resets the CPU values to 0 i.e. the default state. */
    pub fn reset_cpu(&mut self) {
        self.program_counter = 0;
        self.stack_pointer = 0;
        self.instruction_pointer = 0;
        self.accumulator = 0;
        self.register1 = 0;
        self.register2 = 0;
        self.register3 = 0;
        self.register4 = 0;
        self.sign_flag = false;
        self.zero_flag = false;
        self.parity_flag = false;
        self.carry_flag = false;
        self.auxiliary_carry = false;
    }

    /* Increments the program counter. */
    pub fn increment_pc(&mut self) {
        self.program_counter += 1;
    }

    /* Increments the stack pointer. */
    pub fn increment_sp(&mut self) {
        self.stack_pointer += 1;
    }

    /* Increments the instruction pointer. */
    pub fn increment_ip(&mut self) {
        self.instruction_pointer += 1;
    }

    /* Increments the accumulator. */
    pub fn increment_ac(&mut self) {
        self.accumulator += 1;
    }

    /* Sets the sign flag. */
    pub fn set_sign_flag(&mut self) {
        if (self.carry_flag == false) {
            self.carry_flag = true;
        } else {
            self.carry_flag = false;
        }
    }

    /* Sets the zero flag. */
    pub fn set_zero_flag(&mut self) {
        if (self.zero_flag == false) {
            self.zero_flag = true;
        } else {
            self.zero_flag = false;
        }
    }

    /* Sets the parity flag. */
    pub fn set_parity_flag(&mut self) {
        if (self.parity_flag == false) {
            self.parity_flag = true;
        } else {
            self.parity_flag = false;
        }
    }

    /* Sets the carry flag. */
    pub fn set_carry_flag(&mut self) {
        if (self.carry_flag == false) {
            self.carry_flag = true;
        } else {
            self.carry_flag = false;
        }
    }

    /* Sets the auxiliary carry flag. */
    pub fn set_auxiliary_carry_flag(&mut self) {
        if (self.auxiliary_carry == false) {
            self.auxiliary_carry = true;
        } else {
            self.auxiliary_carry = false;
        }
    }

    /* NOP instruction. */
    fn nop_instruction() {
        println!("NOP Instruction.");
    }
 
    /* MOV instruction. */
    fn mov_instruction(register1: u8, register2: u8) {
        println!("MOV Instruction.");
    }

    /* ADD instruction. */
    fn add_instruction(register1: u8, register2: u8) {
        println!("ADD Instruction.");
    }

    /* SUB instruction. */
    fn sub_instruction(register1: u8, register2: u8) {
        println!("SUB Instruction.");
    }

    /* JMP instruction. */
    fn jmp_instruction(register1: u8, register2: u8) {
        println!("JMP Instruction.");
    }

    /* Executes a single instruction. */
    pub fn execute_instruction(&mut self, instruction: instructions, param1: u8, param2: u8) {
        match instruction {
            NOP => {

            },

            MOV => {

            },

            ADD => {

            },

            SUB => {

            },

            JMP => {

            },
        }
    }

    /* Prints the values of the CPU. */
    pub fn print_cpu(&mut self) {
        println!("Program Counter: {}", self.program_counter);
        println!("Stack Pointer: {}", self.stack_pointer);
        println!("Instruction Pointer: {}", self.instruction_pointer);
        println!("Accumulator: {}", self.accumulator);
        println!("Register 1: {}", self.register1);
        println!("Register 2: {}", self.register2);
        println!("Register 3: {}", self.register3);
        println!("Register 4: {}", self.register4);
        println!("Sign Flag: {}", self.sign_flag);
        println!("Zero Flag: {}", self.zero_flag);
        println!("Parity Flag: {}", self.parity_flag);
        println!("Carry Flag: {}", self.carry_flag);
        println!("Auxiliary Carry Flag: {}\n", self.auxiliary_carry);
    }
}