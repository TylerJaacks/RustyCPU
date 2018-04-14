mod cpu;

fn main() {
    println!("Welcome to RustyCPU!\n");

    let mut rusty_cpu = cpu::cpu::new(1, 1, 1, 1, 1, 1, 1, 1, true, true, true, true, true);

    rusty_cpu.register1 = 10;
    rusty_cpu.register2 = 11;

    /* Prints the values of the cpu. */
    rusty_cpu.print_cpu();

    /* Executes a single instruction. */
    rusty_cpu.execute_instruction(cpu::instructions::MOV, cpu::registers::R1, cpu::registers::R2, 0);

    /* Prints the values of the cpu. */
    rusty_cpu.print_cpu();

    /* Executes a single instruction. */
    rusty_cpu.execute_instruction(cpu::instructions::NOP, cpu::registers::NONE, cpu::registers::NONE, 0);

    /* Prints the values of the cpu. */
    rusty_cpu.print_cpu();

    /* Executes a single instruction. */
    rusty_cpu.execute_instruction(cpu::instructions::MOV, cpu::registers::R2, cpu::registers::R3, 0);

    /* Prints the values of the cpu. */
    rusty_cpu.print_cpu();
}