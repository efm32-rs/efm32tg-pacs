#[doc = "Register `SYNCBUSY` reader"]
pub struct R(crate::R<SYNCBUSY_SPEC>);
impl core::ops::Deref for R {
    type Target = crate::R<SYNCBUSY_SPEC>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
impl From<crate::R<SYNCBUSY_SPEC>> for R {
    #[inline(always)]
    fn from(reader: crate::R<SYNCBUSY_SPEC>) -> Self {
        R(reader)
    }
}
#[doc = "Field `CTRL` reader - CTRL Register Busy"]
pub type CTRL_R = crate::BitReader<bool>;
#[doc = "Field `BACTRL` reader - BACTRL Register Busy"]
pub type BACTRL_R = crate::BitReader<bool>;
#[doc = "Field `AREGA` reader - AREGA Register Busy"]
pub type AREGA_R = crate::BitReader<bool>;
#[doc = "Field `AREGB` reader - AREGB Register Busy"]
pub type AREGB_R = crate::BitReader<bool>;
#[doc = "Field `SEGD0L` reader - SEGD0L Register Busy"]
pub type SEGD0L_R = crate::BitReader<bool>;
#[doc = "Field `SEGD1L` reader - SEGD1L Register Busy"]
pub type SEGD1L_R = crate::BitReader<bool>;
#[doc = "Field `SEGD2L` reader - SEGD2L Register Busy"]
pub type SEGD2L_R = crate::BitReader<bool>;
#[doc = "Field `SEGD3L` reader - SEGD3L Register Busy"]
pub type SEGD3L_R = crate::BitReader<bool>;
#[doc = "Field `SEGD4L` reader - SEGD4L Register Busy"]
pub type SEGD4L_R = crate::BitReader<bool>;
#[doc = "Field `SEGD5L` reader - SEGD5L Register Busy"]
pub type SEGD5L_R = crate::BitReader<bool>;
#[doc = "Field `SEGD6L` reader - SEGD6L Register Busy"]
pub type SEGD6L_R = crate::BitReader<bool>;
#[doc = "Field `SEGD7L` reader - SEGD7L Register Busy"]
pub type SEGD7L_R = crate::BitReader<bool>;
impl R {
    #[doc = "Bit 0 - CTRL Register Busy"]
    #[inline(always)]
    pub fn ctrl(&self) -> CTRL_R {
        CTRL_R::new((self.bits & 1) != 0)
    }
    #[doc = "Bit 1 - BACTRL Register Busy"]
    #[inline(always)]
    pub fn bactrl(&self) -> BACTRL_R {
        BACTRL_R::new(((self.bits >> 1) & 1) != 0)
    }
    #[doc = "Bit 2 - AREGA Register Busy"]
    #[inline(always)]
    pub fn arega(&self) -> AREGA_R {
        AREGA_R::new(((self.bits >> 2) & 1) != 0)
    }
    #[doc = "Bit 3 - AREGB Register Busy"]
    #[inline(always)]
    pub fn aregb(&self) -> AREGB_R {
        AREGB_R::new(((self.bits >> 3) & 1) != 0)
    }
    #[doc = "Bit 4 - SEGD0L Register Busy"]
    #[inline(always)]
    pub fn segd0l(&self) -> SEGD0L_R {
        SEGD0L_R::new(((self.bits >> 4) & 1) != 0)
    }
    #[doc = "Bit 5 - SEGD1L Register Busy"]
    #[inline(always)]
    pub fn segd1l(&self) -> SEGD1L_R {
        SEGD1L_R::new(((self.bits >> 5) & 1) != 0)
    }
    #[doc = "Bit 6 - SEGD2L Register Busy"]
    #[inline(always)]
    pub fn segd2l(&self) -> SEGD2L_R {
        SEGD2L_R::new(((self.bits >> 6) & 1) != 0)
    }
    #[doc = "Bit 7 - SEGD3L Register Busy"]
    #[inline(always)]
    pub fn segd3l(&self) -> SEGD3L_R {
        SEGD3L_R::new(((self.bits >> 7) & 1) != 0)
    }
    #[doc = "Bit 16 - SEGD4L Register Busy"]
    #[inline(always)]
    pub fn segd4l(&self) -> SEGD4L_R {
        SEGD4L_R::new(((self.bits >> 16) & 1) != 0)
    }
    #[doc = "Bit 17 - SEGD5L Register Busy"]
    #[inline(always)]
    pub fn segd5l(&self) -> SEGD5L_R {
        SEGD5L_R::new(((self.bits >> 17) & 1) != 0)
    }
    #[doc = "Bit 18 - SEGD6L Register Busy"]
    #[inline(always)]
    pub fn segd6l(&self) -> SEGD6L_R {
        SEGD6L_R::new(((self.bits >> 18) & 1) != 0)
    }
    #[doc = "Bit 19 - SEGD7L Register Busy"]
    #[inline(always)]
    pub fn segd7l(&self) -> SEGD7L_R {
        SEGD7L_R::new(((self.bits >> 19) & 1) != 0)
    }
}
#[doc = "Synchronization Busy Register\n\nThis register you can [`read`](crate::generic::Reg::read). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [syncbusy](index.html) module"]
pub struct SYNCBUSY_SPEC;
impl crate::RegisterSpec for SYNCBUSY_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [syncbusy::R](R) reader structure"]
impl crate::Readable for SYNCBUSY_SPEC {
    type Reader = R;
}
#[doc = "`reset()` method sets SYNCBUSY to value 0"]
impl crate::Resettable for SYNCBUSY_SPEC {
    const RESET_VALUE: Self::Ux = 0;
}
