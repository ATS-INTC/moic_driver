use moic_driver::*;

static MOIC: Moic = Moic::new(0x1000000);


#[allow(unused)]
pub unsafe fn switch_test() {
    let os_tcb = TaskControlBlock::new(9, false);
    MOIC.switch_os(Some(os_tcb));
    MOIC.add(TaskId::virt(0x1999));
    MOIC.add(TaskId::virt(0x2324));
    MOIC.add(TaskId::virt(0x3453));
    for i in 0..2 {
        log::info!("{:x?}", MOIC.fetch());
    }
    let proc_tcb = TaskControlBlock::new(1, false);
    MOIC.switch_process(Some(proc_tcb));
    for i in 0..2 {
        log::info!("{:x?}", MOIC.fetch());
    }
    MOIC.add(TaskId::virt(0x19990109));
    MOIC.add(TaskId::virt(0x19990209));
    MOIC.add(TaskId::virt(0x12330098));
    MOIC.switch_process(None);
    for i in 0..2 {
        log::info!("{:x?}", MOIC.fetch());
    }
    MOIC.switch_process(Some(proc_tcb));
    for i in 0..4 {
        log::info!("{:x?}", MOIC.fetch());
    }
}
