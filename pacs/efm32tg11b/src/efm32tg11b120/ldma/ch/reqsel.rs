#[doc = "Register `REQSEL` reader"]
pub struct R(crate::R<REQSEL_SPEC>);
impl core::ops::Deref for R {
    type Target = crate::R<REQSEL_SPEC>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
impl From<crate::R<REQSEL_SPEC>> for R {
    #[inline(always)]
    fn from(reader: crate::R<REQSEL_SPEC>) -> Self {
        R(reader)
    }
}
#[doc = "Register `REQSEL` writer"]
pub struct W(crate::W<REQSEL_SPEC>);
impl core::ops::Deref for W {
    type Target = crate::W<REQSEL_SPEC>;
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
impl From<crate::W<REQSEL_SPEC>> for W {
    #[inline(always)]
    fn from(writer: crate::W<REQSEL_SPEC>) -> Self {
        W(writer)
    }
}
#[doc = "Field `SIGSEL` reader - Signal Select"]
pub type SIGSEL_R = crate::FieldReader<u8, u8>;
#[doc = "Field `SIGSEL` writer - Signal Select"]
pub type SIGSEL_W<'a, const O: u8> = crate::FieldWriter<'a, u32, REQSEL_SPEC, u8, u8, 4, O>;
#[doc = "Field `SOURCESEL` reader - Source Select"]
pub type SOURCESEL_R = crate::FieldReader<u8, SOURCESEL_A>;
#[doc = "Source Select\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
#[repr(u8)]
pub enum SOURCESEL_A {
    #[doc = "0: No source selected"]
    NONE = 0,
    #[doc = "1: Peripheral Reflex System"]
    PRS = 1,
    #[doc = "8: Analog to Digital Converter 0"]
    ADC0 = 8,
    #[doc = "10: Digital to Analog Converter 0"]
    VDAC0 = 10,
    #[doc = "12: Universal Synchronous/Asynchronous Receiver/Transmitter 0"]
    USART0 = 12,
    #[doc = "13: Universal Synchronous/Asynchronous Receiver/Transmitter 1"]
    USART1 = 13,
    #[doc = "14: Universal Synchronous/Asynchronous Receiver/Transmitter 2"]
    USART2 = 14,
    #[doc = "15: Universal Synchronous/Asynchronous Receiver/Transmitter 3"]
    USART3 = 15,
    #[doc = "18: Universal Asynchronous Receiver/Transmitter 0"]
    UART0 = 18,
    #[doc = "20: Low Energy UART 0"]
    LEUART0 = 20,
    #[doc = "22: I2C 0"]
    I2C0 = 22,
    #[doc = "23: I2C 1"]
    I2C1 = 23,
    #[doc = "25: Timer 0"]
    TIMER0 = 25,
    #[doc = "26: Timer 1"]
    TIMER1 = 26,
    #[doc = "32: Wide Timer 0"]
    WTIMER0 = 32,
    #[doc = "33: Wide Timer 1"]
    WTIMER1 = 33,
    #[doc = "48: Memory System Controller"]
    MSC = 48,
    #[doc = "49: Advanced Encryption Standard Accelerator"]
    CRYPTO0 = 49,
    #[doc = "61: Capacitive touch sense module"]
    CSEN = 61,
    #[doc = "62: Low Energy Sensor Interface"]
    LESENSE = 62,
}
impl From<SOURCESEL_A> for u8 {
    #[inline(always)]
    fn from(variant: SOURCESEL_A) -> Self {
        variant as _
    }
}
impl SOURCESEL_R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> Option<SOURCESEL_A> {
        match self.bits {
            0 => Some(SOURCESEL_A::NONE),
            1 => Some(SOURCESEL_A::PRS),
            8 => Some(SOURCESEL_A::ADC0),
            10 => Some(SOURCESEL_A::VDAC0),
            12 => Some(SOURCESEL_A::USART0),
            13 => Some(SOURCESEL_A::USART1),
            14 => Some(SOURCESEL_A::USART2),
            15 => Some(SOURCESEL_A::USART3),
            18 => Some(SOURCESEL_A::UART0),
            20 => Some(SOURCESEL_A::LEUART0),
            22 => Some(SOURCESEL_A::I2C0),
            23 => Some(SOURCESEL_A::I2C1),
            25 => Some(SOURCESEL_A::TIMER0),
            26 => Some(SOURCESEL_A::TIMER1),
            32 => Some(SOURCESEL_A::WTIMER0),
            33 => Some(SOURCESEL_A::WTIMER1),
            48 => Some(SOURCESEL_A::MSC),
            49 => Some(SOURCESEL_A::CRYPTO0),
            61 => Some(SOURCESEL_A::CSEN),
            62 => Some(SOURCESEL_A::LESENSE),
            _ => None,
        }
    }
    #[doc = "Checks if the value of the field is `NONE`"]
    #[inline(always)]
    pub fn is_none(&self) -> bool {
        *self == SOURCESEL_A::NONE
    }
    #[doc = "Checks if the value of the field is `PRS`"]
    #[inline(always)]
    pub fn is_prs(&self) -> bool {
        *self == SOURCESEL_A::PRS
    }
    #[doc = "Checks if the value of the field is `ADC0`"]
    #[inline(always)]
    pub fn is_adc0(&self) -> bool {
        *self == SOURCESEL_A::ADC0
    }
    #[doc = "Checks if the value of the field is `VDAC0`"]
    #[inline(always)]
    pub fn is_vdac0(&self) -> bool {
        *self == SOURCESEL_A::VDAC0
    }
    #[doc = "Checks if the value of the field is `USART0`"]
    #[inline(always)]
    pub fn is_usart0(&self) -> bool {
        *self == SOURCESEL_A::USART0
    }
    #[doc = "Checks if the value of the field is `USART1`"]
    #[inline(always)]
    pub fn is_usart1(&self) -> bool {
        *self == SOURCESEL_A::USART1
    }
    #[doc = "Checks if the value of the field is `USART2`"]
    #[inline(always)]
    pub fn is_usart2(&self) -> bool {
        *self == SOURCESEL_A::USART2
    }
    #[doc = "Checks if the value of the field is `USART3`"]
    #[inline(always)]
    pub fn is_usart3(&self) -> bool {
        *self == SOURCESEL_A::USART3
    }
    #[doc = "Checks if the value of the field is `UART0`"]
    #[inline(always)]
    pub fn is_uart0(&self) -> bool {
        *self == SOURCESEL_A::UART0
    }
    #[doc = "Checks if the value of the field is `LEUART0`"]
    #[inline(always)]
    pub fn is_leuart0(&self) -> bool {
        *self == SOURCESEL_A::LEUART0
    }
    #[doc = "Checks if the value of the field is `I2C0`"]
    #[inline(always)]
    pub fn is_i2c0(&self) -> bool {
        *self == SOURCESEL_A::I2C0
    }
    #[doc = "Checks if the value of the field is `I2C1`"]
    #[inline(always)]
    pub fn is_i2c1(&self) -> bool {
        *self == SOURCESEL_A::I2C1
    }
    #[doc = "Checks if the value of the field is `TIMER0`"]
    #[inline(always)]
    pub fn is_timer0(&self) -> bool {
        *self == SOURCESEL_A::TIMER0
    }
    #[doc = "Checks if the value of the field is `TIMER1`"]
    #[inline(always)]
    pub fn is_timer1(&self) -> bool {
        *self == SOURCESEL_A::TIMER1
    }
    #[doc = "Checks if the value of the field is `WTIMER0`"]
    #[inline(always)]
    pub fn is_wtimer0(&self) -> bool {
        *self == SOURCESEL_A::WTIMER0
    }
    #[doc = "Checks if the value of the field is `WTIMER1`"]
    #[inline(always)]
    pub fn is_wtimer1(&self) -> bool {
        *self == SOURCESEL_A::WTIMER1
    }
    #[doc = "Checks if the value of the field is `MSC`"]
    #[inline(always)]
    pub fn is_msc(&self) -> bool {
        *self == SOURCESEL_A::MSC
    }
    #[doc = "Checks if the value of the field is `CRYPTO0`"]
    #[inline(always)]
    pub fn is_crypto0(&self) -> bool {
        *self == SOURCESEL_A::CRYPTO0
    }
    #[doc = "Checks if the value of the field is `CSEN`"]
    #[inline(always)]
    pub fn is_csen(&self) -> bool {
        *self == SOURCESEL_A::CSEN
    }
    #[doc = "Checks if the value of the field is `LESENSE`"]
    #[inline(always)]
    pub fn is_lesense(&self) -> bool {
        *self == SOURCESEL_A::LESENSE
    }
}
#[doc = "Field `SOURCESEL` writer - Source Select"]
pub type SOURCESEL_W<'a, const O: u8> =
    crate::FieldWriter<'a, u32, REQSEL_SPEC, u8, SOURCESEL_A, 6, O>;
impl<'a, const O: u8> SOURCESEL_W<'a, O> {
    #[doc = "No source selected"]
    #[inline(always)]
    pub fn none(self) -> &'a mut W {
        self.variant(SOURCESEL_A::NONE)
    }
    #[doc = "Peripheral Reflex System"]
    #[inline(always)]
    pub fn prs(self) -> &'a mut W {
        self.variant(SOURCESEL_A::PRS)
    }
    #[doc = "Analog to Digital Converter 0"]
    #[inline(always)]
    pub fn adc0(self) -> &'a mut W {
        self.variant(SOURCESEL_A::ADC0)
    }
    #[doc = "Digital to Analog Converter 0"]
    #[inline(always)]
    pub fn vdac0(self) -> &'a mut W {
        self.variant(SOURCESEL_A::VDAC0)
    }
    #[doc = "Universal Synchronous/Asynchronous Receiver/Transmitter 0"]
    #[inline(always)]
    pub fn usart0(self) -> &'a mut W {
        self.variant(SOURCESEL_A::USART0)
    }
    #[doc = "Universal Synchronous/Asynchronous Receiver/Transmitter 1"]
    #[inline(always)]
    pub fn usart1(self) -> &'a mut W {
        self.variant(SOURCESEL_A::USART1)
    }
    #[doc = "Universal Synchronous/Asynchronous Receiver/Transmitter 2"]
    #[inline(always)]
    pub fn usart2(self) -> &'a mut W {
        self.variant(SOURCESEL_A::USART2)
    }
    #[doc = "Universal Synchronous/Asynchronous Receiver/Transmitter 3"]
    #[inline(always)]
    pub fn usart3(self) -> &'a mut W {
        self.variant(SOURCESEL_A::USART3)
    }
    #[doc = "Universal Asynchronous Receiver/Transmitter 0"]
    #[inline(always)]
    pub fn uart0(self) -> &'a mut W {
        self.variant(SOURCESEL_A::UART0)
    }
    #[doc = "Low Energy UART 0"]
    #[inline(always)]
    pub fn leuart0(self) -> &'a mut W {
        self.variant(SOURCESEL_A::LEUART0)
    }
    #[doc = "I2C 0"]
    #[inline(always)]
    pub fn i2c0(self) -> &'a mut W {
        self.variant(SOURCESEL_A::I2C0)
    }
    #[doc = "I2C 1"]
    #[inline(always)]
    pub fn i2c1(self) -> &'a mut W {
        self.variant(SOURCESEL_A::I2C1)
    }
    #[doc = "Timer 0"]
    #[inline(always)]
    pub fn timer0(self) -> &'a mut W {
        self.variant(SOURCESEL_A::TIMER0)
    }
    #[doc = "Timer 1"]
    #[inline(always)]
    pub fn timer1(self) -> &'a mut W {
        self.variant(SOURCESEL_A::TIMER1)
    }
    #[doc = "Wide Timer 0"]
    #[inline(always)]
    pub fn wtimer0(self) -> &'a mut W {
        self.variant(SOURCESEL_A::WTIMER0)
    }
    #[doc = "Wide Timer 1"]
    #[inline(always)]
    pub fn wtimer1(self) -> &'a mut W {
        self.variant(SOURCESEL_A::WTIMER1)
    }
    #[doc = "Memory System Controller"]
    #[inline(always)]
    pub fn msc(self) -> &'a mut W {
        self.variant(SOURCESEL_A::MSC)
    }
    #[doc = "Advanced Encryption Standard Accelerator"]
    #[inline(always)]
    pub fn crypto0(self) -> &'a mut W {
        self.variant(SOURCESEL_A::CRYPTO0)
    }
    #[doc = "Capacitive touch sense module"]
    #[inline(always)]
    pub fn csen(self) -> &'a mut W {
        self.variant(SOURCESEL_A::CSEN)
    }
    #[doc = "Low Energy Sensor Interface"]
    #[inline(always)]
    pub fn lesense(self) -> &'a mut W {
        self.variant(SOURCESEL_A::LESENSE)
    }
}
impl R {
    #[doc = "Bits 0:3 - Signal Select"]
    #[inline(always)]
    pub fn sigsel(&self) -> SIGSEL_R {
        SIGSEL_R::new((self.bits & 0x0f) as u8)
    }
    #[doc = "Bits 16:21 - Source Select"]
    #[inline(always)]
    pub fn sourcesel(&self) -> SOURCESEL_R {
        SOURCESEL_R::new(((self.bits >> 16) & 0x3f) as u8)
    }
}
impl W {
    #[doc = "Bits 0:3 - Signal Select"]
    #[inline(always)]
    #[must_use]
    pub fn sigsel(&mut self) -> SIGSEL_W<0> {
        SIGSEL_W::new(self)
    }
    #[doc = "Bits 16:21 - Source Select"]
    #[inline(always)]
    #[must_use]
    pub fn sourcesel(&mut self) -> SOURCESEL_W<16> {
        SOURCESEL_W::new(self)
    }
    #[doc = "Writes raw bits to the register."]
    #[inline(always)]
    pub unsafe fn bits(&mut self, bits: u32) -> &mut Self {
        self.0.bits(bits);
        self
    }
}
#[doc = "Channel Peripheral Request Select Register\n\nThis register you can [`read`](crate::generic::Reg::read), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [reqsel](index.html) module"]
pub struct REQSEL_SPEC;
impl crate::RegisterSpec for REQSEL_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [reqsel::R](R) reader structure"]
impl crate::Readable for REQSEL_SPEC {
    type Reader = R;
}
#[doc = "`write(|w| ..)` method takes [reqsel::W](W) writer structure"]
impl crate::Writable for REQSEL_SPEC {
    type Writer = W;
    const ZERO_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
}
#[doc = "`reset()` method sets REQSEL to value 0"]
impl crate::Resettable for REQSEL_SPEC {
    const RESET_VALUE: Self::Ux = 0;
}
