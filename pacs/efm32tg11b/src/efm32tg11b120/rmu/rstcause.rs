#[doc = "Register `RSTCAUSE` reader"]
pub struct R(crate::R<RSTCAUSE_SPEC>);
impl core::ops::Deref for R {
    type Target = crate::R<RSTCAUSE_SPEC>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
impl From<crate::R<RSTCAUSE_SPEC>> for R {
    #[inline(always)]
    fn from(reader: crate::R<RSTCAUSE_SPEC>) -> Self {
        R(reader)
    }
}
#[doc = "Field `PORST` reader - Power on Reset"]
pub type PORST_R = crate::BitReader<bool>;
#[doc = "Field `AVDDBOD` reader - Brown Out Detector AVDD Reset"]
pub type AVDDBOD_R = crate::BitReader<bool>;
#[doc = "Field `DVDDBOD` reader - Brown Out Detector DVDD Reset"]
pub type DVDDBOD_R = crate::BitReader<bool>;
#[doc = "Field `DECBOD` reader - Brown Out Detector Decouple Domain Reset"]
pub type DECBOD_R = crate::BitReader<bool>;
#[doc = "Field `EXTRST` reader - External Pin Reset"]
pub type EXTRST_R = crate::BitReader<bool>;
#[doc = "Field `LOCKUPRST` reader - LOCKUP Reset"]
pub type LOCKUPRST_R = crate::BitReader<bool>;
#[doc = "Field `SYSREQRST` reader - System Request Reset"]
pub type SYSREQRST_R = crate::BitReader<bool>;
#[doc = "Field `WDOGRST` reader - Watchdog Reset"]
pub type WDOGRST_R = crate::BitReader<bool>;
#[doc = "Field `BUMODERST` reader - Backup Mode Reset"]
pub type BUMODERST_R = crate::BitReader<bool>;
#[doc = "Field `EM4RST` reader - EM4 Reset"]
pub type EM4RST_R = crate::BitReader<bool>;
impl R {
    #[doc = "Bit 0 - Power on Reset"]
    #[inline(always)]
    pub fn porst(&self) -> PORST_R {
        PORST_R::new((self.bits & 1) != 0)
    }
    #[doc = "Bit 2 - Brown Out Detector AVDD Reset"]
    #[inline(always)]
    pub fn avddbod(&self) -> AVDDBOD_R {
        AVDDBOD_R::new(((self.bits >> 2) & 1) != 0)
    }
    #[doc = "Bit 3 - Brown Out Detector DVDD Reset"]
    #[inline(always)]
    pub fn dvddbod(&self) -> DVDDBOD_R {
        DVDDBOD_R::new(((self.bits >> 3) & 1) != 0)
    }
    #[doc = "Bit 4 - Brown Out Detector Decouple Domain Reset"]
    #[inline(always)]
    pub fn decbod(&self) -> DECBOD_R {
        DECBOD_R::new(((self.bits >> 4) & 1) != 0)
    }
    #[doc = "Bit 8 - External Pin Reset"]
    #[inline(always)]
    pub fn extrst(&self) -> EXTRST_R {
        EXTRST_R::new(((self.bits >> 8) & 1) != 0)
    }
    #[doc = "Bit 9 - LOCKUP Reset"]
    #[inline(always)]
    pub fn lockuprst(&self) -> LOCKUPRST_R {
        LOCKUPRST_R::new(((self.bits >> 9) & 1) != 0)
    }
    #[doc = "Bit 10 - System Request Reset"]
    #[inline(always)]
    pub fn sysreqrst(&self) -> SYSREQRST_R {
        SYSREQRST_R::new(((self.bits >> 10) & 1) != 0)
    }
    #[doc = "Bit 11 - Watchdog Reset"]
    #[inline(always)]
    pub fn wdogrst(&self) -> WDOGRST_R {
        WDOGRST_R::new(((self.bits >> 11) & 1) != 0)
    }
    #[doc = "Bit 12 - Backup Mode Reset"]
    #[inline(always)]
    pub fn bumoderst(&self) -> BUMODERST_R {
        BUMODERST_R::new(((self.bits >> 12) & 1) != 0)
    }
    #[doc = "Bit 16 - EM4 Reset"]
    #[inline(always)]
    pub fn em4rst(&self) -> EM4RST_R {
        EM4RST_R::new(((self.bits >> 16) & 1) != 0)
    }
}
#[doc = "Reset Cause Register\n\nThis register you can [`read`](crate::generic::Reg::read). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [rstcause](index.html) module"]
pub struct RSTCAUSE_SPEC;
impl crate::RegisterSpec for RSTCAUSE_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [rstcause::R](R) reader structure"]
impl crate::Readable for RSTCAUSE_SPEC {
    type Reader = R;
}
#[doc = "`reset()` method sets RSTCAUSE to value 0"]
impl crate::Resettable for RSTCAUSE_SPEC {
    const RESET_VALUE: Self::Ux = 0;
}
