#![no_std] // Nao coloca a std pq depende de sistema
#![no_main] // Nao tem entry point com o Runtime Rust
#![feature(custom_test_frameworks)]
#![test_runner(crate::test_runner)]
#![reexport_test_harness_main = "test_main"]

mod vga_buffer; // Import do modulo vga

use core::panic::PanicInfo;

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
#[repr(u32)]
pub enum QemuExitCode {
    Success = 0x10,
    Failed = 0x11,
}

pub fn exit_qemu(exit_code: QemuExitCode) {
    use x86_64::instructions::port::Port;

    unsafe {
        let mut port = Port::new(0xf4);
        port.write(exit_code as u32);
    }
}


#[cfg(test)]
fn test_runner(tests: &[&dyn Fn()]) {
    println!("Running {} tests", tests.len());
    for test in tests {
        test();
    }
    exit_qemu(QemuExitCode::Success);
}

#[test_case]
fn trivial_assertion() {
    print!("trivial assertion... ");
    assert_eq!(1, 1);
    println!("[ok]");
} 

/// Rust precisa de um panic handler..
/// Aqui nao fazemos nada
#[panic_handler]
fn panic(_info: &PanicInfo) -> ! {
    print!("{}", _info);
    loop {}
}

#[no_mangle] // Mantém o nome da funcao como o esperado pelo processador
pub extern "C" fn _start() -> ! {
   println!("Oi Mundo o/\n");

   #[cfg(test)]
   test_main();

    // Essa funcao vira o entry point
    // Faz nada ate desligar
    loop {}
}
