#[doc = "Register `CH1DATA` reader"]
pub struct R(crate::R<CH1DATA_SPEC>);
impl core::ops::Deref for R {
    type Target = crate::R<CH1DATA_SPEC>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
impl From<crate::R<CH1DATA_SPEC>> for R {
    #[inline(always)]
    fn from(reader: crate::R<CH1DATA_SPEC>) -> Self {
        R(reader)
    }
}
#[doc = "Register `CH1DATA` writer"]
pub struct W(crate::W<CH1DATA_SPEC>);
impl core::ops::Deref for W {
    type Target = crate::W<CH1DATA_SPEC>;
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
impl From<crate::W<CH1DATA_SPEC>> for W {
    #[inline(always)]
    fn from(writer: crate::W<CH1DATA_SPEC>) -> Self {
        W(writer)
    }
}
#[doc = "Field `DATA` reader - Channel 1 Data"]
pub type DATA_R = crate::FieldReader<u16, u16>;
#[doc = "Field `DATA` writer - Channel 1 Data"]
pub type DATA_W<'a, const O: u8> = crate::FieldWriter<'a, u32, CH1DATA_SPEC, u16, u16, 12, O>;
impl R {
    #[doc = "Bits 0:11 - Channel 1 Data"]
    #[inline(always)]
    pub fn data(&self) -> DATA_R {
        DATA_R::new((self.bits & 0x0fff) as u16)
    }
}
impl W {
    #[doc = "Bits 0:11 - Channel 1 Data"]
    #[inline(always)]
    #[must_use]
    pub fn data(&mut self) -> DATA_W<0> {
        DATA_W::new(self)
    }
    #[doc = "Writes raw bits to the register."]
    #[inline(always)]
    pub unsafe fn bits(&mut self, bits: u32) -> &mut Self {
        self.0.bits(bits);
        self
    }
}
#[doc = "Channel 1 Data Register\n\nThis register you can [`read`](crate::generic::Reg::read), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [ch1data](index.html) module"]
pub struct CH1DATA_SPEC;
impl crate::RegisterSpec for CH1DATA_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [ch1data::R](R) reader structure"]
impl crate::Readable for CH1DATA_SPEC {
    type Reader = R;
}
#[doc = "`write(|w| ..)` method takes [ch1data::W](W) writer structure"]
impl crate::Writable for CH1DATA_SPEC {
    type Writer = W;
    const ZERO_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
}
#[doc = "`reset()` method sets CH1DATA to value 0x0800"]
impl crate::Resettable for CH1DATA_SPEC {
    const RESET_VALUE: Self::Ux = 0x0800;
}
