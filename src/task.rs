//! Coroutine Control Block structures for more control.
//!

use alloc::boxed::Box;

use crate::{
    cap_queue::{CapQueue, DeviceCapTable},
    ready_queue::ReadyQueue,
};
use core::{fmt::Display, ptr::NonNull};
pub(crate) const TCB_ALIGN: usize = 6;

/// The Identity of `Task`
#[repr(transparent)]
#[derive(Debug, Clone, Copy, PartialEq, Eq, PartialOrd, Ord, Hash)]
pub struct TaskId(usize);

unsafe impl Send for TaskId {}
unsafe impl Sync for TaskId {}

impl TaskId {
    /// 
    pub const EMPTY: Self = Self(0);
    
    /// Assume that val is a valid pointer of `TaskControlBlock`.
    pub(crate) unsafe fn virt(val: usize) -> Self {
        let raw_tcb_ptr = val as *const TaskControlBlock;
        let priority = (&*raw_tcb_ptr).priority;
        let is_preempt = (&*raw_tcb_ptr).is_preempt;
        let mut tid = val;
        tid |= priority << 1;
        if is_preempt {
            tid |= 1;
        }
        Self(tid)
    }

    /// 
    pub(crate) fn value(&self) -> usize {
        self.0
    }
}

impl From<Box<TaskControlBlock>> for TaskId {
    fn from(value: Box<TaskControlBlock>) -> Self {
        let priority = value.priority;
        let is_preempt = value.is_preempt;
        let mut raw_tcb_ptr = Box::into_raw(value) as usize;
        raw_tcb_ptr |= priority << 1;
        if is_preempt {
            raw_tcb_ptr |= 1;
        }
        Self(raw_tcb_ptr)
    }
}

impl Display for TaskId {
    fn fmt(&self, f: &mut core::fmt::Formatter<'_>) -> core::fmt::Result {
        let raw_tcb: *const TaskControlBlock = self.into();
        write!(f, "{}", unsafe {&*raw_tcb})
    }
}

#[repr(usize)]
#[derive(Debug)]
pub enum Status {
    Inited = 0,
    Ready = 1,
}

/// The `TaskControlBlock`
#[repr(C)]
pub struct TaskControlBlock {
    /// 
    pub ready_queue: ReadyQueue,
    /// 
    pub device_cap_table: NonNull<DeviceCapTable>,
    /// 
    pub send_cap_queue: CapQueue,
    /// 
    pub recv_cap_queue: CapQueue,
    /// 
    pub status: Status,
    /// 
    pub priority: usize,
    /// 
    pub is_preempt: bool,
}

impl TaskControlBlock {
    /// 
    pub fn new(priority: usize, is_preempt: bool) -> TaskId {
        let raw_device_table_ptr = Box::into_raw(Box::new(DeviceCapTable::EMPTY));
        let device_cap_table = NonNull::new(raw_device_table_ptr).unwrap();
        let tcb = Box::new(TaskControlBlock {
            ready_queue: ReadyQueue::EMPTY,
            device_cap_table,
            send_cap_queue: CapQueue::EMPTY,
            recv_cap_queue: CapQueue::EMPTY,
            status: Status::Inited,
            priority,
            is_preempt
        });
        TaskId::from(tcb)
    }
}

impl From<&TaskId> for *const TaskControlBlock {
    fn from(value: &TaskId) -> Self {
        let tid = value.0;
        let raw_tcb_ptr = tid & (!0x3f);
        raw_tcb_ptr as _
    }
}

impl Display for TaskControlBlock {
    fn fmt(&self, f: &mut core::fmt::Formatter<'_>) -> core::fmt::Result {
        write!(f, "TaskControlBlock(
ReadyQueue: {:X?},
SendCap: {:X?},
RecvCap: {:X?},
Status: {:?},
Priority: {},
)", 
            self.ready_queue, 
            self.send_cap_queue,
            self.recv_cap_queue,
            self.status,
            self.priority
        )
    }
}