use alloc::vec::Vec;

pub const MAX_EXT_IRQ: usize = 0x100;

// The Capability
#[repr(C)]
#[derive(Debug, Copy, Clone)]
pub struct Capability {
    pub task_id: usize,
    pub target_os_id: usize,
    pub target_proc_id: usize,
    pub target_task_id: usize,
}

impl Capability {
    /// 
    pub const EMPTY: Self = Self {
        task_id: 0,
        target_os_id: 0,
        target_proc_id: 0,
        target_task_id: 0,
    };
}

#[repr(C)]
#[derive(Debug)]
pub struct DeviceCapTable([Capability; MAX_EXT_IRQ]);

impl DeviceCapTable {
    /// 
    pub const EMPTY: Self = Self([Capability::EMPTY; MAX_EXT_IRQ]);
}

// The Capability Queue
#[repr(C)]
#[derive(Debug)]
pub struct CapQueue {
    pub inner: Vec<Capability>,
    pub online: bool,
}

impl CapQueue {
    /// 
    pub const EMPTY: Self = Self {
        inner: Vec::new(),
        online: false
    };
}