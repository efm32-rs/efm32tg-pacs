#[doc = "Register `IFC` writer"]
pub struct W(crate::W<IFC_SPEC>);
impl core::ops::Deref for W {
    type Target = crate::W<IFC_SPEC>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
impl core::ops::DerefMut for W {
    #[inline(always)]
    fn deref_mut(&mut self) -> &mut Self::Target {
        &mut self.0
    }
}
impl From<crate::W<IFC_SPEC>> for W {
    #[inline(always)]
    fn from(writer: crate::W<IFC_SPEC>) -> Self {
        W(writer)
    }
}
#[doc = "Field `HFRCORDY` writer - Clear HFRCORDY Interrupt Flag"]
pub type HFRCORDY_W<'a, const O: u8> = crate::BitWriter<'a, u32, IFC_SPEC, bool, O>;
#[doc = "Field `HFXORDY` writer - Clear HFXORDY Interrupt Flag"]
pub type HFXORDY_W<'a, const O: u8> = crate::BitWriter<'a, u32, IFC_SPEC, bool, O>;
#[doc = "Field `LFRCORDY` writer - Clear LFRCORDY Interrupt Flag"]
pub type LFRCORDY_W<'a, const O: u8> = crate::BitWriter<'a, u32, IFC_SPEC, bool, O>;
#[doc = "Field `LFXORDY` writer - Clear LFXORDY Interrupt Flag"]
pub type LFXORDY_W<'a, const O: u8> = crate::BitWriter<'a, u32, IFC_SPEC, bool, O>;
#[doc = "Field `AUXHFRCORDY` writer - Clear AUXHFRCORDY Interrupt Flag"]
pub type AUXHFRCORDY_W<'a, const O: u8> = crate::BitWriter<'a, u32, IFC_SPEC, bool, O>;
#[doc = "Field `CALRDY` writer - Clear CALRDY Interrupt Flag"]
pub type CALRDY_W<'a, const O: u8> = crate::BitWriter<'a, u32, IFC_SPEC, bool, O>;
#[doc = "Field `CALOF` writer - Clear CALOF Interrupt Flag"]
pub type CALOF_W<'a, const O: u8> = crate::BitWriter<'a, u32, IFC_SPEC, bool, O>;
#[doc = "Field `HFXODISERR` writer - Clear HFXODISERR Interrupt Flag"]
pub type HFXODISERR_W<'a, const O: u8> = crate::BitWriter<'a, u32, IFC_SPEC, bool, O>;
#[doc = "Field `HFXOAUTOSW` writer - Clear HFXOAUTOSW Interrupt Flag"]
pub type HFXOAUTOSW_W<'a, const O: u8> = crate::BitWriter<'a, u32, IFC_SPEC, bool, O>;
#[doc = "Field `HFXOPEAKDETRDY` writer - Clear HFXOPEAKDETRDY Interrupt Flag"]
pub type HFXOPEAKDETRDY_W<'a, const O: u8> = crate::BitWriter<'a, u32, IFC_SPEC, bool, O>;
#[doc = "Field `HFRCODIS` writer - Clear HFRCODIS Interrupt Flag"]
pub type HFRCODIS_W<'a, const O: u8> = crate::BitWriter<'a, u32, IFC_SPEC, bool, O>;
#[doc = "Field `LFTIMEOUTERR` writer - Clear LFTIMEOUTERR Interrupt Flag"]
pub type LFTIMEOUTERR_W<'a, const O: u8> = crate::BitWriter<'a, u32, IFC_SPEC, bool, O>;
#[doc = "Field `DPLLRDY` writer - Clear DPLLRDY Interrupt Flag"]
pub type DPLLRDY_W<'a, const O: u8> = crate::BitWriter<'a, u32, IFC_SPEC, bool, O>;
#[doc = "Field `DPLLLOCKFAILLOW` writer - Clear DPLLLOCKFAILLOW Interrupt Flag"]
pub type DPLLLOCKFAILLOW_W<'a, const O: u8> = crate::BitWriter<'a, u32, IFC_SPEC, bool, O>;
#[doc = "Field `DPLLLOCKFAILHIGH` writer - Clear DPLLLOCKFAILHIGH Interrupt Flag"]
pub type DPLLLOCKFAILHIGH_W<'a, const O: u8> = crate::BitWriter<'a, u32, IFC_SPEC, bool, O>;
#[doc = "Field `LFXOEDGE` writer - Clear LFXOEDGE Interrupt Flag"]
pub type LFXOEDGE_W<'a, const O: u8> = crate::BitWriter<'a, u32, IFC_SPEC, bool, O>;
#[doc = "Field `LFRCOEDGE` writer - Clear LFRCOEDGE Interrupt Flag"]
pub type LFRCOEDGE_W<'a, const O: u8> = crate::BitWriter<'a, u32, IFC_SPEC, bool, O>;
#[doc = "Field `ULFRCOEDGE` writer - Clear ULFRCOEDGE Interrupt Flag"]
pub type ULFRCOEDGE_W<'a, const O: u8> = crate::BitWriter<'a, u32, IFC_SPEC, bool, O>;
#[doc = "Field `CMUERR` writer - Clear CMUERR Interrupt Flag"]
pub type CMUERR_W<'a, const O: u8> = crate::BitWriter<'a, u32, IFC_SPEC, bool, O>;
impl W {
    #[doc = "Bit 0 - Clear HFRCORDY Interrupt Flag"]
    #[inline(always)]
    #[must_use]
    pub fn hfrcordy(&mut self) -> HFRCORDY_W<0> {
        HFRCORDY_W::new(self)
    }
    #[doc = "Bit 1 - Clear HFXORDY Interrupt Flag"]
    #[inline(always)]
    #[must_use]
    pub fn hfxordy(&mut self) -> HFXORDY_W<1> {
        HFXORDY_W::new(self)
    }
    #[doc = "Bit 2 - Clear LFRCORDY Interrupt Flag"]
    #[inline(always)]
    #[must_use]
    pub fn lfrcordy(&mut self) -> LFRCORDY_W<2> {
        LFRCORDY_W::new(self)
    }
    #[doc = "Bit 3 - Clear LFXORDY Interrupt Flag"]
    #[inline(always)]
    #[must_use]
    pub fn lfxordy(&mut self) -> LFXORDY_W<3> {
        LFXORDY_W::new(self)
    }
    #[doc = "Bit 4 - Clear AUXHFRCORDY Interrupt Flag"]
    #[inline(always)]
    #[must_use]
    pub fn auxhfrcordy(&mut self) -> AUXHFRCORDY_W<4> {
        AUXHFRCORDY_W::new(self)
    }
    #[doc = "Bit 5 - Clear CALRDY Interrupt Flag"]
    #[inline(always)]
    #[must_use]
    pub fn calrdy(&mut self) -> CALRDY_W<5> {
        CALRDY_W::new(self)
    }
    #[doc = "Bit 6 - Clear CALOF Interrupt Flag"]
    #[inline(always)]
    #[must_use]
    pub fn calof(&mut self) -> CALOF_W<6> {
        CALOF_W::new(self)
    }
    #[doc = "Bit 8 - Clear HFXODISERR Interrupt Flag"]
    #[inline(always)]
    #[must_use]
    pub fn hfxodiserr(&mut self) -> HFXODISERR_W<8> {
        HFXODISERR_W::new(self)
    }
    #[doc = "Bit 9 - Clear HFXOAUTOSW Interrupt Flag"]
    #[inline(always)]
    #[must_use]
    pub fn hfxoautosw(&mut self) -> HFXOAUTOSW_W<9> {
        HFXOAUTOSW_W::new(self)
    }
    #[doc = "Bit 11 - Clear HFXOPEAKDETRDY Interrupt Flag"]
    #[inline(always)]
    #[must_use]
    pub fn hfxopeakdetrdy(&mut self) -> HFXOPEAKDETRDY_W<11> {
        HFXOPEAKDETRDY_W::new(self)
    }
    #[doc = "Bit 13 - Clear HFRCODIS Interrupt Flag"]
    #[inline(always)]
    #[must_use]
    pub fn hfrcodis(&mut self) -> HFRCODIS_W<13> {
        HFRCODIS_W::new(self)
    }
    #[doc = "Bit 14 - Clear LFTIMEOUTERR Interrupt Flag"]
    #[inline(always)]
    #[must_use]
    pub fn lftimeouterr(&mut self) -> LFTIMEOUTERR_W<14> {
        LFTIMEOUTERR_W::new(self)
    }
    #[doc = "Bit 15 - Clear DPLLRDY Interrupt Flag"]
    #[inline(always)]
    #[must_use]
    pub fn dpllrdy(&mut self) -> DPLLRDY_W<15> {
        DPLLRDY_W::new(self)
    }
    #[doc = "Bit 16 - Clear DPLLLOCKFAILLOW Interrupt Flag"]
    #[inline(always)]
    #[must_use]
    pub fn dplllockfaillow(&mut self) -> DPLLLOCKFAILLOW_W<16> {
        DPLLLOCKFAILLOW_W::new(self)
    }
    #[doc = "Bit 17 - Clear DPLLLOCKFAILHIGH Interrupt Flag"]
    #[inline(always)]
    #[must_use]
    pub fn dplllockfailhigh(&mut self) -> DPLLLOCKFAILHIGH_W<17> {
        DPLLLOCKFAILHIGH_W::new(self)
    }
    #[doc = "Bit 27 - Clear LFXOEDGE Interrupt Flag"]
    #[inline(always)]
    #[must_use]
    pub fn lfxoedge(&mut self) -> LFXOEDGE_W<27> {
        LFXOEDGE_W::new(self)
    }
    #[doc = "Bit 28 - Clear LFRCOEDGE Interrupt Flag"]
    #[inline(always)]
    #[must_use]
    pub fn lfrcoedge(&mut self) -> LFRCOEDGE_W<28> {
        LFRCOEDGE_W::new(self)
    }
    #[doc = "Bit 29 - Clear ULFRCOEDGE Interrupt Flag"]
    #[inline(always)]
    #[must_use]
    pub fn ulfrcoedge(&mut self) -> ULFRCOEDGE_W<29> {
        ULFRCOEDGE_W::new(self)
    }
    #[doc = "Bit 31 - Clear CMUERR Interrupt Flag"]
    #[inline(always)]
    #[must_use]
    pub fn cmuerr(&mut self) -> CMUERR_W<31> {
        CMUERR_W::new(self)
    }
    #[doc = "Writes raw bits to the register."]
    #[inline(always)]
    pub unsafe fn bits(&mut self, bits: u32) -> &mut Self {
        self.0.bits(bits);
        self
    }
}
#[doc = "Interrupt Flag Clear Register\n\nThis register you can [`write_with_zero`](crate::generic::Reg::write_with_zero), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [ifc](index.html) module"]
pub struct IFC_SPEC;
impl crate::RegisterSpec for IFC_SPEC {
    type Ux = u32;
}
#[doc = "`write(|w| ..)` method takes [ifc::W](W) writer structure"]
impl crate::Writable for IFC_SPEC {
    type Writer = W;
    const ZERO_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
}
#[doc = "`reset()` method sets IFC to value 0"]
impl crate::Resettable for IFC_SPEC {
    const RESET_VALUE: Self::Ux = 0;
}
