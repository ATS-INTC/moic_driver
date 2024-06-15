//! This is the driver of moic(Multiple-object-interaction interrupt controller)
//!

#![no_std]
#![deny(missing_docs)]

pub use task::{TaskControlBlock, TaskId};
extern crate alloc;

mod cap_queue;
mod ready_queue;
mod task;


/// moic
#[derive(Debug, Clone, Copy)]
pub struct Moic(usize);

impl Moic {
    ///
    pub const fn new(base_addr: usize) -> Self {
        Self(base_addr)
    }

    /// the mmio registers
    fn regs(&self) -> &'static pac::moic::Hart {
        unsafe { &*(self.0 as *const _) }
    }

    /// Add a task
    pub fn add(&self, task_id: TaskId) {
        self.regs().add().write(|w| unsafe { w.bits(task_id.as_ptr() as _) });
    }

    /// 
    pub fn fetch(&self) -> Option<TaskId> {
        let raw_task_id = self.regs().fetch().read().bits();
        if raw_task_id != 0 {
            Some(unsafe { TaskId::from_ptr(raw_task_id as *const TaskControlBlock) })
        } else {
            None
        }
    }

    /// 
    pub fn switch_hypervisor(&self, hypervisor_id: TaskId) {
        self.regs().switch_hypervisor().write(|w| unsafe { w.bits(hypervisor_id.as_ptr() as _) })
    }

    /// 
    pub fn switch_os(&self, os_id: Option<TaskId>) {
        self.regs().switch_os().write(|w| unsafe { 
            w.bits(os_id.map_or(0, |inner| inner.as_ptr() as _)) 
        })
    }

    /// 
    pub fn switch_process(&self, process_id: Option<TaskId>) {
        self.regs().switch_os().write(|w| unsafe { 
            w.bits(process_id.map_or(0, |inner| inner.as_ptr() as _)) 
        })
    }

    /// 
    pub fn register_sender(&self, send_task_id: TaskId, recv_os_id: TaskId, recv_proc_id: TaskId, recv_task_id: TaskId) {
        self.regs().register_send_task().write(|w| unsafe {
            w.bits(send_task_id.as_ptr() as _)
        });
        self.regs().register_send_target_os().write(|w| unsafe {
            w.bits(recv_os_id.as_ptr() as _)
        });
        self.regs().register_send_target_proc().write(|w| unsafe {
            w.bits(recv_proc_id.as_ptr() as _)
        });
        self.regs().register_send_target_task().write(|w| unsafe {
            w.bits(recv_task_id.as_ptr() as _)
        });
    }

    /// 
    pub fn register_receiver(&self, recv_task_id: TaskId, send_os_id: TaskId, send_proc_id: TaskId, send_task_id: TaskId) {
        self.regs().register_recv_task().write(|w| unsafe {
            w.bits(recv_task_id.as_ptr() as _)
        });
        self.regs().register_recv_target_os().write(|w| unsafe {
            w.bits(send_os_id.as_ptr() as _)
        });
        self.regs().register_recv_target_proc().write(|w| unsafe {
            w.bits(send_proc_id.as_ptr() as _)
        });
        self.regs().register_recv_target_task().write(|w| unsafe {
            w.bits(send_task_id.as_ptr() as _)
        });
    }

    /// 
    pub fn send_intr(&self, recv_os_id: TaskId, recv_proc_id: TaskId, recv_task_id: TaskId) {
        self.regs().send_intr_os().write(|w| unsafe {
            w.bits(recv_os_id.as_ptr() as _)
        });
        self.regs().send_intr_proc().write(|w| unsafe {
            w.bits(recv_proc_id.as_ptr() as _)
        });
        self.regs().send_intr_task().write(|w| unsafe {
            w.bits(recv_task_id.as_ptr() as _)
        });
    }

}
