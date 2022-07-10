#[doc = "Register `OVSCFG` reader"]
pub struct R(crate::R<OVSCFG_SPEC>);
impl core::ops::Deref for R {
    type Target = crate::R<OVSCFG_SPEC>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
impl From<crate::R<OVSCFG_SPEC>> for R {
    #[inline(always)]
    fn from(reader: crate::R<OVSCFG_SPEC>) -> Self {
        R(reader)
    }
}
#[doc = "Register `OVSCFG` writer"]
pub struct W(crate::W<OVSCFG_SPEC>);
impl core::ops::Deref for W {
    type Target = crate::W<OVSCFG_SPEC>;
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
impl From<crate::W<OVSCFG_SPEC>> for W {
    #[inline(always)]
    fn from(writer: crate::W<OVSCFG_SPEC>) -> Self {
        W(writer)
    }
}
#[doc = "Field `FILTLEN` reader - Configure Filter Length for Inputs S0IN and S1IN"]
pub type FILTLEN_R = crate::FieldReader<u8, u8>;
#[doc = "Field `FILTLEN` writer - Configure Filter Length for Inputs S0IN and S1IN"]
pub type FILTLEN_W<'a> = crate::FieldWriter<'a, u32, OVSCFG_SPEC, u8, u8, 8, 0>;
#[doc = "Field `FLUTTERRM` reader - Flutter Remove"]
pub type FLUTTERRM_R = crate::BitReader<bool>;
#[doc = "Field `FLUTTERRM` writer - Flutter Remove"]
pub type FLUTTERRM_W<'a> = crate::BitWriter<'a, u32, OVSCFG_SPEC, bool, 12>;
impl R {
    #[doc = "Bits 0:7 - Configure Filter Length for Inputs S0IN and S1IN"]
    #[inline(always)]
    pub fn filtlen(&self) -> FILTLEN_R {
        FILTLEN_R::new((self.bits & 0xff) as u8)
    }
    #[doc = "Bit 12 - Flutter Remove"]
    #[inline(always)]
    pub fn flutterrm(&self) -> FLUTTERRM_R {
        FLUTTERRM_R::new(((self.bits >> 12) & 1) != 0)
    }
}
impl W {
    #[doc = "Bits 0:7 - Configure Filter Length for Inputs S0IN and S1IN"]
    #[inline(always)]
    pub fn filtlen(&mut self) -> FILTLEN_W {
        FILTLEN_W::new(self)
    }
    #[doc = "Bit 12 - Flutter Remove"]
    #[inline(always)]
    pub fn flutterrm(&mut self) -> FLUTTERRM_W {
        FLUTTERRM_W::new(self)
    }
    #[doc = "Writes raw bits to the register."]
    #[inline(always)]
    pub unsafe fn bits(&mut self, bits: u32) -> &mut Self {
        self.0.bits(bits);
        self
    }
}
#[doc = "Oversampling Config Register\n\nThis register you can [`read`](crate::generic::Reg::read), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [ovscfg](index.html) module"]
pub struct OVSCFG_SPEC;
impl crate::RegisterSpec for OVSCFG_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [ovscfg::R](R) reader structure"]
impl crate::Readable for OVSCFG_SPEC {
    type Reader = R;
}
#[doc = "`write(|w| ..)` method takes [ovscfg::W](W) writer structure"]
impl crate::Writable for OVSCFG_SPEC {
    type Writer = W;
}
#[doc = "`reset()` method sets OVSCFG to value 0"]
impl crate::Resettable for OVSCFG_SPEC {
    #[inline(always)]
    fn reset_value() -> Self::Ux {
        0
    }
}
