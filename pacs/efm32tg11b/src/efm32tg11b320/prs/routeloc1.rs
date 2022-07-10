#[doc = "Register `ROUTELOC1` reader"]
pub struct R(crate::R<ROUTELOC1_SPEC>);
impl core::ops::Deref for R {
    type Target = crate::R<ROUTELOC1_SPEC>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
impl From<crate::R<ROUTELOC1_SPEC>> for R {
    #[inline(always)]
    fn from(reader: crate::R<ROUTELOC1_SPEC>) -> Self {
        R(reader)
    }
}
#[doc = "Register `ROUTELOC1` writer"]
pub struct W(crate::W<ROUTELOC1_SPEC>);
impl core::ops::Deref for W {
    type Target = crate::W<ROUTELOC1_SPEC>;
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
impl From<crate::W<ROUTELOC1_SPEC>> for W {
    #[inline(always)]
    fn from(writer: crate::W<ROUTELOC1_SPEC>) -> Self {
        W(writer)
    }
}
#[doc = "I/O Location\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq)]
#[repr(u8)]
pub enum CH4LOC_A {
    #[doc = "0: Location 0"]
    LOC0 = 0,
    #[doc = "1: Location 1"]
    LOC1 = 1,
    #[doc = "2: Location 2"]
    LOC2 = 2,
}
impl From<CH4LOC_A> for u8 {
    #[inline(always)]
    fn from(variant: CH4LOC_A) -> Self {
        variant as _
    }
}
#[doc = "Field `CH4LOC` reader - I/O Location"]
pub type CH4LOC_R = crate::FieldReader<u8, CH4LOC_A>;
impl CH4LOC_R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> Option<CH4LOC_A> {
        match self.bits {
            0 => Some(CH4LOC_A::LOC0),
            1 => Some(CH4LOC_A::LOC1),
            2 => Some(CH4LOC_A::LOC2),
            _ => None,
        }
    }
    #[doc = "Checks if the value of the field is `LOC0`"]
    #[inline(always)]
    pub fn is_loc0(&self) -> bool {
        *self == CH4LOC_A::LOC0
    }
    #[doc = "Checks if the value of the field is `LOC1`"]
    #[inline(always)]
    pub fn is_loc1(&self) -> bool {
        *self == CH4LOC_A::LOC1
    }
    #[doc = "Checks if the value of the field is `LOC2`"]
    #[inline(always)]
    pub fn is_loc2(&self) -> bool {
        *self == CH4LOC_A::LOC2
    }
}
#[doc = "Field `CH4LOC` writer - I/O Location"]
pub type CH4LOC_W<'a> = crate::FieldWriter<'a, u32, ROUTELOC1_SPEC, u8, CH4LOC_A, 6, 0>;
impl<'a> CH4LOC_W<'a> {
    #[doc = "Location 0"]
    #[inline(always)]
    pub fn loc0(self) -> &'a mut W {
        self.variant(CH4LOC_A::LOC0)
    }
    #[doc = "Location 1"]
    #[inline(always)]
    pub fn loc1(self) -> &'a mut W {
        self.variant(CH4LOC_A::LOC1)
    }
    #[doc = "Location 2"]
    #[inline(always)]
    pub fn loc2(self) -> &'a mut W {
        self.variant(CH4LOC_A::LOC2)
    }
}
#[doc = "I/O Location\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq)]
#[repr(u8)]
pub enum CH5LOC_A {
    #[doc = "0: Location 0"]
    LOC0 = 0,
    #[doc = "1: Location 1"]
    LOC1 = 1,
    #[doc = "2: Location 2"]
    LOC2 = 2,
}
impl From<CH5LOC_A> for u8 {
    #[inline(always)]
    fn from(variant: CH5LOC_A) -> Self {
        variant as _
    }
}
#[doc = "Field `CH5LOC` reader - I/O Location"]
pub type CH5LOC_R = crate::FieldReader<u8, CH5LOC_A>;
impl CH5LOC_R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> Option<CH5LOC_A> {
        match self.bits {
            0 => Some(CH5LOC_A::LOC0),
            1 => Some(CH5LOC_A::LOC1),
            2 => Some(CH5LOC_A::LOC2),
            _ => None,
        }
    }
    #[doc = "Checks if the value of the field is `LOC0`"]
    #[inline(always)]
    pub fn is_loc0(&self) -> bool {
        *self == CH5LOC_A::LOC0
    }
    #[doc = "Checks if the value of the field is `LOC1`"]
    #[inline(always)]
    pub fn is_loc1(&self) -> bool {
        *self == CH5LOC_A::LOC1
    }
    #[doc = "Checks if the value of the field is `LOC2`"]
    #[inline(always)]
    pub fn is_loc2(&self) -> bool {
        *self == CH5LOC_A::LOC2
    }
}
#[doc = "Field `CH5LOC` writer - I/O Location"]
pub type CH5LOC_W<'a> = crate::FieldWriter<'a, u32, ROUTELOC1_SPEC, u8, CH5LOC_A, 6, 8>;
impl<'a> CH5LOC_W<'a> {
    #[doc = "Location 0"]
    #[inline(always)]
    pub fn loc0(self) -> &'a mut W {
        self.variant(CH5LOC_A::LOC0)
    }
    #[doc = "Location 1"]
    #[inline(always)]
    pub fn loc1(self) -> &'a mut W {
        self.variant(CH5LOC_A::LOC1)
    }
    #[doc = "Location 2"]
    #[inline(always)]
    pub fn loc2(self) -> &'a mut W {
        self.variant(CH5LOC_A::LOC2)
    }
}
#[doc = "I/O Location\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq)]
#[repr(u8)]
pub enum CH6LOC_A {
    #[doc = "0: Location 0"]
    LOC0 = 0,
    #[doc = "1: Location 1"]
    LOC1 = 1,
    #[doc = "2: Location 2"]
    LOC2 = 2,
}
impl From<CH6LOC_A> for u8 {
    #[inline(always)]
    fn from(variant: CH6LOC_A) -> Self {
        variant as _
    }
}
#[doc = "Field `CH6LOC` reader - I/O Location"]
pub type CH6LOC_R = crate::FieldReader<u8, CH6LOC_A>;
impl CH6LOC_R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> Option<CH6LOC_A> {
        match self.bits {
            0 => Some(CH6LOC_A::LOC0),
            1 => Some(CH6LOC_A::LOC1),
            2 => Some(CH6LOC_A::LOC2),
            _ => None,
        }
    }
    #[doc = "Checks if the value of the field is `LOC0`"]
    #[inline(always)]
    pub fn is_loc0(&self) -> bool {
        *self == CH6LOC_A::LOC0
    }
    #[doc = "Checks if the value of the field is `LOC1`"]
    #[inline(always)]
    pub fn is_loc1(&self) -> bool {
        *self == CH6LOC_A::LOC1
    }
    #[doc = "Checks if the value of the field is `LOC2`"]
    #[inline(always)]
    pub fn is_loc2(&self) -> bool {
        *self == CH6LOC_A::LOC2
    }
}
#[doc = "Field `CH6LOC` writer - I/O Location"]
pub type CH6LOC_W<'a> = crate::FieldWriter<'a, u32, ROUTELOC1_SPEC, u8, CH6LOC_A, 6, 16>;
impl<'a> CH6LOC_W<'a> {
    #[doc = "Location 0"]
    #[inline(always)]
    pub fn loc0(self) -> &'a mut W {
        self.variant(CH6LOC_A::LOC0)
    }
    #[doc = "Location 1"]
    #[inline(always)]
    pub fn loc1(self) -> &'a mut W {
        self.variant(CH6LOC_A::LOC1)
    }
    #[doc = "Location 2"]
    #[inline(always)]
    pub fn loc2(self) -> &'a mut W {
        self.variant(CH6LOC_A::LOC2)
    }
}
#[doc = "I/O Location\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq)]
#[repr(u8)]
pub enum CH7LOC_A {
    #[doc = "0: Location 0"]
    LOC0 = 0,
    #[doc = "1: Location 1"]
    LOC1 = 1,
    #[doc = "2: Location 2"]
    LOC2 = 2,
}
impl From<CH7LOC_A> for u8 {
    #[inline(always)]
    fn from(variant: CH7LOC_A) -> Self {
        variant as _
    }
}
#[doc = "Field `CH7LOC` reader - I/O Location"]
pub type CH7LOC_R = crate::FieldReader<u8, CH7LOC_A>;
impl CH7LOC_R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> Option<CH7LOC_A> {
        match self.bits {
            0 => Some(CH7LOC_A::LOC0),
            1 => Some(CH7LOC_A::LOC1),
            2 => Some(CH7LOC_A::LOC2),
            _ => None,
        }
    }
    #[doc = "Checks if the value of the field is `LOC0`"]
    #[inline(always)]
    pub fn is_loc0(&self) -> bool {
        *self == CH7LOC_A::LOC0
    }
    #[doc = "Checks if the value of the field is `LOC1`"]
    #[inline(always)]
    pub fn is_loc1(&self) -> bool {
        *self == CH7LOC_A::LOC1
    }
    #[doc = "Checks if the value of the field is `LOC2`"]
    #[inline(always)]
    pub fn is_loc2(&self) -> bool {
        *self == CH7LOC_A::LOC2
    }
}
#[doc = "Field `CH7LOC` writer - I/O Location"]
pub type CH7LOC_W<'a> = crate::FieldWriter<'a, u32, ROUTELOC1_SPEC, u8, CH7LOC_A, 6, 24>;
impl<'a> CH7LOC_W<'a> {
    #[doc = "Location 0"]
    #[inline(always)]
    pub fn loc0(self) -> &'a mut W {
        self.variant(CH7LOC_A::LOC0)
    }
    #[doc = "Location 1"]
    #[inline(always)]
    pub fn loc1(self) -> &'a mut W {
        self.variant(CH7LOC_A::LOC1)
    }
    #[doc = "Location 2"]
    #[inline(always)]
    pub fn loc2(self) -> &'a mut W {
        self.variant(CH7LOC_A::LOC2)
    }
}
impl R {
    #[doc = "Bits 0:5 - I/O Location"]
    #[inline(always)]
    pub fn ch4loc(&self) -> CH4LOC_R {
        CH4LOC_R::new((self.bits & 0x3f) as u8)
    }
    #[doc = "Bits 8:13 - I/O Location"]
    #[inline(always)]
    pub fn ch5loc(&self) -> CH5LOC_R {
        CH5LOC_R::new(((self.bits >> 8) & 0x3f) as u8)
    }
    #[doc = "Bits 16:21 - I/O Location"]
    #[inline(always)]
    pub fn ch6loc(&self) -> CH6LOC_R {
        CH6LOC_R::new(((self.bits >> 16) & 0x3f) as u8)
    }
    #[doc = "Bits 24:29 - I/O Location"]
    #[inline(always)]
    pub fn ch7loc(&self) -> CH7LOC_R {
        CH7LOC_R::new(((self.bits >> 24) & 0x3f) as u8)
    }
}
impl W {
    #[doc = "Bits 0:5 - I/O Location"]
    #[inline(always)]
    pub fn ch4loc(&mut self) -> CH4LOC_W {
        CH4LOC_W::new(self)
    }
    #[doc = "Bits 8:13 - I/O Location"]
    #[inline(always)]
    pub fn ch5loc(&mut self) -> CH5LOC_W {
        CH5LOC_W::new(self)
    }
    #[doc = "Bits 16:21 - I/O Location"]
    #[inline(always)]
    pub fn ch6loc(&mut self) -> CH6LOC_W {
        CH6LOC_W::new(self)
    }
    #[doc = "Bits 24:29 - I/O Location"]
    #[inline(always)]
    pub fn ch7loc(&mut self) -> CH7LOC_W {
        CH7LOC_W::new(self)
    }
    #[doc = "Writes raw bits to the register."]
    #[inline(always)]
    pub unsafe fn bits(&mut self, bits: u32) -> &mut Self {
        self.0.bits(bits);
        self
    }
}
#[doc = "I/O Routing Location Register\n\nThis register you can [`read`](crate::generic::Reg::read), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [routeloc1](index.html) module"]
pub struct ROUTELOC1_SPEC;
impl crate::RegisterSpec for ROUTELOC1_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [routeloc1::R](R) reader structure"]
impl crate::Readable for ROUTELOC1_SPEC {
    type Reader = R;
}
#[doc = "`write(|w| ..)` method takes [routeloc1::W](W) writer structure"]
impl crate::Writable for ROUTELOC1_SPEC {
    type Writer = W;
}
#[doc = "`reset()` method sets ROUTELOC1 to value 0"]
impl crate::Resettable for ROUTELOC1_SPEC {
    #[inline(always)]
    fn reset_value() -> Self::Ux {
        0
    }
}
