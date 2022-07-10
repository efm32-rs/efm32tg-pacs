#[doc = "Register `DPLLCTRL1` reader"]
pub struct R(crate::R<DPLLCTRL1_SPEC>);
impl core::ops::Deref for R {
    type Target = crate::R<DPLLCTRL1_SPEC>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
impl From<crate::R<DPLLCTRL1_SPEC>> for R {
    #[inline(always)]
    fn from(reader: crate::R<DPLLCTRL1_SPEC>) -> Self {
        R(reader)
    }
}
#[doc = "Register `DPLLCTRL1` writer"]
pub struct W(crate::W<DPLLCTRL1_SPEC>);
impl core::ops::Deref for W {
    type Target = crate::W<DPLLCTRL1_SPEC>;
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
impl From<crate::W<DPLLCTRL1_SPEC>> for W {
    #[inline(always)]
    fn from(writer: crate::W<DPLLCTRL1_SPEC>) -> Self {
        W(writer)
    }
}
#[doc = "Field `M` reader - Factor M"]
pub type M_R = crate::FieldReader<u16, u16>;
#[doc = "Field `M` writer - Factor M"]
pub type M_W<'a> = crate::FieldWriter<'a, u32, DPLLCTRL1_SPEC, u16, u16, 12, 0>;
#[doc = "Field `N` reader - Factor N"]
pub type N_R = crate::FieldReader<u16, u16>;
#[doc = "Field `N` writer - Factor N"]
pub type N_W<'a> = crate::FieldWriter<'a, u32, DPLLCTRL1_SPEC, u16, u16, 12, 16>;
impl R {
    #[doc = "Bits 0:11 - Factor M"]
    #[inline(always)]
    pub fn m(&self) -> M_R {
        M_R::new((self.bits & 0x0fff) as u16)
    }
    #[doc = "Bits 16:27 - Factor N"]
    #[inline(always)]
    pub fn n(&self) -> N_R {
        N_R::new(((self.bits >> 16) & 0x0fff) as u16)
    }
}
impl W {
    #[doc = "Bits 0:11 - Factor M"]
    #[inline(always)]
    pub fn m(&mut self) -> M_W {
        M_W::new(self)
    }
    #[doc = "Bits 16:27 - Factor N"]
    #[inline(always)]
    pub fn n(&mut self) -> N_W {
        N_W::new(self)
    }
    #[doc = "Writes raw bits to the register."]
    #[inline(always)]
    pub unsafe fn bits(&mut self, bits: u32) -> &mut Self {
        self.0.bits(bits);
        self
    }
}
#[doc = "DPLL Control Register\n\nThis register you can [`read`](crate::generic::Reg::read), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [dpllctrl1](index.html) module"]
pub struct DPLLCTRL1_SPEC;
impl crate::RegisterSpec for DPLLCTRL1_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [dpllctrl1::R](R) reader structure"]
impl crate::Readable for DPLLCTRL1_SPEC {
    type Reader = R;
}
#[doc = "`write(|w| ..)` method takes [dpllctrl1::W](W) writer structure"]
impl crate::Writable for DPLLCTRL1_SPEC {
    type Writer = W;
}
#[doc = "`reset()` method sets DPLLCTRL1 to value 0"]
impl crate::Resettable for DPLLCTRL1_SPEC {
    #[inline(always)]
    fn reset_value() -> Self::Ux {
        0
    }
}
