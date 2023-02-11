#[doc = "Register `LFBCLKEN0` reader"]
pub struct R(crate::R<LFBCLKEN0_SPEC>);
impl core::ops::Deref for R {
    type Target = crate::R<LFBCLKEN0_SPEC>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
impl From<crate::R<LFBCLKEN0_SPEC>> for R {
    #[inline(always)]
    fn from(reader: crate::R<LFBCLKEN0_SPEC>) -> Self {
        R(reader)
    }
}
#[doc = "Register `LFBCLKEN0` writer"]
pub struct W(crate::W<LFBCLKEN0_SPEC>);
impl core::ops::Deref for W {
    type Target = crate::W<LFBCLKEN0_SPEC>;
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
impl From<crate::W<LFBCLKEN0_SPEC>> for W {
    #[inline(always)]
    fn from(writer: crate::W<LFBCLKEN0_SPEC>) -> Self {
        W(writer)
    }
}
#[doc = "Field `SYSTICK` reader - Clock Enable"]
pub type SYSTICK_R = crate::BitReader<bool>;
#[doc = "Field `SYSTICK` writer - Clock Enable"]
pub type SYSTICK_W<'a, const O: u8> = crate::BitWriter<'a, u32, LFBCLKEN0_SPEC, bool, O>;
#[doc = "Field `LEUART0` reader - Low Energy UART 0 Clock Enable"]
pub type LEUART0_R = crate::BitReader<bool>;
#[doc = "Field `LEUART0` writer - Low Energy UART 0 Clock Enable"]
pub type LEUART0_W<'a, const O: u8> = crate::BitWriter<'a, u32, LFBCLKEN0_SPEC, bool, O>;
#[doc = "Field `CSEN` reader - Capacitive touch sense module Clock Enable"]
pub type CSEN_R = crate::BitReader<bool>;
#[doc = "Field `CSEN` writer - Capacitive touch sense module Clock Enable"]
pub type CSEN_W<'a, const O: u8> = crate::BitWriter<'a, u32, LFBCLKEN0_SPEC, bool, O>;
impl R {
    #[doc = "Bit 0 - Clock Enable"]
    #[inline(always)]
    pub fn systick(&self) -> SYSTICK_R {
        SYSTICK_R::new((self.bits & 1) != 0)
    }
    #[doc = "Bit 1 - Low Energy UART 0 Clock Enable"]
    #[inline(always)]
    pub fn leuart0(&self) -> LEUART0_R {
        LEUART0_R::new(((self.bits >> 1) & 1) != 0)
    }
    #[doc = "Bit 2 - Capacitive touch sense module Clock Enable"]
    #[inline(always)]
    pub fn csen(&self) -> CSEN_R {
        CSEN_R::new(((self.bits >> 2) & 1) != 0)
    }
}
impl W {
    #[doc = "Bit 0 - Clock Enable"]
    #[inline(always)]
    #[must_use]
    pub fn systick(&mut self) -> SYSTICK_W<0> {
        SYSTICK_W::new(self)
    }
    #[doc = "Bit 1 - Low Energy UART 0 Clock Enable"]
    #[inline(always)]
    #[must_use]
    pub fn leuart0(&mut self) -> LEUART0_W<1> {
        LEUART0_W::new(self)
    }
    #[doc = "Bit 2 - Capacitive touch sense module Clock Enable"]
    #[inline(always)]
    #[must_use]
    pub fn csen(&mut self) -> CSEN_W<2> {
        CSEN_W::new(self)
    }
    #[doc = "Writes raw bits to the register."]
    #[inline(always)]
    pub unsafe fn bits(&mut self, bits: u32) -> &mut Self {
        self.0.bits(bits);
        self
    }
}
#[doc = "Low Frequency B Clock Enable Register 0 (Async Reg)\n\nThis register you can [`read`](crate::generic::Reg::read), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [lfbclken0](index.html) module"]
pub struct LFBCLKEN0_SPEC;
impl crate::RegisterSpec for LFBCLKEN0_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [lfbclken0::R](R) reader structure"]
impl crate::Readable for LFBCLKEN0_SPEC {
    type Reader = R;
}
#[doc = "`write(|w| ..)` method takes [lfbclken0::W](W) writer structure"]
impl crate::Writable for LFBCLKEN0_SPEC {
    type Writer = W;
    const ZERO_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
}
#[doc = "`reset()` method sets LFBCLKEN0 to value 0"]
impl crate::Resettable for LFBCLKEN0_SPEC {
    const RESET_VALUE: Self::Ux = 0;
}
