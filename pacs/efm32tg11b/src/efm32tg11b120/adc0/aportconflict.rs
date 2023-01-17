#[doc = "Register `APORTCONFLICT` reader"]
pub struct R(crate::R<APORTCONFLICT_SPEC>);
impl core::ops::Deref for R {
    type Target = crate::R<APORTCONFLICT_SPEC>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
impl From<crate::R<APORTCONFLICT_SPEC>> for R {
    #[inline(always)]
    fn from(reader: crate::R<APORTCONFLICT_SPEC>) -> Self {
        R(reader)
    }
}
#[doc = "Field `APORT0XCONFLICT` reader - 1 If the Bus Connected to APORT0X is in Conflict With Another Peripheral"]
pub type APORT0XCONFLICT_R = crate::BitReader<bool>;
#[doc = "Field `APORT0YCONFLICT` reader - 1 If the Bus Connected to APORT0Y is in Conflict With Another Peripheral"]
pub type APORT0YCONFLICT_R = crate::BitReader<bool>;
#[doc = "Field `APORT1XCONFLICT` reader - 1 If the Bus Connected to APORT1X is in Conflict With Another Peripheral"]
pub type APORT1XCONFLICT_R = crate::BitReader<bool>;
#[doc = "Field `APORT1YCONFLICT` reader - 1 If the Bus Connected to APORT1Y is in Conflict With Another Peripheral"]
pub type APORT1YCONFLICT_R = crate::BitReader<bool>;
#[doc = "Field `APORT2XCONFLICT` reader - 1 If the Bus Connected to APORT2X is in Conflict With Another Peripheral"]
pub type APORT2XCONFLICT_R = crate::BitReader<bool>;
#[doc = "Field `APORT2YCONFLICT` reader - 1 If the Bus Connected to APORT2Y is in Conflict With Another Peripheral"]
pub type APORT2YCONFLICT_R = crate::BitReader<bool>;
#[doc = "Field `APORT3XCONFLICT` reader - 1 If the Bus Connected to APORT3X is in Conflict With Another Peripheral"]
pub type APORT3XCONFLICT_R = crate::BitReader<bool>;
#[doc = "Field `APORT3YCONFLICT` reader - 1 If the Bus Connected to APORT3Y is in Conflict With Another Peripheral"]
pub type APORT3YCONFLICT_R = crate::BitReader<bool>;
#[doc = "Field `APORT4XCONFLICT` reader - 1 If the Bus Connected to APORT4X is in Conflict With Another Peripheral"]
pub type APORT4XCONFLICT_R = crate::BitReader<bool>;
#[doc = "Field `APORT4YCONFLICT` reader - 1 If the Bus Connected to APORT4Y is in Conflict With Another Peripheral"]
pub type APORT4YCONFLICT_R = crate::BitReader<bool>;
impl R {
    #[doc = "Bit 0 - 1 If the Bus Connected to APORT0X is in Conflict With Another Peripheral"]
    #[inline(always)]
    pub fn aport0xconflict(&self) -> APORT0XCONFLICT_R {
        APORT0XCONFLICT_R::new((self.bits & 1) != 0)
    }
    #[doc = "Bit 1 - 1 If the Bus Connected to APORT0Y is in Conflict With Another Peripheral"]
    #[inline(always)]
    pub fn aport0yconflict(&self) -> APORT0YCONFLICT_R {
        APORT0YCONFLICT_R::new(((self.bits >> 1) & 1) != 0)
    }
    #[doc = "Bit 2 - 1 If the Bus Connected to APORT1X is in Conflict With Another Peripheral"]
    #[inline(always)]
    pub fn aport1xconflict(&self) -> APORT1XCONFLICT_R {
        APORT1XCONFLICT_R::new(((self.bits >> 2) & 1) != 0)
    }
    #[doc = "Bit 3 - 1 If the Bus Connected to APORT1Y is in Conflict With Another Peripheral"]
    #[inline(always)]
    pub fn aport1yconflict(&self) -> APORT1YCONFLICT_R {
        APORT1YCONFLICT_R::new(((self.bits >> 3) & 1) != 0)
    }
    #[doc = "Bit 4 - 1 If the Bus Connected to APORT2X is in Conflict With Another Peripheral"]
    #[inline(always)]
    pub fn aport2xconflict(&self) -> APORT2XCONFLICT_R {
        APORT2XCONFLICT_R::new(((self.bits >> 4) & 1) != 0)
    }
    #[doc = "Bit 5 - 1 If the Bus Connected to APORT2Y is in Conflict With Another Peripheral"]
    #[inline(always)]
    pub fn aport2yconflict(&self) -> APORT2YCONFLICT_R {
        APORT2YCONFLICT_R::new(((self.bits >> 5) & 1) != 0)
    }
    #[doc = "Bit 6 - 1 If the Bus Connected to APORT3X is in Conflict With Another Peripheral"]
    #[inline(always)]
    pub fn aport3xconflict(&self) -> APORT3XCONFLICT_R {
        APORT3XCONFLICT_R::new(((self.bits >> 6) & 1) != 0)
    }
    #[doc = "Bit 7 - 1 If the Bus Connected to APORT3Y is in Conflict With Another Peripheral"]
    #[inline(always)]
    pub fn aport3yconflict(&self) -> APORT3YCONFLICT_R {
        APORT3YCONFLICT_R::new(((self.bits >> 7) & 1) != 0)
    }
    #[doc = "Bit 8 - 1 If the Bus Connected to APORT4X is in Conflict With Another Peripheral"]
    #[inline(always)]
    pub fn aport4xconflict(&self) -> APORT4XCONFLICT_R {
        APORT4XCONFLICT_R::new(((self.bits >> 8) & 1) != 0)
    }
    #[doc = "Bit 9 - 1 If the Bus Connected to APORT4Y is in Conflict With Another Peripheral"]
    #[inline(always)]
    pub fn aport4yconflict(&self) -> APORT4YCONFLICT_R {
        APORT4YCONFLICT_R::new(((self.bits >> 9) & 1) != 0)
    }
}
#[doc = "APORT Conflict Status Register\n\nThis register you can [`read`](crate::generic::Reg::read). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [aportconflict](index.html) module"]
pub struct APORTCONFLICT_SPEC;
impl crate::RegisterSpec for APORTCONFLICT_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [aportconflict::R](R) reader structure"]
impl crate::Readable for APORTCONFLICT_SPEC {
    type Reader = R;
}
#[doc = "`reset()` method sets APORTCONFLICT to value 0"]
impl crate::Resettable for APORTCONFLICT_SPEC {
    const RESET_VALUE: Self::Ux = 0;
}
