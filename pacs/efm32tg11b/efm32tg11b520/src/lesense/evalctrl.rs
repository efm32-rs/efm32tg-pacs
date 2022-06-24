#[doc = "Register `EVALCTRL` reader"]
pub struct R(crate::R<EVALCTRL_SPEC>);
impl core::ops::Deref for R {
    type Target = crate::R<EVALCTRL_SPEC>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
impl From<crate::R<EVALCTRL_SPEC>> for R {
    #[inline(always)]
    fn from(reader: crate::R<EVALCTRL_SPEC>) -> Self {
        R(reader)
    }
}
#[doc = "Register `EVALCTRL` writer"]
pub struct W(crate::W<EVALCTRL_SPEC>);
impl core::ops::Deref for W {
    type Target = crate::W<EVALCTRL_SPEC>;
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
impl From<crate::W<EVALCTRL_SPEC>> for W {
    #[inline(always)]
    fn from(writer: crate::W<EVALCTRL_SPEC>) -> Self {
        W(writer)
    }
}
#[doc = "Field `WINSIZE` reader - Sliding Window and Step Detection Size"]
pub type WINSIZE_R = crate::FieldReader<u16, u16>;
#[doc = "Field `WINSIZE` writer - Sliding Window and Step Detection Size"]
pub type WINSIZE_W<'a> = crate::FieldWriter<'a, u32, EVALCTRL_SPEC, u16, u16, 16, 0>;
impl R {
    #[doc = "Bits 0:15 - Sliding Window and Step Detection Size"]
    #[inline(always)]
    pub fn winsize(&self) -> WINSIZE_R {
        WINSIZE_R::new((self.bits & 0xffff) as u16)
    }
}
impl W {
    #[doc = "Bits 0:15 - Sliding Window and Step Detection Size"]
    #[inline(always)]
    pub fn winsize(&mut self) -> WINSIZE_W {
        WINSIZE_W::new(self)
    }
    #[doc = "Writes raw bits to the register."]
    #[inline(always)]
    pub unsafe fn bits(&mut self, bits: u32) -> &mut Self {
        self.0.bits(bits);
        self
    }
}
#[doc = "LESENSE Evaluation Control\n\nThis register you can [`read`](crate::generic::Reg::read), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [evalctrl](index.html) module"]
pub struct EVALCTRL_SPEC;
impl crate::RegisterSpec for EVALCTRL_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [evalctrl::R](R) reader structure"]
impl crate::Readable for EVALCTRL_SPEC {
    type Reader = R;
}
#[doc = "`write(|w| ..)` method takes [evalctrl::W](W) writer structure"]
impl crate::Writable for EVALCTRL_SPEC {
    type Writer = W;
}
#[doc = "`reset()` method sets EVALCTRL to value 0"]
impl crate::Resettable for EVALCTRL_SPEC {
    #[inline(always)]
    fn reset_value() -> Self::Ux {
        0
    }
}
