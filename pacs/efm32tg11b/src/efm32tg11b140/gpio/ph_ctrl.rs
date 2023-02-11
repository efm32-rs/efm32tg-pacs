#[doc = "Register `PH_CTRL` reader"]
pub struct R(crate::R<PH_CTRL_SPEC>);
impl core::ops::Deref for R {
    type Target = crate::R<PH_CTRL_SPEC>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
impl From<crate::R<PH_CTRL_SPEC>> for R {
    #[inline(always)]
    fn from(reader: crate::R<PH_CTRL_SPEC>) -> Self {
        R(reader)
    }
}
#[doc = "Register `PH_CTRL` writer"]
pub struct W(crate::W<PH_CTRL_SPEC>);
impl core::ops::Deref for W {
    type Target = crate::W<PH_CTRL_SPEC>;
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
impl From<crate::W<PH_CTRL_SPEC>> for W {
    #[inline(always)]
    fn from(writer: crate::W<PH_CTRL_SPEC>) -> Self {
        W(writer)
    }
}
#[doc = "Field `DRIVESTRENGTH` reader - Drive Strength for Port"]
pub type DRIVESTRENGTH_R = crate::BitReader<bool>;
#[doc = "Field `DRIVESTRENGTH` writer - Drive Strength for Port"]
pub type DRIVESTRENGTH_W<'a, const O: u8> = crate::BitWriter<'a, u32, PH_CTRL_SPEC, bool, O>;
#[doc = "Field `SLEWRATE` reader - Slewrate Limit for Port"]
pub type SLEWRATE_R = crate::FieldReader<u8, u8>;
#[doc = "Field `SLEWRATE` writer - Slewrate Limit for Port"]
pub type SLEWRATE_W<'a, const O: u8> = crate::FieldWriter<'a, u32, PH_CTRL_SPEC, u8, u8, 3, O>;
#[doc = "Field `DINDIS` reader - Data in Disable"]
pub type DINDIS_R = crate::BitReader<bool>;
#[doc = "Field `DINDIS` writer - Data in Disable"]
pub type DINDIS_W<'a, const O: u8> = crate::BitWriter<'a, u32, PH_CTRL_SPEC, bool, O>;
#[doc = "Field `DRIVESTRENGTHALT` reader - Alternate Drive Strength for Port"]
pub type DRIVESTRENGTHALT_R = crate::BitReader<bool>;
#[doc = "Field `DRIVESTRENGTHALT` writer - Alternate Drive Strength for Port"]
pub type DRIVESTRENGTHALT_W<'a, const O: u8> = crate::BitWriter<'a, u32, PH_CTRL_SPEC, bool, O>;
#[doc = "Field `SLEWRATEALT` reader - Alternate Slewrate Limit for Port"]
pub type SLEWRATEALT_R = crate::FieldReader<u8, u8>;
#[doc = "Field `SLEWRATEALT` writer - Alternate Slewrate Limit for Port"]
pub type SLEWRATEALT_W<'a, const O: u8> = crate::FieldWriter<'a, u32, PH_CTRL_SPEC, u8, u8, 3, O>;
#[doc = "Field `DINDISALT` reader - Alternate Data in Disable"]
pub type DINDISALT_R = crate::BitReader<bool>;
#[doc = "Field `DINDISALT` writer - Alternate Data in Disable"]
pub type DINDISALT_W<'a, const O: u8> = crate::BitWriter<'a, u32, PH_CTRL_SPEC, bool, O>;
impl R {
    #[doc = "Bit 0 - Drive Strength for Port"]
    #[inline(always)]
    pub fn drivestrength(&self) -> DRIVESTRENGTH_R {
        DRIVESTRENGTH_R::new((self.bits & 1) != 0)
    }
    #[doc = "Bits 4:6 - Slewrate Limit for Port"]
    #[inline(always)]
    pub fn slewrate(&self) -> SLEWRATE_R {
        SLEWRATE_R::new(((self.bits >> 4) & 7) as u8)
    }
    #[doc = "Bit 12 - Data in Disable"]
    #[inline(always)]
    pub fn dindis(&self) -> DINDIS_R {
        DINDIS_R::new(((self.bits >> 12) & 1) != 0)
    }
    #[doc = "Bit 16 - Alternate Drive Strength for Port"]
    #[inline(always)]
    pub fn drivestrengthalt(&self) -> DRIVESTRENGTHALT_R {
        DRIVESTRENGTHALT_R::new(((self.bits >> 16) & 1) != 0)
    }
    #[doc = "Bits 20:22 - Alternate Slewrate Limit for Port"]
    #[inline(always)]
    pub fn slewratealt(&self) -> SLEWRATEALT_R {
        SLEWRATEALT_R::new(((self.bits >> 20) & 7) as u8)
    }
    #[doc = "Bit 28 - Alternate Data in Disable"]
    #[inline(always)]
    pub fn dindisalt(&self) -> DINDISALT_R {
        DINDISALT_R::new(((self.bits >> 28) & 1) != 0)
    }
}
impl W {
    #[doc = "Bit 0 - Drive Strength for Port"]
    #[inline(always)]
    #[must_use]
    pub fn drivestrength(&mut self) -> DRIVESTRENGTH_W<0> {
        DRIVESTRENGTH_W::new(self)
    }
    #[doc = "Bits 4:6 - Slewrate Limit for Port"]
    #[inline(always)]
    #[must_use]
    pub fn slewrate(&mut self) -> SLEWRATE_W<4> {
        SLEWRATE_W::new(self)
    }
    #[doc = "Bit 12 - Data in Disable"]
    #[inline(always)]
    #[must_use]
    pub fn dindis(&mut self) -> DINDIS_W<12> {
        DINDIS_W::new(self)
    }
    #[doc = "Bit 16 - Alternate Drive Strength for Port"]
    #[inline(always)]
    #[must_use]
    pub fn drivestrengthalt(&mut self) -> DRIVESTRENGTHALT_W<16> {
        DRIVESTRENGTHALT_W::new(self)
    }
    #[doc = "Bits 20:22 - Alternate Slewrate Limit for Port"]
    #[inline(always)]
    #[must_use]
    pub fn slewratealt(&mut self) -> SLEWRATEALT_W<20> {
        SLEWRATEALT_W::new(self)
    }
    #[doc = "Bit 28 - Alternate Data in Disable"]
    #[inline(always)]
    #[must_use]
    pub fn dindisalt(&mut self) -> DINDISALT_W<28> {
        DINDISALT_W::new(self)
    }
    #[doc = "Writes raw bits to the register."]
    #[inline(always)]
    pub unsafe fn bits(&mut self, bits: u32) -> &mut Self {
        self.0.bits(bits);
        self
    }
}
#[doc = "Port Control Register\n\nThis register you can [`read`](crate::generic::Reg::read), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [ph_ctrl](index.html) module"]
pub struct PH_CTRL_SPEC;
impl crate::RegisterSpec for PH_CTRL_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [ph_ctrl::R](R) reader structure"]
impl crate::Readable for PH_CTRL_SPEC {
    type Reader = R;
}
#[doc = "`write(|w| ..)` method takes [ph_ctrl::W](W) writer structure"]
impl crate::Writable for PH_CTRL_SPEC {
    type Writer = W;
    const ZERO_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
}
#[doc = "`reset()` method sets PH_CTRL to value 0x0050_0050"]
impl crate::Resettable for PH_CTRL_SPEC {
    const RESET_VALUE: Self::Ux = 0x0050_0050;
}
