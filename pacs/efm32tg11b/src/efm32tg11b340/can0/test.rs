#[doc = "Register `TEST` reader"]
pub struct R(crate::R<TEST_SPEC>);
impl core::ops::Deref for R {
    type Target = crate::R<TEST_SPEC>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
impl From<crate::R<TEST_SPEC>> for R {
    #[inline(always)]
    fn from(reader: crate::R<TEST_SPEC>) -> Self {
        R(reader)
    }
}
#[doc = "Register `TEST` writer"]
pub struct W(crate::W<TEST_SPEC>);
impl core::ops::Deref for W {
    type Target = crate::W<TEST_SPEC>;
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
impl From<crate::W<TEST_SPEC>> for W {
    #[inline(always)]
    fn from(writer: crate::W<TEST_SPEC>) -> Self {
        W(writer)
    }
}
#[doc = "Field `BASIC` reader - Basic Mode"]
pub type BASIC_R = crate::BitReader<bool>;
#[doc = "Field `BASIC` writer - Basic Mode"]
pub type BASIC_W<'a> = crate::BitWriter<'a, u32, TEST_SPEC, bool, 2>;
#[doc = "Field `SILENT` reader - Silent Mode"]
pub type SILENT_R = crate::BitReader<bool>;
#[doc = "Field `SILENT` writer - Silent Mode"]
pub type SILENT_W<'a> = crate::BitWriter<'a, u32, TEST_SPEC, bool, 3>;
#[doc = "Field `LBACK` reader - Loopback Mode"]
pub type LBACK_R = crate::BitReader<bool>;
#[doc = "Field `LBACK` writer - Loopback Mode"]
pub type LBACK_W<'a> = crate::BitWriter<'a, u32, TEST_SPEC, bool, 4>;
#[doc = "Control of CAN_TX Pin\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq)]
#[repr(u8)]
pub enum TX_A {
    #[doc = "0: Reset value, CAN_TX is controlled by the CAN Core."]
    CORE = 0,
    #[doc = "1: Sample Point can be monitored at CAN_TX pin."]
    SAMPT = 1,
    #[doc = "2: CAN_TX pin drives a dominant bit (0) value."]
    LOW = 2,
    #[doc = "3: CAN_TX pin drives a recessive bit (1) value."]
    HIGH = 3,
}
impl From<TX_A> for u8 {
    #[inline(always)]
    fn from(variant: TX_A) -> Self {
        variant as _
    }
}
#[doc = "Field `TX` reader - Control of CAN_TX Pin"]
pub type TX_R = crate::FieldReader<u8, TX_A>;
impl TX_R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> TX_A {
        match self.bits {
            0 => TX_A::CORE,
            1 => TX_A::SAMPT,
            2 => TX_A::LOW,
            3 => TX_A::HIGH,
            _ => unreachable!(),
        }
    }
    #[doc = "Checks if the value of the field is `CORE`"]
    #[inline(always)]
    pub fn is_core(&self) -> bool {
        *self == TX_A::CORE
    }
    #[doc = "Checks if the value of the field is `SAMPT`"]
    #[inline(always)]
    pub fn is_sampt(&self) -> bool {
        *self == TX_A::SAMPT
    }
    #[doc = "Checks if the value of the field is `LOW`"]
    #[inline(always)]
    pub fn is_low(&self) -> bool {
        *self == TX_A::LOW
    }
    #[doc = "Checks if the value of the field is `HIGH`"]
    #[inline(always)]
    pub fn is_high(&self) -> bool {
        *self == TX_A::HIGH
    }
}
#[doc = "Field `TX` writer - Control of CAN_TX Pin"]
pub type TX_W<'a> = crate::FieldWriterSafe<'a, u32, TEST_SPEC, u8, TX_A, 2, 5>;
impl<'a> TX_W<'a> {
    #[doc = "Reset value, CAN_TX is controlled by the CAN Core."]
    #[inline(always)]
    pub fn core(self) -> &'a mut W {
        self.variant(TX_A::CORE)
    }
    #[doc = "Sample Point can be monitored at CAN_TX pin."]
    #[inline(always)]
    pub fn sampt(self) -> &'a mut W {
        self.variant(TX_A::SAMPT)
    }
    #[doc = "CAN_TX pin drives a dominant bit (0) value."]
    #[inline(always)]
    pub fn low(self) -> &'a mut W {
        self.variant(TX_A::LOW)
    }
    #[doc = "CAN_TX pin drives a recessive bit (1) value."]
    #[inline(always)]
    pub fn high(self) -> &'a mut W {
        self.variant(TX_A::HIGH)
    }
}
#[doc = "Field `RX` reader - Monitors the Actual Value of CAN_RX Pin"]
pub type RX_R = crate::BitReader<bool>;
impl R {
    #[doc = "Bit 2 - Basic Mode"]
    #[inline(always)]
    pub fn basic(&self) -> BASIC_R {
        BASIC_R::new(((self.bits >> 2) & 1) != 0)
    }
    #[doc = "Bit 3 - Silent Mode"]
    #[inline(always)]
    pub fn silent(&self) -> SILENT_R {
        SILENT_R::new(((self.bits >> 3) & 1) != 0)
    }
    #[doc = "Bit 4 - Loopback Mode"]
    #[inline(always)]
    pub fn lback(&self) -> LBACK_R {
        LBACK_R::new(((self.bits >> 4) & 1) != 0)
    }
    #[doc = "Bits 5:6 - Control of CAN_TX Pin"]
    #[inline(always)]
    pub fn tx(&self) -> TX_R {
        TX_R::new(((self.bits >> 5) & 3) as u8)
    }
    #[doc = "Bit 7 - Monitors the Actual Value of CAN_RX Pin"]
    #[inline(always)]
    pub fn rx(&self) -> RX_R {
        RX_R::new(((self.bits >> 7) & 1) != 0)
    }
}
impl W {
    #[doc = "Bit 2 - Basic Mode"]
    #[inline(always)]
    pub fn basic(&mut self) -> BASIC_W {
        BASIC_W::new(self)
    }
    #[doc = "Bit 3 - Silent Mode"]
    #[inline(always)]
    pub fn silent(&mut self) -> SILENT_W {
        SILENT_W::new(self)
    }
    #[doc = "Bit 4 - Loopback Mode"]
    #[inline(always)]
    pub fn lback(&mut self) -> LBACK_W {
        LBACK_W::new(self)
    }
    #[doc = "Bits 5:6 - Control of CAN_TX Pin"]
    #[inline(always)]
    pub fn tx(&mut self) -> TX_W {
        TX_W::new(self)
    }
    #[doc = "Writes raw bits to the register."]
    #[inline(always)]
    pub unsafe fn bits(&mut self, bits: u32) -> &mut Self {
        self.0.bits(bits);
        self
    }
}
#[doc = "Test Register\n\nThis register you can [`read`](crate::generic::Reg::read), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [test](index.html) module"]
pub struct TEST_SPEC;
impl crate::RegisterSpec for TEST_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [test::R](R) reader structure"]
impl crate::Readable for TEST_SPEC {
    type Reader = R;
}
#[doc = "`write(|w| ..)` method takes [test::W](W) writer structure"]
impl crate::Writable for TEST_SPEC {
    type Writer = W;
}
#[doc = "`reset()` method sets TEST to value 0"]
impl crate::Resettable for TEST_SPEC {
    #[inline(always)]
    fn reset_value() -> Self::Ux {
        0
    }
}
