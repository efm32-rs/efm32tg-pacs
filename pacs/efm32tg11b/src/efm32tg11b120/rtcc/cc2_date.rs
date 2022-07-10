#[doc = "Register `CC2_DATE` reader"]
pub struct R(crate::R<CC2_DATE_SPEC>);
impl core::ops::Deref for R {
    type Target = crate::R<CC2_DATE_SPEC>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
impl From<crate::R<CC2_DATE_SPEC>> for R {
    #[inline(always)]
    fn from(reader: crate::R<CC2_DATE_SPEC>) -> Self {
        R(reader)
    }
}
#[doc = "Register `CC2_DATE` writer"]
pub struct W(crate::W<CC2_DATE_SPEC>);
impl core::ops::Deref for W {
    type Target = crate::W<CC2_DATE_SPEC>;
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
impl From<crate::W<CC2_DATE_SPEC>> for W {
    #[inline(always)]
    fn from(writer: crate::W<CC2_DATE_SPEC>) -> Self {
        W(writer)
    }
}
#[doc = "Field `DAYU` reader - Day of Month/week, Units"]
pub type DAYU_R = crate::FieldReader<u8, u8>;
#[doc = "Field `DAYU` writer - Day of Month/week, Units"]
pub type DAYU_W<'a> = crate::FieldWriter<'a, u32, CC2_DATE_SPEC, u8, u8, 4, 0>;
#[doc = "Field `DAYT` reader - Day of Month/week, Tens"]
pub type DAYT_R = crate::FieldReader<u8, u8>;
#[doc = "Field `DAYT` writer - Day of Month/week, Tens"]
pub type DAYT_W<'a> = crate::FieldWriter<'a, u32, CC2_DATE_SPEC, u8, u8, 2, 4>;
#[doc = "Field `MONTHU` reader - Month, Units"]
pub type MONTHU_R = crate::FieldReader<u8, u8>;
#[doc = "Field `MONTHU` writer - Month, Units"]
pub type MONTHU_W<'a> = crate::FieldWriter<'a, u32, CC2_DATE_SPEC, u8, u8, 4, 8>;
#[doc = "Field `MONTHT` reader - Month, Tens"]
pub type MONTHT_R = crate::BitReader<bool>;
#[doc = "Field `MONTHT` writer - Month, Tens"]
pub type MONTHT_W<'a> = crate::BitWriter<'a, u32, CC2_DATE_SPEC, bool, 12>;
impl R {
    #[doc = "Bits 0:3 - Day of Month/week, Units"]
    #[inline(always)]
    pub fn dayu(&self) -> DAYU_R {
        DAYU_R::new((self.bits & 0x0f) as u8)
    }
    #[doc = "Bits 4:5 - Day of Month/week, Tens"]
    #[inline(always)]
    pub fn dayt(&self) -> DAYT_R {
        DAYT_R::new(((self.bits >> 4) & 3) as u8)
    }
    #[doc = "Bits 8:11 - Month, Units"]
    #[inline(always)]
    pub fn monthu(&self) -> MONTHU_R {
        MONTHU_R::new(((self.bits >> 8) & 0x0f) as u8)
    }
    #[doc = "Bit 12 - Month, Tens"]
    #[inline(always)]
    pub fn montht(&self) -> MONTHT_R {
        MONTHT_R::new(((self.bits >> 12) & 1) != 0)
    }
}
impl W {
    #[doc = "Bits 0:3 - Day of Month/week, Units"]
    #[inline(always)]
    pub fn dayu(&mut self) -> DAYU_W {
        DAYU_W::new(self)
    }
    #[doc = "Bits 4:5 - Day of Month/week, Tens"]
    #[inline(always)]
    pub fn dayt(&mut self) -> DAYT_W {
        DAYT_W::new(self)
    }
    #[doc = "Bits 8:11 - Month, Units"]
    #[inline(always)]
    pub fn monthu(&mut self) -> MONTHU_W {
        MONTHU_W::new(self)
    }
    #[doc = "Bit 12 - Month, Tens"]
    #[inline(always)]
    pub fn montht(&mut self) -> MONTHT_W {
        MONTHT_W::new(self)
    }
    #[doc = "Writes raw bits to the register."]
    #[inline(always)]
    pub unsafe fn bits(&mut self, bits: u32) -> &mut Self {
        self.0.bits(bits);
        self
    }
}
#[doc = "Capture/Compare Date Register\n\nThis register you can [`read`](crate::generic::Reg::read), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [cc2_date](index.html) module"]
pub struct CC2_DATE_SPEC;
impl crate::RegisterSpec for CC2_DATE_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [cc2_date::R](R) reader structure"]
impl crate::Readable for CC2_DATE_SPEC {
    type Reader = R;
}
#[doc = "`write(|w| ..)` method takes [cc2_date::W](W) writer structure"]
impl crate::Writable for CC2_DATE_SPEC {
    type Writer = W;
}
#[doc = "`reset()` method sets CC2_DATE to value 0"]
impl crate::Resettable for CC2_DATE_SPEC {
    #[inline(always)]
    fn reset_value() -> Self::Ux {
        0
    }
}
