#[doc = "Register `ROUTELOC0` reader"]
pub struct R(crate::R<ROUTELOC0_SPEC>);
impl core::ops::Deref for R {
    type Target = crate::R<ROUTELOC0_SPEC>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
impl From<crate::R<ROUTELOC0_SPEC>> for R {
    #[inline(always)]
    fn from(reader: crate::R<ROUTELOC0_SPEC>) -> Self {
        R(reader)
    }
}
#[doc = "Register `ROUTELOC0` writer"]
pub struct W(crate::W<ROUTELOC0_SPEC>);
impl core::ops::Deref for W {
    type Target = crate::W<ROUTELOC0_SPEC>;
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
impl From<crate::W<ROUTELOC0_SPEC>> for W {
    #[inline(always)]
    fn from(writer: crate::W<ROUTELOC0_SPEC>) -> Self {
        W(writer)
    }
}
#[doc = "Field `CLKOUT0LOC` reader - I/O Location"]
pub type CLKOUT0LOC_R = crate::FieldReader<u8, CLKOUT0LOC_A>;
#[doc = "I/O Location\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
#[repr(u8)]
pub enum CLKOUT0LOC_A {
    #[doc = "0: Location 0"]
    LOC0 = 0,
    #[doc = "1: Location 1"]
    LOC1 = 1,
    #[doc = "2: Location 2"]
    LOC2 = 2,
    #[doc = "3: Location 3"]
    LOC3 = 3,
    #[doc = "4: Location 4"]
    LOC4 = 4,
    #[doc = "5: Location 5"]
    LOC5 = 5,
}
impl From<CLKOUT0LOC_A> for u8 {
    #[inline(always)]
    fn from(variant: CLKOUT0LOC_A) -> Self {
        variant as _
    }
}
impl CLKOUT0LOC_R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> Option<CLKOUT0LOC_A> {
        match self.bits {
            0 => Some(CLKOUT0LOC_A::LOC0),
            1 => Some(CLKOUT0LOC_A::LOC1),
            2 => Some(CLKOUT0LOC_A::LOC2),
            3 => Some(CLKOUT0LOC_A::LOC3),
            4 => Some(CLKOUT0LOC_A::LOC4),
            5 => Some(CLKOUT0LOC_A::LOC5),
            _ => None,
        }
    }
    #[doc = "Checks if the value of the field is `LOC0`"]
    #[inline(always)]
    pub fn is_loc0(&self) -> bool {
        *self == CLKOUT0LOC_A::LOC0
    }
    #[doc = "Checks if the value of the field is `LOC1`"]
    #[inline(always)]
    pub fn is_loc1(&self) -> bool {
        *self == CLKOUT0LOC_A::LOC1
    }
    #[doc = "Checks if the value of the field is `LOC2`"]
    #[inline(always)]
    pub fn is_loc2(&self) -> bool {
        *self == CLKOUT0LOC_A::LOC2
    }
    #[doc = "Checks if the value of the field is `LOC3`"]
    #[inline(always)]
    pub fn is_loc3(&self) -> bool {
        *self == CLKOUT0LOC_A::LOC3
    }
    #[doc = "Checks if the value of the field is `LOC4`"]
    #[inline(always)]
    pub fn is_loc4(&self) -> bool {
        *self == CLKOUT0LOC_A::LOC4
    }
    #[doc = "Checks if the value of the field is `LOC5`"]
    #[inline(always)]
    pub fn is_loc5(&self) -> bool {
        *self == CLKOUT0LOC_A::LOC5
    }
}
#[doc = "Field `CLKOUT0LOC` writer - I/O Location"]
pub type CLKOUT0LOC_W<'a, const O: u8> =
    crate::FieldWriter<'a, u32, ROUTELOC0_SPEC, u8, CLKOUT0LOC_A, 6, O>;
impl<'a, const O: u8> CLKOUT0LOC_W<'a, O> {
    #[doc = "Location 0"]
    #[inline(always)]
    pub fn loc0(self) -> &'a mut W {
        self.variant(CLKOUT0LOC_A::LOC0)
    }
    #[doc = "Location 1"]
    #[inline(always)]
    pub fn loc1(self) -> &'a mut W {
        self.variant(CLKOUT0LOC_A::LOC1)
    }
    #[doc = "Location 2"]
    #[inline(always)]
    pub fn loc2(self) -> &'a mut W {
        self.variant(CLKOUT0LOC_A::LOC2)
    }
    #[doc = "Location 3"]
    #[inline(always)]
    pub fn loc3(self) -> &'a mut W {
        self.variant(CLKOUT0LOC_A::LOC3)
    }
    #[doc = "Location 4"]
    #[inline(always)]
    pub fn loc4(self) -> &'a mut W {
        self.variant(CLKOUT0LOC_A::LOC4)
    }
    #[doc = "Location 5"]
    #[inline(always)]
    pub fn loc5(self) -> &'a mut W {
        self.variant(CLKOUT0LOC_A::LOC5)
    }
}
#[doc = "Field `CLKOUT1LOC` reader - I/O Location"]
pub type CLKOUT1LOC_R = crate::FieldReader<u8, CLKOUT1LOC_A>;
#[doc = "I/O Location\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
#[repr(u8)]
pub enum CLKOUT1LOC_A {
    #[doc = "0: Location 0"]
    LOC0 = 0,
    #[doc = "1: Location 1"]
    LOC1 = 1,
    #[doc = "2: Location 2"]
    LOC2 = 2,
    #[doc = "3: Location 3"]
    LOC3 = 3,
    #[doc = "4: Location 4"]
    LOC4 = 4,
    #[doc = "5: Location 5"]
    LOC5 = 5,
}
impl From<CLKOUT1LOC_A> for u8 {
    #[inline(always)]
    fn from(variant: CLKOUT1LOC_A) -> Self {
        variant as _
    }
}
impl CLKOUT1LOC_R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> Option<CLKOUT1LOC_A> {
        match self.bits {
            0 => Some(CLKOUT1LOC_A::LOC0),
            1 => Some(CLKOUT1LOC_A::LOC1),
            2 => Some(CLKOUT1LOC_A::LOC2),
            3 => Some(CLKOUT1LOC_A::LOC3),
            4 => Some(CLKOUT1LOC_A::LOC4),
            5 => Some(CLKOUT1LOC_A::LOC5),
            _ => None,
        }
    }
    #[doc = "Checks if the value of the field is `LOC0`"]
    #[inline(always)]
    pub fn is_loc0(&self) -> bool {
        *self == CLKOUT1LOC_A::LOC0
    }
    #[doc = "Checks if the value of the field is `LOC1`"]
    #[inline(always)]
    pub fn is_loc1(&self) -> bool {
        *self == CLKOUT1LOC_A::LOC1
    }
    #[doc = "Checks if the value of the field is `LOC2`"]
    #[inline(always)]
    pub fn is_loc2(&self) -> bool {
        *self == CLKOUT1LOC_A::LOC2
    }
    #[doc = "Checks if the value of the field is `LOC3`"]
    #[inline(always)]
    pub fn is_loc3(&self) -> bool {
        *self == CLKOUT1LOC_A::LOC3
    }
    #[doc = "Checks if the value of the field is `LOC4`"]
    #[inline(always)]
    pub fn is_loc4(&self) -> bool {
        *self == CLKOUT1LOC_A::LOC4
    }
    #[doc = "Checks if the value of the field is `LOC5`"]
    #[inline(always)]
    pub fn is_loc5(&self) -> bool {
        *self == CLKOUT1LOC_A::LOC5
    }
}
#[doc = "Field `CLKOUT1LOC` writer - I/O Location"]
pub type CLKOUT1LOC_W<'a, const O: u8> =
    crate::FieldWriter<'a, u32, ROUTELOC0_SPEC, u8, CLKOUT1LOC_A, 6, O>;
