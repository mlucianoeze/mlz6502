use mlz6502::cpu::Cpu6502;
use mlz6502::memory::Memory;

fn main() {
    let mut memory = Memory::new();
    memory.load_rom(&[0xA9, 0x69, 0x92]);
    let mut cpu = Cpu6502::new(Box::new(memory));

    cpu.run();
    println!("Execution finished.");
    println!("\nCPU state:");
    println!("  a = {:02x}", cpu.a);
    println!("  x = {:02x}", cpu.x);
    println!("  y = {:02x}", cpu.y);
    println!(
        "  p = {}C | {}Z | {}I | {}D | {}B | {}V | {}N",
        flag_str(cpu.carry()),
        flag_str(cpu.zero()),
        flag_str(cpu.interrupt_disable()),
        flag_str(cpu.decimal()),
        flag_str(cpu.break_flag()),
        flag_str(cpu.overflow()),
        flag_str(cpu.negative())
    );
    println!("  sp = {:02x}", cpu.sp);
    println!("  pc = {:04x}", cpu.pc);
    test_execution_successful(&cpu);
}

fn test_execution_successful(cpu: &Cpu6502) {
    println!();
    if cpu.a == 0x69 {
        println!("Register A is 0x69. Execution successful!");
    } else {
        println!("Register A is not 0x69. Execution failed :(");
    }
}

fn flag_str(flag: bool) -> &'static str {
    if flag {
        ""
    } else {
        "!"
    }
}
