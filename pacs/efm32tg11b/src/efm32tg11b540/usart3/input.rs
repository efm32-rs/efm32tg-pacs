#[doc = "Register `INPUT` reader"]
pub struct R(crate::R<INPUT_SPEC>);
impl core::ops::Deref for R {
    type Target = crate::R<INPUT_SPEC>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
impl From<crate::R<INPUT_SPEC>> for R {
    #[inline(always)]
    fn from(reader: crate::R<INPUT_SPEC>) -> Self {
        R(reader)
    }
}
#[doc = "Register `INPUT` writer"]
pub struct W(crate::W<INPUT_SPEC>);
impl core::ops::Deref for W {
    type Target = crate::W<INPUT_SPEC>;
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
impl From<crate::W<INPUT_SPEC>> for W {
    #[inline(always)]
    fn from(writer: crate::W<INPUT_SPEC>) -> Self {
        W(writer)
    }
}
#[doc = "RX PRS Channel Select\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq)]
#[repr(u8)]
pub enum RXPRSSEL_A {
    #[doc = "0: PRS Channel 0 selected"]
    PRSCH0 = 0,
    #[doc = "1: PRS Channel 1 selected"]
    PRSCH1 = 1,
    #[doc = "2: PRS Channel 2 selected"]
    PRSCH2 = 2,
    #[doc = "3: PRS Channel 3 selected"]
    PRSCH3 = 3,
    #[doc = "4: PRS Channel 4 selected"]
    PRSCH4 = 4,
    #[doc = "5: PRS Channel 5 selected"]
    PRSCH5 = 5,
    #[doc = "6: PRS Channel 6 selected"]
    PRSCH6 = 6,
    #[doc = "7: PRS Channel 7 selected"]
    PRSCH7 = 7,
}
impl From<RXPRSSEL_A> for u8 {
    #[inline(always)]
    fn from(variant: RXPRSSEL_A) -> Self {
        variant as _
    }
}
#[doc = "Field `RXPRSSEL` reader - RX PRS Channel Select"]
pub type RXPRSSEL_R = crate::FieldReader<u8, RXPRSSEL_A>;
impl RXPRSSEL_R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> RXPRSSEL_A {
        match self.bits {
            0 => RXPRSSEL_A::PRSCH0,
            1 => RXPRSSEL_A::PRSCH1,
            2 => RXPRSSEL_A::PRSCH2,
            3 => RXPRSSEL_A::PRSCH3,
            4 => RXPRSSEL_A::PRSCH4,
            5 => RXPRSSEL_A::PRSCH5,
            6 => RXPRSSEL_A::PRSCH6,
            7 => RXPRSSEL_A::PRSCH7,
            _ => unreachable!(),
        }
    }
    #[doc = "Checks if the value of the field is `PRSCH0`"]
    #[inline(always)]
    pub fn is_prsch0(&self) -> bool {
        *self == RXPRSSEL_A::PRSCH0
    }
    #[doc = "Checks if the value of the field is `PRSCH1`"]
    #[inline(always)]
    pub fn is_prsch1(&self) -> bool {
        *self == RXPRSSEL_A::PRSCH1
    }
    #[doc = "Checks if the value of the field is `PRSCH2`"]
    #[inline(always)]
    pub fn is_prsch2(&self) -> bool {
        *self == RXPRSSEL_A::PRSCH2
    }
    #[doc = "Checks if the value of the field is `PRSCH3`"]
    #[inline(always)]
    pub fn is_prsch3(&self) -> bool {
        *self == RXPRSSEL_A::PRSCH3
    }
    #[doc = "Checks if the value of the field is `PRSCH4`"]
    #[inline(always)]
    pub fn is_prsch4(&self) -> bool {
        *self == RXPRSSEL_A::PRSCH4
    }
    #[doc = "Checks if the value of the field is `PRSCH5`"]
    #[inline(always)]
    pub fn is_prsch5(&self) -> bool {
        *self == RXPRSSEL_A::PRSCH5
    }
    #[doc = "Checks if the value of the field is `PRSCH6`"]
    #[inline(always)]
    pub fn is_prsch6(&self) -> bool {
        *self == RXPRSSEL_A::PRSCH6
    }
    #[doc = "Checks if the value of the field is `PRSCH7`"]
    #[inline(always)]
    pub fn is_prsch7(&self) -> bool {
        *self == RXPRSSEL_A::PRSCH7
    }
}
#[doc = "Field `RXPRSSEL` writer - RX PRS Channel Select"]
pub type RXPRSSEL_W<'a> = crate::FieldWriterSafe<'a, u32, INPUT_SPEC, u8, RXPRSSEL_A, 3, 0>;
impl<'a> RXPRSSEL_W<'a> {
    #[doc = "PRS Channel 0 selected"]
    #[inline(always)]
    pub fn prsch0(self) -> &'a mut W {
        self.variant(RXPRSSEL_A::PRSCH0)
    }
    #[doc = "PRS Channel 1 selected"]
    #[inline(always)]
    pub fn prsch1(self) -> &'a mut W {
        self.variant(RXPRSSEL_A::PRSCH1)
    }
    #[doc = "PRS Channel 2 selected"]
    #[inline(always)]
    pub fn prsch2(self) -> &'a mut W {
        self.variant(RXPRSSEL_A::PRSCH2)
    }
    #[doc = "PRS Channel 3 selected"]
    #[inline(always)]
    pub fn prsch3(self) -> &'a mut W {
        self.variant(RXPRSSEL_A::PRSCH3)
    }
    #[doc = "PRS Channel 4 selected"]
    #[inline(always)]
    pub fn prsch4(self) -> &'a mut W {
        self.variant(RXPRSSEL_A::PRSCH4)
    }
    #[doc = "PRS Channel 5 selected"]
    #[inline(always)]
    pub fn prsch5(self) -> &'a mut W {
        self.variant(RXPRSSEL_A::PRSCH5)
    }
    #[doc = "PRS Channel 6 selected"]
    #[inline(always)]
    pub fn prsch6(self) -> &'a mut W {
        self.variant(RXPRSSEL_A::PRSCH6)
    }
    #[doc = "PRS Channel 7 selected"]
    #[inline(always)]
    pub fn prsch7(self) -> &'a mut W {
        self.variant(RXPRSSEL_A::PRSCH7)
    }
}
#[doc = "Field `RXPRS` reader - PRS RX Enable"]
pub type RXPRS_R = crate::BitReader<bool>;
#[doc = "Field `RXPRS` writer - PRS RX Enable"]
pub type RXPRS_W<'a> = crate::BitWriter<'a, u32, INPUT_SPEC, bool, 7>;
#[doc = "CLK PRS Channel Select\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq)]
#[repr(u8)]
pub enum CLKPRSSEL_A {
    #[doc = "0: PRS Channel 0 selected"]
    PRSCH0 = 0,
    #[doc = "1: PRS Channel 1 selected"]
    PRSCH1 = 1,
    #[doc = "2: PRS Channel 2 selected"]
    PRSCH2 = 2,
    #[doc = "3: PRS Channel 3 selected"]
    PRSCH3 = 3,
    #[doc = "4: PRS Channel 4 selected"]
    PRSCH4 = 4,
    #[doc = "5: PRS Channel 5 selected"]
    PRSCH5 = 5,
    #[doc = "6: PRS Channel 6 selected"]
    PRSCH6 = 6,
    #[doc = "7: PRS Channel 7 selected"]
    PRSCH7 = 7,
}
impl From<CLKPRSSEL_A> for u8 {
    #[inline(always)]
    fn from(variant: CLKPRSSEL_A) -> Self {
        variant as _
    }
}
#[doc = "Field `CLKPRSSEL` reader - CLK PRS Channel Select"]
pub type CLKPRSSEL_R = crate::FieldReader<u8, CLKPRSSEL_A>;
impl CLKPRSSEL_R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> CLKPRSSEL_A {
        match self.bits {
            0 => CLKPRSSEL_A::PRSCH0,
            1 => CLKPRSSEL_A::PRSCH1,
            2 => CLKPRSSEL_A::PRSCH2,
            3 => CLKPRSSEL_A::PRSCH3,
            4 => CLKPRSSEL_A::PRSCH4,
            5 => CLKPRSSEL_A::PRSCH5,
            6 => CLKPRSSEL_A::PRSCH6,
            7 => CLKPRSSEL_A::PRSCH7,
            _ => unreachable!(),
        }
    }
    #[doc = "Checks if the value of the field is `PRSCH0`"]
    #[inline(always)]
    pub fn is_prsch0(&self) -> bool {
        *self == CLKPRSSEL_A::PRSCH0
    }
    #[doc = "Checks if the value of the field is `PRSCH1`"]
    #[inline(always)]
    pub fn is_prsch1(&self) -> bool {
        *self == CLKPRSSEL_A::PRSCH1
    }
    #[doc = "Checks if the value of the field is `PRSCH2`"]
    #[inline(always)]
    pub fn is_prsch2(&self) -> bool {
        *self == CLKPRSSEL_A::PRSCH2
    }
    #[doc = "Checks if the value of the field is `PRSCH3`"]
    #[inline(always)]
    pub fn is_prsch3(&self) -> bool {
        *self == CLKPRSSEL_A::PRSCH3
    }
    #[doc = "Checks if the value of the field is `PRSCH4`"]
    #[inline(always)]
    pub fn is_prsch4(&self) -> bool {
        *self == CLKPRSSEL_A::PRSCH4
    }
    #[doc = "Checks if the value of the field is `PRSCH5`"]
    #[inline(always)]
    pub fn is_prsch5(&self) -> bool {
        *self == CLKPRSSEL_A::PRSCH5
    }
    #[doc = "Checks if the value of the field is `PRSCH6`"]
    #[inline(always)]
    pub fn is_prsch6(&self) -> bool {
        *self == CLKPRSSEL_A::PRSCH6
    }
    #[doc = "Checks if the value of the field is `PRSCH7`"]
    #[inline(always)]
    pub fn is_prsch7(&self) -> bool {
        *self == CLKPRSSEL_A::PRSCH7
    }
}
#[doc = "Field `CLKPRSSEL` writer - CLK PRS Channel Select"]
pub type CLKPRSSEL_W<'a> = crate::FieldWriterSafe<'a, u32, INPUT_SPEC, u8, CLKPRSSEL_A, 3, 8>;
impl<'a> CLKPRSSEL_W<'a> {
    #[doc = "PRS Channel 0 selected"]
    #[inline(always)]
    pub fn prsch0(self) -> &'a mut W {
        self.variant(CLKPRSSEL_A::PRSCH0)
    }
    #[doc = "PRS Channel 1 selected"]
    #[inline(always)]
    pub fn prsch1(self) -> &'a mut W {
        self.variant(CLKPRSSEL_A::PRSCH1)
    }
    #[doc = "PRS Channel 2 selected"]
    #[inline(always)]
    pub fn prsch2(self) -> &'a mut W {
        self.variant(CLKPRSSEL_A::PRSCH2)
    }
    #[doc = "PRS Channel 3 selected"]
    #[inline(always)]
    pub fn prsch3(self) -> &'a mut W {
        self.variant(CLKPRSSEL_A::PRSCH3)
    }
    #[doc = "PRS Channel 4 selected"]
    #[inline(always)]
    pub fn prsch4(self) -> &'a mut W {
        self.variant(CLKPRSSEL_A::PRSCH4)
    }
    #[doc = "PRS Channel 5 selected"]
    #[inline(always)]
    pub fn prsch5(self) -> &'a mut W {
        self.variant(CLKPRSSEL_A::PRSCH5)
    }
    #[doc = "PRS Channel 6 selected"]
    #[inline(always)]
    pub fn prsch6(self) -> &'a mut W {
        self.variant(CLKPRSSEL_A::PRSCH6)
    }
    #[doc = "PRS Channel 7 selected"]
    #[inline(always)]
    pub fn prsch7(self) -> &'a mut W {
        self.variant(CLKPRSSEL_A::PRSCH7)
    }
}
#[doc = "Field `CLKPRS` reader - PRS CLK Enable"]
pub type CLKPRS_R = crate::BitReader<bool>;
#[doc = "Field `CLKPRS` writer - PRS CLK Enable"]
pub type CLKPRS_W<'a> = crate::BitWriter<'a, u32, INPUT_SPEC, bool, 15>;
impl R {
    #[doc = "Bits 0:2 - RX PRS Channel Select"]
    #[inline(always)]
    pub fn rxprssel(&self) -> RXPRSSEL_R {
        RXPRSSEL_R::new((self.bits & 7) as u8)
    }
    #[doc = "Bit 7 - PRS RX Enable"]
    #[inline(always)]
    pub fn rxprs(&self) -> RXPRS_R {
        RXPRS_R::new(((self.bits >> 7) & 1) != 0)
    }
    #[doc = "Bits 8:10 - CLK PRS Channel Select"]
    #[inline(always)]
    pub fn clkprssel(&self) -> CLKPRSSEL_R {
        CLKPRSSEL_R::new(((self.bits >> 8) & 7) as u8)
    }
    #[doc = "Bit 15 - PRS CLK Enable"]
    #[inline(always)]
    pub fn clkprs(&self) -> CLKPRS_R {
        CLKPRS_R::new(((self.bits >> 15) & 1) != 0)
    }
}
impl W {
    #[doc = "Bits 0:2 - RX PRS Channel Select"]
    #[inline(always)]
    pub fn rxprssel(&mut self) -> RXPRSSEL_W {
        RXPRSSEL_W::new(self)
    }
    #[doc = "Bit 7 - PRS RX Enable"]
    #[inline(always)]
    pub fn rxprs(&mut self) -> RXPRS_W {
        RXPRS_W::new(self)
    }
    #[doc = "Bits 8:10 - CLK PRS Channel Select"]
    #[inline(always)]
    pub fn clkprssel(&mut self) -> CLKPRSSEL_W {
        CLKPRSSEL_W::new(self)
    }
    #[doc = "Bit 15 - PRS CLK Enable"]
    #[inline(always)]
    pub fn clkprs(&mut self) -> CLKPRS_W {
        CLKPRS_W::new(self)
    }
    #[doc = "Writes raw bits to the register."]
    #[inline(always)]
    pub unsafe fn bits(&mut self, bits: u32) -> &mut Self {
        self.0.bits(bits);
        self
    }
}
#[doc = "USART Input Register\n\nThis register you can [`read`](crate::generic::Reg::read), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [input](index.html) module"]
pub struct INPUT_SPEC;
impl crate::RegisterSpec for INPUT_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [input::R](R) reader structure"]
impl crate::Readable for INPUT_SPEC {
    type Reader = R;
}
#[doc = "`write(|w| ..)` method takes [input::W](W) writer structure"]
impl crate::Writable for INPUT_SPEC {
    type Writer = W;
}
#[doc = "`reset()` method sets INPUT to value 0"]
impl crate::Resettable for INPUT_SPEC {
    #[inline(always)]
    fn reset_value() -> Self::Ux {
        0
    }
}