impl<'a, const O: u8> CLKOUT1LOC_W<'a, O> {
    #[doc = "Location 0"]
    #[inline(always)]
    pub fn loc0(self) -> &'a mut W {
        self.variant(CLKOUT1LOC_A::LOC0)
    }
    #[doc = "Location 1"]
    #[inline(always)]
    pub fn loc1(self) -> &'a mut W {
        self.variant(CLKOUT1LOC_A::LOC1)
    }
    #[doc = "Location 2"]
    #[inline(always)]
    pub fn loc2(self) -> &'a mut W {
        self.variant(CLKOUT1LOC_A::LOC2)
    }
    #[doc = "Location 3"]
    #[inline(always)]
    pub fn loc3(self) -> &'a mut W {
        self.variant(CLKOUT1LOC_A::LOC3)
    }
    #[doc = "Location 4"]
    #[inline(always)]
    pub fn loc4(self) -> &'a mut W {
        self.variant(CLKOUT1LOC_A::LOC4)
    }
    #[doc = "Location 5"]
    #[inline(always)]
    pub fn loc5(self) -> &'a mut W {
        self.variant(CLKOUT1LOC_A::LOC5)
    }
}
#[doc = "Field `CLKOUT2LOC` reader - I/O Location"]
pub type CLKOUT2LOC_R = crate::FieldReader<u8, CLKOUT2LOC_A>;
#[doc = "I/O Location\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
#[repr(u8)]
pub enum CLKOUT2LOC_A {
    #[doc = "0: Location 0"]
    LOC0 = 0,
    #[doc = "1: Location 1"]
    LOC1 = 1,
    #[doc = "2: Location 2"]
    LOC2 = 2,
    #[doc = "3: Location 3"]
    LOC3 = 3,
    #[doc = "4: Location 4"]
    LOC4 = 4,
    #[doc = "5: Location 5"]
    LOC5 = 5,
}
impl From<CLKOUT2LOC_A> for u8 {
    #[inline(always)]
    fn from(variant: CLKOUT2LOC_A) -> Self {
        variant as _
    }
}
impl CLKOUT2LOC_R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> Option<CLKOUT2LOC_A> {
        match self.bits {
            0 => Some(CLKOUT2LOC_A::LOC0),
            1 => Some(CLKOUT2LOC_A::LOC1),
            2 => Some(CLKOUT2LOC_A::LOC2),
            3 => Some(CLKOUT2LOC_A::LOC3),
            4 => Some(CLKOUT2LOC_A::LOC4),
            5 => Some(CLKOUT2LOC_A::LOC5),
            _ => None,
        }
    }
    #[doc = "Checks if the value of the field is `LOC0`"]
    #[inline(always)]
    pub fn is_loc0(&self) -> bool {
        *self == CLKOUT2LOC_A::LOC0
    }
    #[doc = "Checks if the value of the field is `LOC1`"]
    #[inline(always)]
    pub fn is_loc1(&self) -> bool {
        *self == CLKOUT2LOC_A::LOC1
    }
    #[doc = "Checks if the value of the field is `LOC2`"]
    #[inline(always)]
    pub fn is_loc2(&self) -> bool {
        *self == CLKOUT2LOC_A::LOC2
    }
    #[doc = "Checks if the value of the field is `LOC3`"]
    #[inline(always)]
    pub fn is_loc3(&self) -> bool {
        *self == CLKOUT2LOC_A::LOC3
    }
    #[doc = "Checks if the value of the field is `LOC4`"]
    #[inline(always)]
    pub fn is_loc4(&self) -> bool {
        *self == CLKOUT2LOC_A::LOC4
    }
    #[doc = "Checks if the value of the field is `LOC5`"]
    #[inline(always)]
    pub fn is_loc5(&self) -> bool {
        *self == CLKOUT2LOC_A::LOC5
    }
}
#[doc = "Field `CLKOUT2LOC` writer - I/O Location"]
pub type CLKOUT2LOC_W<'a, const O: u8> =
    crate::FieldWriter<'a, u32, ROUTELOC0_SPEC, u8, CLKOUT2LOC_A, 6, O>;
