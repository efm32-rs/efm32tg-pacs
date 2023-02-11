#[doc = "Register `PERIODSEL` reader"]
pub struct R(crate::R<PERIODSEL_SPEC>);
impl core::ops::Deref for R {
    type Target = crate::R<PERIODSEL_SPEC>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
impl From<crate::R<PERIODSEL_SPEC>> for R {
    #[inline(always)]
    fn from(reader: crate::R<PERIODSEL_SPEC>) -> Self {
        R(reader)
    }
}
#[doc = "Register `PERIODSEL` writer"]
pub struct W(crate::W<PERIODSEL_SPEC>);
impl core::ops::Deref for W {
    type Target = crate::W<PERIODSEL_SPEC>;
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
impl From<crate::W<PERIODSEL_SPEC>> for W {
    #[inline(always)]
    fn from(writer: crate::W<PERIODSEL_SPEC>) -> Self {
        W(writer)
    }
}
#[doc = "Field `PERIODSEL` reader - Interrupts/Wakeup Events Period Setting"]
pub type PERIODSEL_R = crate::FieldReader<u8, u8>;
#[doc = "Field `PERIODSEL` writer - Interrupts/Wakeup Events Period Setting"]
pub type PERIODSEL_W<'a, const O: u8> = crate::FieldWriter<'a, u32, PERIODSEL_SPEC, u8, u8, 6, O>;
impl R {
    #[doc = "Bits 0:5 - Interrupts/Wakeup Events Period Setting"]
    #[inline(always)]
    pub fn periodsel(&self) -> PERIODSEL_R {
        PERIODSEL_R::new((self.bits & 0x3f) as u8)
    }
}
impl W {
    #[doc = "Bits 0:5 - Interrupts/Wakeup Events Period Setting"]
    #[inline(always)]
    #[must_use]
    pub fn periodsel(&mut self) -> PERIODSEL_W<0> {
        PERIODSEL_W::new(self)
    }
    #[doc = "Writes raw bits to the register."]
    #[inline(always)]
    pub unsafe fn bits(&mut self, bits: u32) -> &mut Self {
        self.0.bits(bits);
        self
    }
}
#[doc = "Interrupt Duration\n\nThis register you can [`read`](crate::generic::Reg::read), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [periodsel](index.html) module"]
pub struct PERIODSEL_SPEC;
impl crate::RegisterSpec for PERIODSEL_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [periodsel::R](R) reader structure"]
impl crate::Readable for PERIODSEL_SPEC {
    type Reader = R;
}
#[doc = "`write(|w| ..)` method takes [periodsel::W](W) writer structure"]
impl crate::Writable for PERIODSEL_SPEC {
    type Writer = W;
    const ZERO_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
}
#[doc = "`reset()` method sets PERIODSEL to value 0x20"]
impl crate::Resettable for PERIODSEL_SPEC {
    const RESET_VALUE: Self::Ux = 0x20;
}
