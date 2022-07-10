#[doc = "Register `ROUTEPEN` reader"]
pub struct R(crate::R<ROUTEPEN_SPEC>);
impl core::ops::Deref for R {
    type Target = crate::R<ROUTEPEN_SPEC>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
impl From<crate::R<ROUTEPEN_SPEC>> for R {
    #[inline(always)]
    fn from(reader: crate::R<ROUTEPEN_SPEC>) -> Self {
        R(reader)
    }
}
#[doc = "Register `ROUTEPEN` writer"]
pub struct W(crate::W<ROUTEPEN_SPEC>);
impl core::ops::Deref for W {
    type Target = crate::W<ROUTEPEN_SPEC>;
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
impl From<crate::W<ROUTEPEN_SPEC>> for W {
    #[inline(always)]
    fn from(writer: crate::W<ROUTEPEN_SPEC>) -> Self {
        W(writer)
    }
}
#[doc = "Field `OUT0PEN` reader - Output 0 Pin Enable"]
pub type OUT0PEN_R = crate::BitReader<bool>;
#[doc = "Field `OUT0PEN` writer - Output 0 Pin Enable"]
pub type OUT0PEN_W<'a> = crate::BitWriter<'a, u32, ROUTEPEN_SPEC, bool, 0>;
#[doc = "Field `OUT1PEN` reader - Output 1 Pin Enable"]
pub type OUT1PEN_R = crate::BitReader<bool>;
#[doc = "Field `OUT1PEN` writer - Output 1 Pin Enable"]
pub type OUT1PEN_W<'a> = crate::BitWriter<'a, u32, ROUTEPEN_SPEC, bool, 1>;
impl R {
    #[doc = "Bit 0 - Output 0 Pin Enable"]
    #[inline(always)]
    pub fn out0pen(&self) -> OUT0PEN_R {
        OUT0PEN_R::new((self.bits & 1) != 0)
    }
    #[doc = "Bit 1 - Output 1 Pin Enable"]
    #[inline(always)]
    pub fn out1pen(&self) -> OUT1PEN_R {
        OUT1PEN_R::new(((self.bits >> 1) & 1) != 0)
    }
}
impl W {
    #[doc = "Bit 0 - Output 0 Pin Enable"]
    #[inline(always)]
    pub fn out0pen(&mut self) -> OUT0PEN_W {
        OUT0PEN_W::new(self)
    }
    #[doc = "Bit 1 - Output 1 Pin Enable"]
    #[inline(always)]
    pub fn out1pen(&mut self) -> OUT1PEN_W {
        OUT1PEN_W::new(self)
    }
    #[doc = "Writes raw bits to the register."]
    #[inline(always)]
    pub unsafe fn bits(&mut self, bits: u32) -> &mut Self {
        self.0.bits(bits);
        self
    }
}
#[doc = "I/O Routing Pin Enable Register\n\nThis register you can [`read`](crate::generic::Reg::read), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [routepen](index.html) module"]
pub struct ROUTEPEN_SPEC;
impl crate::RegisterSpec for ROUTEPEN_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [routepen::R](R) reader structure"]
impl crate::Readable for ROUTEPEN_SPEC {
    type Reader = R;
}
#[doc = "`write(|w| ..)` method takes [routepen::W](W) writer structure"]
impl crate::Writable for ROUTEPEN_SPEC {
    type Writer = W;
}
#[doc = "`reset()` method sets ROUTEPEN to value 0"]
impl crate::Resettable for ROUTEPEN_SPEC {
    #[inline(always)]
    fn reset_value() -> Self::Ux {
        0
    }
}
