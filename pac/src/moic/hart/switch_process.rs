#[doc = "Register `switch_process` reader"]
pub type R = crate::R<SwitchProcessSpec>;
impl core::fmt::Debug for R {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        write!(f, "{}", self.bits())
    }
}
impl core::fmt::Debug for crate::generic::Reg<SwitchProcessSpec> {
    fn fmt(&self, f: &mut core::fmt::Formatter<'_>) -> core::fmt::Result {
        core::fmt::Debug::fmt(&self.read(), f)
    }
}
#[doc = "Switch process.\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`switch_process::R`](R).  See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct SwitchProcessSpec;
impl crate::RegisterSpec for SwitchProcessSpec {
    type Ux = u64;
}
#[doc = "`read()` method returns [`switch_process::R`](R) reader structure"]
impl crate::Readable for SwitchProcessSpec {}
#[doc = "`reset()` method sets switch_process to value 0"]
impl crate::Resettable for SwitchProcessSpec {
    const RESET_VALUE: u64 = 0;
}
