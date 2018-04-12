
pub struct cpu {
    // private static int pc = 0;
    // private static int sp = 1000;
    // private static int ir = 0;
    // private static int ac = 0;
    // private static int x, y;
    // private static int instructionsToInterrupt;
    // private static int instructionsSinceInterrupt = 0;
    // private static boolean exitFlag = false;
    // private static boolean kernelModeFlag = false;
    pub program_counter: i16,
    pub stack_pointer: i16,
    pub instruction_pointer: i16,
    pub accumulator: i16,
    pub register1: i16,
    pub register2: i16,
    pub register3: i16,
    pub register4: i16,
}

impl cpu {
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