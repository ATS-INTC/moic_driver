//! Coroutine Control Block structures for more control.
//!

use alloc::boxed::Box;

use crate::{
    cap_queue::{CapQueue, DeviceCapTable},
    ready_queue::ReadyQueue,
};
use core::{fmt::Display, ptr::NonNull};

/// The Identity of `Task`
#[repr(transparent)]
#[derive(Debug, Clone, Copy, PartialEq, Eq, PartialOrd, Ord, Hash)]
pub struct TaskId {
    ptr: NonNull<TaskControlBlock>,
}

unsafe impl Send for TaskId {}
unsafe impl Sync for TaskId {}

impl TaskId {
    /// 
    pub const fn virt(val: usize) -> Self {
        Self {
            ptr: unsafe { NonNull::new_unchecked(val as *mut _) }
        }
    }

    /// The raw pointer
    pub fn as_ptr(&self) -> *const TaskControlBlock {
        self.ptr.as_ptr()
    }

    // build the `TaskId` from raw pointer
    pub(crate) unsafe fn from_ptr(ptr: *const TaskControlBlock) -> Self {
        Self {
            ptr: NonNull::new(ptr as *mut TaskControlBlock).unwrap(),
        }
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
            priority
        });
        let mut raw_ptr = Box::into_raw(tcb) as usize;
        raw_ptr |= priority << 1;
        if is_preempt {
            raw_ptr |= 1;
        }
        TaskId {
            ptr: NonNull::new(raw_ptr as *mut _).unwrap()
        }
    }
}

impl Display for TaskControlBlock {
    fn fmt(&self, f: &mut core::fmt::Formatter<'_>) -> core::fmt::Result {
        write!(f, "({:?})", self.ready_queue.inner.as_ptr())
    }
}