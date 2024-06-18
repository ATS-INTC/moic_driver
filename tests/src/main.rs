#![no_std]
#![feature(naked_functions)]
#![feature(asm_const)]
#![feature(panic_info_message)]
#![feature(alloc_error_handler)]
#![no_main]

mod console;
mod mem;
mod task;
mod trap;
mod user;

extern crate alloc;

#[boot::riscv_entry(boot_stack: 0x8000)]
fn main(_hart_id: usize) {
    console::init();
    mem::init_heap();
    trap::init();
    log::info!("hello");
    unsafe {
        task::tests();
        // user::user_interrupt_test();
    }
    unreachable!();
}

use riscv::register::*;
#[allow(unused)]
unsafe fn ucsr_test() {
    log::info!("ustatus {:?}", ustatus::read());
    ustatus::set_uie();
    log::info!("ustatus {:?}", ustatus::read());
    utvec::write(user::__alltraps_u as usize, utvec::TrapMode::Direct);
    log::info!("utvec {:X?}", utvec::read());
    log::info!("uscratch {:#X?}", uscratch::read());
    uscratch::write(0x19990109);
    log::info!("uscratch {:#X?}", uscratch::read());
    log::info!("uepc {:#X?}", uepc::read());
    uepc::write(0x19990109);
    log::info!("uepc {:#X?}", uepc::read());
    log::info!("ucause {:X?}", ucause::read());
    ucause::write(0x19990109);
    log::info!("ucause {:X?}", ucause::read());
    log::info!("utval {:#X?}", utval::read());
    utval::write(0x19990109);
    log::info!("utval {:#X?}", utval::read());
    log::info!("sedeleg {:X?}", sedeleg::read());
    log::info!("sideleg {:X?}", sideleg::read());
    sideleg::set_usoft();
    sideleg::set_uext();
    log::info!("sideleg {:X?}", sideleg::read());

    log::info!("uie {:X?}", uie::read());
    uie::set_usoft();
    uie::set_uext();
    // uie::set_utimer();
    log::info!("uie {:X?}", uie::read());

    log::info!("uip {:X?}", uip::read());
    uip::set_usoft();
    // log::info!("uip {:X?}", uip::read());
    // uip::set_uext();
    // log::info!("uip {:X?}", uip::read());
    // uip::set_utimer();
    log::info!("uip {:X?}", uip::read());
}


