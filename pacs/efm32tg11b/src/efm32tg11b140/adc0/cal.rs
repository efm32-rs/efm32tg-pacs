#[doc = "Register `CAL` reader"]
pub struct R(crate::R<CAL_SPEC>);
impl core::ops::Deref for R {
    type Target = crate::R<CAL_SPEC>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
impl From<crate::R<CAL_SPEC>> for R {
    #[inline(always)]
    fn from(reader: crate::R<CAL_SPEC>) -> Self {
        R(reader)
    }
}
#[doc = "Register `CAL` writer"]
pub struct W(crate::W<CAL_SPEC>);
impl core::ops::Deref for W {
    type Target = crate::W<CAL_SPEC>;
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
impl From<crate::W<CAL_SPEC>> for W {
    #[inline(always)]
    fn from(writer: crate::W<CAL_SPEC>) -> Self {
        W(writer)
    }
}
#[doc = "Field `SINGLEOFFSET` reader - Single Mode Offset Calibration Value for Differential or Positive Single-ended Mode"]
pub type SINGLEOFFSET_R = crate::FieldReader<u8, u8>;
#[doc = "Field `SINGLEOFFSET` writer - Single Mode Offset Calibration Value for Differential or Positive Single-ended Mode"]
pub type SINGLEOFFSET_W<'a, const O: u8> = crate::FieldWriter<'a, u32, CAL_SPEC, u8, u8, 4, O>;
#[doc = "Field `SINGLEOFFSETINV` reader - Single Mode Offset Calibration Value for Negative Single-ended Mode"]
pub type SINGLEOFFSETINV_R = crate::FieldReader<u8, u8>;
#[doc = "Field `SINGLEOFFSETINV` writer - Single Mode Offset Calibration Value for Negative Single-ended Mode"]
pub type SINGLEOFFSETINV_W<'a, const O: u8> = crate::FieldWriter<'a, u32, CAL_SPEC, u8, u8, 4, O>;
#[doc = "Field `SINGLEGAIN` reader - Single Mode Gain Calibration Value"]
pub type SINGLEGAIN_R = crate::FieldReader<u8, u8>;
#[doc = "Field `SINGLEGAIN` writer - Single Mode Gain Calibration Value"]
pub type SINGLEGAIN_W<'a, const O: u8> = crate::FieldWriter<'a, u32, CAL_SPEC, u8, u8, 7, O>;
#[doc = "Field `OFFSETINVMODE` reader - Negative Single-ended Offset Calibration is Enabled"]
pub type OFFSETINVMODE_R = crate::BitReader<bool>;
#[doc = "Field `OFFSETINVMODE` writer - Negative Single-ended Offset Calibration is Enabled"]
pub type OFFSETINVMODE_W<'a, const O: u8> = crate::BitWriter<'a, u32, CAL_SPEC, bool, O>;
#[doc = "Field `SCANOFFSET` reader - Scan Mode Offset Calibration Value for Differential or Positive Single-ended Mode"]
pub type SCANOFFSET_R = crate::FieldReader<u8, u8>;
#[doc = "Field `SCANOFFSET` writer - Scan Mode Offset Calibration Value for Differential or Positive Single-ended Mode"]
pub type SCANOFFSET_W<'a, const O: u8> = crate::FieldWriter<'a, u32, CAL_SPEC, u8, u8, 4, O>;
#[doc = "Field `SCANOFFSETINV` reader - Scan Mode Offset Calibration Value for Negative Single-ended Mode"]
pub type SCANOFFSETINV_R = crate::FieldReader<u8, u8>;
#[doc = "Field `SCANOFFSETINV` writer - Scan Mode Offset Calibration Value for Negative Single-ended Mode"]
pub type SCANOFFSETINV_W<'a, const O: u8> = crate::FieldWriter<'a, u32, CAL_SPEC, u8, u8, 4, O>;
#[doc = "Field `SCANGAIN` reader - Scan Mode Gain Calibration Value"]
pub type SCANGAIN_R = crate::FieldReader<u8, u8>;
#[doc = "Field `SCANGAIN` writer - Scan Mode Gain Calibration Value"]
pub type SCANGAIN_W<'a, const O: u8> = crate::FieldWriter<'a, u32, CAL_SPEC, u8, u8, 7, O>;
#[doc = "Field `CALEN` reader - Calibration Mode is Enabled"]
pub type CALEN_R = crate::BitReader<bool>;
#[doc = "Field `CALEN` writer - Calibration Mode is Enabled"]
pub type CALEN_W<'a, const O: u8> = crate::BitWriter<'a, u32, CAL_SPEC, bool, O>;
impl R {
    #[doc = "Bits 0:3 - Single Mode Offset Calibration Value for Differential or Positive Single-ended Mode"]
    #[inline(always)]
    pub fn singleoffset(&self) -> SINGLEOFFSET_R {
        SINGLEOFFSET_R::new((self.bits & 0x0f) as u8)
    }
    #[doc = "Bits 4:7 - Single Mode Offset Calibration Value for Negative Single-ended Mode"]
    #[inline(always)]
    pub fn singleoffsetinv(&self) -> SINGLEOFFSETINV_R {
        SINGLEOFFSETINV_R::new(((self.bits >> 4) & 0x0f) as u8)
    }
    #[doc = "Bits 8:14 - Single Mode Gain Calibration Value"]
    #[inline(always)]
    pub fn singlegain(&self) -> SINGLEGAIN_R {
        SINGLEGAIN_R::new(((self.bits >> 8) & 0x7f) as u8)
    }
    #[doc = "Bit 15 - Negative Single-ended Offset Calibration is Enabled"]
    #[inline(always)]
    pub fn offsetinvmode(&self) -> OFFSETINVMODE_R {
        OFFSETINVMODE_R::new(((self.bits >> 15) & 1) != 0)
    }
    #[doc = "Bits 16:19 - Scan Mode Offset Calibration Value for Differential or Positive Single-ended Mode"]
    #[inline(always)]
    pub fn scanoffset(&self) -> SCANOFFSET_R {
        SCANOFFSET_R::new(((self.bits >> 16) & 0x0f) as u8)
    }
    #[doc = "Bits 20:23 - Scan Mode Offset Calibration Value for Negative Single-ended Mode"]
    #[inline(always)]
    pub fn scanoffsetinv(&self) -> SCANOFFSETINV_R {
        SCANOFFSETINV_R::new(((self.bits >> 20) & 0x0f) as u8)
    }
    #[doc = "Bits 24:30 - Scan Mode Gain Calibration Value"]
    #[inline(always)]
    pub fn scangain(&self) -> SCANGAIN_R {
        SCANGAIN_R::new(((self.bits >> 24) & 0x7f) as u8)
    }
    #[doc = "Bit 31 - Calibration Mode is Enabled"]
    #[inline(always)]
    pub fn calen(&self) -> CALEN_R {
        CALEN_R::new(((self.bits >> 31) & 1) != 0)
    }
}
impl W {
    #[doc = "Bits 0:3 - Single Mode Offset Calibration Value for Differential or Positive Single-ended Mode"]
    #[inline(always)]
    #[must_use]
    pub fn singleoffset(&mut self) -> SINGLEOFFSET_W<0> {
        SINGLEOFFSET_W::new(self)
    }
    #[doc = "Bits 4:7 - Single Mode Offset Calibration Value for Negative Single-ended Mode"]
    #[inline(always)]
    #[must_use]
    pub fn singleoffsetinv(&mut self) -> SINGLEOFFSETINV_W<4> {
        SINGLEOFFSETINV_W::new(self)
    }
    #[doc = "Bits 8:14 - Single Mode Gain Calibration Value"]
    #[inline(always)]
    #[must_use]
    pub fn singlegain(&mut self) -> SINGLEGAIN_W<8> {
        SINGLEGAIN_W::new(self)
    }
    #[doc = "Bit 15 - Negative Single-ended Offset Calibration is Enabled"]
    #[inline(always)]
    #[must_use]
    pub fn offsetinvmode(&mut self) -> OFFSETINVMODE_W<15> {
        OFFSETINVMODE_W::new(self)
    }
    #[doc = "Bits 16:19 - Scan Mode Offset Calibration Value for Differential or Positive Single-ended Mode"]
    #[inline(always)]
    #[must_use]
    pub fn scanoffset(&mut self) -> SCANOFFSET_W<16> {
        SCANOFFSET_W::new(self)
    }
    #[doc = "Bits 20:23 - Scan Mode Offset Calibration Value for Negative Single-ended Mode"]
    #[inline(always)]
    #[must_use]
    pub fn scanoffsetinv(&mut self) -> SCANOFFSETINV_W<20> {
        SCANOFFSETINV_W::new(self)
    }
    #[doc = "Bits 24:30 - Scan Mode Gain Calibration Value"]
    #[inline(always)]
    #[must_use]
    pub fn scangain(&mut self) -> SCANGAIN_W<24> {
        SCANGAIN_W::new(self)
    }
    #[doc = "Bit 31 - Calibration Mode is Enabled"]
    #[inline(always)]
    #[must_use]
    pub fn calen(&mut self) -> CALEN_W<31> {
        CALEN_W::new(self)
    }
    #[doc = "Writes raw bits to the register."]
    #[inline(always)]
    pub unsafe fn bits(&mut self, bits: u32) -> &mut Self {
        self.0.bits(bits);
        self
    }
}
#[doc = "Calibration Register\n\nThis register you can [`read`](crate::generic::Reg::read), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [cal](index.html) module"]
pub struct CAL_SPEC;
impl crate::RegisterSpec for CAL_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [cal::R](R) reader structure"]
impl crate::Readable for CAL_SPEC {
    type Reader = R;
}
#[doc = "`write(|w| ..)` method takes [cal::W](W) writer structure"]
impl crate::Writable for CAL_SPEC {
    type Writer = W;
    const ZERO_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
}
#[doc = "`reset()` method sets CAL to value 0x4078_4078"]
impl crate::Resettable for CAL_SPEC {
    const RESET_VALUE: Self::Ux = 0x4078_4078;
}
