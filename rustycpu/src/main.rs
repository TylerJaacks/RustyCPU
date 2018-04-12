mod cpu;

fn main() {
    println!("Welcome to RustyCPU!\n");

    let mut rusty_cpu = cpu::cpu::new(1, 1, 1, 1, 1, 1, 1, 1, true, true, true, true, true);

    /* Resets the cpu to default values. */
    rusty_cpu.reset_cpu();

    /* Resets the cpu to default values. */
    rusty_cpu.reset_cpu();

    /* Prints the values of the cpu. */
    rusty_cpu.print_cpu();

    /* Executes a single instruction. */
    
}