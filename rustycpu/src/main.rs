mod cpu;

fn main() {
    println!("Welcome to RustyCPU!\n");

    let mut rusty_cpu = cpu::cpu::new(1, 1, 1, 1, 1, 1, 1, 1);

    /* Resets the cpu to default values. */
    rusty_cpu.reset_cpu();

    for x in 0..100 {
        /* Increment program counter. */
        rusty_cpu.increment_pc();

        /* Prints the values of the cpu. */
        rusty_cpu.print_cpu();
    }
}