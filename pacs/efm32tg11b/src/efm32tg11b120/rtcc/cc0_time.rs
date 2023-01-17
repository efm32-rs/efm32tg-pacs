#[doc = "Register `CC0_TIME` reader"]
pub struct R(crate::R<CC0_TIME_SPEC>);
impl core::ops::Deref for R {
    type Target = crate::R<CC0_TIME_SPEC>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
impl From<crate::R<CC0_TIME_SPEC>> for R {
    #[inline(always)]
    fn from(reader: crate::R<CC0_TIME_SPEC>) -> Self {
        R(reader)
    }
}
#[doc = "Register `CC0_TIME` writer"]
pub struct W(crate::W<CC0_TIME_SPEC>);
impl core::ops::Deref for W {
    type Target = crate::W<CC0_TIME_SPEC>;
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
impl From<crate::W<CC0_TIME_SPEC>> for W {
    #[inline(always)]
    fn from(writer: crate::W<CC0_TIME_SPEC>) -> Self {
        W(writer)
    }
}
#[doc = "Field `SECU` reader - Seconds, Units"]
pub type SECU_R = crate::FieldReader<u8, u8>;
#[doc = "Field `SECU` writer - Seconds, Units"]
pub type SECU_W<'a, const O: u8> = crate::FieldWriter<'a, u32, CC0_TIME_SPEC, u8, u8, 4, O>;
#[doc = "Field `SECT` reader - Seconds, Tens"]
pub type SECT_R = crate::FieldReader<u8, u8>;
#[doc = "Field `SECT` writer - Seconds, Tens"]
pub type SECT_W<'a, const O: u8> = crate::FieldWriter<'a, u32, CC0_TIME_SPEC, u8, u8, 3, O>;
#[doc = "Field `MINU` reader - Minutes, Units"]
pub type MINU_R = crate::FieldReader<u8, u8>;
#[doc = "Field `MINU` writer - Minutes, Units"]
pub type MINU_W<'a, const O: u8> = crate::FieldWriter<'a, u32, CC0_TIME_SPEC, u8, u8, 4, O>;
#[doc = "Field `MINT` reader - Minutes, Tens"]
pub type MINT_R = crate::FieldReader<u8, u8>;
#[doc = "Field `MINT` writer - Minutes, Tens"]
pub type MINT_W<'a, const O: u8> = crate::FieldWriter<'a, u32, CC0_TIME_SPEC, u8, u8, 3, O>;
#[doc = "Field `HOURU` reader - Hours, Units"]
pub type HOURU_R = crate::FieldReader<u8, u8>;
#[doc = "Field `HOURU` writer - Hours, Units"]
pub type HOURU_W<'a, const O: u8> = crate::FieldWriter<'a, u32, CC0_TIME_SPEC, u8, u8, 4, O>;
#[doc = "Field `HOURT` reader - Hours, Tens"]
pub type HOURT_R = crate::FieldReader<u8, u8>;
#[doc = "Field `HOURT` writer - Hours, Tens"]
pub type HOURT_W<'a, const O: u8> = crate::FieldWriter<'a, u32, CC0_TIME_SPEC, u8, u8, 2, O>;
impl R {
    #[doc = "Bits 0:3 - Seconds, Units"]
    #[inline(always)]
    pub fn secu(&self) -> SECU_R {
        SECU_R::new((self.bits & 0x0f) as u8)
    }
    #[doc = "Bits 4:6 - Seconds, Tens"]
    #[inline(always)]
    pub fn sect(&self) -> SECT_R {
        SECT_R::new(((self.bits >> 4) & 7) as u8)
    }
    #[doc = "Bits 8:11 - Minutes, Units"]
    #[inline(always)]
    pub fn minu(&self) -> MINU_R {
        MINU_R::new(((self.bits >> 8) & 0x0f) as u8)
    }
    #[doc = "Bits 12:14 - Minutes, Tens"]
    #[inline(always)]
    pub fn mint(&self) -> MINT_R {
        MINT_R::new(((self.bits >> 12) & 7) as u8)
    }
    #[doc = "Bits 16:19 - Hours, Units"]
    #[inline(always)]
    pub fn houru(&self) -> HOURU_R {
        HOURU_R::new(((self.bits >> 16) & 0x0f) as u8)
    }
    #[doc = "Bits 20:21 - Hours, Tens"]
    #[inline(always)]
    pub fn hourt(&self) -> HOURT_R {
        HOURT_R::new(((self.bits >> 20) & 3) as u8)
    }
}
impl W {
    #[doc = "Bits 0:3 - Seconds, Units"]
    #[inline(always)]
    #[must_use]
    pub fn secu(&mut self) -> SECU_W<0> {
        SECU_W::new(self)
    }
    #[doc = "Bits 4:6 - Seconds, Tens"]
    #[inline(always)]
    #[must_use]
    pub fn sect(&mut self) -> SECT_W<4> {
        SECT_W::new(self)
    }
    #[doc = "Bits 8:11 - Minutes, Units"]
    #[inline(always)]
    #[must_use]
    pub fn minu(&mut self) -> MINU_W<8> {
        MINU_W::new(self)
    }
    #[doc = "Bits 12:14 - Minutes, Tens"]
    #[inline(always)]
    #[must_use]
    pub fn mint(&mut self) -> MINT_W<12> {
        MINT_W::new(self)
    }
    #[doc = "Bits 16:19 - Hours, Units"]
    #[inline(always)]
    #[must_use]
    pub fn houru(&mut self) -> HOURU_W<16> {
        HOURU_W::new(self)
    }
    #[doc = "Bits 20:21 - Hours, Tens"]
    #[inline(always)]
    #[must_use]
    pub fn hourt(&mut self) -> HOURT_W<20> {
        HOURT_W::new(self)
    }
    #[doc = "Writes raw bits to the register."]
    #[inline(always)]
    pub unsafe fn bits(&mut self, bits: u32) -> &mut Self {
        self.0.bits(bits);
        self
    }
}
#[doc = "Capture/Compare Time Register\n\nThis register you can [`read`](crate::generic::Reg::read), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [cc0_time](index.html) module"]
pub struct CC0_TIME_SPEC;
impl crate::RegisterSpec for CC0_TIME_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [cc0_time::R](R) reader structure"]
impl crate::Readable for CC0_TIME_SPEC {
    type Reader = R;
}
#[doc = "`write(|w| ..)` method takes [cc0_time::W](W) writer structure"]
impl crate::Writable for CC0_TIME_SPEC {
    type Writer = W;
    const ZERO_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
}
#[doc = "`reset()` method sets CC0_TIME to value 0"]
impl crate::Resettable for CC0_TIME_SPEC {
    const RESET_VALUE: Self::Ux = 0;
}
