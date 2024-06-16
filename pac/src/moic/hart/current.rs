#[doc = "Register `current` reader"]
pub type R = crate::R<CurrentSpec>;
impl core::fmt::Debug for R {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        write!(f, "{}", self.bits())
    }
}
impl core::fmt::Debug for crate::generic::Reg<CurrentSpec> {
    fn fmt(&self, f: &mut core::fmt::Formatter<'_>) -> core::fmt::Result {
        core::fmt::Debug::fmt(&self.read(), f)
    }
}
#[doc = "Get the current task.\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`current::R`](R).  See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct CurrentSpec;
impl crate::RegisterSpec for CurrentSpec {
    type Ux = u64;
}
#[doc = "`read()` method returns [`current::R`](R) reader structure"]
impl crate::Readable for CurrentSpec {}
#[doc = "`reset()` method sets current to value 0"]
impl crate::Resettable for CurrentSpec {
    const RESET_VALUE: u64 = 0;
}
