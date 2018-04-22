#![allow(warnings)]
mod cpu;

fn main() {
    println!("Welcome to RustyCPU!\n");

    let mut rusty_cpu = cpu::cpu::new(0, 0, 0, 0, 0, 0, 0, 0, false, false, false, false, false);

    rusty_cpu.reset_cpu();

    rusty_cpu.fetch_and_execute_instructions("D:\\Documents\\Projects\\Rust Projects\\RustyCPU\\test_programs\\MovingRegisters.tj");

    rusty_cpu.reset_cpu();
}