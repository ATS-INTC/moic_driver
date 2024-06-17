use alloc::vec::Vec;
use moic_driver::*;

static MOIC: Moic = Moic::new(0x1000000);

pub fn tests() {
    unsafe {
        create_drop_task_test();
        switch_ready_queue_test();
        register_device_cap_test();
        register_send_cap_test();
        register_recv_cap_test();
        remove_task_test();
    }
}

#[allow(unused)]
fn create_drop_task_test() {
    const TASK_NUM: usize = 0x21;
    let mut task_vec = Vec::new();
    for i in 0..TASK_NUM {
        let taski = TaskControlBlock::new(i, false);
        task_vec.push(taski);
    }
    for i in 0..TASK_NUM {
        let taski = task_vec.pop().unwrap();
        taski.manual_drop();
    }
    log::info!("create_drop_task_test passed!");
}

#[allow(unused)]
unsafe fn switch_ready_queue_test() {
    use alloc::collections::BTreeSet;
    let mut os_task_set = BTreeSet::new();
    let os_tid = TaskControlBlock::new(9, false);
    MOIC.switch_os(Some(os_tid));
    const ADD_COUNT: usize = 0x1000;
    for i in 0..ADD_COUNT {
        let process_i = TaskControlBlock::new(i, false);
        os_task_set.insert(process_i);
        MOIC.add(process_i);
    }
    for i in 0..1 {
        let task = MOIC.fetch();
        assert!(task.is_some());
        assert!(os_task_set.contains(task.as_ref().unwrap()));
        os_task_set.remove(task.as_ref().unwrap());
        task.unwrap().manual_drop();
    }
    let mut proc_task_set = BTreeSet::new();
    let proc_tid = TaskControlBlock::new(1, false);
    MOIC.switch_process(Some(proc_tid));
    for i in 0..2 {
        assert!(MOIC.fetch().is_none());
    }
    for i in 0..3 {
        let process_i = TaskControlBlock::new(i, false);
        proc_task_set.insert(process_i);
        MOIC.add(process_i);
    }
    MOIC.switch_process(None);
    for i in 0..ADD_COUNT - 1 {
        let task = MOIC.fetch();
        assert!(task.is_some());
        assert!(os_task_set.contains(task.as_ref().unwrap()));
        os_task_set.remove(task.as_ref().unwrap());
        task.unwrap().manual_drop();
    }
    assert!(os_task_set.is_empty());
    assert!(MOIC.fetch().is_none());
    MOIC.switch_process(Some(proc_tid));
    for i in 0..3 {
        let task = MOIC.fetch();
        assert!(task.is_some());
        assert!(proc_task_set.contains(task.as_ref().unwrap()));
        proc_task_set.remove(task.as_ref().unwrap());
        task.unwrap().manual_drop();
    }
    assert!(proc_task_set.is_empty());
    assert!(MOIC.fetch().is_none());
    MOIC.switch_process(None);
    os_tid.manual_drop();
    proc_tid.manual_drop();
    log::info!("switch ready_queue test passed!");
}

#[allow(unused)]
pub unsafe fn register_device_cap_test() {
    let os_tid = TaskControlBlock::new(9, false);
    let os_tcb: &mut TaskControlBlock = os_tid.into();
    MOIC.switch_os(Some(os_tid));
    const REGISTER_COUNT: usize = 0x10;
    for i in 0..REGISTER_COUNT {
       MOIC.register_receiver(TaskId::virt(0x19990100 + i), TaskId::EMPTY, TaskId::EMPTY, TaskId::virt(i));
    }
    let proc_tid = TaskControlBlock::new(0, false);
    let proc_tcb: &mut TaskControlBlock = proc_tid.into();
    MOIC.switch_process(Some(proc_tid));
    for i in 0..REGISTER_COUNT {
        MOIC.register_receiver(TaskId::virt(0x1999 + i), TaskId::EMPTY, TaskId::EMPTY, TaskId::virt(i));
    }
    let os_device_table = os_tcb.device_cap().iter();
    for i in 0..REGISTER_COUNT {
        assert!(os_device_table[i] == TaskId::virt(0x19990100 + i));
    }
    MOIC.switch_process(None);
    let proc_device_table = proc_tcb.device_cap().iter();
    for i in 0..REGISTER_COUNT {
        assert!(proc_device_table[i] == TaskId::virt(0x1999 + i));
    }
    os_tid.manual_drop();
    proc_tid.manual_drop();
    log::info!("register device_cap test passed!");
}


