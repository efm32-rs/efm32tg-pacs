#[doc = "Register `PPUPATD1` reader"]
pub struct R(crate::R<PPUPATD1_SPEC>);
impl core::ops::Deref for R {
    type Target = crate::R<PPUPATD1_SPEC>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
impl From<crate::R<PPUPATD1_SPEC>> for R {
    #[inline(always)]
    fn from(reader: crate::R<PPUPATD1_SPEC>) -> Self {
        R(reader)
    }
}
#[doc = "Register `PPUPATD1` writer"]
pub struct W(crate::W<PPUPATD1_SPEC>);
impl core::ops::Deref for W {
    type Target = crate::W<PPUPATD1_SPEC>;
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
impl From<crate::W<PPUPATD1_SPEC>> for W {
    #[inline(always)]
    fn from(writer: crate::W<PPUPATD1_SPEC>) -> Self {
        W(writer)
    }
}
#[doc = "Field `USART3` reader - Universal Synchronous/Asynchronous Receiver/Transmitter 3 access control bit"]
pub type USART3_R = crate::BitReader<bool>;
#[doc = "Field `USART3` writer - Universal Synchronous/Asynchronous Receiver/Transmitter 3 access control bit"]
pub type USART3_W<'a> = crate::BitWriter<'a, u32, PPUPATD1_SPEC, bool, 0>;
#[doc = "Field `WDOG0` reader - Watchdog access control bit"]
pub type WDOG0_R = crate::BitReader<bool>;
#[doc = "Field `WDOG0` writer - Watchdog access control bit"]
pub type WDOG0_W<'a> = crate::BitWriter<'a, u32, PPUPATD1_SPEC, bool, 1>;
#[doc = "Field `WTIMER0` reader - Wide Timer 0 access control bit"]
pub type WTIMER0_R = crate::BitReader<bool>;
#[doc = "Field `WTIMER0` writer - Wide Timer 0 access control bit"]
pub type WTIMER0_W<'a> = crate::BitWriter<'a, u32, PPUPATD1_SPEC, bool, 2>;
#[doc = "Field `WTIMER1` reader - Wide Timer 1 access control bit"]
pub type WTIMER1_R = crate::BitReader<bool>;
#[doc = "Field `WTIMER1` writer - Wide Timer 1 access control bit"]
pub type WTIMER1_W<'a> = crate::BitWriter<'a, u32, PPUPATD1_SPEC, bool, 3>;
impl R {
    #[doc = "Bit 0 - Universal Synchronous/Asynchronous Receiver/Transmitter 3 access control bit"]
    #[inline(always)]
    pub fn usart3(&self) -> USART3_R {
        USART3_R::new((self.bits & 1) != 0)
    }
    #[doc = "Bit 1 - Watchdog access control bit"]
    #[inline(always)]
    pub fn wdog0(&self) -> WDOG0_R {
        WDOG0_R::new(((self.bits >> 1) & 1) != 0)
    }
    #[doc = "Bit 2 - Wide Timer 0 access control bit"]
    #[inline(always)]
    pub fn wtimer0(&self) -> WTIMER0_R {
        WTIMER0_R::new(((self.bits >> 2) & 1) != 0)
    }
    #[doc = "Bit 3 - Wide Timer 1 access control bit"]
    #[inline(always)]
    pub fn wtimer1(&self) -> WTIMER1_R {
        WTIMER1_R::new(((self.bits >> 3) & 1) != 0)
    }
}
impl W {
    #[doc = "Bit 0 - Universal Synchronous/Asynchronous Receiver/Transmitter 3 access control bit"]
    #[inline(always)]
    pub fn usart3(&mut self) -> USART3_W {
        USART3_W::new(self)
    }
    #[doc = "Bit 1 - Watchdog access control bit"]
    #[inline(always)]
    pub fn wdog0(&mut self) -> WDOG0_W {
        WDOG0_W::new(self)
    }
    #[doc = "Bit 2 - Wide Timer 0 access control bit"]
    #[inline(always)]
    pub fn wtimer0(&mut self) -> WTIMER0_W {
        WTIMER0_W::new(self)
    }
    #[doc = "Bit 3 - Wide Timer 1 access control bit"]
    #[inline(always)]
    pub fn wtimer1(&mut self) -> WTIMER1_W {
        WTIMER1_W::new(self)
    }
    #[doc = "Writes raw bits to the register."]
    #[inline(always)]
    pub unsafe fn bits(&mut self, bits: u32) -> &mut Self {
        self.0.bits(bits);
        self
    }
}
#[doc = "PPU Privilege Access Type Descriptor 1\n\nThis register you can [`read`](crate::generic::Reg::read), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [ppupatd1](index.html) module"]
pub struct PPUPATD1_SPEC;
impl crate::RegisterSpec for PPUPATD1_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [ppupatd1::R](R) reader structure"]
impl crate::Readable for PPUPATD1_SPEC {
    type Reader = R;
}
#[doc = "`write(|w| ..)` method takes [ppupatd1::W](W) writer structure"]
impl crate::Writable for PPUPATD1_SPEC {
    type Writer = W;
}
#[doc = "`reset()` method sets PPUPATD1 to value 0"]
impl crate::Resettable for PPUPATD1_SPEC {
    #[inline(always)]
    fn reset_value() -> Self::Ux {
        0
    }
}
