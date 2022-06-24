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
#[doc = "Field `CLKOUT0PEN` reader - CLKOUT0 Pin Enable"]
pub type CLKOUT0PEN_R = crate::BitReader<bool>;
#[doc = "Field `CLKOUT0PEN` writer - CLKOUT0 Pin Enable"]
pub type CLKOUT0PEN_W<'a> = crate::BitWriter<'a, u32, ROUTEPEN_SPEC, bool, 0>;
#[doc = "Field `CLKOUT1PEN` reader - CLKOUT1 Pin Enable"]
pub type CLKOUT1PEN_R = crate::BitReader<bool>;
#[doc = "Field `CLKOUT1PEN` writer - CLKOUT1 Pin Enable"]
pub type CLKOUT1PEN_W<'a> = crate::BitWriter<'a, u32, ROUTEPEN_SPEC, bool, 1>;
#[doc = "Field `CLKOUT2PEN` reader - CLKOUT2 Pin Enable"]
pub type CLKOUT2PEN_R = crate::BitReader<bool>;
#[doc = "Field `CLKOUT2PEN` writer - CLKOUT2 Pin Enable"]
pub type CLKOUT2PEN_W<'a> = crate::BitWriter<'a, u32, ROUTEPEN_SPEC, bool, 2>;
#[doc = "Field `CLKIN0PEN` reader - CLKIN0 Pin Enable"]
pub type CLKIN0PEN_R = crate::BitReader<bool>;
#[doc = "Field `CLKIN0PEN` writer - CLKIN0 Pin Enable"]
pub type CLKIN0PEN_W<'a> = crate::BitWriter<'a, u32, ROUTEPEN_SPEC, bool, 28>;
impl R {
    #[doc = "Bit 0 - CLKOUT0 Pin Enable"]
    #[inline(always)]
    pub fn clkout0pen(&self) -> CLKOUT0PEN_R {
        CLKOUT0PEN_R::new((self.bits & 1) != 0)
    }
    #[doc = "Bit 1 - CLKOUT1 Pin Enable"]
    #[inline(always)]
    pub fn clkout1pen(&self) -> CLKOUT1PEN_R {
        CLKOUT1PEN_R::new(((self.bits >> 1) & 1) != 0)
    }
    #[doc = "Bit 2 - CLKOUT2 Pin Enable"]
    #[inline(always)]
    pub fn clkout2pen(&self) -> CLKOUT2PEN_R {
        CLKOUT2PEN_R::new(((self.bits >> 2) & 1) != 0)
    }
    #[doc = "Bit 28 - CLKIN0 Pin Enable"]
    #[inline(always)]
    pub fn clkin0pen(&self) -> CLKIN0PEN_R {
        CLKIN0PEN_R::new(((self.bits >> 28) & 1) != 0)
    }
}
impl W {
    #[doc = "Bit 0 - CLKOUT0 Pin Enable"]
    #[inline(always)]
    pub fn clkout0pen(&mut self) -> CLKOUT0PEN_W {
        CLKOUT0PEN_W::new(self)
    }
    #[doc = "Bit 1 - CLKOUT1 Pin Enable"]
    #[inline(always)]
    pub fn clkout1pen(&mut self) -> CLKOUT1PEN_W {
        CLKOUT1PEN_W::new(self)
    }
    #[doc = "Bit 2 - CLKOUT2 Pin Enable"]
    #[inline(always)]
    pub fn clkout2pen(&mut self) -> CLKOUT2PEN_W {
        CLKOUT2PEN_W::new(self)
    }
    #[doc = "Bit 28 - CLKIN0 Pin Enable"]
    #[inline(always)]
    pub fn clkin0pen(&mut self) -> CLKIN0PEN_W {
        CLKIN0PEN_W::new(self)
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
