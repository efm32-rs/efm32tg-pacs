#[doc = "Register `DCDCLNCOMPCTRL` reader"]
pub struct R(crate::R<DCDCLNCOMPCTRL_SPEC>);
impl core::ops::Deref for R {
    type Target = crate::R<DCDCLNCOMPCTRL_SPEC>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
impl From<crate::R<DCDCLNCOMPCTRL_SPEC>> for R {
    #[inline(always)]
    fn from(reader: crate::R<DCDCLNCOMPCTRL_SPEC>) -> Self {
        R(reader)
    }
}
#[doc = "Register `DCDCLNCOMPCTRL` writer"]
pub struct W(crate::W<DCDCLNCOMPCTRL_SPEC>);
impl core::ops::Deref for W {
    type Target = crate::W<DCDCLNCOMPCTRL_SPEC>;
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
impl From<crate::W<DCDCLNCOMPCTRL_SPEC>> for W {
    #[inline(always)]
    fn from(writer: crate::W<DCDCLNCOMPCTRL_SPEC>) -> Self {
        W(writer)
    }
}
#[doc = "Field `COMPENR1` reader - Low Noise Mode Compensator R1 Trim Value"]
pub type COMPENR1_R = crate::FieldReader<u8, u8>;
#[doc = "Field `COMPENR1` writer - Low Noise Mode Compensator R1 Trim Value"]
pub type COMPENR1_W<'a, const O: u8> =
    crate::FieldWriter<'a, u32, DCDCLNCOMPCTRL_SPEC, u8, u8, 3, O>;
#[doc = "Field `COMPENR2` reader - Low Noise Mode Compensator R2 Trim Value"]
pub type COMPENR2_R = crate::FieldReader<u8, u8>;
#[doc = "Field `COMPENR2` writer - Low Noise Mode Compensator R2 Trim Value"]
pub type COMPENR2_W<'a, const O: u8> =
    crate::FieldWriter<'a, u32, DCDCLNCOMPCTRL_SPEC, u8, u8, 5, O>;
#[doc = "Field `COMPENR3` reader - Low Noise Mode Compensator R3 Trim Value"]
pub type COMPENR3_R = crate::FieldReader<u8, u8>;
#[doc = "Field `COMPENR3` writer - Low Noise Mode Compensator R3 Trim Value"]
pub type COMPENR3_W<'a, const O: u8> =
    crate::FieldWriter<'a, u32, DCDCLNCOMPCTRL_SPEC, u8, u8, 4, O>;
#[doc = "Field `COMPENC1` reader - Low Noise Mode Compensator C1 Trim Value"]
pub type COMPENC1_R = crate::FieldReader<u8, u8>;
#[doc = "Field `COMPENC1` writer - Low Noise Mode Compensator C1 Trim Value"]
pub type COMPENC1_W<'a, const O: u8> =
    crate::FieldWriter<'a, u32, DCDCLNCOMPCTRL_SPEC, u8, u8, 2, O>;
#[doc = "Field `COMPENC2` reader - Low Noise Mode Compensator C2 Trim Value"]
pub type COMPENC2_R = crate::FieldReader<u8, u8>;
#[doc = "Field `COMPENC2` writer - Low Noise Mode Compensator C2 Trim Value"]
pub type COMPENC2_W<'a, const O: u8> =
    crate::FieldWriter<'a, u32, DCDCLNCOMPCTRL_SPEC, u8, u8, 3, O>;
#[doc = "Field `COMPENC3` reader - Low Noise Mode Compensator C3 Trim Value"]
pub type COMPENC3_R = crate::FieldReader<u8, u8>;
#[doc = "Field `COMPENC3` writer - Low Noise Mode Compensator C3 Trim Value"]
pub type COMPENC3_W<'a, const O: u8> =
    crate::FieldWriter<'a, u32, DCDCLNCOMPCTRL_SPEC, u8, u8, 4, O>;
impl R {
    #[doc = "Bits 0:2 - Low Noise Mode Compensator R1 Trim Value"]
    #[inline(always)]
    pub fn compenr1(&self) -> COMPENR1_R {
        COMPENR1_R::new((self.bits & 7) as u8)
    }
    #[doc = "Bits 4:8 - Low Noise Mode Compensator R2 Trim Value"]
    #[inline(always)]
    pub fn compenr2(&self) -> COMPENR2_R {
        COMPENR2_R::new(((self.bits >> 4) & 0x1f) as u8)
    }
    #[doc = "Bits 12:15 - Low Noise Mode Compensator R3 Trim Value"]
    #[inline(always)]
    pub fn compenr3(&self) -> COMPENR3_R {
        COMPENR3_R::new(((self.bits >> 12) & 0x0f) as u8)
    }
    #[doc = "Bits 20:21 - Low Noise Mode Compensator C1 Trim Value"]
    #[inline(always)]
    pub fn compenc1(&self) -> COMPENC1_R {
        COMPENC1_R::new(((self.bits >> 20) & 3) as u8)
    }
    #[doc = "Bits 24:26 - Low Noise Mode Compensator C2 Trim Value"]
    #[inline(always)]
    pub fn compenc2(&self) -> COMPENC2_R {
        COMPENC2_R::new(((self.bits >> 24) & 7) as u8)
    }
    #[doc = "Bits 28:31 - Low Noise Mode Compensator C3 Trim Value"]
    #[inline(always)]
    pub fn compenc3(&self) -> COMPENC3_R {
        COMPENC3_R::new(((self.bits >> 28) & 0x0f) as u8)
    }
}
impl W {
    #[doc = "Bits 0:2 - Low Noise Mode Compensator R1 Trim Value"]
    #[inline(always)]
    #[must_use]
    pub fn compenr1(&mut self) -> COMPENR1_W<0> {
        COMPENR1_W::new(self)
    }
    #[doc = "Bits 4:8 - Low Noise Mode Compensator R2 Trim Value"]
    #[inline(always)]
    #[must_use]
    pub fn compenr2(&mut self) -> COMPENR2_W<4> {
        COMPENR2_W::new(self)
    }
    #[doc = "Bits 12:15 - Low Noise Mode Compensator R3 Trim Value"]
    #[inline(always)]
    #[must_use]
    pub fn compenr3(&mut self) -> COMPENR3_W<12> {
        COMPENR3_W::new(self)
    }
    #[doc = "Bits 20:21 - Low Noise Mode Compensator C1 Trim Value"]
    #[inline(always)]
    #[must_use]
    pub fn compenc1(&mut self) -> COMPENC1_W<20> {
        COMPENC1_W::new(self)
    }
    #[doc = "Bits 24:26 - Low Noise Mode Compensator C2 Trim Value"]
    #[inline(always)]
    #[must_use]
    pub fn compenc2(&mut self) -> COMPENC2_W<24> {
        COMPENC2_W::new(self)
    }
    #[doc = "Bits 28:31 - Low Noise Mode Compensator C3 Trim Value"]
    #[inline(always)]
    #[must_use]
    pub fn compenc3(&mut self) -> COMPENC3_W<28> {
        COMPENC3_W::new(self)
    }
    #[doc = "Writes raw bits to the register."]
    #[inline(always)]
    pub unsafe fn bits(&mut self, bits: u32) -> &mut Self {
        self.0.bits(bits);
        self
    }
}
#[doc = "DCDC Low Noise Compensator Control Register\n\nThis register you can [`read`](crate::generic::Reg::read), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [dcdclncompctrl](index.html) module"]
pub struct DCDCLNCOMPCTRL_SPEC;
impl crate::RegisterSpec for DCDCLNCOMPCTRL_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [dcdclncompctrl::R](R) reader structure"]
impl crate::Readable for DCDCLNCOMPCTRL_SPEC {
    type Reader = R;
}
#[doc = "`write(|w| ..)` method takes [dcdclncompctrl::W](W) writer structure"]
impl crate::Writable for DCDCLNCOMPCTRL_SPEC {
    type Writer = W;
    const ZERO_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
}
#[doc = "`reset()` method sets DCDCLNCOMPCTRL to value 0x5720_4077"]
impl crate::Resettable for DCDCLNCOMPCTRL_SPEC {
    const RESET_VALUE: Self::Ux = 0x5720_4077;
}
