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
#[doc = "Field `RXPEN` reader - RX Pin Enable"]
pub type RXPEN_R = crate::BitReader<bool>;
#[doc = "Field `RXPEN` writer - RX Pin Enable"]
pub type RXPEN_W<'a> = crate::BitWriter<'a, u32, ROUTEPEN_SPEC, bool, 0>;
#[doc = "Field `TXPEN` reader - TX Pin Enable"]
pub type TXPEN_R = crate::BitReader<bool>;
#[doc = "Field `TXPEN` writer - TX Pin Enable"]
pub type TXPEN_W<'a> = crate::BitWriter<'a, u32, ROUTEPEN_SPEC, bool, 1>;
#[doc = "Field `CSPEN` reader - CS Pin Enable"]
pub type CSPEN_R = crate::BitReader<bool>;
#[doc = "Field `CSPEN` writer - CS Pin Enable"]
pub type CSPEN_W<'a> = crate::BitWriter<'a, u32, ROUTEPEN_SPEC, bool, 2>;
#[doc = "Field `CLKPEN` reader - CLK Pin Enable"]
pub type CLKPEN_R = crate::BitReader<bool>;
#[doc = "Field `CLKPEN` writer - CLK Pin Enable"]
pub type CLKPEN_W<'a> = crate::BitWriter<'a, u32, ROUTEPEN_SPEC, bool, 3>;
#[doc = "Field `CTSPEN` reader - CTS Pin Enable"]
pub type CTSPEN_R = crate::BitReader<bool>;
#[doc = "Field `CTSPEN` writer - CTS Pin Enable"]
pub type CTSPEN_W<'a> = crate::BitWriter<'a, u32, ROUTEPEN_SPEC, bool, 4>;
#[doc = "Field `RTSPEN` reader - RTS Pin Enable"]
pub type RTSPEN_R = crate::BitReader<bool>;
#[doc = "Field `RTSPEN` writer - RTS Pin Enable"]
pub type RTSPEN_W<'a> = crate::BitWriter<'a, u32, ROUTEPEN_SPEC, bool, 5>;
impl R {
    #[doc = "Bit 0 - RX Pin Enable"]
    #[inline(always)]
    pub fn rxpen(&self) -> RXPEN_R {
        RXPEN_R::new((self.bits & 1) != 0)
    }
    #[doc = "Bit 1 - TX Pin Enable"]
    #[inline(always)]
    pub fn txpen(&self) -> TXPEN_R {
        TXPEN_R::new(((self.bits >> 1) & 1) != 0)
    }
    #[doc = "Bit 2 - CS Pin Enable"]
    #[inline(always)]
    pub fn cspen(&self) -> CSPEN_R {
        CSPEN_R::new(((self.bits >> 2) & 1) != 0)
    }
    #[doc = "Bit 3 - CLK Pin Enable"]
    #[inline(always)]
    pub fn clkpen(&self) -> CLKPEN_R {
        CLKPEN_R::new(((self.bits >> 3) & 1) != 0)
    }
    #[doc = "Bit 4 - CTS Pin Enable"]
    #[inline(always)]
    pub fn ctspen(&self) -> CTSPEN_R {
        CTSPEN_R::new(((self.bits >> 4) & 1) != 0)
    }
    #[doc = "Bit 5 - RTS Pin Enable"]
    #[inline(always)]
    pub fn rtspen(&self) -> RTSPEN_R {
        RTSPEN_R::new(((self.bits >> 5) & 1) != 0)
    }
}
impl W {
    #[doc = "Bit 0 - RX Pin Enable"]
    #[inline(always)]
    pub fn rxpen(&mut self) -> RXPEN_W {
        RXPEN_W::new(self)
    }
    #[doc = "Bit 1 - TX Pin Enable"]
    #[inline(always)]
    pub fn txpen(&mut self) -> TXPEN_W {
        TXPEN_W::new(self)
    }
    #[doc = "Bit 2 - CS Pin Enable"]
    #[inline(always)]
    pub fn cspen(&mut self) -> CSPEN_W {
        CSPEN_W::new(self)
    }
    #[doc = "Bit 3 - CLK Pin Enable"]
    #[inline(always)]
    pub fn clkpen(&mut self) -> CLKPEN_W {
        CLKPEN_W::new(self)
    }
    #[doc = "Bit 4 - CTS Pin Enable"]
    #[inline(always)]
    pub fn ctspen(&mut self) -> CTSPEN_W {
        CTSPEN_W::new(self)
    }
    #[doc = "Bit 5 - RTS Pin Enable"]
    #[inline(always)]
    pub fn rtspen(&mut self) -> RTSPEN_W {
        RTSPEN_W::new(self)
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
