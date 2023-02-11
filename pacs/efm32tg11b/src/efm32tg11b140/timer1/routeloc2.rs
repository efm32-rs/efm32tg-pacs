#[doc = "Register `ROUTELOC2` reader"]
pub struct R(crate::R<ROUTELOC2_SPEC>);
impl core::ops::Deref for R {
    type Target = crate::R<ROUTELOC2_SPEC>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
impl From<crate::R<ROUTELOC2_SPEC>> for R {
    #[inline(always)]
    fn from(reader: crate::R<ROUTELOC2_SPEC>) -> Self {
        R(reader)
    }
}
#[doc = "Register `ROUTELOC2` writer"]
pub struct W(crate::W<ROUTELOC2_SPEC>);
impl core::ops::Deref for W {
    type Target = crate::W<ROUTELOC2_SPEC>;
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
impl From<crate::W<ROUTELOC2_SPEC>> for W {
    #[inline(always)]
    fn from(writer: crate::W<ROUTELOC2_SPEC>) -> Self {
        W(writer)
    }
}
#[doc = "Field `CDTI0LOC` reader - I/O Location"]
pub type CDTI0LOC_R = crate::FieldReader<u8, CDTI0LOC_A>;
#[doc = "I/O Location\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
#[repr(u8)]
pub enum CDTI0LOC_A {
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
}
impl From<CDTI0LOC_A> for u8 {
    #[inline(always)]
    fn from(variant: CDTI0LOC_A) -> Self {
        variant as _
    }
}
impl CDTI0LOC_R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> Option<CDTI0LOC_A> {
        match self.bits {
            0 => Some(CDTI0LOC_A::LOC0),
            1 => Some(CDTI0LOC_A::LOC1),
            2 => Some(CDTI0LOC_A::LOC2),
            3 => Some(CDTI0LOC_A::LOC3),
            4 => Some(CDTI0LOC_A::LOC4),
            _ => None,
        }
    }
    #[doc = "Checks if the value of the field is `LOC0`"]
    #[inline(always)]
    pub fn is_loc0(&self) -> bool {
        *self == CDTI0LOC_A::LOC0
    }
    #[doc = "Checks if the value of the field is `LOC1`"]
    #[inline(always)]
    pub fn is_loc1(&self) -> bool {
        *self == CDTI0LOC_A::LOC1
    }
    #[doc = "Checks if the value of the field is `LOC2`"]
    #[inline(always)]
    pub fn is_loc2(&self) -> bool {
        *self == CDTI0LOC_A::LOC2
    }
    #[doc = "Checks if the value of the field is `LOC3`"]
    #[inline(always)]
    pub fn is_loc3(&self) -> bool {
        *self == CDTI0LOC_A::LOC3
    }
    #[doc = "Checks if the value of the field is `LOC4`"]
    #[inline(always)]
    pub fn is_loc4(&self) -> bool {
        *self == CDTI0LOC_A::LOC4
    }
}
#[doc = "Field `CDTI0LOC` writer - I/O Location"]
pub type CDTI0LOC_W<'a, const O: u8> =
    crate::FieldWriter<'a, u32, ROUTELOC2_SPEC, u8, CDTI0LOC_A, 6, O>;
impl<'a, const O: u8> CDTI0LOC_W<'a, O> {
    #[doc = "Location 0"]
    #[inline(always)]
    pub fn loc0(self) -> &'a mut W {
        self.variant(CDTI0LOC_A::LOC0)
    }
    #[doc = "Location 1"]
    #[inline(always)]
    pub fn loc1(self) -> &'a mut W {
        self.variant(CDTI0LOC_A::LOC1)
    }
    #[doc = "Location 2"]
    #[inline(always)]
    pub fn loc2(self) -> &'a mut W {
        self.variant(CDTI0LOC_A::LOC2)
    }
    #[doc = "Location 3"]
    #[inline(always)]
    pub fn loc3(self) -> &'a mut W {
        self.variant(CDTI0LOC_A::LOC3)
    }
    #[doc = "Location 4"]
    #[inline(always)]
    pub fn loc4(self) -> &'a mut W {
        self.variant(CDTI0LOC_A::LOC4)
    }
}
#[doc = "Field `CDTI1LOC` reader - I/O Location"]
pub type CDTI1LOC_R = crate::FieldReader<u8, CDTI1LOC_A>;
#[doc = "I/O Location\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
#[repr(u8)]
pub enum CDTI1LOC_A {
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
}
impl From<CDTI1LOC_A> for u8 {
    #[inline(always)]
    fn from(variant: CDTI1LOC_A) -> Self {
        variant as _
    }
}
impl CDTI1LOC_R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> Option<CDTI1LOC_A> {
        match self.bits {
            0 => Some(CDTI1LOC_A::LOC0),
            1 => Some(CDTI1LOC_A::LOC1),
            2 => Some(CDTI1LOC_A::LOC2),
            3 => Some(CDTI1LOC_A::LOC3),
            4 => Some(CDTI1LOC_A::LOC4),
            _ => None,
        }
    }
    #[doc = "Checks if the value of the field is `LOC0`"]
    #[inline(always)]
    pub fn is_loc0(&self) -> bool {
        *self == CDTI1LOC_A::LOC0
    }
    #[doc = "Checks if the value of the field is `LOC1`"]
    #[inline(always)]
    pub fn is_loc1(&self) -> bool {
        *self == CDTI1LOC_A::LOC1
    }
    #[doc = "Checks if the value of the field is `LOC2`"]
    #[inline(always)]
    pub fn is_loc2(&self) -> bool {
        *self == CDTI1LOC_A::LOC2
    }
    #[doc = "Checks if the value of the field is `LOC3`"]
    #[inline(always)]
    pub fn is_loc3(&self) -> bool {
        *self == CDTI1LOC_A::LOC3
    }
    #[doc = "Checks if the value of the field is `LOC4`"]
    #[inline(always)]
    pub fn is_loc4(&self) -> bool {
        *self == CDTI1LOC_A::LOC4
    }
}
#[doc = "Field `CDTI1LOC` writer - I/O Location"]
pub type CDTI1LOC_W<'a, const O: u8> =
    crate::FieldWriter<'a, u32, ROUTELOC2_SPEC, u8, CDTI1LOC_A, 6, O>;
