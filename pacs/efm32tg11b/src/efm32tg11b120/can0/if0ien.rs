#[doc = "Register `IF0IEN` reader"]
pub struct R(crate::R<IF0IEN_SPEC>);
impl core::ops::Deref for R {
    type Target = crate::R<IF0IEN_SPEC>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
impl From<crate::R<IF0IEN_SPEC>> for R {
    #[inline(always)]
    fn from(reader: crate::R<IF0IEN_SPEC>) -> Self {
        R(reader)
    }
}
#[doc = "Register `IF0IEN` writer"]
pub struct W(crate::W<IF0IEN_SPEC>);
impl core::ops::Deref for W {
    type Target = crate::W<IF0IEN_SPEC>;
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
impl From<crate::W<IF0IEN_SPEC>> for W {
    #[inline(always)]
    fn from(writer: crate::W<IF0IEN_SPEC>) -> Self {
        W(writer)
    }
}
#[doc = "Field `MESSAGE` reader - MESSAGE Interrupt Enable"]
pub type MESSAGE_R = crate::FieldReader<u32, u32>;
#[doc = "Field `MESSAGE` writer - MESSAGE Interrupt Enable"]
pub type MESSAGE_W<'a> = crate::FieldWriter<'a, u32, IF0IEN_SPEC, u32, u32, 32, 0>;
impl R {
    #[doc = "Bits 0:31 - MESSAGE Interrupt Enable"]
    #[inline(always)]
    pub fn message(&self) -> MESSAGE_R {
        MESSAGE_R::new(self.bits)
    }
}
impl W {
    #[doc = "Bits 0:31 - MESSAGE Interrupt Enable"]
    #[inline(always)]
    pub fn message(&mut self) -> MESSAGE_W {
        MESSAGE_W::new(self)
    }
    #[doc = "Writes raw bits to the register."]
    #[inline(always)]
    pub unsafe fn bits(&mut self, bits: u32) -> &mut Self {
        self.0.bits(bits);
        self
    }
}
#[doc = "Message Object Interrupt Enable Register\n\nThis register you can [`read`](crate::generic::Reg::read), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [if0ien](index.html) module"]
pub struct IF0IEN_SPEC;
impl crate::RegisterSpec for IF0IEN_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [if0ien::R](R) reader structure"]
impl crate::Readable for IF0IEN_SPEC {
    type Reader = R;
}
#[doc = "`write(|w| ..)` method takes [if0ien::W](W) writer structure"]
impl crate::Writable for IF0IEN_SPEC {
    type Writer = W;
}
#[doc = "`reset()` method sets IF0IEN to value 0xffff_ffff"]
impl crate::Resettable for IF0IEN_SPEC {
    #[inline(always)]
    fn reset_value() -> Self::Ux {
        0xffff_ffff
    }
}
