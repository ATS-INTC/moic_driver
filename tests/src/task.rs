use core::ptr::addr_of;



#[repr(C, align(64))]
pub struct TaskControlBlock {
    pub ready_queue: ReadyQueue,
}

#[repr(C)]
pub struct ReadyQueue {
    pub ptr: usize,
    pub len: usize,
    pub cap: usize,
    pub online: bool,
}

static TASK_POLL: [usize; 4] = [0x19990129, 0x19990219, 0x199902f9, 0x199902e9];
static mut OS_TCB: TaskControlBlock = TaskControlBlock {
    ready_queue: ReadyQueue {
        ptr: 0,
        len: 0,
        cap: 0,
        online: false,
    }
};

#[allow(unused)]
pub unsafe fn switch_test() {
    OS_TCB.ready_queue.ptr = addr_of!(TASK_POLL) as usize;
    OS_TCB.ready_queue.len = 4;
    OS_TCB.ready_queue.cap = 4;
    let os_id = addr_of!(OS_TCB) as usize;
    let switch_os_addr = 0x1000018usize as *mut usize;
    switch_os_addr.write_volatile(os_id);
    let switch_proc_addr = 0x1000010usize as *mut usize;
    switch_proc_addr.write_volatile(0);
    let fetch_ptr = 0x1000008usize as *mut usize;
    for i in 0..4 {
        log::info!("{:#X}", fetch_ptr.read_volatile());
    }
}