impl<'a, const O: u8> CDTI1LOC_W<'a, O> {
    #[doc = "Location 0"]
    #[inline(always)]
    pub fn loc0(self) -> &'a mut W {
        self.variant(CDTI1LOC_A::LOC0)
    }
    #[doc = "Location 1"]
    #[inline(always)]
    pub fn loc1(self) -> &'a mut W {
        self.variant(CDTI1LOC_A::LOC1)
    }
    #[doc = "Location 2"]
    #[inline(always)]
    pub fn loc2(self) -> &'a mut W {
        self.variant(CDTI1LOC_A::LOC2)
    }
    #[doc = "Location 3"]
    #[inline(always)]
    pub fn loc3(self) -> &'a mut W {
        self.variant(CDTI1LOC_A::LOC3)
    }
    #[doc = "Location 4"]
    #[inline(always)]
    pub fn loc4(self) -> &'a mut W {
        self.variant(CDTI1LOC_A::LOC4)
    }
}
#[doc = "Field `CDTI2LOC` reader - I/O Location"]
pub type CDTI2LOC_R = crate::FieldReader<u8, CDTI2LOC_A>;
#[doc = "I/O Location\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
#[repr(u8)]
pub enum CDTI2LOC_A {
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
}
impl From<CDTI2LOC_A> for u8 {
    #[inline(always)]
    fn from(variant: CDTI2LOC_A) -> Self {
        variant as _
    }
}
impl CDTI2LOC_R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> Option<CDTI2LOC_A> {
        match self.bits {
            0 => Some(CDTI2LOC_A::LOC0),
            1 => Some(CDTI2LOC_A::LOC1),
            2 => Some(CDTI2LOC_A::LOC2),
            3 => Some(CDTI2LOC_A::LOC3),
            4 => Some(CDTI2LOC_A::LOC4),
            _ => None,
        }
    }
    #[doc = "Checks if the value of the field is `LOC0`"]
    #[inline(always)]
    pub fn is_loc0(&self) -> bool {
        *self == CDTI2LOC_A::LOC0
    }
    #[doc = "Checks if the value of the field is `LOC1`"]
    #[inline(always)]
    pub fn is_loc1(&self) -> bool {
        *self == CDTI2LOC_A::LOC1
    }
    #[doc = "Checks if the value of the field is `LOC2`"]
    #[inline(always)]
    pub fn is_loc2(&self) -> bool {
        *self == CDTI2LOC_A::LOC2
    }
    #[doc = "Checks if the value of the field is `LOC3`"]
    #[inline(always)]
    pub fn is_loc3(&self) -> bool {
        *self == CDTI2LOC_A::LOC3
    }
    #[doc = "Checks if the value of the field is `LOC4`"]
    #[inline(always)]
    pub fn is_loc4(&self) -> bool {
        *self == CDTI2LOC_A::LOC4
    }
}
#[doc = "Field `CDTI2LOC` writer - I/O Location"]
pub type CDTI2LOC_W<'a, const O: u8> =
    crate::FieldWriter<'a, u32, ROUTELOC2_SPEC, u8, CDTI2LOC_A, 6, O>;
