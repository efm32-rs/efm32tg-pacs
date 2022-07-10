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
#[doc = "Field `HFRCORDY` reader - HFRCO Ready Interrupt Flag"]
pub type HFRCORDY_R = crate::BitReader<bool>;
#[doc = "Field `HFXORDY` reader - HFXO Ready Interrupt Flag"]
pub type HFXORDY_R = crate::BitReader<bool>;
#[doc = "Field `LFRCORDY` reader - LFRCO Ready Interrupt Flag"]
pub type LFRCORDY_R = crate::BitReader<bool>;
#[doc = "Field `LFXORDY` reader - LFXO Ready Interrupt Flag"]
pub type LFXORDY_R = crate::BitReader<bool>;
#[doc = "Field `AUXHFRCORDY` reader - AUXHFRCO Ready Interrupt Flag"]
pub type AUXHFRCORDY_R = crate::BitReader<bool>;
#[doc = "Field `CALRDY` reader - Calibration Ready Interrupt Flag"]
pub type CALRDY_R = crate::BitReader<bool>;
#[doc = "Field `CALOF` reader - Calibration Overflow Interrupt Flag"]
pub type CALOF_R = crate::BitReader<bool>;
#[doc = "Field `HFXODISERR` reader - HFXO Disable Error Interrupt Flag"]
pub type HFXODISERR_R = crate::BitReader<bool>;
#[doc = "Field `HFXOAUTOSW` reader - HFXO Automatic Switch Interrupt Flag"]
pub type HFXOAUTOSW_R = crate::BitReader<bool>;
#[doc = "Field `HFXOPEAKDETRDY` reader - HFXO Automatic Peak Detection Ready Interrupt Flag"]
pub type HFXOPEAKDETRDY_R = crate::BitReader<bool>;
#[doc = "Field `HFRCODIS` reader - HFRCO Disable Interrupt Flag"]
pub type HFRCODIS_R = crate::BitReader<bool>;
#[doc = "Field `LFTIMEOUTERR` reader - Low Frequency Timeout Error Interrupt Flag"]
pub type LFTIMEOUTERR_R = crate::BitReader<bool>;
#[doc = "Field `DPLLRDY` reader - DPLL Lock Interrupt Flag"]
pub type DPLLRDY_R = crate::BitReader<bool>;
#[doc = "Field `DPLLLOCKFAILLOW` reader - DPLL Lock Failure Low Interrupt Flag"]
pub type DPLLLOCKFAILLOW_R = crate::BitReader<bool>;
#[doc = "Field `DPLLLOCKFAILHIGH` reader - DPLL Lock Failure Low Interrupt Flag"]
pub type DPLLLOCKFAILHIGH_R = crate::BitReader<bool>;
#[doc = "Field `LFXOEDGE` reader - LFXO Clock Edge Detected Interrupt Flag"]
pub type LFXOEDGE_R = crate::BitReader<bool>;
#[doc = "Field `LFRCOEDGE` reader - LFRCO Clock Edge Detected Interrupt Flag"]
pub type LFRCOEDGE_R = crate::BitReader<bool>;
#[doc = "Field `ULFRCOEDGE` reader - ULFRCO Clock Edge Detected Interrupt Flag"]
pub type ULFRCOEDGE_R = crate::BitReader<bool>;
#[doc = "Field `CMUERR` reader - CMU Error Interrupt Flag"]
pub type CMUERR_R = crate::BitReader<bool>;
impl R {
    #[doc = "Bit 0 - HFRCO Ready Interrupt Flag"]
    #[inline(always)]
    pub fn hfrcordy(&self) -> HFRCORDY_R {
        HFRCORDY_R::new((self.bits & 1) != 0)
    }
    #[doc = "Bit 1 - HFXO Ready Interrupt Flag"]
    #[inline(always)]
    pub fn hfxordy(&self) -> HFXORDY_R {
        HFXORDY_R::new(((self.bits >> 1) & 1) != 0)
    }
    #[doc = "Bit 2 - LFRCO Ready Interrupt Flag"]
    #[inline(always)]
    pub fn lfrcordy(&self) -> LFRCORDY_R {
        LFRCORDY_R::new(((self.bits >> 2) & 1) != 0)
    }
    #[doc = "Bit 3 - LFXO Ready Interrupt Flag"]
    #[inline(always)]
    pub fn lfxordy(&self) -> LFXORDY_R {
        LFXORDY_R::new(((self.bits >> 3) & 1) != 0)
    }
    #[doc = "Bit 4 - AUXHFRCO Ready Interrupt Flag"]
    #[inline(always)]
    pub fn auxhfrcordy(&self) -> AUXHFRCORDY_R {
        AUXHFRCORDY_R::new(((self.bits >> 4) & 1) != 0)
    }
    #[doc = "Bit 5 - Calibration Ready Interrupt Flag"]
    #[inline(always)]
    pub fn calrdy(&self) -> CALRDY_R {
        CALRDY_R::new(((self.bits >> 5) & 1) != 0)
    }
    #[doc = "Bit 6 - Calibration Overflow Interrupt Flag"]
    #[inline(always)]
    pub fn calof(&self) -> CALOF_R {
        CALOF_R::new(((self.bits >> 6) & 1) != 0)
    }
    #[doc = "Bit 8 - HFXO Disable Error Interrupt Flag"]
    #[inline(always)]
    pub fn hfxodiserr(&self) -> HFXODISERR_R {
        HFXODISERR_R::new(((self.bits >> 8) & 1) != 0)
    }
    #[doc = "Bit 9 - HFXO Automatic Switch Interrupt Flag"]
    #[inline(always)]
    pub fn hfxoautosw(&self) -> HFXOAUTOSW_R {
        HFXOAUTOSW_R::new(((self.bits >> 9) & 1) != 0)
    }
    #[doc = "Bit 11 - HFXO Automatic Peak Detection Ready Interrupt Flag"]
    #[inline(always)]
    pub fn hfxopeakdetrdy(&self) -> HFXOPEAKDETRDY_R {
        HFXOPEAKDETRDY_R::new(((self.bits >> 11) & 1) != 0)
    }
    #[doc = "Bit 13 - HFRCO Disable Interrupt Flag"]
    #[inline(always)]
    pub fn hfrcodis(&self) -> HFRCODIS_R {
        HFRCODIS_R::new(((self.bits >> 13) & 1) != 0)
    }
    #[doc = "Bit 14 - Low Frequency Timeout Error Interrupt Flag"]
    #[inline(always)]
    pub fn lftimeouterr(&self) -> LFTIMEOUTERR_R {
        LFTIMEOUTERR_R::new(((self.bits >> 14) & 1) != 0)
    }
    #[doc = "Bit 15 - DPLL Lock Interrupt Flag"]
    #[inline(always)]
    pub fn dpllrdy(&self) -> DPLLRDY_R {
        DPLLRDY_R::new(((self.bits >> 15) & 1) != 0)
    }
    #[doc = "Bit 16 - DPLL Lock Failure Low Interrupt Flag"]
    #[inline(always)]
    pub fn dplllockfaillow(&self) -> DPLLLOCKFAILLOW_R {
        DPLLLOCKFAILLOW_R::new(((self.bits >> 16) & 1) != 0)
    }
    #[doc = "Bit 17 - DPLL Lock Failure Low Interrupt Flag"]
    #[inline(always)]
    pub fn dplllockfailhigh(&self) -> DPLLLOCKFAILHIGH_R {
        DPLLLOCKFAILHIGH_R::new(((self.bits >> 17) & 1) != 0)
    }
    #[doc = "Bit 27 - LFXO Clock Edge Detected Interrupt Flag"]
    #[inline(always)]
    pub fn lfxoedge(&self) -> LFXOEDGE_R {
        LFXOEDGE_R::new(((self.bits >> 27) & 1) != 0)
    }
    #[doc = "Bit 28 - LFRCO Clock Edge Detected Interrupt Flag"]
    #[inline(always)]
    pub fn lfrcoedge(&self) -> LFRCOEDGE_R {
        LFRCOEDGE_R::new(((self.bits >> 28) & 1) != 0)
    }
    #[doc = "Bit 29 - ULFRCO Clock Edge Detected Interrupt Flag"]
    #[inline(always)]
    pub fn ulfrcoedge(&self) -> ULFRCOEDGE_R {
        ULFRCOEDGE_R::new(((self.bits >> 29) & 1) != 0)
    }
    #[doc = "Bit 31 - CMU Error Interrupt Flag"]
    #[inline(always)]
    pub fn cmuerr(&self) -> CMUERR_R {
        CMUERR_R::new(((self.bits >> 31) & 1) != 0)
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
#[doc = "`reset()` method sets IF to value 0x01"]
impl crate::Resettable for IF_SPEC {
    #[inline(always)]
    fn reset_value() -> Self::Ux {
        0x01
    }
}
