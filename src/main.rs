use mlz6502::{Cpu6502, Memory};

fn main() {
    let mut memory = Memory::new();
    memory.load_rom(&[0xA9, 0x69, 0x92]);
    let mut cpu = Cpu6502::new(memory);

    cpu.run();
    println!("Execution finished.");
    println!("\nCPU state:");
    println!("  a = {:02x}", cpu.a);
    println!("  x = {:02x}", cpu.x);
    println!("  y = {:02x}", cpu.y);
    println!("  p = {}C | {}Z | {}I | {}D | {}B | {}V | {}N", flag_str(cpu.p & 0x80), flag_str(cpu.p & 0x40), flag_str(cpu.p & 0x20), flag_str(cpu.p & 0x10), flag_str(cpu.p & 0x08), flag_str(cpu.p & 0x04), flag_str(cpu.p & 0x02));
    println!("  sp = {:02x}", cpu.sp);
    println!("  pc = {:04x}", cpu.pc);
    test_execution_successful(&cpu);
}

fn test_execution_successful(cpu: &Cpu6502<Memory>) {
    println!();
    if cpu.a == 0x69 {
        println!("Register A is 0x69. Execution successful!");
    } else {
        println!("Register A is not 0x69. Execution failed :(");
    }
}

fn flag_str(masked_p: u8) -> &'static str {
    if masked_p == 0 { "!" } else { "" }
}