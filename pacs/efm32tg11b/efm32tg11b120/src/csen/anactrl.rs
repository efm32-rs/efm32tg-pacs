#[doc = "Register `ANACTRL` reader"]
pub struct R(crate::R<ANACTRL_SPEC>);
impl core::ops::Deref for R {
    type Target = crate::R<ANACTRL_SPEC>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
impl From<crate::R<ANACTRL_SPEC>> for R {
    #[inline(always)]
    fn from(reader: crate::R<ANACTRL_SPEC>) -> Self {
        R(reader)
    }
}
#[doc = "Register `ANACTRL` writer"]
pub struct W(crate::W<ANACTRL_SPEC>);
impl core::ops::Deref for W {
    type Target = crate::W<ANACTRL_SPEC>;
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
impl From<crate::W<ANACTRL_SPEC>> for W {
    #[inline(always)]
    fn from(writer: crate::W<ANACTRL_SPEC>) -> Self {
        W(writer)
    }
}
#[doc = "Field `IREFPROG` reader - Reference Current Control."]
pub type IREFPROG_R = crate::FieldReader<u8, u8>;
#[doc = "Field `IREFPROG` writer - Reference Current Control."]
pub type IREFPROG_W<'a> = crate::FieldWriter<'a, u32, ANACTRL_SPEC, u8, u8, 3, 4>;
#[doc = "Field `IDACIREFS` reader - Current DAC and Reference Current Scale"]
pub type IDACIREFS_R = crate::FieldReader<u8, u8>;
#[doc = "Field `IDACIREFS` writer - Current DAC and Reference Current Scale"]
pub type IDACIREFS_W<'a> = crate::FieldWriter<'a, u32, ANACTRL_SPEC, u8, u8, 3, 8>;
#[doc = "Field `TRSTPROG` reader - Reset Timing"]
pub type TRSTPROG_R = crate::FieldReader<u8, u8>;
#[doc = "Field `TRSTPROG` writer - Reset Timing"]
pub type TRSTPROG_W<'a> = crate::FieldWriter<'a, u32, ANACTRL_SPEC, u8, u8, 3, 20>;
impl R {
    #[doc = "Bits 4:6 - Reference Current Control."]
    #[inline(always)]
    pub fn irefprog(&self) -> IREFPROG_R {
        IREFPROG_R::new(((self.bits >> 4) & 7) as u8)
    }
    #[doc = "Bits 8:10 - Current DAC and Reference Current Scale"]
    #[inline(always)]
    pub fn idacirefs(&self) -> IDACIREFS_R {
        IDACIREFS_R::new(((self.bits >> 8) & 7) as u8)
    }
    #[doc = "Bits 20:22 - Reset Timing"]
    #[inline(always)]
    pub fn trstprog(&self) -> TRSTPROG_R {
        TRSTPROG_R::new(((self.bits >> 20) & 7) as u8)
    }
}
impl W {
    #[doc = "Bits 4:6 - Reference Current Control."]
    #[inline(always)]
    pub fn irefprog(&mut self) -> IREFPROG_W {
        IREFPROG_W::new(self)
    }
    #[doc = "Bits 8:10 - Current DAC and Reference Current Scale"]
    #[inline(always)]
    pub fn idacirefs(&mut self) -> IDACIREFS_W {
        IDACIREFS_W::new(self)
    }
    #[doc = "Bits 20:22 - Reset Timing"]
    #[inline(always)]
    pub fn trstprog(&mut self) -> TRSTPROG_W {
        TRSTPROG_W::new(self)
    }
    #[doc = "Writes raw bits to the register."]
    #[inline(always)]
    pub unsafe fn bits(&mut self, bits: u32) -> &mut Self {
        self.0.bits(bits);
        self
    }
}
#[doc = "Analog Control\n\nThis register you can [`read`](crate::generic::Reg::read), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [anactrl](index.html) module"]
pub struct ANACTRL_SPEC;
impl crate::RegisterSpec for ANACTRL_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [anactrl::R](R) reader structure"]
impl crate::Readable for ANACTRL_SPEC {
    type Reader = R;
}
#[doc = "`write(|w| ..)` method takes [anactrl::W](W) writer structure"]
impl crate::Writable for ANACTRL_SPEC {
    type Writer = W;
}
#[doc = "`reset()` method sets ANACTRL to value 0x70"]
impl crate::Resettable for ANACTRL_SPEC {
    #[inline(always)]
    fn reset_value() -> Self::Ux {
        0x70
    }
}