impl<'a, const O: u8> CLKOUT2LOC_W<'a, O> {
    #[doc = "Location 0"]
    #[inline(always)]
    pub fn loc0(self) -> &'a mut W {
        self.variant(CLKOUT2LOC_A::LOC0)
    }
    #[doc = "Location 1"]
    #[inline(always)]
    pub fn loc1(self) -> &'a mut W {
        self.variant(CLKOUT2LOC_A::LOC1)
    }
    #[doc = "Location 2"]
    #[inline(always)]
    pub fn loc2(self) -> &'a mut W {
        self.variant(CLKOUT2LOC_A::LOC2)
    }
    #[doc = "Location 3"]
    #[inline(always)]
    pub fn loc3(self) -> &'a mut W {
        self.variant(CLKOUT2LOC_A::LOC3)
    }
    #[doc = "Location 4"]
    #[inline(always)]
    pub fn loc4(self) -> &'a mut W {
        self.variant(CLKOUT2LOC_A::LOC4)
    }
    #[doc = "Location 5"]
    #[inline(always)]
    pub fn loc5(self) -> &'a mut W {
        self.variant(CLKOUT2LOC_A::LOC5)
    }
}
impl R {
    #[doc = "Bits 0:5 - I/O Location"]
    #[inline(always)]
    pub fn clkout0loc(&self) -> CLKOUT0LOC_R {
        CLKOUT0LOC_R::new((self.bits & 0x3f) as u8)
    }
    #[doc = "Bits 8:13 - I/O Location"]
    #[inline(always)]
    pub fn clkout1loc(&self) -> CLKOUT1LOC_R {
        CLKOUT1LOC_R::new(((self.bits >> 8) & 0x3f) as u8)
    }
    #[doc = "Bits 16:21 - I/O Location"]
    #[inline(always)]
    pub fn clkout2loc(&self) -> CLKOUT2LOC_R {
        CLKOUT2LOC_R::new(((self.bits >> 16) & 0x3f) as u8)
    }
}
impl W {
    #[doc = "Bits 0:5 - I/O Location"]
    #[inline(always)]
    #[must_use]
    pub fn clkout0loc(&mut self) -> CLKOUT0LOC_W<0> {
        CLKOUT0LOC_W::new(self)
    }
    #[doc = "Bits 8:13 - I/O Location"]
    #[inline(always)]
    #[must_use]
    pub fn clkout1loc(&mut self) -> CLKOUT1LOC_W<8> {
        CLKOUT1LOC_W::new(self)
    }
    #[doc = "Bits 16:21 - I/O Location"]
    #[inline(always)]
    #[must_use]
    pub fn clkout2loc(&mut self) -> CLKOUT2LOC_W<16> {
        CLKOUT2LOC_W::new(self)
    }
    #[doc = "Writes raw bits to the register."]
    #[inline(always)]
    pub unsafe fn bits(&mut self, bits: u32) -> &mut Self {
        self.0.bits(bits);
        self
    }
}
#[doc = "I/O Routing Location Register\n\nThis register you can [`read`](crate::generic::Reg::read), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [routeloc0](index.html) module"]
pub struct ROUTELOC0_SPEC;
impl crate::RegisterSpec for ROUTELOC0_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [routeloc0::R](R) reader structure"]
impl crate::Readable for ROUTELOC0_SPEC {
    type Reader = R;
}
#[doc = "`write(|w| ..)` method takes [routeloc0::W](W) writer structure"]
impl crate::Writable for ROUTELOC0_SPEC {
    type Writer = W;
    const ZERO_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
}
#[doc = "`reset()` method sets ROUTELOC0 to value 0"]
impl crate::Resettable for ROUTELOC0_SPEC {
    const RESET_VALUE: Self::Ux = 0;
}
