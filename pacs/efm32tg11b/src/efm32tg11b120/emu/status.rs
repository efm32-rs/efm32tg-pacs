#[doc = "Register `STATUS` reader"]
pub struct R(crate::R<STATUS_SPEC>);
impl core::ops::Deref for R {
    type Target = crate::R<STATUS_SPEC>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
impl From<crate::R<STATUS_SPEC>> for R {
    #[inline(always)]
    fn from(reader: crate::R<STATUS_SPEC>) -> Self {
        R(reader)
    }
}
#[doc = "Field `VMONRDY` reader - VMON Ready"]
pub type VMONRDY_R = crate::BitReader<bool>;
#[doc = "Field `VMONAVDD` reader - VMON AVDD Channel"]
pub type VMONAVDD_R = crate::BitReader<bool>;
#[doc = "Field `VMONALTAVDD` reader - Alternate VMON AVDD Channel"]
pub type VMONALTAVDD_R = crate::BitReader<bool>;
#[doc = "Field `VMONDVDD` reader - VMON DVDD Channel"]
pub type VMONDVDD_R = crate::BitReader<bool>;
#[doc = "Field `VMONIO0` reader - VMON IOVDD0 Channel"]
pub type VMONIO0_R = crate::BitReader<bool>;
#[doc = "Field `VMONBUVDD` reader - VMON BUVDD Channel"]
pub type VMONBUVDD_R = crate::BitReader<bool>;
#[doc = "Field `VMONFVDD` reader - VMON VDDFLASH Channel"]
pub type VMONFVDD_R = crate::BitReader<bool>;
#[doc = "Field `BURDY` reader - Backup Mode Ready"]
pub type BURDY_R = crate::BitReader<bool>;
#[doc = "Field `VSCALE` reader - Current Voltage Scale Value"]
pub type VSCALE_R = crate::FieldReader<u8, VSCALE_A>;
#[doc = "Current Voltage Scale Value\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
#[repr(u8)]
pub enum VSCALE_A {
    #[doc = "0: Voltage Scale Level 2"]
    VSCALE2 = 0,
    #[doc = "2: Voltage Scale Level 0"]
    VSCALE0 = 2,
    #[doc = "3: RESV"]
    RESV = 3,
}
impl From<VSCALE_A> for u8 {
    #[inline(always)]
    fn from(variant: VSCALE_A) -> Self {
        variant as _
    }
}
impl VSCALE_R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> Option<VSCALE_A> {
        match self.bits {
            0 => Some(VSCALE_A::VSCALE2),
            2 => Some(VSCALE_A::VSCALE0),
            3 => Some(VSCALE_A::RESV),
            _ => None,
        }
    }
    #[doc = "Checks if the value of the field is `VSCALE2`"]
    #[inline(always)]
    pub fn is_vscale2(&self) -> bool {
        *self == VSCALE_A::VSCALE2
    }
    #[doc = "Checks if the value of the field is `VSCALE0`"]
    #[inline(always)]
    pub fn is_vscale0(&self) -> bool {
        *self == VSCALE_A::VSCALE0
    }
    #[doc = "Checks if the value of the field is `RESV`"]
    #[inline(always)]
    pub fn is_resv(&self) -> bool {
        *self == VSCALE_A::RESV
    }
}
#[doc = "Field `VSCALEBUSY` reader - System is Busy Scaling Voltage"]
pub type VSCALEBUSY_R = crate::BitReader<bool>;
#[doc = "Field `EM4IORET` reader - IO Retention Status"]
pub type EM4IORET_R = crate::BitReader<bool>;
#[doc = "Field `TEMPACTIVE` reader - Temperature Measurement Active"]
pub type TEMPACTIVE_R = crate::BitReader<bool>;
impl R {
    #[doc = "Bit 0 - VMON Ready"]
    #[inline(always)]
    pub fn vmonrdy(&self) -> VMONRDY_R {
        VMONRDY_R::new((self.bits & 1) != 0)
    }
    #[doc = "Bit 1 - VMON AVDD Channel"]
    #[inline(always)]
    pub fn vmonavdd(&self) -> VMONAVDD_R {
        VMONAVDD_R::new(((self.bits >> 1) & 1) != 0)
    }
    #[doc = "Bit 2 - Alternate VMON AVDD Channel"]
    #[inline(always)]
    pub fn vmonaltavdd(&self) -> VMONALTAVDD_R {
        VMONALTAVDD_R::new(((self.bits >> 2) & 1) != 0)
    }
    #[doc = "Bit 3 - VMON DVDD Channel"]
    #[inline(always)]
    pub fn vmondvdd(&self) -> VMONDVDD_R {
        VMONDVDD_R::new(((self.bits >> 3) & 1) != 0)
    }
    #[doc = "Bit 4 - VMON IOVDD0 Channel"]
    #[inline(always)]
    pub fn vmonio0(&self) -> VMONIO0_R {
        VMONIO0_R::new(((self.bits >> 4) & 1) != 0)
    }
    #[doc = "Bit 7 - VMON BUVDD Channel"]
    #[inline(always)]
    pub fn vmonbuvdd(&self) -> VMONBUVDD_R {
        VMONBUVDD_R::new(((self.bits >> 7) & 1) != 0)
    }
    #[doc = "Bit 8 - VMON VDDFLASH Channel"]
    #[inline(always)]
    pub fn vmonfvdd(&self) -> VMONFVDD_R {
        VMONFVDD_R::new(((self.bits >> 8) & 1) != 0)
    }
    #[doc = "Bit 12 - Backup Mode Ready"]
    #[inline(always)]
    pub fn burdy(&self) -> BURDY_R {
        BURDY_R::new(((self.bits >> 12) & 1) != 0)
    }
    #[doc = "Bits 16:17 - Current Voltage Scale Value"]
    #[inline(always)]
    pub fn vscale(&self) -> VSCALE_R {
        VSCALE_R::new(((self.bits >> 16) & 3) as u8)
    }
    #[doc = "Bit 18 - System is Busy Scaling Voltage"]
    #[inline(always)]
    pub fn vscalebusy(&self) -> VSCALEBUSY_R {
        VSCALEBUSY_R::new(((self.bits >> 18) & 1) != 0)
    }
    #[doc = "Bit 20 - IO Retention Status"]
    #[inline(always)]
    pub fn em4ioret(&self) -> EM4IORET_R {
        EM4IORET_R::new(((self.bits >> 20) & 1) != 0)
    }
    #[doc = "Bit 26 - Temperature Measurement Active"]
    #[inline(always)]
    pub fn tempactive(&self) -> TEMPACTIVE_R {
        TEMPACTIVE_R::new(((self.bits >> 26) & 1) != 0)
    }
}
#[doc = "Status Register\n\nThis register you can [`read`](crate::generic::Reg::read). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [status](index.html) module"]
pub struct STATUS_SPEC;
impl crate::RegisterSpec for STATUS_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [status::R](R) reader structure"]
impl crate::Readable for STATUS_SPEC {
    type Reader = R;
}
#[doc = "`reset()` method sets STATUS to value 0"]
impl crate::Resettable for STATUS_SPEC {
    const RESET_VALUE: Self::Ux = 0;
}
