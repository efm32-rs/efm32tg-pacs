#[doc = "Register `BUF9_DATA` reader"]
pub struct R(crate::R<BUF9_DATA_SPEC>);
impl core::ops::Deref for R {
    type Target = crate::R<BUF9_DATA_SPEC>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
impl From<crate::R<BUF9_DATA_SPEC>> for R {
    #[inline(always)]
    fn from(reader: crate::R<BUF9_DATA_SPEC>) -> Self {
        R(reader)
    }
}
#[doc = "Register `BUF9_DATA` writer"]
pub struct W(crate::W<BUF9_DATA_SPEC>);
impl core::ops::Deref for W {
    type Target = crate::W<BUF9_DATA_SPEC>;
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
impl From<crate::W<BUF9_DATA_SPEC>> for W {
    #[inline(always)]
    fn from(writer: crate::W<BUF9_DATA_SPEC>) -> Self {
        W(writer)
    }
}
#[doc = "Field `DATA` reader - Scan Result Buffer"]
pub type DATA_R = crate::FieldReader<u16, u16>;
#[doc = "Field `DATA` writer - Scan Result Buffer"]
pub type DATA_W<'a> = crate::FieldWriter<'a, u32, BUF9_DATA_SPEC, u16, u16, 16, 0>;
#[doc = "Field `DATASRC` reader - Result Data Source"]
pub type DATASRC_R = crate::FieldReader<u8, u8>;
impl R {
    #[doc = "Bits 0:15 - Scan Result Buffer"]
    #[inline(always)]
    pub fn data(&self) -> DATA_R {
        DATA_R::new((self.bits & 0xffff) as u16)
    }
    #[doc = "Bits 16:19 - Result Data Source"]
    #[inline(always)]
    pub fn datasrc(&self) -> DATASRC_R {
        DATASRC_R::new(((self.bits >> 16) & 0x0f) as u8)
    }
}
impl W {
    #[doc = "Bits 0:15 - Scan Result Buffer"]
    #[inline(always)]
    pub fn data(&mut self) -> DATA_W {
        DATA_W::new(self)
    }
    #[doc = "Writes raw bits to the register."]
    #[inline(always)]
    pub unsafe fn bits(&mut self, bits: u32) -> &mut Self {
        self.0.bits(bits);
        self
    }
}
#[doc = "Scan Results\n\nThis register you can [`read`](crate::generic::Reg::read), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [buf9_data](index.html) module"]
pub struct BUF9_DATA_SPEC;
impl crate::RegisterSpec for BUF9_DATA_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [buf9_data::R](R) reader structure"]
impl crate::Readable for BUF9_DATA_SPEC {
    type Reader = R;
}
#[doc = "`write(|w| ..)` method takes [buf9_data::W](W) writer structure"]
impl crate::Writable for BUF9_DATA_SPEC {
    type Writer = W;
}
#[doc = "`reset()` method sets BUF9_DATA to value 0"]
impl crate::Resettable for BUF9_DATA_SPEC {
    #[inline(always)]
    fn reset_value() -> Self::Ux {
        0
    }
}
