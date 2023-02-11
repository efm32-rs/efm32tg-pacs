#[doc = "Register `IF` reader"]
pub struct R(crate::R<IF_SPEC>);
impl core::ops::Deref for R {
    type Target = crate::R<IF_SPEC>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
impl From<crate::R<IF_SPEC>> for R {
    #[inline(always)]
    fn from(reader: crate::R<IF_SPEC>) -> Self {
        R(reader)
    }
}
#[doc = "Field `CH0CD` reader - Channel 0 Conversion Done Interrupt Flag"]
pub type CH0CD_R = crate::BitReader<bool>;
#[doc = "Field `CH1CD` reader - Channel 1 Conversion Done Interrupt Flag"]
pub type CH1CD_R = crate::BitReader<bool>;
#[doc = "Field `CH0OF` reader - Channel 0 Data Overflow Interrupt Flag"]
pub type CH0OF_R = crate::BitReader<bool>;
#[doc = "Field `CH1OF` reader - Channel 1 Data Overflow Interrupt Flag"]
pub type CH1OF_R = crate::BitReader<bool>;
#[doc = "Field `CH0UF` reader - Channel 0 Data Underflow Interrupt Flag"]
pub type CH0UF_R = crate::BitReader<bool>;
#[doc = "Field `CH1UF` reader - Channel 1 Data Underflow Interrupt Flag"]
pub type CH1UF_R = crate::BitReader<bool>;
#[doc = "Field `CH0BL` reader - Channel 0 Buffer Level Interrupt Flag"]
pub type CH0BL_R = crate::BitReader<bool>;
#[doc = "Field `CH1BL` reader - Channel 1 Buffer Level Interrupt Flag"]
pub type CH1BL_R = crate::BitReader<bool>;
#[doc = "Field `EM23ERR` reader - EM2/3 Entry Error Flag"]
pub type EM23ERR_R = crate::BitReader<bool>;
#[doc = "Field `OPA0APORTCONFLICT` reader - OPA0 Bus Conflict Output Interrupt Flag"]
pub type OPA0APORTCONFLICT_R = crate::BitReader<bool>;
#[doc = "Field `OPA1APORTCONFLICT` reader - OPA1 Bus Conflict Output Interrupt Flag"]
pub type OPA1APORTCONFLICT_R = crate::BitReader<bool>;
#[doc = "Field `OPA2APORTCONFLICT` reader - OPA2 Bus Conflict Output Interrupt Flag"]
pub type OPA2APORTCONFLICT_R = crate::BitReader<bool>;
#[doc = "Field `OPA3APORTCONFLICT` reader - OPA3 Bus Conflict Output Interrupt Flag"]
pub type OPA3APORTCONFLICT_R = crate::BitReader<bool>;
#[doc = "Field `OPA0PRSTIMEDERR` reader - OPA0 PRS Trigger Mode Error Interrupt Flag"]
pub type OPA0PRSTIMEDERR_R = crate::BitReader<bool>;
#[doc = "Field `OPA1PRSTIMEDERR` reader - OPA1 PRS Trigger Mode Error Interrupt Flag"]
pub type OPA1PRSTIMEDERR_R = crate::BitReader<bool>;
#[doc = "Field `OPA2PRSTIMEDERR` reader - OPA2 PRS Trigger Mode Error Interrupt Flag"]
pub type OPA2PRSTIMEDERR_R = crate::BitReader<bool>;
#[doc = "Field `OPA3PRSTIMEDERR` reader - OPA3 PRS Trigger Mode Error Interrupt Flag"]
pub type OPA3PRSTIMEDERR_R = crate::BitReader<bool>;
#[doc = "Field `OPA0OUTVALID` reader - OPA0 Output Valid Interrupt Flag"]
pub type OPA0OUTVALID_R = crate::BitReader<bool>;
#[doc = "Field `OPA1OUTVALID` reader - OPA1 Output Valid Interrupt Flag"]
pub type OPA1OUTVALID_R = crate::BitReader<bool>;
#[doc = "Field `OPA2OUTVALID` reader - OPA3 Output Valid Interrupt Flag"]
pub type OPA2OUTVALID_R = crate::BitReader<bool>;
#[doc = "Field `OPA3OUTVALID` reader - OPA3 Output Valid Interrupt Flag"]
pub type OPA3OUTVALID_R = crate::BitReader<bool>;
impl R {
    #[doc = "Bit 0 - Channel 0 Conversion Done Interrupt Flag"]
    #[inline(always)]
    pub fn ch0cd(&self) -> CH0CD_R {
        CH0CD_R::new((self.bits & 1) != 0)
    }
    #[doc = "Bit 1 - Channel 1 Conversion Done Interrupt Flag"]
    #[inline(always)]
    pub fn ch1cd(&self) -> CH1CD_R {
        CH1CD_R::new(((self.bits >> 1) & 1) != 0)
    }
    #[doc = "Bit 2 - Channel 0 Data Overflow Interrupt Flag"]
    #[inline(always)]
    pub fn ch0of(&self) -> CH0OF_R {
        CH0OF_R::new(((self.bits >> 2) & 1) != 0)
    }
    #[doc = "Bit 3 - Channel 1 Data Overflow Interrupt Flag"]
    #[inline(always)]
    pub fn ch1of(&self) -> CH1OF_R {
        CH1OF_R::new(((self.bits >> 3) & 1) != 0)
    }
    #[doc = "Bit 4 - Channel 0 Data Underflow Interrupt Flag"]
    #[inline(always)]
    pub fn ch0uf(&self) -> CH0UF_R {
        CH0UF_R::new(((self.bits >> 4) & 1) != 0)
    }
    #[doc = "Bit 5 - Channel 1 Data Underflow Interrupt Flag"]
    #[inline(always)]
    pub fn ch1uf(&self) -> CH1UF_R {
        CH1UF_R::new(((self.bits >> 5) & 1) != 0)
    }
    #[doc = "Bit 6 - Channel 0 Buffer Level Interrupt Flag"]
    #[inline(always)]
    pub fn ch0bl(&self) -> CH0BL_R {
        CH0BL_R::new(((self.bits >> 6) & 1) != 0)
    }
    #[doc = "Bit 7 - Channel 1 Buffer Level Interrupt Flag"]
    #[inline(always)]
    pub fn ch1bl(&self) -> CH1BL_R {
        CH1BL_R::new(((self.bits >> 7) & 1) != 0)
    }
    #[doc = "Bit 15 - EM2/3 Entry Error Flag"]
    #[inline(always)]
    pub fn em23err(&self) -> EM23ERR_R {
        EM23ERR_R::new(((self.bits >> 15) & 1) != 0)
    }
    #[doc = "Bit 16 - OPA0 Bus Conflict Output Interrupt Flag"]
    #[inline(always)]
    pub fn opa0aportconflict(&self) -> OPA0APORTCONFLICT_R {
        OPA0APORTCONFLICT_R::new(((self.bits >> 16) & 1) != 0)
    }
    #[doc = "Bit 17 - OPA1 Bus Conflict Output Interrupt Flag"]
    #[inline(always)]
    pub fn opa1aportconflict(&self) -> OPA1APORTCONFLICT_R {
        OPA1APORTCONFLICT_R::new(((self.bits >> 17) & 1) != 0)
    }
    #[doc = "Bit 18 - OPA2 Bus Conflict Output Interrupt Flag"]
    #[inline(always)]
    pub fn opa2aportconflict(&self) -> OPA2APORTCONFLICT_R {
        OPA2APORTCONFLICT_R::new(((self.bits >> 18) & 1) != 0)
    }
    #[doc = "Bit 19 - OPA3 Bus Conflict Output Interrupt Flag"]
    #[inline(always)]
    pub fn opa3aportconflict(&self) -> OPA3APORTCONFLICT_R {
        OPA3APORTCONFLICT_R::new(((self.bits >> 19) & 1) != 0)
    }
    #[doc = "Bit 20 - OPA0 PRS Trigger Mode Error Interrupt Flag"]
    #[inline(always)]
    pub fn opa0prstimederr(&self) -> OPA0PRSTIMEDERR_R {
        OPA0PRSTIMEDERR_R::new(((self.bits >> 20) & 1) != 0)
    }
    #[doc = "Bit 21 - OPA1 PRS Trigger Mode Error Interrupt Flag"]
    #[inline(always)]
    pub fn opa1prstimederr(&self) -> OPA1PRSTIMEDERR_R {
        OPA1PRSTIMEDERR_R::new(((self.bits >> 21) & 1) != 0)
    }
    #[doc = "Bit 22 - OPA2 PRS Trigger Mode Error Interrupt Flag"]
    #[inline(always)]
    pub fn opa2prstimederr(&self) -> OPA2PRSTIMEDERR_R {
        OPA2PRSTIMEDERR_R::new(((self.bits >> 22) & 1) != 0)
    }
    #[doc = "Bit 23 - OPA3 PRS Trigger Mode Error Interrupt Flag"]
    #[inline(always)]
    pub fn opa3prstimederr(&self) -> OPA3PRSTIMEDERR_R {
        OPA3PRSTIMEDERR_R::new(((self.bits >> 23) & 1) != 0)
    }
    #[doc = "Bit 28 - OPA0 Output Valid Interrupt Flag"]
    #[inline(always)]
    pub fn opa0outvalid(&self) -> OPA0OUTVALID_R {
        OPA0OUTVALID_R::new(((self.bits >> 28) & 1) != 0)
    }
    #[doc = "Bit 29 - OPA1 Output Valid Interrupt Flag"]
    #[inline(always)]
    pub fn opa1outvalid(&self) -> OPA1OUTVALID_R {
        OPA1OUTVALID_R::new(((self.bits >> 29) & 1) != 0)
    }
    #[doc = "Bit 30 - OPA3 Output Valid Interrupt Flag"]
    #[inline(always)]
    pub fn opa2outvalid(&self) -> OPA2OUTVALID_R {
        OPA2OUTVALID_R::new(((self.bits >> 30) & 1) != 0)
    }
    #[doc = "Bit 31 - OPA3 Output Valid Interrupt Flag"]
    #[inline(always)]
    pub fn opa3outvalid(&self) -> OPA3OUTVALID_R {
        OPA3OUTVALID_R::new(((self.bits >> 31) & 1) != 0)
    }
}
#[doc = "Interrupt Flag Register\n\nThis register you can [`read`](crate::generic::Reg::read). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [if_](index.html) module"]
pub struct IF_SPEC;
impl crate::RegisterSpec for IF_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [if_::R](R) reader structure"]
impl crate::Readable for IF_SPEC {
    type Reader = R;
}
#[doc = "`reset()` method sets IF to value 0xc0"]
impl crate::Resettable for IF_SPEC {
    const RESET_VALUE: Self::Ux = 0xc0;
}
