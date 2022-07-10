#[doc = "Register `DATE` reader"]
pub struct R(crate::R<DATE_SPEC>);
impl core::ops::Deref for R {
    type Target = crate::R<DATE_SPEC>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
impl From<crate::R<DATE_SPEC>> for R {
    #[inline(always)]
    fn from(reader: crate::R<DATE_SPEC>) -> Self {
        R(reader)
    }
}
#[doc = "Register `DATE` writer"]
pub struct W(crate::W<DATE_SPEC>);
impl core::ops::Deref for W {
    type Target = crate::W<DATE_SPEC>;
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
impl From<crate::W<DATE_SPEC>> for W {
    #[inline(always)]
    fn from(writer: crate::W<DATE_SPEC>) -> Self {
        W(writer)
    }
}
#[doc = "Field `DAYOMU` reader - Day of Month, Units"]
pub type DAYOMU_R = crate::FieldReader<u8, u8>;
#[doc = "Field `DAYOMU` writer - Day of Month, Units"]
pub type DAYOMU_W<'a> = crate::FieldWriter<'a, u32, DATE_SPEC, u8, u8, 4, 0>;
#[doc = "Field `DAYOMT` reader - Day of Month, Tens"]
pub type DAYOMT_R = crate::FieldReader<u8, u8>;
#[doc = "Field `DAYOMT` writer - Day of Month, Tens"]
pub type DAYOMT_W<'a> = crate::FieldWriter<'a, u32, DATE_SPEC, u8, u8, 2, 4>;
#[doc = "Field `MONTHU` reader - Month, Units"]
pub type MONTHU_R = crate::FieldReader<u8, u8>;
#[doc = "Field `MONTHU` writer - Month, Units"]
pub type MONTHU_W<'a> = crate::FieldWriter<'a, u32, DATE_SPEC, u8, u8, 4, 8>;
#[doc = "Field `MONTHT` reader - Month, Tens"]
pub type MONTHT_R = crate::BitReader<bool>;
#[doc = "Field `MONTHT` writer - Month, Tens"]
pub type MONTHT_W<'a> = crate::BitWriter<'a, u32, DATE_SPEC, bool, 12>;
#[doc = "Field `YEARU` reader - Year, Units"]
pub type YEARU_R = crate::FieldReader<u8, u8>;
#[doc = "Field `YEARU` writer - Year, Units"]
pub type YEARU_W<'a> = crate::FieldWriter<'a, u32, DATE_SPEC, u8, u8, 4, 16>;
#[doc = "Field `YEART` reader - Year, Tens"]
pub type YEART_R = crate::FieldReader<u8, u8>;
#[doc = "Field `YEART` writer - Year, Tens"]
pub type YEART_W<'a> = crate::FieldWriter<'a, u32, DATE_SPEC, u8, u8, 4, 20>;
#[doc = "Field `DAYOW` reader - Day of Week"]
pub type DAYOW_R = crate::FieldReader<u8, u8>;
#[doc = "Field `DAYOW` writer - Day of Week"]
pub type DAYOW_W<'a> = crate::FieldWriter<'a, u32, DATE_SPEC, u8, u8, 3, 24>;
impl R {
    #[doc = "Bits 0:3 - Day of Month, Units"]
    #[inline(always)]
    pub fn dayomu(&self) -> DAYOMU_R {
        DAYOMU_R::new((self.bits & 0x0f) as u8)
    }
    #[doc = "Bits 4:5 - Day of Month, Tens"]
    #[inline(always)]
    pub fn dayomt(&self) -> DAYOMT_R {
        DAYOMT_R::new(((self.bits >> 4) & 3) as u8)
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
    #[doc = "Bits 16:19 - Year, Units"]
    #[inline(always)]
    pub fn yearu(&self) -> YEARU_R {
        YEARU_R::new(((self.bits >> 16) & 0x0f) as u8)
    }
    #[doc = "Bits 20:23 - Year, Tens"]
    #[inline(always)]
    pub fn yeart(&self) -> YEART_R {
        YEART_R::new(((self.bits >> 20) & 0x0f) as u8)
    }
    #[doc = "Bits 24:26 - Day of Week"]
    #[inline(always)]
    pub fn dayow(&self) -> DAYOW_R {
        DAYOW_R::new(((self.bits >> 24) & 7) as u8)
    }
}
impl W {
    #[doc = "Bits 0:3 - Day of Month, Units"]
    #[inline(always)]
    pub fn dayomu(&mut self) -> DAYOMU_W {
        DAYOMU_W::new(self)
    }
    #[doc = "Bits 4:5 - Day of Month, Tens"]
    #[inline(always)]
    pub fn dayomt(&mut self) -> DAYOMT_W {
        DAYOMT_W::new(self)
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
    #[doc = "Bits 16:19 - Year, Units"]
    #[inline(always)]
    pub fn yearu(&mut self) -> YEARU_W {
        YEARU_W::new(self)
    }
    #[doc = "Bits 20:23 - Year, Tens"]
    #[inline(always)]
    pub fn yeart(&mut self) -> YEART_W {
        YEART_W::new(self)
    }
    #[doc = "Bits 24:26 - Day of Week"]
    #[inline(always)]
    pub fn dayow(&mut self) -> DAYOW_W {
        DAYOW_W::new(self)
    }
    #[doc = "Writes raw bits to the register."]
    #[inline(always)]
    pub unsafe fn bits(&mut self, bits: u32) -> &mut Self {
        self.0.bits(bits);
        self
    }
}
#[doc = "Date Register\n\nThis register you can [`read`](crate::generic::Reg::read), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [date](index.html) module"]
pub struct DATE_SPEC;
impl crate::RegisterSpec for DATE_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [date::R](R) reader structure"]
impl crate::Readable for DATE_SPEC {
    type Reader = R;
}
#[doc = "`write(|w| ..)` method takes [date::W](W) writer structure"]
impl crate::Writable for DATE_SPEC {
    type Writer = W;
}
#[doc = "`reset()` method sets DATE to value 0"]
impl crate::Resettable for DATE_SPEC {
    #[inline(always)]
    fn reset_value() -> Self::Ux {
        0
    }
}
