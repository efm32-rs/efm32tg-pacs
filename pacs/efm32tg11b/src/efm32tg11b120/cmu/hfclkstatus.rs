#[doc = "Register `HFCLKSTATUS` reader"]
pub struct R(crate::R<HFCLKSTATUS_SPEC>);
impl core::ops::Deref for R {
    type Target = crate::R<HFCLKSTATUS_SPEC>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
impl From<crate::R<HFCLKSTATUS_SPEC>> for R {
    #[inline(always)]
    fn from(reader: crate::R<HFCLKSTATUS_SPEC>) -> Self {
        R(reader)
    }
}
#[doc = "Field `SELECTED` reader - HFCLK Selected"]
pub type SELECTED_R = crate::FieldReader<u8, SELECTED_A>;
#[doc = "HFCLK Selected\n\nValue on reset: 1"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
#[repr(u8)]
pub enum SELECTED_A {
    #[doc = "1: HFRCO is selected as HFCLK clock source"]
    HFRCO = 1,
    #[doc = "2: HFXO is selected as HFCLK clock source"]
    HFXO = 2,
    #[doc = "3: LFRCO is selected as HFCLK clock source"]
    LFRCO = 3,
    #[doc = "4: LFXO is selected as HFCLK clock source"]
    LFXO = 4,
    #[doc = "5: HFRCO divided by 2 is selected as HFCLK clock source"]
    HFRCODIV2 = 5,
    #[doc = "7: CLKIN0 is selected as HFCLK clock source"]
    CLKIN0 = 7,
}
impl From<SELECTED_A> for u8 {
    #[inline(always)]
    fn from(variant: SELECTED_A) -> Self {
        variant as _
    }
}
impl SELECTED_R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> Option<SELECTED_A> {
        match self.bits {
            1 => Some(SELECTED_A::HFRCO),
            2 => Some(SELECTED_A::HFXO),
            3 => Some(SELECTED_A::LFRCO),
            4 => Some(SELECTED_A::LFXO),
            5 => Some(SELECTED_A::HFRCODIV2),
            7 => Some(SELECTED_A::CLKIN0),
            _ => None,
        }
    }
    #[doc = "Checks if the value of the field is `HFRCO`"]
    #[inline(always)]
    pub fn is_hfrco(&self) -> bool {
        *self == SELECTED_A::HFRCO
    }
    #[doc = "Checks if the value of the field is `HFXO`"]
    #[inline(always)]
    pub fn is_hfxo(&self) -> bool {
        *self == SELECTED_A::HFXO
    }
    #[doc = "Checks if the value of the field is `LFRCO`"]
    #[inline(always)]
    pub fn is_lfrco(&self) -> bool {
        *self == SELECTED_A::LFRCO
    }
    #[doc = "Checks if the value of the field is `LFXO`"]
    #[inline(always)]
    pub fn is_lfxo(&self) -> bool {
        *self == SELECTED_A::LFXO
    }
    #[doc = "Checks if the value of the field is `HFRCODIV2`"]
    #[inline(always)]
    pub fn is_hfrcodiv2(&self) -> bool {
        *self == SELECTED_A::HFRCODIV2
    }
    #[doc = "Checks if the value of the field is `CLKIN0`"]
    #[inline(always)]
    pub fn is_clkin0(&self) -> bool {
        *self == SELECTED_A::CLKIN0
    }
}
impl R {
    #[doc = "Bits 0:2 - HFCLK Selected"]
    #[inline(always)]
    pub fn selected(&self) -> SELECTED_R {
        SELECTED_R::new((self.bits & 7) as u8)
    }
}
#[doc = "HFCLK Status Register\n\nThis register you can [`read`](crate::generic::Reg::read). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [hfclkstatus](index.html) module"]
pub struct HFCLKSTATUS_SPEC;
impl crate::RegisterSpec for HFCLKSTATUS_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [hfclkstatus::R](R) reader structure"]
impl crate::Readable for HFCLKSTATUS_SPEC {
    type Reader = R;
}
#[doc = "`reset()` method sets HFCLKSTATUS to value 0x01"]
impl crate::Resettable for HFCLKSTATUS_SPEC {
    const RESET_VALUE: Self::Ux = 0x01;
}
