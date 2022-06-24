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
#[doc = "Field `SWCLKPEN` reader - Serial Wire Clock Pin Enable"]
pub type SWCLKPEN_R = crate::BitReader<bool>;
#[doc = "Field `SWCLKPEN` writer - Serial Wire Clock Pin Enable"]
pub type SWCLKPEN_W<'a> = crate::BitWriter<'a, u32, ROUTE_SPEC, bool, 0>;
#[doc = "Field `SWDIOPEN` reader - Serial Wire Data Pin Enable"]
pub type SWDIOPEN_R = crate::BitReader<bool>;
#[doc = "Field `SWDIOPEN` writer - Serial Wire Data Pin Enable"]
pub type SWDIOPEN_W<'a> = crate::BitWriter<'a, u32, ROUTE_SPEC, bool, 1>;
#[doc = "Field `SWOPEN` reader - Serial Wire Viewer Output Pin Enable"]
pub type SWOPEN_R = crate::BitReader<bool>;
#[doc = "Field `SWOPEN` writer - Serial Wire Viewer Output Pin Enable"]
pub type SWOPEN_W<'a> = crate::BitWriter<'a, u32, ROUTE_SPEC, bool, 2>;
#[doc = "I/O Location\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq)]
#[repr(u8)]
pub enum SWLOCATION_A {
    #[doc = "0: Location 0"]
    LOC0 = 0,
    #[doc = "1: Location 1"]
    LOC1 = 1,
}
impl From<SWLOCATION_A> for u8 {
    #[inline(always)]
    fn from(variant: SWLOCATION_A) -> Self {
        variant as _
    }
}
#[doc = "Field `SWLOCATION` reader - I/O Location"]
pub type SWLOCATION_R = crate::FieldReader<u8, SWLOCATION_A>;
impl SWLOCATION_R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> Option<SWLOCATION_A> {
        match self.bits {
            0 => Some(SWLOCATION_A::LOC0),
            1 => Some(SWLOCATION_A::LOC1),
            _ => None,
        }
    }
    #[doc = "Checks if the value of the field is `LOC0`"]
    #[inline(always)]
    pub fn is_loc0(&self) -> bool {
        *self == SWLOCATION_A::LOC0
    }
    #[doc = "Checks if the value of the field is `LOC1`"]
    #[inline(always)]
    pub fn is_loc1(&self) -> bool {
        *self == SWLOCATION_A::LOC1
    }
}
#[doc = "Field `SWLOCATION` writer - I/O Location"]
pub type SWLOCATION_W<'a> = crate::FieldWriter<'a, u32, ROUTE_SPEC, u8, SWLOCATION_A, 2, 8>;
impl<'a> SWLOCATION_W<'a> {
    #[doc = "Location 0"]
    #[inline(always)]
    pub fn loc0(self) -> &'a mut W {
        self.variant(SWLOCATION_A::LOC0)
    }
    #[doc = "Location 1"]
    #[inline(always)]
    pub fn loc1(self) -> &'a mut W {
        self.variant(SWLOCATION_A::LOC1)
    }
}
impl R {
    #[doc = "Bit 0 - Serial Wire Clock Pin Enable"]
    #[inline(always)]
    pub fn swclkpen(&self) -> SWCLKPEN_R {
        SWCLKPEN_R::new((self.bits & 1) != 0)
    }
    #[doc = "Bit 1 - Serial Wire Data Pin Enable"]
    #[inline(always)]
    pub fn swdiopen(&self) -> SWDIOPEN_R {
        SWDIOPEN_R::new(((self.bits >> 1) & 1) != 0)
    }
    #[doc = "Bit 2 - Serial Wire Viewer Output Pin Enable"]
    #[inline(always)]
    pub fn swopen(&self) -> SWOPEN_R {
        SWOPEN_R::new(((self.bits >> 2) & 1) != 0)
    }
    #[doc = "Bits 8:9 - I/O Location"]
    #[inline(always)]
    pub fn swlocation(&self) -> SWLOCATION_R {
        SWLOCATION_R::new(((self.bits >> 8) & 3) as u8)
    }
}
impl W {
    #[doc = "Bit 0 - Serial Wire Clock Pin Enable"]
    #[inline(always)]
    pub fn swclkpen(&mut self) -> SWCLKPEN_W {
        SWCLKPEN_W::new(self)
    }
    #[doc = "Bit 1 - Serial Wire Data Pin Enable"]
    #[inline(always)]
    pub fn swdiopen(&mut self) -> SWDIOPEN_W {
        SWDIOPEN_W::new(self)
    }
    #[doc = "Bit 2 - Serial Wire Viewer Output Pin Enable"]
    #[inline(always)]
    pub fn swopen(&mut self) -> SWOPEN_W {
        SWOPEN_W::new(self)
    }
    #[doc = "Bits 8:9 - I/O Location"]
    #[inline(always)]
    pub fn swlocation(&mut self) -> SWLOCATION_W {
        SWLOCATION_W::new(self)
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
#[doc = "`reset()` method sets ROUTE to value 0x03"]
impl crate::Resettable for ROUTE_SPEC {
    #[inline(always)]
    fn reset_value() -> Self::Ux {
        0x03
    }
}
