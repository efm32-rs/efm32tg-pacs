#[doc = "Register `DCDCCLIMCTRL` reader"]
pub struct R(crate::R<DCDCCLIMCTRL_SPEC>);
impl core::ops::Deref for R {
    type Target = crate::R<DCDCCLIMCTRL_SPEC>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
impl From<crate::R<DCDCCLIMCTRL_SPEC>> for R {
    #[inline(always)]
    fn from(reader: crate::R<DCDCCLIMCTRL_SPEC>) -> Self {
        R(reader)
    }
}
#[doc = "Register `DCDCCLIMCTRL` writer"]
pub struct W(crate::W<DCDCCLIMCTRL_SPEC>);
impl core::ops::Deref for W {
    type Target = crate::W<DCDCCLIMCTRL_SPEC>;
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
impl From<crate::W<DCDCCLIMCTRL_SPEC>> for W {
    #[inline(always)]
    fn from(writer: crate::W<DCDCCLIMCTRL_SPEC>) -> Self {
        W(writer)
    }
}
#[doc = "Field `CLIMBLANKDLY` reader - Reserved for internal use. Do not change."]
pub type CLIMBLANKDLY_R = crate::FieldReader<u8, u8>;
#[doc = "Field `CLIMBLANKDLY` writer - Reserved for internal use. Do not change."]
pub type CLIMBLANKDLY_W<'a, const O: u8> =
    crate::FieldWriter<'a, u32, DCDCCLIMCTRL_SPEC, u8, u8, 2, O>;
#[doc = "Field `BYPLIMEN` reader - Bypass Current Limit Enable"]
pub type BYPLIMEN_R = crate::BitReader<bool>;
#[doc = "Field `BYPLIMEN` writer - Bypass Current Limit Enable"]
pub type BYPLIMEN_W<'a, const O: u8> = crate::BitWriter<'a, u32, DCDCCLIMCTRL_SPEC, bool, O>;
impl R {
    #[doc = "Bits 8:9 - Reserved for internal use. Do not change."]
    #[inline(always)]
    pub fn climblankdly(&self) -> CLIMBLANKDLY_R {
        CLIMBLANKDLY_R::new(((self.bits >> 8) & 3) as u8)
    }
    #[doc = "Bit 13 - Bypass Current Limit Enable"]
    #[inline(always)]
    pub fn byplimen(&self) -> BYPLIMEN_R {
        BYPLIMEN_R::new(((self.bits >> 13) & 1) != 0)
    }
}
impl W {
    #[doc = "Bits 8:9 - Reserved for internal use. Do not change."]
    #[inline(always)]
    #[must_use]
    pub fn climblankdly(&mut self) -> CLIMBLANKDLY_W<8> {
        CLIMBLANKDLY_W::new(self)
    }
    #[doc = "Bit 13 - Bypass Current Limit Enable"]
    #[inline(always)]
    #[must_use]
    pub fn byplimen(&mut self) -> BYPLIMEN_W<13> {
        BYPLIMEN_W::new(self)
    }
    #[doc = "Writes raw bits to the register."]
    #[inline(always)]
    pub unsafe fn bits(&mut self, bits: u32) -> &mut Self {
        self.0.bits(bits);
        self
    }
}
#[doc = "DCDC Power Train PFET Current Limiter Control Register\n\nThis register you can [`read`](crate::generic::Reg::read), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [dcdcclimctrl](index.html) module"]
pub struct DCDCCLIMCTRL_SPEC;
impl crate::RegisterSpec for DCDCCLIMCTRL_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [dcdcclimctrl::R](R) reader structure"]
impl crate::Readable for DCDCCLIMCTRL_SPEC {
    type Reader = R;
}
#[doc = "`write(|w| ..)` method takes [dcdcclimctrl::W](W) writer structure"]
impl crate::Writable for DCDCCLIMCTRL_SPEC {
    type Writer = W;
    const ZERO_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
}
#[doc = "`reset()` method sets DCDCCLIMCTRL to value 0x0100"]
impl crate::Resettable for DCDCCLIMCTRL_SPEC {
    const RESET_VALUE: Self::Ux = 0x0100;
}
