#[cfg(test)]
mod tests {
    /* Tests creating a new CPU. */
    #[test]
    fn new_cpu_test() {
        let rusty_cpu = cpu::cpu::new(1, 1, 1, 1, 1, 1, 1, 1, true, true, true, true, true);

        assert_eq!(rusty_cpu.program_counter, 1);
        assert_eq!(rusty_cpu.stack_pointer, 1);
        assert_eq!(rusty_cpu.instruction_pointer, 1);
        assert_eq!(rusty_cpu.accumulator, 1);
        assert_eq!(rusty_cpu.register1, 1);
        assert_eq!(rusty_cpu.register2, 1);
        assert_eq!(rusty_cpu.register3, 1);
        assert_eq!(rusty_cpu.register4, 1);
        assert_eq!(rusty_cpu.sign_flag, true);
        assert_eq!(rusty_cpu.zero_flag, true);
        assert_eq!(rusty_cpu.parity_flag, true);
        assert_eq!(rusty_cpu.carry_flag, true);
        assert_eq!(rusty_cpu.auxiliary_carry, true);
    }

    /* Tests reseting the cpu. */
    #[test]
    fn reset_cpu_test() {
        let rusty_cpu = cpu::cpu::new(1, 1, 1, 1, 1, 1, 1, 1, true, true, true, true, true);

        rusty_cpu.reset_cpu();

        assert_eq!(rusty_cpu.program_counter, 0);
        assert_eq!(rusty_cpu.stack_pointer, 0);
        assert_eq!(rusty_cpu.instruction_pointer, 0);
        assert_eq!(rusty_cpu.accumulator, 0);
        assert_eq!(rusty_cpu.register1, 0);
        assert_eq!(rusty_cpu.register2, 0);
        assert_eq!(rusty_cpu.register3, 0);
        assert_eq!(rusty_cpu.register4, 0);
        assert_eq!(rusty_cpu.sign_flag, false);
        assert_eq!(rusty_cpu.zero_flag, false);
        assert_eq!(rusty_cpu.parity_flag, false);
        assert_eq!(rusty_cpu.carry_flag, false);
        assert_eq!(rusty_cpu.auxiliary_carry, false);
    }
}