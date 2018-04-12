
pub struct cpu {
    // private static int instructionsToInterrupt;
    // private static int instructionsSinceInterrupt = 0;
    // private static boolean exitFlag = false;
    // private static boolean kernelModeFlag = false;
    program_counter: i16,
    stack_pointer: i16,
    instruction_pointer: i16,
    accumulator: i16,
    register1: i16,
    register2: i16,
    register3: i16,
    register4: i16,
}

impl cpu {
    /* Constructs a new CPU struct with values. */
    pub fn new(pc: i16, sp: i16, ip: i16, ac: i16, r1: i16, r2: i16, r3: i16, r4: i16) -> cpu {
        cpu {
            program_counter: pc,
            stack_pointer: sp,
            instruction_pointer: ip,
            accumulator: ac,
            register1: r1,
            register2: r2,
            register3: r3,
            register4: r4
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
    }

    pub fn increment_pc(&mut self) {
        self.program_counter += 1;
    }

    pub fn increment_sp(&mut self) {
        self.stack_pointer += 1;
    }

    pub fn increment_ip(&mut self) {
        self.instruction_pointer += 1;
    }

    pub fn increment_ac(&mut self) {
        self.accumulator += 1;
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
        println!("Register 4: {}\n", self.register4);
    }
}