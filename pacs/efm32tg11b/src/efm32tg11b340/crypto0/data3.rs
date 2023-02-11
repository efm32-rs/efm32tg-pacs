#[doc = "Register `DATA3` reader"]
pub struct R(crate::R<DATA3_SPEC>);
impl core::ops::Deref for R {
    type Target = crate::R<DATA3_SPEC>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
impl From<crate::R<DATA3_SPEC>> for R {
    #[inline(always)]
    fn from(reader: crate::R<DATA3_SPEC>) -> Self {
        R(reader)
    }
}
#[doc = "Register `DATA3` writer"]
pub struct W(crate::W<DATA3_SPEC>);
impl core::ops::Deref for W {
    type Target = crate::W<DATA3_SPEC>;
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
impl From<crate::W<DATA3_SPEC>> for W {
    #[inline(always)]
    fn from(writer: crate::W<DATA3_SPEC>) -> Self {
        W(writer)
    }
}
#[doc = "Field `DATA3` reader - Data 3 Access"]
pub type DATA3_R = crate::FieldReader<u32, u32>;
#[doc = "Field `DATA3` writer - Data 3 Access"]
pub type DATA3_W<'a, const O: u8> = crate::FieldWriter<'a, u32, DATA3_SPEC, u32, u32, 32, O>;
impl R {
    #[doc = "Bits 0:31 - Data 3 Access"]
    #[inline(always)]
    pub fn data3(&self) -> DATA3_R {
        DATA3_R::new(self.bits)
    }
}
impl W {
    #[doc = "Bits 0:31 - Data 3 Access"]
    #[inline(always)]
    #[must_use]
    pub fn data3(&mut self) -> DATA3_W<0> {
        DATA3_W::new(self)
    }
    #[doc = "Writes raw bits to the register."]
    #[inline(always)]
    pub unsafe fn bits(&mut self, bits: u32) -> &mut Self {
        self.0.bits(bits);
        self
    }
}
#[doc = "DATA3 Register Access\n\nThis register you can [`read`](crate::generic::Reg::read), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [data3](index.html) module\n\nOne or more dependent resources other than the current register are immediately affected by a read operation."]
pub struct DATA3_SPEC;
impl crate::RegisterSpec for DATA3_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [data3::R](R) reader structure"]
impl crate::Readable for DATA3_SPEC {
    type Reader = R;
}
#[doc = "`write(|w| ..)` method takes [data3::W](W) writer structure"]
impl crate::Writable for DATA3_SPEC {
    type Writer = W;
    const ZERO_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
}
#[doc = "`reset()` method sets DATA3 to value 0"]
impl crate::Resettable for DATA3_SPEC {
    const RESET_VALUE: Self::Ux = 0;
}
