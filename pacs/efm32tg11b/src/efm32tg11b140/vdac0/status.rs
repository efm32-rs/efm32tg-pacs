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
#[doc = "Field `CH0ENS` reader - Channel 0 Enabled Status"]
pub type CH0ENS_R = crate::BitReader<bool>;
#[doc = "Field `CH1ENS` reader - Channel 1 Enabled Status"]
pub type CH1ENS_R = crate::BitReader<bool>;
#[doc = "Field `CH0BL` reader - Channel 0 Buffer Level"]
pub type CH0BL_R = crate::BitReader<bool>;
#[doc = "Field `CH1BL` reader - Channel 1 Buffer Level"]
pub type CH1BL_R = crate::BitReader<bool>;
#[doc = "Field `CH0WARM` reader - Channel 0 Warm"]
pub type CH0WARM_R = crate::BitReader<bool>;
#[doc = "Field `CH1WARM` reader - Channel 1 Warm"]
pub type CH1WARM_R = crate::BitReader<bool>;
#[doc = "Field `OPA0APORTCONFLICT` reader - OPA0 Bus Conflict Output"]
pub type OPA0APORTCONFLICT_R = crate::BitReader<bool>;
#[doc = "Field `OPA1APORTCONFLICT` reader - OPA1 Bus Conflict Output"]
pub type OPA1APORTCONFLICT_R = crate::BitReader<bool>;
#[doc = "Field `OPA2APORTCONFLICT` reader - OPA2 Bus Conflict Output"]
pub type OPA2APORTCONFLICT_R = crate::BitReader<bool>;
#[doc = "Field `OPA3APORTCONFLICT` reader - OPA3 Bus Conflict Output"]
pub type OPA3APORTCONFLICT_R = crate::BitReader<bool>;
#[doc = "Field `OPA0ENS` reader - OPA0 Enabled Status"]
pub type OPA0ENS_R = crate::BitReader<bool>;
#[doc = "Field `OPA1ENS` reader - OPA1 Enabled Status"]
pub type OPA1ENS_R = crate::BitReader<bool>;
#[doc = "Field `OPA2ENS` reader - OPA2 Enabled Status"]
pub type OPA2ENS_R = crate::BitReader<bool>;
#[doc = "Field `OPA3ENS` reader - OPA3 Enabled Status"]
pub type OPA3ENS_R = crate::BitReader<bool>;
#[doc = "Field `OPA0WARM` reader - OPA0 Warm Status"]
pub type OPA0WARM_R = crate::BitReader<bool>;
#[doc = "Field `OPA1WARM` reader - OPA1 Warm Status"]
pub type OPA1WARM_R = crate::BitReader<bool>;
#[doc = "Field `OPA2WARM` reader - OPA2 Warm Status"]
pub type OPA2WARM_R = crate::BitReader<bool>;
#[doc = "Field `OPA3WARM` reader - OPA3 Warm Status"]
pub type OPA3WARM_R = crate::BitReader<bool>;
#[doc = "Field `OPA0OUTVALID` reader - OPA0 Output Valid Status"]
pub type OPA0OUTVALID_R = crate::BitReader<bool>;
#[doc = "Field `OPA1OUTVALID` reader - OPA1 Output Valid Status"]
pub type OPA1OUTVALID_R = crate::BitReader<bool>;
#[doc = "Field `OPA2OUTVALID` reader - OPA2 Output Valid Status"]
pub type OPA2OUTVALID_R = crate::BitReader<bool>;
#[doc = "Field `OPA3OUTVALID` reader - OPA3 Output Valid Status"]
pub type OPA3OUTVALID_R = crate::BitReader<bool>;
impl R {
    #[doc = "Bit 0 - Channel 0 Enabled Status"]
    #[inline(always)]
    pub fn ch0ens(&self) -> CH0ENS_R {
        CH0ENS_R::new((self.bits & 1) != 0)
    }
    #[doc = "Bit 1 - Channel 1 Enabled Status"]
    #[inline(always)]
    pub fn ch1ens(&self) -> CH1ENS_R {
        CH1ENS_R::new(((self.bits >> 1) & 1) != 0)
    }
    #[doc = "Bit 2 - Channel 0 Buffer Level"]
    #[inline(always)]
    pub fn ch0bl(&self) -> CH0BL_R {
        CH0BL_R::new(((self.bits >> 2) & 1) != 0)
    }
    #[doc = "Bit 3 - Channel 1 Buffer Level"]
    #[inline(always)]
    pub fn ch1bl(&self) -> CH1BL_R {
        CH1BL_R::new(((self.bits >> 3) & 1) != 0)
    }
    #[doc = "Bit 4 - Channel 0 Warm"]
    #[inline(always)]
    pub fn ch0warm(&self) -> CH0WARM_R {
        CH0WARM_R::new(((self.bits >> 4) & 1) != 0)
    }
    #[doc = "Bit 5 - Channel 1 Warm"]
    #[inline(always)]
    pub fn ch1warm(&self) -> CH1WARM_R {
        CH1WARM_R::new(((self.bits >> 5) & 1) != 0)
    }
    #[doc = "Bit 16 - OPA0 Bus Conflict Output"]
    #[inline(always)]
    pub fn opa0aportconflict(&self) -> OPA0APORTCONFLICT_R {
        OPA0APORTCONFLICT_R::new(((self.bits >> 16) & 1) != 0)
    }
    #[doc = "Bit 17 - OPA1 Bus Conflict Output"]
    #[inline(always)]
    pub fn opa1aportconflict(&self) -> OPA1APORTCONFLICT_R {
        OPA1APORTCONFLICT_R::new(((self.bits >> 17) & 1) != 0)
    }
    #[doc = "Bit 18 - OPA2 Bus Conflict Output"]
    #[inline(always)]
    pub fn opa2aportconflict(&self) -> OPA2APORTCONFLICT_R {
        OPA2APORTCONFLICT_R::new(((self.bits >> 18) & 1) != 0)
    }
    #[doc = "Bit 19 - OPA3 Bus Conflict Output"]
    #[inline(always)]
    pub fn opa3aportconflict(&self) -> OPA3APORTCONFLICT_R {
        OPA3APORTCONFLICT_R::new(((self.bits >> 19) & 1) != 0)
    }
    #[doc = "Bit 20 - OPA0 Enabled Status"]
    #[inline(always)]
    pub fn opa0ens(&self) -> OPA0ENS_R {
        OPA0ENS_R::new(((self.bits >> 20) & 1) != 0)
    }
    #[doc = "Bit 21 - OPA1 Enabled Status"]
    #[inline(always)]
    pub fn opa1ens(&self) -> OPA1ENS_R {
        OPA1ENS_R::new(((self.bits >> 21) & 1) != 0)
    }
    #[doc = "Bit 22 - OPA2 Enabled Status"]
    #[inline(always)]
    pub fn opa2ens(&self) -> OPA2ENS_R {
        OPA2ENS_R::new(((self.bits >> 22) & 1) != 0)
    }
    #[doc = "Bit 23 - OPA3 Enabled Status"]
    #[inline(always)]
    pub fn opa3ens(&self) -> OPA3ENS_R {
        OPA3ENS_R::new(((self.bits >> 23) & 1) != 0)
    }
    #[doc = "Bit 24 - OPA0 Warm Status"]
    #[inline(always)]
    pub fn opa0warm(&self) -> OPA0WARM_R {
        OPA0WARM_R::new(((self.bits >> 24) & 1) != 0)
    }
    #[doc = "Bit 25 - OPA1 Warm Status"]
    #[inline(always)]
    pub fn opa1warm(&self) -> OPA1WARM_R {
        OPA1WARM_R::new(((self.bits >> 25) & 1) != 0)
    }
    #[doc = "Bit 26 - OPA2 Warm Status"]
    #[inline(always)]
    pub fn opa2warm(&self) -> OPA2WARM_R {
        OPA2WARM_R::new(((self.bits >> 26) & 1) != 0)
    }
    #[doc = "Bit 27 - OPA3 Warm Status"]
    #[inline(always)]
    pub fn opa3warm(&self) -> OPA3WARM_R {
        OPA3WARM_R::new(((self.bits >> 27) & 1) != 0)
    }
    #[doc = "Bit 28 - OPA0 Output Valid Status"]
    #[inline(always)]
    pub fn opa0outvalid(&self) -> OPA0OUTVALID_R {
        OPA0OUTVALID_R::new(((self.bits >> 28) & 1) != 0)
    }
    #[doc = "Bit 29 - OPA1 Output Valid Status"]
    #[inline(always)]
    pub fn opa1outvalid(&self) -> OPA1OUTVALID_R {
        OPA1OUTVALID_R::new(((self.bits >> 29) & 1) != 0)
    }
    #[doc = "Bit 30 - OPA2 Output Valid Status"]
    #[inline(always)]
    pub fn opa2outvalid(&self) -> OPA2OUTVALID_R {
        OPA2OUTVALID_R::new(((self.bits >> 30) & 1) != 0)
    }
    #[doc = "Bit 31 - OPA3 Output Valid Status"]
    #[inline(always)]
    pub fn opa3outvalid(&self) -> OPA3OUTVALID_R {
        OPA3OUTVALID_R::new(((self.bits >> 31) & 1) != 0)
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
#[doc = "`reset()` method sets STATUS to value 0x0c"]
impl crate::Resettable for STATUS_SPEC {
    const RESET_VALUE: Self::Ux = 0x0c;
}
