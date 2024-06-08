#![no_std]
#![feature(naked_functions)]
#![feature(asm_const)]
#![feature(panic_info_message)]
#![feature(alloc_error_handler)]


#![no_main]

mod console;
mod mem;

#[boot::riscv_entry(boot_stack: 0x8000)]
fn main(_hart_id: usize) {
    console::init();
    mem::init_heap();
    log::info!("hello");
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
    unreachable!();
}


