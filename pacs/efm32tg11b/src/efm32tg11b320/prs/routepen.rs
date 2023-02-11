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
#[doc = "Field `CH0PEN` reader - CH0 Pin Enable"]
pub type CH0PEN_R = crate::BitReader<bool>;
#[doc = "Field `CH0PEN` writer - CH0 Pin Enable"]
pub type CH0PEN_W<'a, const O: u8> = crate::BitWriter<'a, u32, ROUTEPEN_SPEC, bool, O>;
#[doc = "Field `CH1PEN` reader - CH1 Pin Enable"]
pub type CH1PEN_R = crate::BitReader<bool>;
#[doc = "Field `CH1PEN` writer - CH1 Pin Enable"]
pub type CH1PEN_W<'a, const O: u8> = crate::BitWriter<'a, u32, ROUTEPEN_SPEC, bool, O>;
#[doc = "Field `CH2PEN` reader - CH2 Pin Enable"]
pub type CH2PEN_R = crate::BitReader<bool>;
#[doc = "Field `CH2PEN` writer - CH2 Pin Enable"]
pub type CH2PEN_W<'a, const O: u8> = crate::BitWriter<'a, u32, ROUTEPEN_SPEC, bool, O>;
#[doc = "Field `CH3PEN` reader - CH3 Pin Enable"]
pub type CH3PEN_R = crate::BitReader<bool>;
#[doc = "Field `CH3PEN` writer - CH3 Pin Enable"]
pub type CH3PEN_W<'a, const O: u8> = crate::BitWriter<'a, u32, ROUTEPEN_SPEC, bool, O>;
#[doc = "Field `CH4PEN` reader - CH4 Pin Enable"]
pub type CH4PEN_R = crate::BitReader<bool>;
#[doc = "Field `CH4PEN` writer - CH4 Pin Enable"]
pub type CH4PEN_W<'a, const O: u8> = crate::BitWriter<'a, u32, ROUTEPEN_SPEC, bool, O>;
#[doc = "Field `CH5PEN` reader - CH5 Pin Enable"]
pub type CH5PEN_R = crate::BitReader<bool>;
#[doc = "Field `CH5PEN` writer - CH5 Pin Enable"]
pub type CH5PEN_W<'a, const O: u8> = crate::BitWriter<'a, u32, ROUTEPEN_SPEC, bool, O>;
#[doc = "Field `CH6PEN` reader - CH6 Pin Enable"]
pub type CH6PEN_R = crate::BitReader<bool>;
#[doc = "Field `CH6PEN` writer - CH6 Pin Enable"]
pub type CH6PEN_W<'a, const O: u8> = crate::BitWriter<'a, u32, ROUTEPEN_SPEC, bool, O>;
#[doc = "Field `CH7PEN` reader - CH7 Pin Enable"]
pub type CH7PEN_R = crate::BitReader<bool>;
#[doc = "Field `CH7PEN` writer - CH7 Pin Enable"]
pub type CH7PEN_W<'a, const O: u8> = crate::BitWriter<'a, u32, ROUTEPEN_SPEC, bool, O>;
impl R {
    #[doc = "Bit 0 - CH0 Pin Enable"]
    #[inline(always)]
    pub fn ch0pen(&self) -> CH0PEN_R {
        CH0PEN_R::new((self.bits & 1) != 0)
    }
    #[doc = "Bit 1 - CH1 Pin Enable"]
    #[inline(always)]
    pub fn ch1pen(&self) -> CH1PEN_R {
        CH1PEN_R::new(((self.bits >> 1) & 1) != 0)
    }
    #[doc = "Bit 2 - CH2 Pin Enable"]
    #[inline(always)]
    pub fn ch2pen(&self) -> CH2PEN_R {
        CH2PEN_R::new(((self.bits >> 2) & 1) != 0)
    }
    #[doc = "Bit 3 - CH3 Pin Enable"]
    #[inline(always)]
    pub fn ch3pen(&self) -> CH3PEN_R {
        CH3PEN_R::new(((self.bits >> 3) & 1) != 0)
    }
    #[doc = "Bit 4 - CH4 Pin Enable"]
    #[inline(always)]
    pub fn ch4pen(&self) -> CH4PEN_R {
        CH4PEN_R::new(((self.bits >> 4) & 1) != 0)
    }
    #[doc = "Bit 5 - CH5 Pin Enable"]
    #[inline(always)]
    pub fn ch5pen(&self) -> CH5PEN_R {
        CH5PEN_R::new(((self.bits >> 5) & 1) != 0)
    }
    #[doc = "Bit 6 - CH6 Pin Enable"]
    #[inline(always)]
    pub fn ch6pen(&self) -> CH6PEN_R {
        CH6PEN_R::new(((self.bits >> 6) & 1) != 0)
    }
    #[doc = "Bit 7 - CH7 Pin Enable"]
    #[inline(always)]
    pub fn ch7pen(&self) -> CH7PEN_R {
        CH7PEN_R::new(((self.bits >> 7) & 1) != 0)
    }
}
impl W {
    #[doc = "Bit 0 - CH0 Pin Enable"]
    #[inline(always)]
    #[must_use]
    pub fn ch0pen(&mut self) -> CH0PEN_W<0> {
        CH0PEN_W::new(self)
    }
    #[doc = "Bit 1 - CH1 Pin Enable"]
    #[inline(always)]
    #[must_use]
    pub fn ch1pen(&mut self) -> CH1PEN_W<1> {
        CH1PEN_W::new(self)
    }
    #[doc = "Bit 2 - CH2 Pin Enable"]
    #[inline(always)]
    #[must_use]
    pub fn ch2pen(&mut self) -> CH2PEN_W<2> {
        CH2PEN_W::new(self)
    }
    #[doc = "Bit 3 - CH3 Pin Enable"]
    #[inline(always)]
    #[must_use]
    pub fn ch3pen(&mut self) -> CH3PEN_W<3> {
        CH3PEN_W::new(self)
    }
    #[doc = "Bit 4 - CH4 Pin Enable"]
    #[inline(always)]
    #[must_use]
    pub fn ch4pen(&mut self) -> CH4PEN_W<4> {
        CH4PEN_W::new(self)
    }
    #[doc = "Bit 5 - CH5 Pin Enable"]
    #[inline(always)]
    #[must_use]
    pub fn ch5pen(&mut self) -> CH5PEN_W<5> {
        CH5PEN_W::new(self)
    }
    #[doc = "Bit 6 - CH6 Pin Enable"]
    #[inline(always)]
    #[must_use]
    pub fn ch6pen(&mut self) -> CH6PEN_W<6> {
        CH6PEN_W::new(self)
    }
    #[doc = "Bit 7 - CH7 Pin Enable"]
    #[inline(always)]
    #[must_use]
    pub fn ch7pen(&mut self) -> CH7PEN_W<7> {
        CH7PEN_W::new(self)
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
    const ZERO_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
}
#[doc = "`reset()` method sets ROUTEPEN to value 0"]
impl crate::Resettable for ROUTEPEN_SPEC {
    const RESET_VALUE: Self::Ux = 0;
}
