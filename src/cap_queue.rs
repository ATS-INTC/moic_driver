use alloc::vec::Vec;
use crate::TaskId;
pub const MAX_EXT_IRQ: usize = 0x100;

// The Capability
#[repr(C)]
#[derive(Debug, Copy, Clone)]
pub struct Capability {
    pub task_id: TaskId,
    pub target_os_id: TaskId,
    pub target_proc_id: TaskId,
    pub target_task_id: TaskId,
}

impl Capability {
    /// 
    pub const EMPTY: Self = Self {
        task_id: TaskId::EMPTY,
        target_os_id: TaskId::EMPTY,
        target_proc_id: TaskId::EMPTY,
        target_task_id: TaskId::EMPTY,
    };
}

#[repr(C)]
#[derive(Debug)]
pub struct DeviceCapTable(pub (crate)[Capability; MAX_EXT_IRQ]);

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