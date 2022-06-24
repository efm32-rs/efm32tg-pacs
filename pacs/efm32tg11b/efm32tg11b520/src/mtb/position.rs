#[doc = "Register `POSITION` reader"]
pub struct R(crate::R<POSITION_SPEC>);
impl core::ops::Deref for R {
    type Target = crate::R<POSITION_SPEC>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
impl From<crate::R<POSITION_SPEC>> for R {
    #[inline(always)]
    fn from(reader: crate::R<POSITION_SPEC>) -> Self {
        R(reader)
    }
}
#[doc = "Register `POSITION` writer"]
pub struct W(crate::W<POSITION_SPEC>);
impl core::ops::Deref for W {
    type Target = crate::W<POSITION_SPEC>;
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
impl From<crate::W<POSITION_SPEC>> for W {
    #[inline(always)]
    fn from(writer: crate::W<POSITION_SPEC>) -> Self {
        W(writer)
    }
}
#[doc = "Field `WRAP` reader - Trace wrap bit."]
pub type WRAP_R = crate::BitReader<bool>;
#[doc = "Field `WRAP` writer - Trace wrap bit."]
pub type WRAP_W<'a> = crate::BitWriter<'a, u32, POSITION_SPEC, bool, 2>;
#[doc = "Field `POINTER` reader - Trace packet location pointer."]
pub type POINTER_R = crate::FieldReader<u32, u32>;
#[doc = "Field `POINTER` writer - Trace packet location pointer."]
pub type POINTER_W<'a> = crate::FieldWriter<'a, u32, POSITION_SPEC, u32, u32, 29, 3>;
impl R {
    #[doc = "Bit 2 - Trace wrap bit."]
    #[inline(always)]
    pub fn wrap(&self) -> WRAP_R {
        WRAP_R::new(((self.bits >> 2) & 1) != 0)
    }
    #[doc = "Bits 3:31 - Trace packet location pointer."]
    #[inline(always)]
    pub fn pointer(&self) -> POINTER_R {
        POINTER_R::new(((self.bits >> 3) & 0x1fff_ffff) as u32)
    }
}
impl W {
    #[doc = "Bit 2 - Trace wrap bit."]
    #[inline(always)]
    pub fn wrap(&mut self) -> WRAP_W {
        WRAP_W::new(self)
    }
    #[doc = "Bits 3:31 - Trace packet location pointer."]
    #[inline(always)]
    pub fn pointer(&mut self) -> POINTER_W {
        POINTER_W::new(self)
    }
    #[doc = "Writes raw bits to the register."]
    #[inline(always)]
    pub unsafe fn bits(&mut self, bits: u32) -> &mut Self {
        self.0.bits(bits);
        self
    }
}
#[doc = "MTB Trace Position Register.\n\nThis register you can [`read`](crate::generic::Reg::read), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [position](index.html) module"]
pub struct POSITION_SPEC;
impl crate::RegisterSpec for POSITION_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [position::R](R) reader structure"]
impl crate::Readable for POSITION_SPEC {
    type Reader = R;
}
#[doc = "`write(|w| ..)` method takes [position::W](W) writer structure"]
impl crate::Writable for POSITION_SPEC {
    type Writer = W;
}
#[doc = "`reset()` method sets POSITION to value 0"]
impl crate::Resettable for POSITION_SPEC {
    #[inline(always)]
    fn reset_value() -> Self::Ux {
        0
    }
}
