#[doc = "Register `OPA2_CAL` reader"]
pub struct R(crate::R<OPA2_CAL_SPEC>);
impl core::ops::Deref for R {
    type Target = crate::R<OPA2_CAL_SPEC>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
impl From<crate::R<OPA2_CAL_SPEC>> for R {
    #[inline(always)]
    fn from(reader: crate::R<OPA2_CAL_SPEC>) -> Self {
        R(reader)
    }
}
#[doc = "Register `OPA2_CAL` writer"]
pub struct W(crate::W<OPA2_CAL_SPEC>);
impl core::ops::Deref for W {
    type Target = crate::W<OPA2_CAL_SPEC>;
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
impl From<crate::W<OPA2_CAL_SPEC>> for W {
    #[inline(always)]
    fn from(writer: crate::W<OPA2_CAL_SPEC>) -> Self {
        W(writer)
    }
}
#[doc = "Field `CM1` reader - Compensation Cap Cm1 Trim Value"]
pub type CM1_R = crate::FieldReader<u8, u8>;
#[doc = "Field `CM1` writer - Compensation Cap Cm1 Trim Value"]
pub type CM1_W<'a, const O: u8> = crate::FieldWriter<'a, u32, OPA2_CAL_SPEC, u8, u8, 4, O>;
#[doc = "Field `CM2` reader - Compensation Cap Cm2 Trim Value"]
pub type CM2_R = crate::FieldReader<u8, u8>;
#[doc = "Field `CM2` writer - Compensation Cap Cm2 Trim Value"]
pub type CM2_W<'a, const O: u8> = crate::FieldWriter<'a, u32, OPA2_CAL_SPEC, u8, u8, 4, O>;
#[doc = "Field `CM3` reader - Compensation Cap Cm3 Trim Value"]
pub type CM3_R = crate::FieldReader<u8, u8>;
#[doc = "Field `CM3` writer - Compensation Cap Cm3 Trim Value"]
pub type CM3_W<'a, const O: u8> = crate::FieldWriter<'a, u32, OPA2_CAL_SPEC, u8, u8, 2, O>;
#[doc = "Field `GM` reader - Gm Trim Value"]
pub type GM_R = crate::FieldReader<u8, u8>;
#[doc = "Field `GM` writer - Gm Trim Value"]
pub type GM_W<'a, const O: u8> = crate::FieldWriter<'a, u32, OPA2_CAL_SPEC, u8, u8, 3, O>;
#[doc = "Field `GM3` reader - Gm3 Trim Value"]
pub type GM3_R = crate::FieldReader<u8, u8>;
#[doc = "Field `GM3` writer - Gm3 Trim Value"]
pub type GM3_W<'a, const O: u8> = crate::FieldWriter<'a, u32, OPA2_CAL_SPEC, u8, u8, 2, O>;
#[doc = "Field `OFFSETP` reader - OPAx Non-Inverting Input Offset Configuration Value"]
pub type OFFSETP_R = crate::FieldReader<u8, u8>;
#[doc = "Field `OFFSETP` writer - OPAx Non-Inverting Input Offset Configuration Value"]
pub type OFFSETP_W<'a, const O: u8> = crate::FieldWriter<'a, u32, OPA2_CAL_SPEC, u8, u8, 5, O>;
#[doc = "Field `OFFSETN` reader - OPAx Inverting Input Offset Configuration Value"]
pub type OFFSETN_R = crate::FieldReader<u8, u8>;
#[doc = "Field `OFFSETN` writer - OPAx Inverting Input Offset Configuration Value"]
pub type OFFSETN_W<'a, const O: u8> = crate::FieldWriter<'a, u32, OPA2_CAL_SPEC, u8, u8, 5, O>;
impl R {
    #[doc = "Bits 0:3 - Compensation Cap Cm1 Trim Value"]
    #[inline(always)]
    pub fn cm1(&self) -> CM1_R {
        CM1_R::new((self.bits & 0x0f) as u8)
    }
    #[doc = "Bits 5:8 - Compensation Cap Cm2 Trim Value"]
    #[inline(always)]
    pub fn cm2(&self) -> CM2_R {
        CM2_R::new(((self.bits >> 5) & 0x0f) as u8)
    }
    #[doc = "Bits 10:11 - Compensation Cap Cm3 Trim Value"]
    #[inline(always)]
    pub fn cm3(&self) -> CM3_R {
        CM3_R::new(((self.bits >> 10) & 3) as u8)
    }
    #[doc = "Bits 13:15 - Gm Trim Value"]
    #[inline(always)]
    pub fn gm(&self) -> GM_R {
        GM_R::new(((self.bits >> 13) & 7) as u8)
    }
    #[doc = "Bits 17:18 - Gm3 Trim Value"]
    #[inline(always)]
    pub fn gm3(&self) -> GM3_R {
        GM3_R::new(((self.bits >> 17) & 3) as u8)
    }
    #[doc = "Bits 20:24 - OPAx Non-Inverting Input Offset Configuration Value"]
    #[inline(always)]
    pub fn offsetp(&self) -> OFFSETP_R {
        OFFSETP_R::new(((self.bits >> 20) & 0x1f) as u8)
    }
    #[doc = "Bits 26:30 - OPAx Inverting Input Offset Configuration Value"]
    #[inline(always)]
    pub fn offsetn(&self) -> OFFSETN_R {
        OFFSETN_R::new(((self.bits >> 26) & 0x1f) as u8)
    }
}
impl W {
    #[doc = "Bits 0:3 - Compensation Cap Cm1 Trim Value"]
    #[inline(always)]
    #[must_use]
    pub fn cm1(&mut self) -> CM1_W<0> {
        CM1_W::new(self)
    }
    #[doc = "Bits 5:8 - Compensation Cap Cm2 Trim Value"]
    #[inline(always)]
    #[must_use]
    pub fn cm2(&mut self) -> CM2_W<5> {
        CM2_W::new(self)
    }
    #[doc = "Bits 10:11 - Compensation Cap Cm3 Trim Value"]
    #[inline(always)]
    #[must_use]
    pub fn cm3(&mut self) -> CM3_W<10> {
        CM3_W::new(self)
    }
    #[doc = "Bits 13:15 - Gm Trim Value"]
    #[inline(always)]
    #[must_use]
    pub fn gm(&mut self) -> GM_W<13> {
        GM_W::new(self)
    }
    #[doc = "Bits 17:18 - Gm3 Trim Value"]
    #[inline(always)]
    #[must_use]
    pub fn gm3(&mut self) -> GM3_W<17> {
        GM3_W::new(self)
    }
    #[doc = "Bits 20:24 - OPAx Non-Inverting Input Offset Configuration Value"]
    #[inline(always)]
    #[must_use]
    pub fn offsetp(&mut self) -> OFFSETP_W<20> {
        OFFSETP_W::new(self)
    }
    #[doc = "Bits 26:30 - OPAx Inverting Input Offset Configuration Value"]
    #[inline(always)]
    #[must_use]
    pub fn offsetn(&mut self) -> OFFSETN_W<26> {
        OFFSETN_W::new(self)
    }
    #[doc = "Writes raw bits to the register."]
    #[inline(always)]
    pub unsafe fn bits(&mut self, bits: u32) -> &mut Self {
        self.0.bits(bits);
        self
    }
}
#[doc = "Operational Amplifier Calibration Register\n\nThis register you can [`read`](crate::generic::Reg::read), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [opa2_cal](index.html) module"]
pub struct OPA2_CAL_SPEC;
impl crate::RegisterSpec for OPA2_CAL_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [opa2_cal::R](R) reader structure"]
impl crate::Readable for OPA2_CAL_SPEC {
    type Reader = R;
}
#[doc = "`write(|w| ..)` method takes [opa2_cal::W](W) writer structure"]
impl crate::Writable for OPA2_CAL_SPEC {
    type Writer = W;
    const ZERO_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
}
#[doc = "`reset()` method sets OPA2_CAL to value 0x80e7"]
impl crate::Resettable for OPA2_CAL_SPEC {
    const RESET_VALUE: Self::Ux = 0x80e7;
}
