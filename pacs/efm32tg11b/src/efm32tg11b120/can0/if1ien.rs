#[doc = "Register `IF1IEN` reader"]
pub struct R(crate::R<IF1IEN_SPEC>);
impl core::ops::Deref for R {
    type Target = crate::R<IF1IEN_SPEC>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
impl From<crate::R<IF1IEN_SPEC>> for R {
    #[inline(always)]
    fn from(reader: crate::R<IF1IEN_SPEC>) -> Self {
        R(reader)
    }
}
#[doc = "Register `IF1IEN` writer"]
pub struct W(crate::W<IF1IEN_SPEC>);
impl core::ops::Deref for W {
    type Target = crate::W<IF1IEN_SPEC>;
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
impl From<crate::W<IF1IEN_SPEC>> for W {
    #[inline(always)]
    fn from(writer: crate::W<IF1IEN_SPEC>) -> Self {
        W(writer)
    }
}
#[doc = "Field `STATUS` reader - STATUS Interrupt Enable"]
pub type STATUS_R = crate::BitReader<bool>;
#[doc = "Field `STATUS` writer - STATUS Interrupt Enable"]
pub type STATUS_W<'a, const O: u8> = crate::BitWriter<'a, u32, IF1IEN_SPEC, bool, O>;
impl R {
    #[doc = "Bit 0 - STATUS Interrupt Enable"]
    #[inline(always)]
    pub fn status(&self) -> STATUS_R {
        STATUS_R::new((self.bits & 1) != 0)
    }
}
impl W {
    #[doc = "Bit 0 - STATUS Interrupt Enable"]
    #[inline(always)]
    #[must_use]
    pub fn status(&mut self) -> STATUS_W<0> {
        STATUS_W::new(self)
    }
    #[doc = "Writes raw bits to the register."]
    #[inline(always)]
    pub unsafe fn bits(&mut self, bits: u32) -> &mut Self {
        self.0.bits(bits);
        self
    }
}
#[doc = "Status Interrupt Enable Register\n\nThis register you can [`read`](crate::generic::Reg::read), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [if1ien](index.html) module"]
pub struct IF1IEN_SPEC;
impl crate::RegisterSpec for IF1IEN_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [if1ien::R](R) reader structure"]
impl crate::Readable for IF1IEN_SPEC {
    type Reader = R;
}
#[doc = "`write(|w| ..)` method takes [if1ien::W](W) writer structure"]
impl crate::Writable for IF1IEN_SPEC {
    type Writer = W;
    const ZERO_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
}
#[doc = "`reset()` method sets IF1IEN to value 0x01"]
impl crate::Resettable for IF1IEN_SPEC {
    const RESET_VALUE: Self::Ux = 0x01;
}
