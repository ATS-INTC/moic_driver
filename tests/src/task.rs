use core::ptr::addr_of;



#[repr(C, align(64))]
#[derive(Debug)]
pub struct TaskControlBlock {
    pub ready_queue: ReadyQueue,
    pub device_cap: usize,
    pub send_cap: CapQueue,
    pub recv_cap: CapQueue,
}

#[repr(C)]
#[derive(Debug)]
pub struct ReadyQueue {
    pub ptr: usize,
    pub len: usize,
    pub cap: usize,
    pub online: bool,
}

#[repr(C)]
#[derive(Debug)]
pub struct CapQueue {
    pub ptr: usize,
    pub len: usize,
    pub cap: usize,
    pub online: bool,
}

#[repr(C)]
#[derive(Debug, Copy, Clone)]
pub struct Capability {
    pub task_id: usize,
    pub target_os_id: usize,
    pub target_proc_id: usize,
    pub target_task_id: usize,
}

static TASK_POLL: [usize; 4] = [0x19990129, 0x19990219, 0x199902f9, 0x199902e9];
static DEVICE_CAP: [Capability; 0x100] = [Capability {
    task_id: 0,
    target_os_id: 0,
    target_proc_id: 0,
    target_task_id: 0,
}; 0x100];
static SEND_CAP_POLL: [Capability; 6] =  [Capability {
    task_id: 0x19990109,
    target_os_id: 0x1988,
    target_proc_id: 0x1778,
    target_task_id: 0x7898,
}; 6];
static mut OS_TCB: TaskControlBlock = TaskControlBlock {
    ready_queue: ReadyQueue {
        ptr: 0,
        len: 0,
        cap: 0,
        online: false,
    },
    device_cap: 0,
    send_cap: CapQueue {
        ptr: 0,
        len: 0,
        cap: 0,
        online: false,
    },
    recv_cap: CapQueue {
        ptr: 0,
        len: 0,
        cap: 0,
        online: false,
    },
};

static PROC_TASK_POLL: [usize; 5] = [0x20990129, 0x20990219, 0x209902f9, 0x209902e9, 0x229902e9];
static mut PROC_TCB: TaskControlBlock = TaskControlBlock {
    ready_queue: ReadyQueue {
        ptr: 0,
        len: 0,
        cap: 0,
        online: false,
    },
    device_cap: 0,
    send_cap: CapQueue {
        ptr: 0,
        len: 0,
        cap: 0,
        online: false,
    },
    recv_cap: CapQueue {
        ptr: 0,
        len: 0,
        cap: 0,
        online: false,
    },
};

#[allow(unused)]
pub unsafe fn switch_test() {
    OS_TCB.ready_queue.ptr = addr_of!(TASK_POLL) as usize;
    OS_TCB.ready_queue.len = 4;
    OS_TCB.ready_queue.cap = 4;
    OS_TCB.device_cap = addr_of!(DEVICE_CAP) as usize;
    OS_TCB.send_cap.ptr = addr_of!(SEND_CAP_POLL) as usize;
    OS_TCB.send_cap.len = 6;
    OS_TCB.send_cap.cap = 6;
    let os_id = addr_of!(OS_TCB) as usize;
    let switch_os_addr = 0x1000018usize as *mut usize;
    switch_os_addr.write_volatile(os_id);

    let register_recv_task = 0x1000020usize as *mut usize;
    let register_recv_target_os = 0x1000028usize as *mut usize;
    let register_recv_target_proc = 0x1000030usize as *mut usize;
    let register_recv_target_task = 0x1000038usize as *mut usize;

    register_recv_task.write_volatile(0x1999);
    register_recv_target_os.write_volatile(0);
    register_recv_target_proc.write_volatile(0);
    register_recv_target_task.write_volatile(0);

    register_recv_task.write_volatile(0x1998786);
    register_recv_target_os.write_volatile(0);
    register_recv_target_proc.write_volatile(0);
    register_recv_target_task.write_volatile(1);

    let switch_proc_addr = 0x1000010usize as *mut usize;
    switch_proc_addr.write_volatile(0);
    log::info!("{:#X?}", SEND_CAP_POLL);
    let fetch_ptr = 0x1000008usize as *mut usize;
    for i in 0..3 {
        log::info!("{:#X}", fetch_ptr.read_volatile());
    }
    PROC_TCB.ready_queue.ptr = addr_of!(PROC_TASK_POLL) as usize;
    PROC_TCB.ready_queue.len = 5;
    PROC_TCB.ready_queue.cap = 5;
    let proc_id = addr_of!(PROC_TCB) as usize;
    switch_proc_addr.write_volatile(proc_id);
    for i in 0..8 {
        log::info!("{:#X}", fetch_ptr.read_volatile());
    }
    log::info!("{:#X?}", PROC_TCB);
    log::info!("{:#X?}", &DEVICE_CAP[0..3]);
    switch_proc_addr.write_volatile(0);
    log::info!("{:#X?}", OS_TCB);
    for i in 0..8 {
        log::info!("{:#X}", fetch_ptr.read_volatile());
    }

    register_recv_task.write_volatile(0x78786);
    register_recv_target_os.write_volatile(0);
    register_recv_target_proc.write_volatile(0);
    register_recv_target_task.write_volatile(2);
    log::info!("{:#X?}", &DEVICE_CAP[0..3]);
    switch_proc_addr.write_volatile(proc_id);
    log::info!("{:#X?}", &DEVICE_CAP[0..3]);
    log::info!("{:#X?}", SEND_CAP_POLL);

}