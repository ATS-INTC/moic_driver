#[doc = "Register `switch_os` reader"]
pub type R = crate::R<SwitchOsSpec>;
impl core::fmt::Debug for R {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        write!(f, "{}", self.bits())
    }
}
impl core::fmt::Debug for crate::generic::Reg<SwitchOsSpec> {
    fn fmt(&self, f: &mut core::fmt::Formatter<'_>) -> core::fmt::Result {
        core::fmt::Debug::fmt(&self.read(), f)
    }
}
#[doc = "Switch os.\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`switch_os::R`](R).  See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct SwitchOsSpec;
impl crate::RegisterSpec for SwitchOsSpec {
    type Ux = u64;
}
#[doc = "`read()` method returns [`switch_os::R`](R) reader structure"]
impl crate::Readable for SwitchOsSpec {}
#[doc = "`reset()` method sets switch_os to value 0"]
impl crate::Resettable for SwitchOsSpec {
    const RESET_VALUE: u64 = 0;
}
