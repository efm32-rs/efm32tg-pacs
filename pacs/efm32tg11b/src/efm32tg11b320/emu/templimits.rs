#[doc = "Register `TEMPLIMITS` reader"]
pub struct R(crate::R<TEMPLIMITS_SPEC>);
impl core::ops::Deref for R {
    type Target = crate::R<TEMPLIMITS_SPEC>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
impl From<crate::R<TEMPLIMITS_SPEC>> for R {
    #[inline(always)]
    fn from(reader: crate::R<TEMPLIMITS_SPEC>) -> Self {
        R(reader)
    }
}
#[doc = "Register `TEMPLIMITS` writer"]
pub struct W(crate::W<TEMPLIMITS_SPEC>);
impl core::ops::Deref for W {
    type Target = crate::W<TEMPLIMITS_SPEC>;
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
impl From<crate::W<TEMPLIMITS_SPEC>> for W {
    #[inline(always)]
    fn from(writer: crate::W<TEMPLIMITS_SPEC>) -> Self {
        W(writer)
    }
}
#[doc = "Field `TEMPLOW` reader - Temperature Low Limit"]
pub type TEMPLOW_R = crate::FieldReader<u8, u8>;
#[doc = "Field `TEMPLOW` writer - Temperature Low Limit"]
pub type TEMPLOW_W<'a> = crate::FieldWriter<'a, u32, TEMPLIMITS_SPEC, u8, u8, 8, 0>;
#[doc = "Field `TEMPHIGH` reader - Temperature High Limit"]
pub type TEMPHIGH_R = crate::FieldReader<u8, u8>;
#[doc = "Field `TEMPHIGH` writer - Temperature High Limit"]
pub type TEMPHIGH_W<'a> = crate::FieldWriter<'a, u32, TEMPLIMITS_SPEC, u8, u8, 8, 8>;
#[doc = "Field `EM4WUEN` reader - Enable EM4 Wakeup Due to Low/high Temperature"]
pub type EM4WUEN_R = crate::BitReader<bool>;
#[doc = "Field `EM4WUEN` writer - Enable EM4 Wakeup Due to Low/high Temperature"]
pub type EM4WUEN_W<'a> = crate::BitWriter<'a, u32, TEMPLIMITS_SPEC, bool, 16>;
impl R {
    #[doc = "Bits 0:7 - Temperature Low Limit"]
    #[inline(always)]
    pub fn templow(&self) -> TEMPLOW_R {
        TEMPLOW_R::new((self.bits & 0xff) as u8)
    }
    #[doc = "Bits 8:15 - Temperature High Limit"]
    #[inline(always)]
    pub fn temphigh(&self) -> TEMPHIGH_R {
        TEMPHIGH_R::new(((self.bits >> 8) & 0xff) as u8)
    }
    #[doc = "Bit 16 - Enable EM4 Wakeup Due to Low/high Temperature"]
    #[inline(always)]
    pub fn em4wuen(&self) -> EM4WUEN_R {
        EM4WUEN_R::new(((self.bits >> 16) & 1) != 0)
    }
}
impl W {
    #[doc = "Bits 0:7 - Temperature Low Limit"]
    #[inline(always)]
    pub fn templow(&mut self) -> TEMPLOW_W {
        TEMPLOW_W::new(self)
    }
    #[doc = "Bits 8:15 - Temperature High Limit"]
    #[inline(always)]
    pub fn temphigh(&mut self) -> TEMPHIGH_W {
        TEMPHIGH_W::new(self)
    }
    #[doc = "Bit 16 - Enable EM4 Wakeup Due to Low/high Temperature"]
    #[inline(always)]
    pub fn em4wuen(&mut self) -> EM4WUEN_W {
        EM4WUEN_W::new(self)
    }
    #[doc = "Writes raw bits to the register."]
    #[inline(always)]
    pub unsafe fn bits(&mut self, bits: u32) -> &mut Self {
        self.0.bits(bits);
        self
    }
}
#[doc = "Temperature Limits for Interrupt Generation\n\nThis register you can [`read`](crate::generic::Reg::read), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [templimits](index.html) module"]
pub struct TEMPLIMITS_SPEC;
impl crate::RegisterSpec for TEMPLIMITS_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [templimits::R](R) reader structure"]
impl crate::Readable for TEMPLIMITS_SPEC {
    type Reader = R;
}
#[doc = "`write(|w| ..)` method takes [templimits::W](W) writer structure"]
impl crate::Writable for TEMPLIMITS_SPEC {
    type Writer = W;
}
#[doc = "`reset()` method sets TEMPLIMITS to value 0xff00"]
impl crate::Resettable for TEMPLIMITS_SPEC {
    #[inline(always)]
    fn reset_value() -> Self::Ux {
        0xff00
    }
}