impl<'a, const O: u8> CDTI2LOC_W<'a, O> {
    #[doc = "Location 0"]
    #[inline(always)]
    pub fn loc0(self) -> &'a mut W {
        self.variant(CDTI2LOC_A::LOC0)
    }
    #[doc = "Location 1"]
    #[inline(always)]
    pub fn loc1(self) -> &'a mut W {
        self.variant(CDTI2LOC_A::LOC1)
    }
    #[doc = "Location 2"]
    #[inline(always)]
    pub fn loc2(self) -> &'a mut W {
        self.variant(CDTI2LOC_A::LOC2)
    }
    #[doc = "Location 3"]
    #[inline(always)]
    pub fn loc3(self) -> &'a mut W {
        self.variant(CDTI2LOC_A::LOC3)
    }
    #[doc = "Location 4"]
    #[inline(always)]
    pub fn loc4(self) -> &'a mut W {
        self.variant(CDTI2LOC_A::LOC4)
    }
}
impl R {
    #[doc = "Bits 0:5 - I/O Location"]
    #[inline(always)]
    pub fn cdti0loc(&self) -> CDTI0LOC_R {
        CDTI0LOC_R::new((self.bits & 0x3f) as u8)
    }
    #[doc = "Bits 8:13 - I/O Location"]
    #[inline(always)]
    pub fn cdti1loc(&self) -> CDTI1LOC_R {
        CDTI1LOC_R::new(((self.bits >> 8) & 0x3f) as u8)
    }
    #[doc = "Bits 16:21 - I/O Location"]
    #[inline(always)]
    pub fn cdti2loc(&self) -> CDTI2LOC_R {
        CDTI2LOC_R::new(((self.bits >> 16) & 0x3f) as u8)
    }
}
impl W {
    #[doc = "Bits 0:5 - I/O Location"]
    #[inline(always)]
    #[must_use]
    pub fn cdti0loc(&mut self) -> CDTI0LOC_W<0> {
        CDTI0LOC_W::new(self)
    }
    #[doc = "Bits 8:13 - I/O Location"]
    #[inline(always)]
    #[must_use]
    pub fn cdti1loc(&mut self) -> CDTI1LOC_W<8> {
        CDTI1LOC_W::new(self)
    }
    #[doc = "Bits 16:21 - I/O Location"]
    #[inline(always)]
    #[must_use]
    pub fn cdti2loc(&mut self) -> CDTI2LOC_W<16> {
        CDTI2LOC_W::new(self)
    }
    #[doc = "Writes raw bits to the register."]
    #[inline(always)]
    pub unsafe fn bits(&mut self, bits: u32) -> &mut Self {
        self.0.bits(bits);
        self
    }
}
#[doc = "I/O Routing Location Register\n\nThis register you can [`read`](crate::generic::Reg::read), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [routeloc2](index.html) module"]
pub struct ROUTELOC2_SPEC;
impl crate::RegisterSpec for ROUTELOC2_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [routeloc2::R](R) reader structure"]
impl crate::Readable for ROUTELOC2_SPEC {
    type Reader = R;
}
#[doc = "`write(|w| ..)` method takes [routeloc2::W](W) writer structure"]
impl crate::Writable for ROUTELOC2_SPEC {
    type Writer = W;
    const ZERO_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
}
#[doc = "`reset()` method sets ROUTELOC2 to value 0"]
impl crate::Resettable for ROUTELOC2_SPEC {
    const RESET_VALUE: Self::Ux = 0;
}
