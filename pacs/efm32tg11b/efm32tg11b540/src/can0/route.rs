#[doc = "Register `ROUTE` reader"]
pub struct R(crate::R<ROUTE_SPEC>);
impl core::ops::Deref for R {
    type Target = crate::R<ROUTE_SPEC>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
impl From<crate::R<ROUTE_SPEC>> for R {
    #[inline(always)]
    fn from(reader: crate::R<ROUTE_SPEC>) -> Self {
        R(reader)
    }
}
#[doc = "Register `ROUTE` writer"]
pub struct W(crate::W<ROUTE_SPEC>);
impl core::ops::Deref for W {
    type Target = crate::W<ROUTE_SPEC>;
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
impl From<crate::W<ROUTE_SPEC>> for W {
    #[inline(always)]
    fn from(writer: crate::W<ROUTE_SPEC>) -> Self {
        W(writer)
    }
}
#[doc = "Field `TXPEN` reader - TX Pin Enable"]
pub type TXPEN_R = crate::BitReader<bool>;
#[doc = "Field `TXPEN` writer - TX Pin Enable"]
pub type TXPEN_W<'a> = crate::BitWriter<'a, u32, ROUTE_SPEC, bool, 0>;
#[doc = "RX Pin Location\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq)]
#[repr(u8)]
pub enum RXLOC_A {
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
    #[doc = "6: Location 6"]
    LOC6 = 6,
    #[doc = "7: Location 7"]
    LOC7 = 7,
}
impl From<RXLOC_A> for u8 {
    #[inline(always)]
    fn from(variant: RXLOC_A) -> Self {
        variant as _
    }
}
#[doc = "Field `RXLOC` reader - RX Pin Location"]
pub type RXLOC_R = crate::FieldReader<u8, RXLOC_A>;
impl RXLOC_R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> Option<RXLOC_A> {
        match self.bits {
            0 => Some(RXLOC_A::LOC0),
            1 => Some(RXLOC_A::LOC1),
            2 => Some(RXLOC_A::LOC2),
            3 => Some(RXLOC_A::LOC3),
            4 => Some(RXLOC_A::LOC4),
            5 => Some(RXLOC_A::LOC5),
            6 => Some(RXLOC_A::LOC6),
            7 => Some(RXLOC_A::LOC7),
            _ => None,
        }
    }
    #[doc = "Checks if the value of the field is `LOC0`"]
    #[inline(always)]
    pub fn is_loc0(&self) -> bool {
        *self == RXLOC_A::LOC0
    }
    #[doc = "Checks if the value of the field is `LOC1`"]
    #[inline(always)]
    pub fn is_loc1(&self) -> bool {
        *self == RXLOC_A::LOC1
    }
    #[doc = "Checks if the value of the field is `LOC2`"]
    #[inline(always)]
    pub fn is_loc2(&self) -> bool {
        *self == RXLOC_A::LOC2
    }
    #[doc = "Checks if the value of the field is `LOC3`"]
    #[inline(always)]
    pub fn is_loc3(&self) -> bool {
        *self == RXLOC_A::LOC3
    }
    #[doc = "Checks if the value of the field is `LOC4`"]
    #[inline(always)]
    pub fn is_loc4(&self) -> bool {
        *self == RXLOC_A::LOC4
    }
    #[doc = "Checks if the value of the field is `LOC5`"]
    #[inline(always)]
    pub fn is_loc5(&self) -> bool {
        *self == RXLOC_A::LOC5
    }
    #[doc = "Checks if the value of the field is `LOC6`"]
    #[inline(always)]
    pub fn is_loc6(&self) -> bool {
        *self == RXLOC_A::LOC6
    }
    #[doc = "Checks if the value of the field is `LOC7`"]
    #[inline(always)]
    pub fn is_loc7(&self) -> bool {
        *self == RXLOC_A::LOC7
    }
}
#[doc = "Field `RXLOC` writer - RX Pin Location"]
pub type RXLOC_W<'a> = crate::FieldWriter<'a, u32, ROUTE_SPEC, u8, RXLOC_A, 6, 2>;
impl<'a> RXLOC_W<'a> {
    #[doc = "Location 0"]
    #[inline(always)]
    pub fn loc0(self) -> &'a mut W {
        self.variant(RXLOC_A::LOC0)
    }
    #[doc = "Location 1"]
    #[inline(always)]
    pub fn loc1(self) -> &'a mut W {
        self.variant(RXLOC_A::LOC1)
    }
    #[doc = "Location 2"]
    #[inline(always)]
    pub fn loc2(self) -> &'a mut W {
        self.variant(RXLOC_A::LOC2)
    }
    #[doc = "Location 3"]
    #[inline(always)]
    pub fn loc3(self) -> &'a mut W {
        self.variant(RXLOC_A::LOC3)
    }
    #[doc = "Location 4"]
    #[inline(always)]
    pub fn loc4(self) -> &'a mut W {
        self.variant(RXLOC_A::LOC4)
    }
    #[doc = "Location 5"]
    #[inline(always)]
    pub fn loc5(self) -> &'a mut W {
        self.variant(RXLOC_A::LOC5)
    }
    #[doc = "Location 6"]
    #[inline(always)]
    pub fn loc6(self) -> &'a mut W {
        self.variant(RXLOC_A::LOC6)
    }
    #[doc = "Location 7"]
    #[inline(always)]
    pub fn loc7(self) -> &'a mut W {
        self.variant(RXLOC_A::LOC7)
    }
}
#[doc = "TX Pin Location\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq)]
#[repr(u8)]
pub enum TXLOC_A {
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
    #[doc = "6: Location 6"]
    LOC6 = 6,
    #[doc = "7: Location 7"]
    LOC7 = 7,
}
impl From<TXLOC_A> for u8 {
    #[inline(always)]
    fn from(variant: TXLOC_A) -> Self {
        variant as _
    }
}
#[doc = "Field `TXLOC` reader - TX Pin Location"]
pub type TXLOC_R = crate::FieldReader<u8, TXLOC_A>;
impl TXLOC_R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> Option<TXLOC_A> {
        match self.bits {
            0 => Some(TXLOC_A::LOC0),
            1 => Some(TXLOC_A::LOC1),
            2 => Some(TXLOC_A::LOC2),
            3 => Some(TXLOC_A::LOC3),
            4 => Some(TXLOC_A::LOC4),
            5 => Some(TXLOC_A::LOC5),
            6 => Some(TXLOC_A::LOC6),
            7 => Some(TXLOC_A::LOC7),
            _ => None,
        }
    }
    #[doc = "Checks if the value of the field is `LOC0`"]
    #[inline(always)]
    pub fn is_loc0(&self) -> bool {
        *self == TXLOC_A::LOC0
    }
    #[doc = "Checks if the value of the field is `LOC1`"]
    #[inline(always)]
    pub fn is_loc1(&self) -> bool {
        *self == TXLOC_A::LOC1
    }
    #[doc = "Checks if the value of the field is `LOC2`"]
    #[inline(always)]
    pub fn is_loc2(&self) -> bool {
        *self == TXLOC_A::LOC2
    }
    #[doc = "Checks if the value of the field is `LOC3`"]
    #[inline(always)]
    pub fn is_loc3(&self) -> bool {
        *self == TXLOC_A::LOC3
    }
    #[doc = "Checks if the value of the field is `LOC4`"]
    #[inline(always)]
    pub fn is_loc4(&self) -> bool {
        *self == TXLOC_A::LOC4
    }
    #[doc = "Checks if the value of the field is `LOC5`"]
    #[inline(always)]
    pub fn is_loc5(&self) -> bool {
        *self == TXLOC_A::LOC5
    }
    #[doc = "Checks if the value of the field is `LOC6`"]
    #[inline(always)]
    pub fn is_loc6(&self) -> bool {
        *self == TXLOC_A::LOC6
    }
    #[doc = "Checks if the value of the field is `LOC7`"]
    #[inline(always)]
    pub fn is_loc7(&self) -> bool {
        *self == TXLOC_A::LOC7
    }
}
#[doc = "Field `TXLOC` writer - TX Pin Location"]
pub type TXLOC_W<'a> = crate::FieldWriter<'a, u32, ROUTE_SPEC, u8, TXLOC_A, 6, 8>;
impl<'a> TXLOC_W<'a> {
    #[doc = "Location 0"]
    #[inline(always)]
    pub fn loc0(self) -> &'a mut W {
        self.variant(TXLOC_A::LOC0)
    }
    #[doc = "Location 1"]
    #[inline(always)]
    pub fn loc1(self) -> &'a mut W {
        self.variant(TXLOC_A::LOC1)
    }
    #[doc = "Location 2"]
    #[inline(always)]
    pub fn loc2(self) -> &'a mut W {
        self.variant(TXLOC_A::LOC2)
    }
    #[doc = "Location 3"]
    #[inline(always)]
    pub fn loc3(self) -> &'a mut W {
        self.variant(TXLOC_A::LOC3)
    }
    #[doc = "Location 4"]
    #[inline(always)]
    pub fn loc4(self) -> &'a mut W {
        self.variant(TXLOC_A::LOC4)
    }
    #[doc = "Location 5"]
    #[inline(always)]
    pub fn loc5(self) -> &'a mut W {
        self.variant(TXLOC_A::LOC5)
    }
    #[doc = "Location 6"]
    #[inline(always)]
    pub fn loc6(self) -> &'a mut W {
        self.variant(TXLOC_A::LOC6)
    }
    #[doc = "Location 7"]
    #[inline(always)]
    pub fn loc7(self) -> &'a mut W {
        self.variant(TXLOC_A::LOC7)
    }
}
impl R {
    #[doc = "Bit 0 - TX Pin Enable"]
    #[inline(always)]
    pub fn txpen(&self) -> TXPEN_R {
        TXPEN_R::new((self.bits & 1) != 0)
    }
    #[doc = "Bits 2:7 - RX Pin Location"]
    #[inline(always)]
    pub fn rxloc(&self) -> RXLOC_R {
        RXLOC_R::new(((self.bits >> 2) & 0x3f) as u8)
    }
    #[doc = "Bits 8:13 - TX Pin Location"]
    #[inline(always)]
    pub fn txloc(&self) -> TXLOC_R {
        TXLOC_R::new(((self.bits >> 8) & 0x3f) as u8)
    }
}
impl W {
    #[doc = "Bit 0 - TX Pin Enable"]
    #[inline(always)]
    pub fn txpen(&mut self) -> TXPEN_W {
        TXPEN_W::new(self)
    }
    #[doc = "Bits 2:7 - RX Pin Location"]
    #[inline(always)]
    pub fn rxloc(&mut self) -> RXLOC_W {
        RXLOC_W::new(self)
    }
    #[doc = "Bits 8:13 - TX Pin Location"]
    #[inline(always)]
    pub fn txloc(&mut self) -> TXLOC_W {
        TXLOC_W::new(self)
    }
    #[doc = "Writes raw bits to the register."]
    #[inline(always)]
    pub unsafe fn bits(&mut self, bits: u32) -> &mut Self {
        self.0.bits(bits);
        self
    }
}
#[doc = "I/O Routing Register\n\nThis register you can [`read`](crate::generic::Reg::read), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [route](index.html) module"]
pub struct ROUTE_SPEC;
impl crate::RegisterSpec for ROUTE_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [route::R](R) reader structure"]
impl crate::Readable for ROUTE_SPEC {
    type Reader = R;
}
#[doc = "`write(|w| ..)` method takes [route::W](W) writer structure"]
impl crate::Writable for ROUTE_SPEC {
    type Writer = W;
}
#[doc = "`reset()` method sets ROUTE to value 0"]
impl crate::Resettable for ROUTE_SPEC {
    #[inline(always)]
    fn reset_value() -> Self::Ux {
        0
    }
}
