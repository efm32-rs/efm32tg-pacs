#[doc = "Register `BIASCTRL` reader"]
pub struct R(crate::R<BIASCTRL_SPEC>);
impl core::ops::Deref for R {
    type Target = crate::R<BIASCTRL_SPEC>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
impl From<crate::R<BIASCTRL_SPEC>> for R {
    #[inline(always)]
    fn from(reader: crate::R<BIASCTRL_SPEC>) -> Self {
        R(reader)
    }
}
#[doc = "Register `BIASCTRL` writer"]
pub struct W(crate::W<BIASCTRL_SPEC>);
impl core::ops::Deref for W {
    type Target = crate::W<BIASCTRL_SPEC>;
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
impl From<crate::W<BIASCTRL_SPEC>> for W {
    #[inline(always)]
    fn from(writer: crate::W<BIASCTRL_SPEC>) -> Self {
        W(writer)
    }
}
#[doc = "Field `SPEED` reader - SPEED Adjustment"]
pub type SPEED_R = crate::FieldReader<u8, u8>;
#[doc = "Field `SPEED` writer - SPEED Adjustment"]
pub type SPEED_W<'a, const O: u8> = crate::FieldWriter<'a, u32, BIASCTRL_SPEC, u8, u8, 3, O>;
#[doc = "Field `BUFDRV` reader - Buffer Drive Strength"]
pub type BUFDRV_R = crate::FieldReader<u8, u8>;
#[doc = "Field `BUFDRV` writer - Buffer Drive Strength"]
pub type BUFDRV_W<'a, const O: u8> = crate::FieldWriter<'a, u32, BIASCTRL_SPEC, u8, u8, 4, O>;
#[doc = "Field `BUFBIAS` reader - Buffer Bias Setting"]
pub type BUFBIAS_R = crate::FieldReader<u8, u8>;
#[doc = "Field `BUFBIAS` writer - Buffer Bias Setting"]
pub type BUFBIAS_W<'a, const O: u8> = crate::FieldWriter<'a, u32, BIASCTRL_SPEC, u8, u8, 3, O>;
impl R {
    #[doc = "Bits 0:2 - SPEED Adjustment"]
    #[inline(always)]
    pub fn speed(&self) -> SPEED_R {
        SPEED_R::new((self.bits & 7) as u8)
    }
    #[doc = "Bits 4:7 - Buffer Drive Strength"]
    #[inline(always)]
    pub fn bufdrv(&self) -> BUFDRV_R {
        BUFDRV_R::new(((self.bits >> 4) & 0x0f) as u8)
    }
    #[doc = "Bits 10:12 - Buffer Bias Setting"]
    #[inline(always)]
    pub fn bufbias(&self) -> BUFBIAS_R {
        BUFBIAS_R::new(((self.bits >> 10) & 7) as u8)
    }
}
impl W {
    #[doc = "Bits 0:2 - SPEED Adjustment"]
    #[inline(always)]
    #[must_use]
    pub fn speed(&mut self) -> SPEED_W<0> {
        SPEED_W::new(self)
    }
    #[doc = "Bits 4:7 - Buffer Drive Strength"]
    #[inline(always)]
    #[must_use]
    pub fn bufdrv(&mut self) -> BUFDRV_W<4> {
        BUFDRV_W::new(self)
    }
    #[doc = "Bits 10:12 - Buffer Bias Setting"]
    #[inline(always)]
    #[must_use]
    pub fn bufbias(&mut self) -> BUFBIAS_W<10> {
        BUFBIAS_W::new(self)
    }
    #[doc = "Writes raw bits to the register."]
    #[inline(always)]
    pub unsafe fn bits(&mut self, bits: u32) -> &mut Self {
        self.0.bits(bits);
        self
    }
}
#[doc = "Analog BIAS Control\n\nThis register you can [`read`](crate::generic::Reg::read), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [biasctrl](index.html) module"]
pub struct BIASCTRL_SPEC;
impl crate::RegisterSpec for BIASCTRL_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [biasctrl::R](R) reader structure"]
impl crate::Readable for BIASCTRL_SPEC {
    type Reader = R;
}
#[doc = "`write(|w| ..)` method takes [biasctrl::W](W) writer structure"]
impl crate::Writable for BIASCTRL_SPEC {
    type Writer = W;
    const ZERO_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
}
#[doc = "`reset()` method sets BIASCTRL to value 0"]
impl crate::Resettable for BIASCTRL_SPEC {
    const RESET_VALUE: Self::Ux = 0;
}
