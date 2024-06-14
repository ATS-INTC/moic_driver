#[doc = "Register `fetch` reader"]
pub type R = crate::R<FetchSpec>;
impl core::fmt::Debug for R {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        write!(f, "{}", self.bits())
    }
}
impl core::fmt::Debug for crate::generic::Reg<FetchSpec> {
    fn fmt(&self, f: &mut core::fmt::Formatter<'_>) -> core::fmt::Result {
        core::fmt::Debug::fmt(&self.read(), f)
    }
}
#[doc = "Fetch a task from the priority queue.\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`fetch::R`](R).  See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct FetchSpec;
impl crate::RegisterSpec for FetchSpec {
    type Ux = u64;
}
#[doc = "`read()` method returns [`fetch::R`](R) reader structure"]
impl crate::Readable for FetchSpec {}
#[doc = "`reset()` method sets fetch to value 0"]
impl crate::Resettable for FetchSpec {
    const RESET_VALUE: u64 = 0;
}
