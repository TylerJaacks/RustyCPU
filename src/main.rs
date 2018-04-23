#![allow(warnings)]
use std::fmt;
use std::env;
use std::path::PathBuf;

mod cpu;

fn main() {
    println!("Welcome to RustyCPU!\n");

    let mut rusty_cpu = cpu::cpu::new(0, 0, 0, 0, 0, 0, 0, 0, false, false, false, false, false);

    rusty_cpu.reset_cpu();

    match env::home_dir() {
        Some(path) => {
            let mut file = PathBuf::from(r"C:\\TestProgram.tj");

            rusty_cpu.fetch_and_execute_instructions(file);
        },
        None => println!("Can't find home directory."),
    }

    rusty_cpu.reset_cpu();
}