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
#[doc = "Field `SINGLE` reader - Single Conversion Complete Interrupt Flag"]
pub type SINGLE_R = crate::BitReader<bool>;
#[doc = "Field `SCAN` reader - Scan Conversion Complete Interrupt Flag"]
pub type SCAN_R = crate::BitReader<bool>;
#[doc = "Field `SINGLEOF` reader - Single FIFO Overflow Interrupt Flag"]
pub type SINGLEOF_R = crate::BitReader<bool>;
#[doc = "Field `SCANOF` reader - Scan FIFO Overflow Interrupt Flag"]
pub type SCANOF_R = crate::BitReader<bool>;
#[doc = "Field `SINGLEUF` reader - Single FIFO Underflow Interrupt Flag"]
pub type SINGLEUF_R = crate::BitReader<bool>;
#[doc = "Field `SCANUF` reader - Scan FIFO Underflow Interrupt Flag"]
pub type SCANUF_R = crate::BitReader<bool>;
#[doc = "Field `SINGLECMP` reader - Single Result Compare Match Interrupt Flag"]
pub type SINGLECMP_R = crate::BitReader<bool>;
#[doc = "Field `SCANCMP` reader - Scan Result Compare Match Interrupt Flag"]
pub type SCANCMP_R = crate::BitReader<bool>;
#[doc = "Field `VREFOV` reader - VREF Over Voltage Interrupt Flag"]
pub type VREFOV_R = crate::BitReader<bool>;
#[doc = "Field `PROGERR` reader - Programming Error Interrupt Flag"]
pub type PROGERR_R = crate::BitReader<bool>;
#[doc = "Field `SCANEXTPEND` reader - External Scan Trigger Pending Flag"]
pub type SCANEXTPEND_R = crate::BitReader<bool>;
#[doc = "Field `SCANPEND` reader - Scan Trigger Pending Flag"]
pub type SCANPEND_R = crate::BitReader<bool>;
#[doc = "Field `PRSTIMEDERR` reader - PRS Timed Mode Error Flag"]
pub type PRSTIMEDERR_R = crate::BitReader<bool>;
#[doc = "Field `EM23ERR` reader - EM23 Entry Error Flag"]
pub type EM23ERR_R = crate::BitReader<bool>;
impl R {
    #[doc = "Bit 0 - Single Conversion Complete Interrupt Flag"]
    #[inline(always)]
    pub fn single(&self) -> SINGLE_R {
        SINGLE_R::new((self.bits & 1) != 0)
    }
    #[doc = "Bit 1 - Scan Conversion Complete Interrupt Flag"]
    #[inline(always)]
    pub fn scan(&self) -> SCAN_R {
        SCAN_R::new(((self.bits >> 1) & 1) != 0)
    }
    #[doc = "Bit 8 - Single FIFO Overflow Interrupt Flag"]
    #[inline(always)]
    pub fn singleof(&self) -> SINGLEOF_R {
        SINGLEOF_R::new(((self.bits >> 8) & 1) != 0)
    }
    #[doc = "Bit 9 - Scan FIFO Overflow Interrupt Flag"]
    #[inline(always)]
    pub fn scanof(&self) -> SCANOF_R {
        SCANOF_R::new(((self.bits >> 9) & 1) != 0)
    }
    #[doc = "Bit 10 - Single FIFO Underflow Interrupt Flag"]
    #[inline(always)]
    pub fn singleuf(&self) -> SINGLEUF_R {
        SINGLEUF_R::new(((self.bits >> 10) & 1) != 0)
    }
    #[doc = "Bit 11 - Scan FIFO Underflow Interrupt Flag"]
    #[inline(always)]
    pub fn scanuf(&self) -> SCANUF_R {
        SCANUF_R::new(((self.bits >> 11) & 1) != 0)
    }
    #[doc = "Bit 16 - Single Result Compare Match Interrupt Flag"]
    #[inline(always)]
    pub fn singlecmp(&self) -> SINGLECMP_R {
        SINGLECMP_R::new(((self.bits >> 16) & 1) != 0)
    }
    #[doc = "Bit 17 - Scan Result Compare Match Interrupt Flag"]
    #[inline(always)]
    pub fn scancmp(&self) -> SCANCMP_R {
        SCANCMP_R::new(((self.bits >> 17) & 1) != 0)
    }
    #[doc = "Bit 24 - VREF Over Voltage Interrupt Flag"]
    #[inline(always)]
    pub fn vrefov(&self) -> VREFOV_R {
        VREFOV_R::new(((self.bits >> 24) & 1) != 0)
    }
    #[doc = "Bit 25 - Programming Error Interrupt Flag"]
    #[inline(always)]
    pub fn progerr(&self) -> PROGERR_R {
        PROGERR_R::new(((self.bits >> 25) & 1) != 0)
    }
    #[doc = "Bit 26 - External Scan Trigger Pending Flag"]
    #[inline(always)]
    pub fn scanextpend(&self) -> SCANEXTPEND_R {
        SCANEXTPEND_R::new(((self.bits >> 26) & 1) != 0)
    }
    #[doc = "Bit 27 - Scan Trigger Pending Flag"]
    #[inline(always)]
    pub fn scanpend(&self) -> SCANPEND_R {
        SCANPEND_R::new(((self.bits >> 27) & 1) != 0)
    }
    #[doc = "Bit 28 - PRS Timed Mode Error Flag"]
    #[inline(always)]
    pub fn prstimederr(&self) -> PRSTIMEDERR_R {
        PRSTIMEDERR_R::new(((self.bits >> 28) & 1) != 0)
    }
    #[doc = "Bit 29 - EM23 Entry Error Flag"]
    #[inline(always)]
    pub fn em23err(&self) -> EM23ERR_R {
        EM23ERR_R::new(((self.bits >> 29) & 1) != 0)
    }
}
#[doc = "Interrupt Flag Register\n\nThis register you can [`read`](crate::generic::Reg::read). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [if_](index.html) module"]
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
