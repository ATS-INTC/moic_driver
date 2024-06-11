use crate::trap::TrapContext;
use riscv::register::{
    mtvec::TrapMode,
    sstatus::{self, Sstatus},
    sscratch,
    utvec, uip, uie, ustatus
};
static USER_STACK: [u8; 0x1000] = [0u8; 0x1000];
static KERNEL_STACK: [u8; 0x1000] = [0u8; 0x1000];


pub fn user_ctx() -> TrapContext {
    let mut sstatus = sstatus::read();
    sstatus.set_spp(sstatus::SPP::User);
    let mut ctx = TrapContext {
        x: [0; 32],
        sstatus,
        sepc: user_entry as _,
    };
    ctx.x[2] = USER_STACK.as_ptr() as usize;
    unsafe { sscratch::write(KERNEL_STACK.as_ptr() as usize); }
    ctx
}

pub fn user_entry() {
    let _a = 0x1000;
    unsafe {
        utvec::write(__alltraps_u as usize, TrapMode::Direct);
        
    }
    // unsafe { core::arch::asm!("ebreak"); }
}



#[naked]
pub unsafe extern "C" fn trap_return(ctx: &TrapContext) {
    core::arch::asm!(
r"
    .align 2
    mv sp, a0
    ld t0, 32*8(sp)
    ld t1, 33*8(sp)
    ld t2, 2*8(sp)
    csrw sstatus, t0
    csrw sepc, t1
    csrw sscratch, t2
    ld x1, 1*8(sp)
    ld x3, 3*8(sp)
    addi sp, sp, 34*8
    csrr sp, sscratch
    sret",
    options(noreturn)
    )
}



#[naked]
pub unsafe extern "C" fn __alltraps_u() {
    core::arch::asm!(
r"
    .align 2
    addi sp, sp, -34*8
    ",
    options(noreturn)
    )
}