mod cpu;

fn main() {
    println!("Welcome to RustyCPU!\n");

    let mut rusty_cpu = cpu::cpu::new(1, 1, 1, 1, 1, 1, 1, 1);

    /* Prints the values of the cpu. */
    rusty_cpu.print_cpu();

    /* Resets the cpu to default values. */
    rusty_cpu.reset_cpu();

    /* Prints the values of the cpu. */
    rusty_cpu.print_cpu();
}