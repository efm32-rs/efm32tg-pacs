#[doc = "Register `EXTIFCTRL` reader"]
pub struct R(crate::R<EXTIFCTRL_SPEC>);
impl core::ops::Deref for R {
    type Target = crate::R<EXTIFCTRL_SPEC>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
impl From<crate::R<EXTIFCTRL_SPEC>> for R {
    #[inline(always)]
    fn from(reader: crate::R<EXTIFCTRL_SPEC>) -> Self {
        R(reader)
    }
}
#[doc = "Register `EXTIFCTRL` writer"]
pub struct W(crate::W<EXTIFCTRL_SPEC>);
impl core::ops::Deref for W {
    type Target = crate::W<EXTIFCTRL_SPEC>;
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
impl From<crate::W<EXTIFCTRL_SPEC>> for W {
    #[inline(always)]
    fn from(writer: crate::W<EXTIFCTRL_SPEC>) -> Self {
        W(writer)
    }
}
#[doc = "Field `EN` reader - Enable External Interface"]
pub type EN_R = crate::BitReader<bool>;
#[doc = "Field `EN` writer - Enable External Interface"]
pub type EN_W<'a> = crate::BitWriter<'a, u32, EXTIFCTRL_SPEC, bool, 0>;
#[doc = "APORT Selection for External Interface\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq)]
#[repr(u8)]
pub enum APORTSEL_A {
    #[doc = "0: APORT0X used. EXT_BASE = ACMP_INPUTSEL_POSSEL_APORT0XCH0."]
    APORT0X = 0,
    #[doc = "1: APORT0Y used. EXT_BASE = ACMP_INPUTSEL_POSSEL_APORT0YCH0."]
    APORT0Y = 1,
    #[doc = "2: APORT1X used. EXT_BASE = ACMP_INPUTSEL_POSSEL_APORT1XCH0."]
    APORT1X = 2,
    #[doc = "3: APORT1Y used. EXT_BASE = ACMP_INPUTSEL_POSSEL_APORT1XCH0."]
    APORT1Y = 3,
    #[doc = "4: APORT1X/Y used. EXT_BASE = ACMP_INPUTSEL_POSSEL_APORT1XCH0."]
    APORT1XY = 4,
    #[doc = "5: APORT2X used. EXT_BASE = ACMP_INPUTSEL_POSSEL_APORT2YCH0."]
    APORT2X = 5,
    #[doc = "6: APORT2Y used. EXT_BASE = ACMP_INPUTSEL_POSSEL_APORT2YCH0."]
    APORT2Y = 6,
    #[doc = "7: APORT2Y/X used. EXT_BASE = ACMP_INPUTSEL_POSSEL_APORT2YCH0."]
    APORT2YX = 7,
    #[doc = "8: APORT3X used. EXT_BASE = ACMP_INPUTSEL_POSSEL_APORT3XCH0."]
    APORT3X = 8,
    #[doc = "9: APORT3Y used. EXT_BASE = ACMP_INPUTSEL_POSSEL_APORT3XCH0."]
    APORT3Y = 9,
    #[doc = "10: APORT3X/Y used. EXT_BASE = ACMP_INPUTSEL_POSSEL_APORT3XCH0."]
    APORT3XY = 10,
    #[doc = "11: APORT4X used. EXT_BASE = ACMP_INPUTSEL_POSSEL_APORT4YCH0."]
    APORT4X = 11,
    #[doc = "12: APORT4Y used. EXT_BASE = ACMP_INPUTSEL_POSSEL_APORT4YCH0."]
    APORT4Y = 12,
    #[doc = "13: APORT4Y/X used. EXT_BASE = ACMP_INPUTSEL_POSSEL_APORT4YCH0."]
    APORT4YX = 13,
}
impl From<APORTSEL_A> for u8 {
    #[inline(always)]
    fn from(variant: APORTSEL_A) -> Self {
        variant as _
    }
}
#[doc = "Field `APORTSEL` reader - APORT Selection for External Interface"]
pub type APORTSEL_R = crate::FieldReader<u8, APORTSEL_A>;
impl APORTSEL_R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> Option<APORTSEL_A> {
        match self.bits {
            0 => Some(APORTSEL_A::APORT0X),
            1 => Some(APORTSEL_A::APORT0Y),
            2 => Some(APORTSEL_A::APORT1X),
            3 => Some(APORTSEL_A::APORT1Y),
            4 => Some(APORTSEL_A::APORT1XY),
            5 => Some(APORTSEL_A::APORT2X),
            6 => Some(APORTSEL_A::APORT2Y),
            7 => Some(APORTSEL_A::APORT2YX),
            8 => Some(APORTSEL_A::APORT3X),
            9 => Some(APORTSEL_A::APORT3Y),
            10 => Some(APORTSEL_A::APORT3XY),
            11 => Some(APORTSEL_A::APORT4X),
            12 => Some(APORTSEL_A::APORT4Y),
            13 => Some(APORTSEL_A::APORT4YX),
            _ => None,
        }
    }
    #[doc = "Checks if the value of the field is `APORT0X`"]
    #[inline(always)]
    pub fn is_aport0x(&self) -> bool {
        *self == APORTSEL_A::APORT0X
    }
    #[doc = "Checks if the value of the field is `APORT0Y`"]
    #[inline(always)]
    pub fn is_aport0y(&self) -> bool {
        *self == APORTSEL_A::APORT0Y
    }
    #[doc = "Checks if the value of the field is `APORT1X`"]
    #[inline(always)]
    pub fn is_aport1x(&self) -> bool {
        *self == APORTSEL_A::APORT1X
    }
    #[doc = "Checks if the value of the field is `APORT1Y`"]
    #[inline(always)]
    pub fn is_aport1y(&self) -> bool {
        *self == APORTSEL_A::APORT1Y
    }
    #[doc = "Checks if the value of the field is `APORT1XY`"]
    #[inline(always)]
    pub fn is_aport1xy(&self) -> bool {
        *self == APORTSEL_A::APORT1XY
    }
    #[doc = "Checks if the value of the field is `APORT2X`"]
    #[inline(always)]
    pub fn is_aport2x(&self) -> bool {
        *self == APORTSEL_A::APORT2X
    }
    #[doc = "Checks if the value of the field is `APORT2Y`"]
    #[inline(always)]
    pub fn is_aport2y(&self) -> bool {
        *self == APORTSEL_A::APORT2Y
    }
    #[doc = "Checks if the value of the field is `APORT2YX`"]
    #[inline(always)]
    pub fn is_aport2yx(&self) -> bool {
        *self == APORTSEL_A::APORT2YX
    }
    #[doc = "Checks if the value of the field is `APORT3X`"]
    #[inline(always)]
    pub fn is_aport3x(&self) -> bool {
        *self == APORTSEL_A::APORT3X
    }
    #[doc = "Checks if the value of the field is `APORT3Y`"]
    #[inline(always)]
    pub fn is_aport3y(&self) -> bool {
        *self == APORTSEL_A::APORT3Y
    }
    #[doc = "Checks if the value of the field is `APORT3XY`"]
    #[inline(always)]
    pub fn is_aport3xy(&self) -> bool {
        *self == APORTSEL_A::APORT3XY
    }
    #[doc = "Checks if the value of the field is `APORT4X`"]
    #[inline(always)]
    pub fn is_aport4x(&self) -> bool {
        *self == APORTSEL_A::APORT4X
    }
    #[doc = "Checks if the value of the field is `APORT4Y`"]
    #[inline(always)]
    pub fn is_aport4y(&self) -> bool {
        *self == APORTSEL_A::APORT4Y
    }
    #[doc = "Checks if the value of the field is `APORT4YX`"]
    #[inline(always)]
    pub fn is_aport4yx(&self) -> bool {
        *self == APORTSEL_A::APORT4YX
    }
}
#[doc = "Field `APORTSEL` writer - APORT Selection for External Interface"]
pub type APORTSEL_W<'a> = crate::FieldWriter<'a, u32, EXTIFCTRL_SPEC, u8, APORTSEL_A, 4, 4>;
impl<'a> APORTSEL_W<'a> {
    #[doc = "APORT0X used. EXT_BASE = ACMP_INPUTSEL_POSSEL_APORT0XCH0."]
    #[inline(always)]
    pub fn aport0x(self) -> &'a mut W {
        self.variant(APORTSEL_A::APORT0X)
    }
    #[doc = "APORT0Y used. EXT_BASE = ACMP_INPUTSEL_POSSEL_APORT0YCH0."]
    #[inline(always)]
    pub fn aport0y(self) -> &'a mut W {
        self.variant(APORTSEL_A::APORT0Y)
    }
    #[doc = "APORT1X used. EXT_BASE = ACMP_INPUTSEL_POSSEL_APORT1XCH0."]
    #[inline(always)]
    pub fn aport1x(self) -> &'a mut W {
        self.variant(APORTSEL_A::APORT1X)
    }
    #[doc = "APORT1Y used. EXT_BASE = ACMP_INPUTSEL_POSSEL_APORT1XCH0."]
    #[inline(always)]
    pub fn aport1y(self) -> &'a mut W {
        self.variant(APORTSEL_A::APORT1Y)
    }
    #[doc = "APORT1X/Y used. EXT_BASE = ACMP_INPUTSEL_POSSEL_APORT1XCH0."]
    #[inline(always)]
    pub fn aport1xy(self) -> &'a mut W {
        self.variant(APORTSEL_A::APORT1XY)
    }
    #[doc = "APORT2X used. EXT_BASE = ACMP_INPUTSEL_POSSEL_APORT2YCH0."]
    #[inline(always)]
    pub fn aport2x(self) -> &'a mut W {
        self.variant(APORTSEL_A::APORT2X)
    }
    #[doc = "APORT2Y used. EXT_BASE = ACMP_INPUTSEL_POSSEL_APORT2YCH0."]
    #[inline(always)]
    pub fn aport2y(self) -> &'a mut W {
        self.variant(APORTSEL_A::APORT2Y)
    }
    #[doc = "APORT2Y/X used. EXT_BASE = ACMP_INPUTSEL_POSSEL_APORT2YCH0."]
    #[inline(always)]
    pub fn aport2yx(self) -> &'a mut W {
        self.variant(APORTSEL_A::APORT2YX)
    }
    #[doc = "APORT3X used. EXT_BASE = ACMP_INPUTSEL_POSSEL_APORT3XCH0."]
    #[inline(always)]
    pub fn aport3x(self) -> &'a mut W {
        self.variant(APORTSEL_A::APORT3X)
    }
    #[doc = "APORT3Y used. EXT_BASE = ACMP_INPUTSEL_POSSEL_APORT3XCH0."]
    #[inline(always)]
    pub fn aport3y(self) -> &'a mut W {
        self.variant(APORTSEL_A::APORT3Y)
    }
    #[doc = "APORT3X/Y used. EXT_BASE = ACMP_INPUTSEL_POSSEL_APORT3XCH0."]
    #[inline(always)]
    pub fn aport3xy(self) -> &'a mut W {
        self.variant(APORTSEL_A::APORT3XY)
    }
    #[doc = "APORT4X used. EXT_BASE = ACMP_INPUTSEL_POSSEL_APORT4YCH0."]
    #[inline(always)]
    pub fn aport4x(self) -> &'a mut W {
        self.variant(APORTSEL_A::APORT4X)
    }
    #[doc = "APORT4Y used. EXT_BASE = ACMP_INPUTSEL_POSSEL_APORT4YCH0."]
    #[inline(always)]
    pub fn aport4y(self) -> &'a mut W {
        self.variant(APORTSEL_A::APORT4Y)
    }
    #[doc = "APORT4Y/X used. EXT_BASE = ACMP_INPUTSEL_POSSEL_APORT4YCH0."]
    #[inline(always)]
    pub fn aport4yx(self) -> &'a mut W {
        self.variant(APORTSEL_A::APORT4YX)
    }
}
impl R {
    #[doc = "Bit 0 - Enable External Interface"]
    #[inline(always)]
    pub fn en(&self) -> EN_R {
        EN_R::new((self.bits & 1) != 0)
    }
    #[doc = "Bits 4:7 - APORT Selection for External Interface"]
    #[inline(always)]
    pub fn aportsel(&self) -> APORTSEL_R {
        APORTSEL_R::new(((self.bits >> 4) & 0x0f) as u8)
    }
}
impl W {
    #[doc = "Bit 0 - Enable External Interface"]
    #[inline(always)]
    pub fn en(&mut self) -> EN_W {
        EN_W::new(self)
    }
    #[doc = "Bits 4:7 - APORT Selection for External Interface"]
    #[inline(always)]
    pub fn aportsel(&mut self) -> APORTSEL_W {
        APORTSEL_W::new(self)
    }
    #[doc = "Writes raw bits to the register."]
    #[inline(always)]
    pub unsafe fn bits(&mut self, bits: u32) -> &mut Self {
        self.0.bits(bits);
        self
    }
}
#[doc = "External Override Interface Control\n\nThis register you can [`read`](crate::generic::Reg::read), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [extifctrl](index.html) module"]
pub struct EXTIFCTRL_SPEC;
impl crate::RegisterSpec for EXTIFCTRL_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [extifctrl::R](R) reader structure"]
impl crate::Readable for EXTIFCTRL_SPEC {
    type Reader = R;
}
#[doc = "`write(|w| ..)` method takes [extifctrl::W](W) writer structure"]
impl crate::Writable for EXTIFCTRL_SPEC {
    type Writer = W;
}
#[doc = "`reset()` method sets EXTIFCTRL to value 0"]
impl crate::Resettable for EXTIFCTRL_SPEC {
    #[inline(always)]
    fn reset_value() -> Self::Ux {
        0
    }
}
