use moic_driver::*;

static MOIC: Moic = Moic::new(0x1000000);


#[allow(unused)]
pub unsafe fn switch_test() {
    let os_tcb = TaskControlBlock::new(9);
    MOIC.switch_os(Some(os_tcb));
    MOIC.add(TaskId::virt(0x1999));
    for i in 0..4 {
        log::info!("{:#x?}", MOIC.fetch());
    }
}
