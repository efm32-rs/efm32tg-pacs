#[doc = "Register `IEN` reader"]
pub struct R(crate::R<IEN_SPEC>);
impl core::ops::Deref for R {
    type Target = crate::R<IEN_SPEC>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
impl From<crate::R<IEN_SPEC>> for R {
    #[inline(always)]
    fn from(reader: crate::R<IEN_SPEC>) -> Self {
        R(reader)
    }
}
#[doc = "Register `IEN` writer"]
pub struct W(crate::W<IEN_SPEC>);
impl core::ops::Deref for W {
    type Target = crate::W<IEN_SPEC>;
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
impl From<crate::W<IEN_SPEC>> for W {
    #[inline(always)]
    fn from(writer: crate::W<IEN_SPEC>) -> Self {
        W(writer)
    }
}
#[doc = "Field `HFRCORDY` reader - HFRCORDY Interrupt Enable"]
pub type HFRCORDY_R = crate::BitReader<bool>;
#[doc = "Field `HFRCORDY` writer - HFRCORDY Interrupt Enable"]
pub type HFRCORDY_W<'a> = crate::BitWriter<'a, u32, IEN_SPEC, bool, 0>;
#[doc = "Field `HFXORDY` reader - HFXORDY Interrupt Enable"]
pub type HFXORDY_R = crate::BitReader<bool>;
#[doc = "Field `HFXORDY` writer - HFXORDY Interrupt Enable"]
pub type HFXORDY_W<'a> = crate::BitWriter<'a, u32, IEN_SPEC, bool, 1>;
#[doc = "Field `LFRCORDY` reader - LFRCORDY Interrupt Enable"]
pub type LFRCORDY_R = crate::BitReader<bool>;
#[doc = "Field `LFRCORDY` writer - LFRCORDY Interrupt Enable"]
pub type LFRCORDY_W<'a> = crate::BitWriter<'a, u32, IEN_SPEC, bool, 2>;
#[doc = "Field `LFXORDY` reader - LFXORDY Interrupt Enable"]
pub type LFXORDY_R = crate::BitReader<bool>;
#[doc = "Field `LFXORDY` writer - LFXORDY Interrupt Enable"]
pub type LFXORDY_W<'a> = crate::BitWriter<'a, u32, IEN_SPEC, bool, 3>;
#[doc = "Field `AUXHFRCORDY` reader - AUXHFRCORDY Interrupt Enable"]
pub type AUXHFRCORDY_R = crate::BitReader<bool>;
#[doc = "Field `AUXHFRCORDY` writer - AUXHFRCORDY Interrupt Enable"]
pub type AUXHFRCORDY_W<'a> = crate::BitWriter<'a, u32, IEN_SPEC, bool, 4>;
#[doc = "Field `CALRDY` reader - CALRDY Interrupt Enable"]
pub type CALRDY_R = crate::BitReader<bool>;
#[doc = "Field `CALRDY` writer - CALRDY Interrupt Enable"]
pub type CALRDY_W<'a> = crate::BitWriter<'a, u32, IEN_SPEC, bool, 5>;
#[doc = "Field `CALOF` reader - CALOF Interrupt Enable"]
pub type CALOF_R = crate::BitReader<bool>;
#[doc = "Field `CALOF` writer - CALOF Interrupt Enable"]
pub type CALOF_W<'a> = crate::BitWriter<'a, u32, IEN_SPEC, bool, 6>;
#[doc = "Field `HFXODISERR` reader - HFXODISERR Interrupt Enable"]
pub type HFXODISERR_R = crate::BitReader<bool>;
#[doc = "Field `HFXODISERR` writer - HFXODISERR Interrupt Enable"]
pub type HFXODISERR_W<'a> = crate::BitWriter<'a, u32, IEN_SPEC, bool, 8>;
#[doc = "Field `HFXOAUTOSW` reader - HFXOAUTOSW Interrupt Enable"]
pub type HFXOAUTOSW_R = crate::BitReader<bool>;
#[doc = "Field `HFXOAUTOSW` writer - HFXOAUTOSW Interrupt Enable"]
pub type HFXOAUTOSW_W<'a> = crate::BitWriter<'a, u32, IEN_SPEC, bool, 9>;
#[doc = "Field `HFXOPEAKDETRDY` reader - HFXOPEAKDETRDY Interrupt Enable"]
pub type HFXOPEAKDETRDY_R = crate::BitReader<bool>;
#[doc = "Field `HFXOPEAKDETRDY` writer - HFXOPEAKDETRDY Interrupt Enable"]
pub type HFXOPEAKDETRDY_W<'a> = crate::BitWriter<'a, u32, IEN_SPEC, bool, 11>;
#[doc = "Field `HFRCODIS` reader - HFRCODIS Interrupt Enable"]
pub type HFRCODIS_R = crate::BitReader<bool>;
#[doc = "Field `HFRCODIS` writer - HFRCODIS Interrupt Enable"]
pub type HFRCODIS_W<'a> = crate::BitWriter<'a, u32, IEN_SPEC, bool, 13>;
#[doc = "Field `LFTIMEOUTERR` reader - LFTIMEOUTERR Interrupt Enable"]
pub type LFTIMEOUTERR_R = crate::BitReader<bool>;
#[doc = "Field `LFTIMEOUTERR` writer - LFTIMEOUTERR Interrupt Enable"]
pub type LFTIMEOUTERR_W<'a> = crate::BitWriter<'a, u32, IEN_SPEC, bool, 14>;
#[doc = "Field `DPLLRDY` reader - DPLLRDY Interrupt Enable"]
pub type DPLLRDY_R = crate::BitReader<bool>;
#[doc = "Field `DPLLRDY` writer - DPLLRDY Interrupt Enable"]
pub type DPLLRDY_W<'a> = crate::BitWriter<'a, u32, IEN_SPEC, bool, 15>;
#[doc = "Field `DPLLLOCKFAILLOW` reader - DPLLLOCKFAILLOW Interrupt Enable"]
pub type DPLLLOCKFAILLOW_R = crate::BitReader<bool>;
#[doc = "Field `DPLLLOCKFAILLOW` writer - DPLLLOCKFAILLOW Interrupt Enable"]
pub type DPLLLOCKFAILLOW_W<'a> = crate::BitWriter<'a, u32, IEN_SPEC, bool, 16>;
#[doc = "Field `DPLLLOCKFAILHIGH` reader - DPLLLOCKFAILHIGH Interrupt Enable"]
pub type DPLLLOCKFAILHIGH_R = crate::BitReader<bool>;
#[doc = "Field `DPLLLOCKFAILHIGH` writer - DPLLLOCKFAILHIGH Interrupt Enable"]
pub type DPLLLOCKFAILHIGH_W<'a> = crate::BitWriter<'a, u32, IEN_SPEC, bool, 17>;
#[doc = "Field `LFXOEDGE` reader - LFXOEDGE Interrupt Enable"]
pub type LFXOEDGE_R = crate::BitReader<bool>;
#[doc = "Field `LFXOEDGE` writer - LFXOEDGE Interrupt Enable"]
pub type LFXOEDGE_W<'a> = crate::BitWriter<'a, u32, IEN_SPEC, bool, 27>;
#[doc = "Field `LFRCOEDGE` reader - LFRCOEDGE Interrupt Enable"]
pub type LFRCOEDGE_R = crate::BitReader<bool>;
#[doc = "Field `LFRCOEDGE` writer - LFRCOEDGE Interrupt Enable"]
pub type LFRCOEDGE_W<'a> = crate::BitWriter<'a, u32, IEN_SPEC, bool, 28>;
#[doc = "Field `ULFRCOEDGE` reader - ULFRCOEDGE Interrupt Enable"]
pub type ULFRCOEDGE_R = crate::BitReader<bool>;
#[doc = "Field `ULFRCOEDGE` writer - ULFRCOEDGE Interrupt Enable"]
pub type ULFRCOEDGE_W<'a> = crate::BitWriter<'a, u32, IEN_SPEC, bool, 29>;
#[doc = "Field `CMUERR` reader - CMUERR Interrupt Enable"]
pub type CMUERR_R = crate::BitReader<bool>;
#[doc = "Field `CMUERR` writer - CMUERR Interrupt Enable"]
pub type CMUERR_W<'a> = crate::BitWriter<'a, u32, IEN_SPEC, bool, 31>;
impl R {
    #[doc = "Bit 0 - HFRCORDY Interrupt Enable"]
    #[inline(always)]
    pub fn hfrcordy(&self) -> HFRCORDY_R {
        HFRCORDY_R::new((self.bits & 1) != 0)
    }
    #[doc = "Bit 1 - HFXORDY Interrupt Enable"]
    #[inline(always)]
    pub fn hfxordy(&self) -> HFXORDY_R {
        HFXORDY_R::new(((self.bits >> 1) & 1) != 0)
    }
    #[doc = "Bit 2 - LFRCORDY Interrupt Enable"]
    #[inline(always)]
    pub fn lfrcordy(&self) -> LFRCORDY_R {
        LFRCORDY_R::new(((self.bits >> 2) & 1) != 0)
    }
    #[doc = "Bit 3 - LFXORDY Interrupt Enable"]
    #[inline(always)]
    pub fn lfxordy(&self) -> LFXORDY_R {
        LFXORDY_R::new(((self.bits >> 3) & 1) != 0)
    }
    #[doc = "Bit 4 - AUXHFRCORDY Interrupt Enable"]
    #[inline(always)]
    pub fn auxhfrcordy(&self) -> AUXHFRCORDY_R {
        AUXHFRCORDY_R::new(((self.bits >> 4) & 1) != 0)
    }
    #[doc = "Bit 5 - CALRDY Interrupt Enable"]
    #[inline(always)]
    pub fn calrdy(&self) -> CALRDY_R {
        CALRDY_R::new(((self.bits >> 5) & 1) != 0)
    }
    #[doc = "Bit 6 - CALOF Interrupt Enable"]
    #[inline(always)]
    pub fn calof(&self) -> CALOF_R {
        CALOF_R::new(((self.bits >> 6) & 1) != 0)
    }
    #[doc = "Bit 8 - HFXODISERR Interrupt Enable"]
    #[inline(always)]
    pub fn hfxodiserr(&self) -> HFXODISERR_R {
        HFXODISERR_R::new(((self.bits >> 8) & 1) != 0)
    }
    #[doc = "Bit 9 - HFXOAUTOSW Interrupt Enable"]
    #[inline(always)]
    pub fn hfxoautosw(&self) -> HFXOAUTOSW_R {
        HFXOAUTOSW_R::new(((self.bits >> 9) & 1) != 0)
    }
    #[doc = "Bit 11 - HFXOPEAKDETRDY Interrupt Enable"]
    #[inline(always)]
    pub fn hfxopeakdetrdy(&self) -> HFXOPEAKDETRDY_R {
        HFXOPEAKDETRDY_R::new(((self.bits >> 11) & 1) != 0)
    }
    #[doc = "Bit 13 - HFRCODIS Interrupt Enable"]
    #[inline(always)]
    pub fn hfrcodis(&self) -> HFRCODIS_R {
        HFRCODIS_R::new(((self.bits >> 13) & 1) != 0)
    }
    #[doc = "Bit 14 - LFTIMEOUTERR Interrupt Enable"]
    #[inline(always)]
    pub fn lftimeouterr(&self) -> LFTIMEOUTERR_R {
        LFTIMEOUTERR_R::new(((self.bits >> 14) & 1) != 0)
    }
    #[doc = "Bit 15 - DPLLRDY Interrupt Enable"]
    #[inline(always)]
    pub fn dpllrdy(&self) -> DPLLRDY_R {
        DPLLRDY_R::new(((self.bits >> 15) & 1) != 0)
    }
    #[doc = "Bit 16 - DPLLLOCKFAILLOW Interrupt Enable"]
    #[inline(always)]
    pub fn dplllockfaillow(&self) -> DPLLLOCKFAILLOW_R {
        DPLLLOCKFAILLOW_R::new(((self.bits >> 16) & 1) != 0)
    }
    #[doc = "Bit 17 - DPLLLOCKFAILHIGH Interrupt Enable"]
    #[inline(always)]
    pub fn dplllockfailhigh(&self) -> DPLLLOCKFAILHIGH_R {
        DPLLLOCKFAILHIGH_R::new(((self.bits >> 17) & 1) != 0)
    }
    #[doc = "Bit 27 - LFXOEDGE Interrupt Enable"]
    #[inline(always)]
    pub fn lfxoedge(&self) -> LFXOEDGE_R {
        LFXOEDGE_R::new(((self.bits >> 27) & 1) != 0)
    }
    #[doc = "Bit 28 - LFRCOEDGE Interrupt Enable"]
    #[inline(always)]
    pub fn lfrcoedge(&self) -> LFRCOEDGE_R {
        LFRCOEDGE_R::new(((self.bits >> 28) & 1) != 0)
    }
    #[doc = "Bit 29 - ULFRCOEDGE Interrupt Enable"]
    #[inline(always)]
    pub fn ulfrcoedge(&self) -> ULFRCOEDGE_R {
        ULFRCOEDGE_R::new(((self.bits >> 29) & 1) != 0)
    }
    #[doc = "Bit 31 - CMUERR Interrupt Enable"]
    #[inline(always)]
    pub fn cmuerr(&self) -> CMUERR_R {
        CMUERR_R::new(((self.bits >> 31) & 1) != 0)
    }
}
impl W {
    #[doc = "Bit 0 - HFRCORDY Interrupt Enable"]
    #[inline(always)]
    pub fn hfrcordy(&mut self) -> HFRCORDY_W {
        HFRCORDY_W::new(self)
    }
    #[doc = "Bit 1 - HFXORDY Interrupt Enable"]
    #[inline(always)]
    pub fn hfxordy(&mut self) -> HFXORDY_W {
        HFXORDY_W::new(self)
    }
    #[doc = "Bit 2 - LFRCORDY Interrupt Enable"]
    #[inline(always)]
    pub fn lfrcordy(&mut self) -> LFRCORDY_W {
        LFRCORDY_W::new(self)
    }
    #[doc = "Bit 3 - LFXORDY Interrupt Enable"]
    #[inline(always)]
    pub fn lfxordy(&mut self) -> LFXORDY_W {
        LFXORDY_W::new(self)
    }
    #[doc = "Bit 4 - AUXHFRCORDY Interrupt Enable"]
    #[inline(always)]
    pub fn auxhfrcordy(&mut self) -> AUXHFRCORDY_W {
        AUXHFRCORDY_W::new(self)
    }
    #[doc = "Bit 5 - CALRDY Interrupt Enable"]
    #[inline(always)]
    pub fn calrdy(&mut self) -> CALRDY_W {
        CALRDY_W::new(self)
    }
    #[doc = "Bit 6 - CALOF Interrupt Enable"]
    #[inline(always)]
    pub fn calof(&mut self) -> CALOF_W {
        CALOF_W::new(self)
    }
    #[doc = "Bit 8 - HFXODISERR Interrupt Enable"]
    #[inline(always)]
    pub fn hfxodiserr(&mut self) -> HFXODISERR_W {
        HFXODISERR_W::new(self)
    }
    #[doc = "Bit 9 - HFXOAUTOSW Interrupt Enable"]
    #[inline(always)]
    pub fn hfxoautosw(&mut self) -> HFXOAUTOSW_W {
        HFXOAUTOSW_W::new(self)
    }
    #[doc = "Bit 11 - HFXOPEAKDETRDY Interrupt Enable"]
    #[inline(always)]
    pub fn hfxopeakdetrdy(&mut self) -> HFXOPEAKDETRDY_W {
        HFXOPEAKDETRDY_W::new(self)
    }
    #[doc = "Bit 13 - HFRCODIS Interrupt Enable"]
    #[inline(always)]
    pub fn hfrcodis(&mut self) -> HFRCODIS_W {
        HFRCODIS_W::new(self)
    }
    #[doc = "Bit 14 - LFTIMEOUTERR Interrupt Enable"]
    #[inline(always)]
    pub fn lftimeouterr(&mut self) -> LFTIMEOUTERR_W {
        LFTIMEOUTERR_W::new(self)
    }
    #[doc = "Bit 15 - DPLLRDY Interrupt Enable"]
    #[inline(always)]
    pub fn dpllrdy(&mut self) -> DPLLRDY_W {
        DPLLRDY_W::new(self)
    }
    #[doc = "Bit 16 - DPLLLOCKFAILLOW Interrupt Enable"]
    #[inline(always)]
    pub fn dplllockfaillow(&mut self) -> DPLLLOCKFAILLOW_W {
        DPLLLOCKFAILLOW_W::new(self)
    }
    #[doc = "Bit 17 - DPLLLOCKFAILHIGH Interrupt Enable"]
    #[inline(always)]
    pub fn dplllockfailhigh(&mut self) -> DPLLLOCKFAILHIGH_W {
        DPLLLOCKFAILHIGH_W::new(self)
    }
    #[doc = "Bit 27 - LFXOEDGE Interrupt Enable"]
    #[inline(always)]
    pub fn lfxoedge(&mut self) -> LFXOEDGE_W {
        LFXOEDGE_W::new(self)
    }
    #[doc = "Bit 28 - LFRCOEDGE Interrupt Enable"]
    #[inline(always)]
    pub fn lfrcoedge(&mut self) -> LFRCOEDGE_W {
        LFRCOEDGE_W::new(self)
    }
    #[doc = "Bit 29 - ULFRCOEDGE Interrupt Enable"]
    #[inline(always)]
    pub fn ulfrcoedge(&mut self) -> ULFRCOEDGE_W {
        ULFRCOEDGE_W::new(self)
    }
    #[doc = "Bit 31 - CMUERR Interrupt Enable"]
    #[inline(always)]
    pub fn cmuerr(&mut self) -> CMUERR_W {
        CMUERR_W::new(self)
    }
    #[doc = "Writes raw bits to the register."]
    #[inline(always)]
    pub unsafe fn bits(&mut self, bits: u32) -> &mut Self {
        self.0.bits(bits);
        self
    }
}
#[doc = "Interrupt Enable Register\n\nThis register you can [`read`](crate::generic::Reg::read), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [ien](index.html) module"]
pub struct IEN_SPEC;
impl crate::RegisterSpec for IEN_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [ien::R](R) reader structure"]
impl crate::Readable for IEN_SPEC {
    type Reader = R;
}
#[doc = "`write(|w| ..)` method takes [ien::W](W) writer structure"]
impl crate::Writable for IEN_SPEC {
    type Writer = W;
}
#[doc = "`reset()` method sets IEN to value 0"]
impl crate::Resettable for IEN_SPEC {
    #[inline(always)]
    fn reset_value() -> Self::Ux {
        0
    }
}
