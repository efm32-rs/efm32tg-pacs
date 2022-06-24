#[doc = "Register `IF` reader"]
pub struct R(crate::R<IF_SPEC>);
impl core::ops::Deref for R {
    type Target = crate::R<IF_SPEC>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
impl From<crate::R<IF_SPEC>> for R {
    #[inline(always)]
    fn from(reader: crate::R<IF_SPEC>) -> Self {
        R(reader)
    }
}
#[doc = "Field `TOUT` reader - WDOG Timeout Interrupt Flag"]
pub type TOUT_R = crate::BitReader<bool>;
#[doc = "Field `WARN` reader - WDOG Warning Timeout Interrupt Flag"]
pub type WARN_R = crate::BitReader<bool>;
#[doc = "Field `WIN` reader - WDOG Window Interrupt Flag"]
pub type WIN_R = crate::BitReader<bool>;
#[doc = "Field `PEM0` reader - PRS Channel Zero Event Missing Interrupt Flag"]
pub type PEM0_R = crate::BitReader<bool>;
#[doc = "Field `PEM1` reader - PRS Channel One Event Missing Interrupt Flag"]
pub type PEM1_R = crate::BitReader<bool>;
impl R {
    #[doc = "Bit 0 - WDOG Timeout Interrupt Flag"]
    #[inline(always)]
    pub fn tout(&self) -> TOUT_R {
        TOUT_R::new((self.bits & 1) != 0)
    }
    #[doc = "Bit 1 - WDOG Warning Timeout Interrupt Flag"]
    #[inline(always)]
    pub fn warn(&self) -> WARN_R {
        WARN_R::new(((self.bits >> 1) & 1) != 0)
    }
    #[doc = "Bit 2 - WDOG Window Interrupt Flag"]
    #[inline(always)]
    pub fn win(&self) -> WIN_R {
        WIN_R::new(((self.bits >> 2) & 1) != 0)
    }
    #[doc = "Bit 3 - PRS Channel Zero Event Missing Interrupt Flag"]
    #[inline(always)]
    pub fn pem0(&self) -> PEM0_R {
        PEM0_R::new(((self.bits >> 3) & 1) != 0)
    }
    #[doc = "Bit 4 - PRS Channel One Event Missing Interrupt Flag"]
    #[inline(always)]
    pub fn pem1(&self) -> PEM1_R {
        PEM1_R::new(((self.bits >> 4) & 1) != 0)
    }
}
#[doc = "Watchdog Interrupt Flags\n\nThis register you can [`read`](crate::generic::Reg::read). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [if_](index.html) module"]
pub struct IF_SPEC;
impl crate::RegisterSpec for IF_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [if_::R](R) reader structure"]
impl crate::Readable for IF_SPEC {
    type Reader = R;
}
#[doc = "`reset()` method sets IF to value 0"]
impl crate::Resettable for IF_SPEC {
    #[inline(always)]
    fn reset_value() -> Self::Ux {
        0
    }
}
