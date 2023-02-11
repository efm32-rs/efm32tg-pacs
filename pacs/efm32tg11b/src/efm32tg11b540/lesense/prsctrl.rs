#[doc = "Register `PRSCTRL` reader"]
pub struct R(crate::R<PRSCTRL_SPEC>);
impl core::ops::Deref for R {
    type Target = crate::R<PRSCTRL_SPEC>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
impl From<crate::R<PRSCTRL_SPEC>> for R {
    #[inline(always)]
    fn from(reader: crate::R<PRSCTRL_SPEC>) -> Self {
        R(reader)
    }
}
#[doc = "Register `PRSCTRL` writer"]
pub struct W(crate::W<PRSCTRL_SPEC>);
impl core::ops::Deref for W {
    type Target = crate::W<PRSCTRL_SPEC>;
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
impl From<crate::W<PRSCTRL_SPEC>> for W {
    #[inline(always)]
    fn from(writer: crate::W<PRSCTRL_SPEC>) -> Self {
        W(writer)
    }
}
#[doc = "Field `DECCMPVAL` reader - Decoder State Compare Value"]
pub type DECCMPVAL_R = crate::FieldReader<u8, u8>;
#[doc = "Field `DECCMPVAL` writer - Decoder State Compare Value"]
pub type DECCMPVAL_W<'a, const O: u8> = crate::FieldWriter<'a, u32, PRSCTRL_SPEC, u8, u8, 5, O>;
#[doc = "Field `DECCMPMASK` reader - Decoder State Compare Value Mask"]
pub type DECCMPMASK_R = crate::FieldReader<u8, u8>;
#[doc = "Field `DECCMPMASK` writer - Decoder State Compare Value Mask"]
pub type DECCMPMASK_W<'a, const O: u8> = crate::FieldWriter<'a, u32, PRSCTRL_SPEC, u8, u8, 5, O>;
#[doc = "Field `DECCMPEN` reader - Enable PRS Output DECCMP"]
pub type DECCMPEN_R = crate::BitReader<bool>;
#[doc = "Field `DECCMPEN` writer - Enable PRS Output DECCMP"]
pub type DECCMPEN_W<'a, const O: u8> = crate::BitWriter<'a, u32, PRSCTRL_SPEC, bool, O>;
impl R {
    #[doc = "Bits 0:4 - Decoder State Compare Value"]
    #[inline(always)]
    pub fn deccmpval(&self) -> DECCMPVAL_R {
        DECCMPVAL_R::new((self.bits & 0x1f) as u8)
    }
    #[doc = "Bits 8:12 - Decoder State Compare Value Mask"]
    #[inline(always)]
    pub fn deccmpmask(&self) -> DECCMPMASK_R {
        DECCMPMASK_R::new(((self.bits >> 8) & 0x1f) as u8)
    }
    #[doc = "Bit 16 - Enable PRS Output DECCMP"]
    #[inline(always)]
    pub fn deccmpen(&self) -> DECCMPEN_R {
        DECCMPEN_R::new(((self.bits >> 16) & 1) != 0)
    }
}
impl W {
    #[doc = "Bits 0:4 - Decoder State Compare Value"]
    #[inline(always)]
    #[must_use]
    pub fn deccmpval(&mut self) -> DECCMPVAL_W<0> {
        DECCMPVAL_W::new(self)
    }
    #[doc = "Bits 8:12 - Decoder State Compare Value Mask"]
    #[inline(always)]
    #[must_use]
    pub fn deccmpmask(&mut self) -> DECCMPMASK_W<8> {
        DECCMPMASK_W::new(self)
    }
    #[doc = "Bit 16 - Enable PRS Output DECCMP"]
    #[inline(always)]
    #[must_use]
    pub fn deccmpen(&mut self) -> DECCMPEN_W<16> {
        DECCMPEN_W::new(self)
    }
    #[doc = "Writes raw bits to the register."]
    #[inline(always)]
    pub unsafe fn bits(&mut self, bits: u32) -> &mut Self {
        self.0.bits(bits);
        self
    }
}
#[doc = "PRS Control Register\n\nThis register you can [`read`](crate::generic::Reg::read), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [prsctrl](index.html) module"]
pub struct PRSCTRL_SPEC;
impl crate::RegisterSpec for PRSCTRL_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [prsctrl::R](R) reader structure"]
impl crate::Readable for PRSCTRL_SPEC {
    type Reader = R;
}
#[doc = "`write(|w| ..)` method takes [prsctrl::W](W) writer structure"]
impl crate::Writable for PRSCTRL_SPEC {
    type Writer = W;
    const ZERO_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
}
#[doc = "`reset()` method sets PRSCTRL to value 0"]
impl crate::Resettable for PRSCTRL_SPEC {
    const RESET_VALUE: Self::Ux = 0;
}
