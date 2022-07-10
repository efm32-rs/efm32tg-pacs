#[doc = "Register `HFPERCLKEN1` reader"]
pub struct R(crate::R<HFPERCLKEN1_SPEC>);
impl core::ops::Deref for R {
    type Target = crate::R<HFPERCLKEN1_SPEC>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
impl From<crate::R<HFPERCLKEN1_SPEC>> for R {
    #[inline(always)]
    fn from(reader: crate::R<HFPERCLKEN1_SPEC>) -> Self {
        R(reader)
    }
}
#[doc = "Register `HFPERCLKEN1` writer"]
pub struct W(crate::W<HFPERCLKEN1_SPEC>);
impl core::ops::Deref for W {
    type Target = crate::W<HFPERCLKEN1_SPEC>;
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
impl From<crate::W<HFPERCLKEN1_SPEC>> for W {
    #[inline(always)]
    fn from(writer: crate::W<HFPERCLKEN1_SPEC>) -> Self {
        W(writer)
    }
}
#[doc = "Field `UART0` reader - Universal Asynchronous Receiver/Transmitter 0 Clock Enable"]
pub type UART0_R = crate::BitReader<bool>;
#[doc = "Field `UART0` writer - Universal Asynchronous Receiver/Transmitter 0 Clock Enable"]
pub type UART0_W<'a> = crate::BitWriter<'a, u32, HFPERCLKEN1_SPEC, bool, 0>;
#[doc = "Field `WTIMER0` reader - Wide Timer 0 Clock Enable"]
pub type WTIMER0_R = crate::BitReader<bool>;
#[doc = "Field `WTIMER0` writer - Wide Timer 0 Clock Enable"]
pub type WTIMER0_W<'a> = crate::BitWriter<'a, u32, HFPERCLKEN1_SPEC, bool, 1>;
#[doc = "Field `WTIMER1` reader - Wide Timer 1 Clock Enable"]
pub type WTIMER1_R = crate::BitReader<bool>;
#[doc = "Field `WTIMER1` writer - Wide Timer 1 Clock Enable"]
pub type WTIMER1_W<'a> = crate::BitWriter<'a, u32, HFPERCLKEN1_SPEC, bool, 2>;
#[doc = "Field `CAN0` reader - CAN 0 Clock Enable"]
pub type CAN0_R = crate::BitReader<bool>;
#[doc = "Field `CAN0` writer - CAN 0 Clock Enable"]
pub type CAN0_W<'a> = crate::BitWriter<'a, u32, HFPERCLKEN1_SPEC, bool, 3>;
#[doc = "Field `VDAC0` reader - Digital to Analog Converter 0 Clock Enable"]
pub type VDAC0_R = crate::BitReader<bool>;
#[doc = "Field `VDAC0` writer - Digital to Analog Converter 0 Clock Enable"]
pub type VDAC0_W<'a> = crate::BitWriter<'a, u32, HFPERCLKEN1_SPEC, bool, 4>;
#[doc = "Field `CSEN` reader - Capacitive touch sense module Clock Enable"]
pub type CSEN_R = crate::BitReader<bool>;
#[doc = "Field `CSEN` writer - Capacitive touch sense module Clock Enable"]
pub type CSEN_W<'a> = crate::BitWriter<'a, u32, HFPERCLKEN1_SPEC, bool, 5>;
impl R {
    #[doc = "Bit 0 - Universal Asynchronous Receiver/Transmitter 0 Clock Enable"]
    #[inline(always)]
    pub fn uart0(&self) -> UART0_R {
        UART0_R::new((self.bits & 1) != 0)
    }
    #[doc = "Bit 1 - Wide Timer 0 Clock Enable"]
    #[inline(always)]
    pub fn wtimer0(&self) -> WTIMER0_R {
        WTIMER0_R::new(((self.bits >> 1) & 1) != 0)
    }
    #[doc = "Bit 2 - Wide Timer 1 Clock Enable"]
    #[inline(always)]
    pub fn wtimer1(&self) -> WTIMER1_R {
        WTIMER1_R::new(((self.bits >> 2) & 1) != 0)
    }
    #[doc = "Bit 3 - CAN 0 Clock Enable"]
    #[inline(always)]
    pub fn can0(&self) -> CAN0_R {
        CAN0_R::new(((self.bits >> 3) & 1) != 0)
    }
    #[doc = "Bit 4 - Digital to Analog Converter 0 Clock Enable"]
    #[inline(always)]
    pub fn vdac0(&self) -> VDAC0_R {
        VDAC0_R::new(((self.bits >> 4) & 1) != 0)
    }
    #[doc = "Bit 5 - Capacitive touch sense module Clock Enable"]
    #[inline(always)]
    pub fn csen(&self) -> CSEN_R {
        CSEN_R::new(((self.bits >> 5) & 1) != 0)
    }
}
impl W {
    #[doc = "Bit 0 - Universal Asynchronous Receiver/Transmitter 0 Clock Enable"]
    #[inline(always)]
    pub fn uart0(&mut self) -> UART0_W {
        UART0_W::new(self)
    }
    #[doc = "Bit 1 - Wide Timer 0 Clock Enable"]
    #[inline(always)]
    pub fn wtimer0(&mut self) -> WTIMER0_W {
        WTIMER0_W::new(self)
    }
    #[doc = "Bit 2 - Wide Timer 1 Clock Enable"]
    #[inline(always)]
    pub fn wtimer1(&mut self) -> WTIMER1_W {
        WTIMER1_W::new(self)
    }
    #[doc = "Bit 3 - CAN 0 Clock Enable"]
    #[inline(always)]
    pub fn can0(&mut self) -> CAN0_W {
        CAN0_W::new(self)
    }
    #[doc = "Bit 4 - Digital to Analog Converter 0 Clock Enable"]
    #[inline(always)]
    pub fn vdac0(&mut self) -> VDAC0_W {
        VDAC0_W::new(self)
    }
    #[doc = "Bit 5 - Capacitive touch sense module Clock Enable"]
    #[inline(always)]
    pub fn csen(&mut self) -> CSEN_W {
        CSEN_W::new(self)
    }
    #[doc = "Writes raw bits to the register."]
    #[inline(always)]
    pub unsafe fn bits(&mut self, bits: u32) -> &mut Self {
        self.0.bits(bits);
        self
    }
}
#[doc = "High Frequency Peripheral Clock Enable Register 1\n\nThis register you can [`read`](crate::generic::Reg::read), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [hfperclken1](index.html) module"]
pub struct HFPERCLKEN1_SPEC;
impl crate::RegisterSpec for HFPERCLKEN1_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [hfperclken1::R](R) reader structure"]
impl crate::Readable for HFPERCLKEN1_SPEC {
    type Reader = R;
}
#[doc = "`write(|w| ..)` method takes [hfperclken1::W](W) writer structure"]
impl crate::Writable for HFPERCLKEN1_SPEC {
    type Writer = W;
}
#[doc = "`reset()` method sets HFPERCLKEN1 to value 0"]
impl crate::Resettable for HFPERCLKEN1_SPEC {
    #[inline(always)]
    fn reset_value() -> Self::Ux {
        0
    }
}