#[allow(unused)]
pub unsafe fn register_send_cap_test() {
    let os_tid = TaskControlBlock::new(9, false);
    let os_tcb: &mut TaskControlBlock = os_tid.into();
    MOIC.switch_os(Some(os_tid));
    let proc_tid = TaskControlBlock::new(0, false);
    let proc_tcb: &mut TaskControlBlock = proc_tid.into();
    const REGISTER_COUNT: usize = 0x10;
    for i in 0..REGISTER_COUNT {
       MOIC.register_sender(TaskId::virt(0x19990100 + i), os_tid, proc_tid, TaskId::virt(0x19990109 + i));
    }
    MOIC.switch_process(Some(proc_tid));
    let os_send_cap = os_tcb.send_cap();
    for i in 0..REGISTER_COUNT {
        assert!(os_send_cap[i] == Capability { 
            task_id: TaskId::virt(0x19990100 + i), 
            target_os_id: os_tid, 
            target_proc_id: proc_tid, 
            target_task_id: TaskId::virt(0x19990109 + i)
        });
    }
    let proc2 = TaskControlBlock::new(8, false);
    let proc_send_cap = proc_tcb.send_cap();
    for i in 0..REGISTER_COUNT {
        MOIC.register_sender(TaskId::virt(0x1999 + i), os_tid, proc2, TaskId::virt(0x199 + i));
    }
    MOIC.switch_process(None);
    let proc_send_cap = proc_tcb.send_cap();
    for i in 0..REGISTER_COUNT {
        assert!(proc_send_cap[i] == Capability { 
            task_id: TaskId::virt(0x1999 + i), 
            target_os_id: os_tid, 
            target_proc_id: proc2, 
            target_task_id: TaskId::virt(0x199 + i)
        });
    }
    os_tid.manual_drop();
    proc_tid.manual_drop();
    proc2.manual_drop();
    log::info!("register send_cap test passed!");
}

#[allow(unused)]
pub unsafe fn register_recv_cap_test() {
    let os_tid = TaskControlBlock::new(9, false);
    let os_tcb: &mut TaskControlBlock = os_tid.into();
    MOIC.switch_os(Some(os_tid));
    let proc_tid = TaskControlBlock::new(0, false);
    let proc_tcb: &mut TaskControlBlock = proc_tid.into();
    const REGISTER_COUNT: usize = 0x10;
    for i in 0..REGISTER_COUNT {
       MOIC.register_receiver(TaskId::virt(0x19990100 + i), os_tid, proc_tid, TaskId::virt(0x19990109 + i));
    }
    MOIC.switch_process(Some(proc_tid));
    let os_recv_cap = os_tcb.recv_cap();
    for i in 0..REGISTER_COUNT {
        assert!(os_recv_cap[i] == Capability { 
            task_id: TaskId::virt(0x19990100 + i), 
            target_os_id: os_tid, 
            target_proc_id: proc_tid, 
            target_task_id: TaskId::virt(0x19990109 + i)
        });
    }
    let proc2 = TaskControlBlock::new(8, false);
    for i in 0..REGISTER_COUNT {
        MOIC.register_receiver(TaskId::virt(0x1999 + i), os_tid, proc2, TaskId::virt(0x199 + i));
    }
    MOIC.switch_process(None);
    let proc_recv_cap = proc_tcb.recv_cap();
    for i in 0..REGISTER_COUNT {
        assert!(proc_recv_cap[i] == Capability { 
            task_id: TaskId::virt(0x1999 + i), 
            target_os_id: os_tid, 
            target_proc_id: proc2, 
            target_task_id: TaskId::virt(0x199 + i)
        });
    }
    os_tid.manual_drop();
    proc_tid.manual_drop();
    proc2.manual_drop();
    log::info!("register recv_cap test passed!");
}


#[allow(unused)]
unsafe fn remove_task_test() {
    use alloc::collections::BTreeSet;
    let mut os_task_set = BTreeSet::new();
    let os_tid = TaskControlBlock::new(9, false);
    MOIC.switch_os(Some(os_tid));
    const ADD_COUNT: usize = 0x1000;
    for i in 0..ADD_COUNT {
        let process_i = TaskControlBlock::new(i, false);
        os_task_set.insert(process_i);
        MOIC.add(process_i);
    }
    for tid in os_task_set.into_iter() {
        MOIC.remove_task(tid);
    }
    assert!(MOIC.fetch().is_none());
    let proc_tid = TaskControlBlock::new(0, false);
    MOIC.switch_process(Some(proc_tid));
    os_tid.manual_drop();
    log::info!("remove task test passed!");
}
