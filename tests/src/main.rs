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

#[boot::riscv_entry(boot_stack: 0x8000)]
fn main(_hart_id: usize) {
    console::init();
    mem::init_heap();
    trap::init();
    log::info!("hello");
    unsafe {
        task::switch_test();
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

#[allow(unused)]
unsafe fn user_interrupt_test() {
    sideleg::set_usoft();
    ustatus::set_uie();
    uie::set_usoft();
    // uip::set_usoft();
    let ctx = user::user_ctx();
    user::trap_return(&ctx);
}

#[allow(unused)]
fn moic_test() {
    let add_ptr = 0x1000000usize as *mut usize;
    let register_recv_task = 0x1000020usize as *mut usize;
    let register_recv_target_os = 0x1000028usize as *mut usize;
    let register_recv_target_proc = 0x1000030usize as *mut usize;
    let register_recv_target_task = 0x1000038usize as *mut usize;

    let fetch_ptr = 0x1000008usize as *mut usize;
    unsafe {
        add_ptr.write_volatile(0x300);
        log::info!("{:#X}", fetch_ptr.read_volatile());
        log::info!("{:#X}", fetch_ptr.read_volatile());
        add_ptr.write_volatile(0x300);
        register_recv_task.write_volatile(0);
        register_recv_target_os.write_volatile(0);
        register_recv_target_proc.write_volatile(0);
        register_recv_target_task.write_volatile(1);
        register_recv_task.write_volatile(0);
        register_recv_target_os.write_volatile(0);
        register_recv_target_proc.write_volatile(0);
        register_recv_target_task.write_volatile(1);
        register_recv_task.write_volatile(1);
        register_recv_target_os.write_volatile(2);
        register_recv_target_proc.write_volatile(3);
        register_recv_target_task.write_volatile(4);
        register_recv_task.write_volatile(1);
        register_recv_target_os.write_volatile(2);
        register_recv_target_proc.write_volatile(3);
        register_recv_target_task.write_volatile(4);
    }
    let register_send_task = 0x1000040usize as *mut usize;
    let register_send_target_os = 0x1000048usize as *mut usize;
    let register_send_target_proc = 0x1000050usize as *mut usize;
    let register_send_target_task = 0x1000058usize as *mut usize;
    unsafe {
        register_send_task.write_volatile(0);
        register_send_target_os.write_volatile(0);
        register_send_target_proc.write_volatile(0);
        register_send_target_task.write_volatile(1);
        register_send_task.write_volatile(0);
        register_send_target_os.write_volatile(0);
        register_send_target_proc.write_volatile(0);
        register_send_target_task.write_volatile(1);
        register_send_task.write_volatile(1);
        register_send_target_os.write_volatile(2);
        register_send_target_proc.write_volatile(3);
        register_send_target_task.write_volatile(4);
        register_send_task.write_volatile(1);
        register_send_target_os.write_volatile(2);
        register_send_target_proc.write_volatile(3);
        register_send_target_task.write_volatile(4);
    }
}
