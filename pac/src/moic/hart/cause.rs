#[doc = "Register `cause` reader"]
pub type R = crate::R<CauseSpec>;
impl core::fmt::Debug for R {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        write!(f, "{}", self.bits())
    }
}
impl core::fmt::Debug for crate::generic::Reg<CauseSpec> {
    fn fmt(&self, f: &mut core::fmt::Formatter<'_>) -> core::fmt::Result {
        core::fmt::Debug::fmt(&self.read(), f)
    }
}
#[doc = "Record the cause of interrupt.\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`cause::R`](R).  See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct CauseSpec;
impl crate::RegisterSpec for CauseSpec {
    type Ux = u64;
}
#[doc = "`read()` method returns [`cause::R`](R) reader structure"]
impl crate::Readable for CauseSpec {}
#[doc = "`reset()` method sets cause to value 0"]
impl crate::Resettable for CauseSpec {
    const RESET_VALUE: u64 = 0;
}
