#[doc = "Register `KEYBUF` reader"]
pub struct R(crate::R<KEYBUF_SPEC>);
impl core::ops::Deref for R {
    type Target = crate::R<KEYBUF_SPEC>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
impl From<crate::R<KEYBUF_SPEC>> for R {
    #[inline(always)]
    fn from(reader: crate::R<KEYBUF_SPEC>) -> Self {
        R(reader)
    }
}
#[doc = "Register `KEYBUF` writer"]
pub struct W(crate::W<KEYBUF_SPEC>);
impl core::ops::Deref for W {
    type Target = crate::W<KEYBUF_SPEC>;
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
impl From<crate::W<KEYBUF_SPEC>> for W {
    #[inline(always)]
    fn from(writer: crate::W<KEYBUF_SPEC>) -> Self {
        W(writer)
    }
}
#[doc = "Field `KEYBUF` reader - Key Buffer Access"]
pub type KEYBUF_R = crate::FieldReader<u32, u32>;
#[doc = "Field `KEYBUF` writer - Key Buffer Access"]
pub type KEYBUF_W<'a> = crate::FieldWriter<'a, u32, KEYBUF_SPEC, u32, u32, 32, 0>;
impl R {
    #[doc = "Bits 0:31 - Key Buffer Access"]
    #[inline(always)]
    pub fn keybuf(&self) -> KEYBUF_R {
        KEYBUF_R::new(self.bits)
    }
}
impl W {
    #[doc = "Bits 0:31 - Key Buffer Access"]
    #[inline(always)]
    pub fn keybuf(&mut self) -> KEYBUF_W {
        KEYBUF_W::new(self)
    }
    #[doc = "Writes raw bits to the register."]
    #[inline(always)]
    pub unsafe fn bits(&mut self, bits: u32) -> &mut Self {
        self.0.bits(bits);
        self
    }
}
#[doc = "KEY Buffer Register Access\n\nThis register you can [`read`](crate::generic::Reg::read), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [keybuf](index.html) module\n\nOne or more dependent resources other than the current register are immediately affected by a read operation."]
pub struct KEYBUF_SPEC;
impl crate::RegisterSpec for KEYBUF_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [keybuf::R](R) reader structure"]
impl crate::Readable for KEYBUF_SPEC {
    type Reader = R;
}
#[doc = "`write(|w| ..)` method takes [keybuf::W](W) writer structure"]
impl crate::Writable for KEYBUF_SPEC {
    type Writer = W;
}
#[doc = "`reset()` method sets KEYBUF to value 0"]
impl crate::Resettable for KEYBUF_SPEC {
    #[inline(always)]
    fn reset_value() -> Self::Ux {
        0
    }
}
