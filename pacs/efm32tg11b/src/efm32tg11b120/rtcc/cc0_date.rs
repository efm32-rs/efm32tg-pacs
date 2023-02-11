#[doc = "Register `CC0_DATE` reader"]
pub struct R(crate::R<CC0_DATE_SPEC>);
impl core::ops::Deref for R {
    type Target = crate::R<CC0_DATE_SPEC>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
impl From<crate::R<CC0_DATE_SPEC>> for R {
    #[inline(always)]
    fn from(reader: crate::R<CC0_DATE_SPEC>) -> Self {
        R(reader)
    }
}
#[doc = "Register `CC0_DATE` writer"]
pub struct W(crate::W<CC0_DATE_SPEC>);
impl core::ops::Deref for W {
    type Target = crate::W<CC0_DATE_SPEC>;
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
impl From<crate::W<CC0_DATE_SPEC>> for W {
    #[inline(always)]
    fn from(writer: crate::W<CC0_DATE_SPEC>) -> Self {
        W(writer)
    }
}
#[doc = "Field `DAYU` reader - Day of Month/week, Units"]
pub type DAYU_R = crate::FieldReader<u8, u8>;
#[doc = "Field `DAYU` writer - Day of Month/week, Units"]
pub type DAYU_W<'a, const O: u8> = crate::FieldWriter<'a, u32, CC0_DATE_SPEC, u8, u8, 4, O>;
#[doc = "Field `DAYT` reader - Day of Month/week, Tens"]
pub type DAYT_R = crate::FieldReader<u8, u8>;
#[doc = "Field `DAYT` writer - Day of Month/week, Tens"]
pub type DAYT_W<'a, const O: u8> = crate::FieldWriter<'a, u32, CC0_DATE_SPEC, u8, u8, 2, O>;
#[doc = "Field `MONTHU` reader - Month, Units"]
pub type MONTHU_R = crate::FieldReader<u8, u8>;
#[doc = "Field `MONTHU` writer - Month, Units"]
pub type MONTHU_W<'a, const O: u8> = crate::FieldWriter<'a, u32, CC0_DATE_SPEC, u8, u8, 4, O>;
#[doc = "Field `MONTHT` reader - Month, Tens"]
pub type MONTHT_R = crate::BitReader<bool>;
#[doc = "Field `MONTHT` writer - Month, Tens"]
pub type MONTHT_W<'a, const O: u8> = crate::BitWriter<'a, u32, CC0_DATE_SPEC, bool, O>;
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
    #[must_use]
    pub fn dayu(&mut self) -> DAYU_W<0> {
        DAYU_W::new(self)
    }
    #[doc = "Bits 4:5 - Day of Month/week, Tens"]
    #[inline(always)]
    #[must_use]
    pub fn dayt(&mut self) -> DAYT_W<4> {
        DAYT_W::new(self)
    }
    #[doc = "Bits 8:11 - Month, Units"]
    #[inline(always)]
    #[must_use]
    pub fn monthu(&mut self) -> MONTHU_W<8> {
        MONTHU_W::new(self)
    }
    #[doc = "Bit 12 - Month, Tens"]
    #[inline(always)]
    #[must_use]
    pub fn montht(&mut self) -> MONTHT_W<12> {
        MONTHT_W::new(self)
    }
    #[doc = "Writes raw bits to the register."]
    #[inline(always)]
    pub unsafe fn bits(&mut self, bits: u32) -> &mut Self {
        self.0.bits(bits);
        self
    }
}
#[doc = "Capture/Compare Date Register\n\nThis register you can [`read`](crate::generic::Reg::read), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [cc0_date](index.html) module"]
pub struct CC0_DATE_SPEC;
impl crate::RegisterSpec for CC0_DATE_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [cc0_date::R](R) reader structure"]
impl crate::Readable for CC0_DATE_SPEC {
    type Reader = R;
}
#[doc = "`write(|w| ..)` method takes [cc0_date::W](W) writer structure"]
impl crate::Writable for CC0_DATE_SPEC {
    type Writer = W;
    const ZERO_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
}
#[doc = "`reset()` method sets CC0_DATE to value 0"]
impl crate::Resettable for CC0_DATE_SPEC {
    const RESET_VALUE: Self::Ux = 0;
}
