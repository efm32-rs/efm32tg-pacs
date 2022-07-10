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
#[doc = "Field `OFFSETTRIM` reader - Input Buffer Offset Calibration Value"]
pub type OFFSETTRIM_R = crate::FieldReader<u8, u8>;
#[doc = "Field `OFFSETTRIM` writer - Input Buffer Offset Calibration Value"]
pub type OFFSETTRIM_W<'a> = crate::FieldWriter<'a, u32, CAL_SPEC, u8, u8, 3, 0>;
#[doc = "Field `GAINERRTRIM` reader - Gain Error Trim Value"]
pub type GAINERRTRIM_R = crate::FieldReader<u8, u8>;
#[doc = "Field `GAINERRTRIM` writer - Gain Error Trim Value"]
pub type GAINERRTRIM_W<'a> = crate::FieldWriter<'a, u32, CAL_SPEC, u8, u8, 6, 8>;
#[doc = "Field `GAINERRTRIMCH1` reader - Gain Error Trim Value for CH1"]
pub type GAINERRTRIMCH1_R = crate::FieldReader<u8, u8>;
#[doc = "Field `GAINERRTRIMCH1` writer - Gain Error Trim Value for CH1"]
pub type GAINERRTRIMCH1_W<'a> = crate::FieldWriter<'a, u32, CAL_SPEC, u8, u8, 4, 16>;
impl R {
    #[doc = "Bits 0:2 - Input Buffer Offset Calibration Value"]
    #[inline(always)]
    pub fn offsettrim(&self) -> OFFSETTRIM_R {
        OFFSETTRIM_R::new((self.bits & 7) as u8)
    }
    #[doc = "Bits 8:13 - Gain Error Trim Value"]
    #[inline(always)]
    pub fn gainerrtrim(&self) -> GAINERRTRIM_R {
        GAINERRTRIM_R::new(((self.bits >> 8) & 0x3f) as u8)
    }
    #[doc = "Bits 16:19 - Gain Error Trim Value for CH1"]
    #[inline(always)]
    pub fn gainerrtrimch1(&self) -> GAINERRTRIMCH1_R {
        GAINERRTRIMCH1_R::new(((self.bits >> 16) & 0x0f) as u8)
    }
}
impl W {
    #[doc = "Bits 0:2 - Input Buffer Offset Calibration Value"]
    #[inline(always)]
    pub fn offsettrim(&mut self) -> OFFSETTRIM_W {
        OFFSETTRIM_W::new(self)
    }
    #[doc = "Bits 8:13 - Gain Error Trim Value"]
    #[inline(always)]
    pub fn gainerrtrim(&mut self) -> GAINERRTRIM_W {
        GAINERRTRIM_W::new(self)
    }
    #[doc = "Bits 16:19 - Gain Error Trim Value for CH1"]
    #[inline(always)]
    pub fn gainerrtrimch1(&mut self) -> GAINERRTRIMCH1_W {
        GAINERRTRIMCH1_W::new(self)
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
}
#[doc = "`reset()` method sets CAL to value 0x0008_2004"]
impl crate::Resettable for CAL_SPEC {
    #[inline(always)]
    fn reset_value() -> Self::Ux {
        0x0008_2004
    }
}
