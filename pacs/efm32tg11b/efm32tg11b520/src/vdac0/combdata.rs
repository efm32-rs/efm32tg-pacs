#[doc = "Register `COMBDATA` reader"]
pub struct R(crate::R<COMBDATA_SPEC>);
impl core::ops::Deref for R {
    type Target = crate::R<COMBDATA_SPEC>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
impl From<crate::R<COMBDATA_SPEC>> for R {
    #[inline(always)]
    fn from(reader: crate::R<COMBDATA_SPEC>) -> Self {
        R(reader)
    }
}
#[doc = "Register `COMBDATA` writer"]
pub struct W(crate::W<COMBDATA_SPEC>);
impl core::ops::Deref for W {
    type Target = crate::W<COMBDATA_SPEC>;
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
impl From<crate::W<COMBDATA_SPEC>> for W {
    #[inline(always)]
    fn from(writer: crate::W<COMBDATA_SPEC>) -> Self {
        W(writer)
    }
}
#[doc = "Field `CH0DATA` reader - Channel 0 Data"]
pub type CH0DATA_R = crate::FieldReader<u16, u16>;
#[doc = "Field `CH0DATA` writer - Channel 0 Data"]
pub type CH0DATA_W<'a> = crate::FieldWriter<'a, u32, COMBDATA_SPEC, u16, u16, 12, 0>;
#[doc = "Field `CH1DATA` reader - Channel 1 Data"]
pub type CH1DATA_R = crate::FieldReader<u16, u16>;
#[doc = "Field `CH1DATA` writer - Channel 1 Data"]
pub type CH1DATA_W<'a> = crate::FieldWriter<'a, u32, COMBDATA_SPEC, u16, u16, 12, 16>;
impl R {
    #[doc = "Bits 0:11 - Channel 0 Data"]
    #[inline(always)]
    pub fn ch0data(&self) -> CH0DATA_R {
        CH0DATA_R::new((self.bits & 0x0fff) as u16)
    }
    #[doc = "Bits 16:27 - Channel 1 Data"]
    #[inline(always)]
    pub fn ch1data(&self) -> CH1DATA_R {
        CH1DATA_R::new(((self.bits >> 16) & 0x0fff) as u16)
    }
}
impl W {
    #[doc = "Bits 0:11 - Channel 0 Data"]
    #[inline(always)]
    pub fn ch0data(&mut self) -> CH0DATA_W {
        CH0DATA_W::new(self)
    }
    #[doc = "Bits 16:27 - Channel 1 Data"]
    #[inline(always)]
    pub fn ch1data(&mut self) -> CH1DATA_W {
        CH1DATA_W::new(self)
    }
    #[doc = "Writes raw bits to the register."]
    #[inline(always)]
    pub unsafe fn bits(&mut self, bits: u32) -> &mut Self {
        self.0.bits(bits);
        self
    }
}
#[doc = "Combined Data Register\n\nThis register you can [`read`](crate::generic::Reg::read), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [combdata](index.html) module"]
pub struct COMBDATA_SPEC;
impl crate::RegisterSpec for COMBDATA_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [combdata::R](R) reader structure"]
impl crate::Readable for COMBDATA_SPEC {
    type Reader = R;
}
#[doc = "`write(|w| ..)` method takes [combdata::W](W) writer structure"]
impl crate::Writable for COMBDATA_SPEC {
    type Writer = W;
}
#[doc = "`reset()` method sets COMBDATA to value 0x0800_0800"]
impl crate::Resettable for COMBDATA_SPEC {
    #[inline(always)]
    fn reset_value() -> Self::Ux {
        0x0800_0800
    }
}
