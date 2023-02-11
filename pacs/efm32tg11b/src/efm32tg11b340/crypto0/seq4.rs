#[doc = "Register `SEQ4` reader"]
pub struct R(crate::R<SEQ4_SPEC>);
impl core::ops::Deref for R {
    type Target = crate::R<SEQ4_SPEC>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
impl From<crate::R<SEQ4_SPEC>> for R {
    #[inline(always)]
    fn from(reader: crate::R<SEQ4_SPEC>) -> Self {
        R(reader)
    }
}
#[doc = "Register `SEQ4` writer"]
pub struct W(crate::W<SEQ4_SPEC>);
impl core::ops::Deref for W {
    type Target = crate::W<SEQ4_SPEC>;
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
impl From<crate::W<SEQ4_SPEC>> for W {
    #[inline(always)]
    fn from(writer: crate::W<SEQ4_SPEC>) -> Self {
        W(writer)
    }
}
#[doc = "Field `INSTR16` reader - Sequence Instruction 16"]
pub type INSTR16_R = crate::FieldReader<u8, u8>;
#[doc = "Field `INSTR16` writer - Sequence Instruction 16"]
pub type INSTR16_W<'a, const O: u8> = crate::FieldWriter<'a, u32, SEQ4_SPEC, u8, u8, 8, O>;
#[doc = "Field `INSTR17` reader - Sequence Instruction 17"]
pub type INSTR17_R = crate::FieldReader<u8, u8>;
#[doc = "Field `INSTR17` writer - Sequence Instruction 17"]
pub type INSTR17_W<'a, const O: u8> = crate::FieldWriter<'a, u32, SEQ4_SPEC, u8, u8, 8, O>;
#[doc = "Field `INSTR18` reader - Sequence Instruction 18"]
pub type INSTR18_R = crate::FieldReader<u8, u8>;
#[doc = "Field `INSTR18` writer - Sequence Instruction 18"]
pub type INSTR18_W<'a, const O: u8> = crate::FieldWriter<'a, u32, SEQ4_SPEC, u8, u8, 8, O>;
#[doc = "Field `INSTR19` reader - Sequence Instruction 19"]
pub type INSTR19_R = crate::FieldReader<u8, u8>;
#[doc = "Field `INSTR19` writer - Sequence Instruction 19"]
pub type INSTR19_W<'a, const O: u8> = crate::FieldWriter<'a, u32, SEQ4_SPEC, u8, u8, 8, O>;
impl R {
    #[doc = "Bits 0:7 - Sequence Instruction 16"]
    #[inline(always)]
    pub fn instr16(&self) -> INSTR16_R {
        INSTR16_R::new((self.bits & 0xff) as u8)
    }
    #[doc = "Bits 8:15 - Sequence Instruction 17"]
    #[inline(always)]
    pub fn instr17(&self) -> INSTR17_R {
        INSTR17_R::new(((self.bits >> 8) & 0xff) as u8)
    }
    #[doc = "Bits 16:23 - Sequence Instruction 18"]
    #[inline(always)]
    pub fn instr18(&self) -> INSTR18_R {
        INSTR18_R::new(((self.bits >> 16) & 0xff) as u8)
    }
    #[doc = "Bits 24:31 - Sequence Instruction 19"]
    #[inline(always)]
    pub fn instr19(&self) -> INSTR19_R {
        INSTR19_R::new(((self.bits >> 24) & 0xff) as u8)
    }
}
impl W {
    #[doc = "Bits 0:7 - Sequence Instruction 16"]
    #[inline(always)]
    #[must_use]
    pub fn instr16(&mut self) -> INSTR16_W<0> {
        INSTR16_W::new(self)
    }
    #[doc = "Bits 8:15 - Sequence Instruction 17"]
    #[inline(always)]
    #[must_use]
    pub fn instr17(&mut self) -> INSTR17_W<8> {
        INSTR17_W::new(self)
    }
    #[doc = "Bits 16:23 - Sequence Instruction 18"]
    #[inline(always)]
    #[must_use]
    pub fn instr18(&mut self) -> INSTR18_W<16> {
        INSTR18_W::new(self)
    }
    #[doc = "Bits 24:31 - Sequence Instruction 19"]
    #[inline(always)]
    #[must_use]
    pub fn instr19(&mut self) -> INSTR19_W<24> {
        INSTR19_W::new(self)
    }
    #[doc = "Writes raw bits to the register."]
    #[inline(always)]
    pub unsafe fn bits(&mut self, bits: u32) -> &mut Self {
        self.0.bits(bits);
        self
    }
}
#[doc = "Sequence Register 4\n\nThis register you can [`read`](crate::generic::Reg::read), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [seq4](index.html) module"]
pub struct SEQ4_SPEC;
impl crate::RegisterSpec for SEQ4_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [seq4::R](R) reader structure"]
impl crate::Readable for SEQ4_SPEC {
    type Reader = R;
}
#[doc = "`write(|w| ..)` method takes [seq4::W](W) writer structure"]
impl crate::Writable for SEQ4_SPEC {
    type Writer = W;
    const ZERO_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
}
#[doc = "`reset()` method sets SEQ4 to value 0"]
impl crate::Resettable for SEQ4_SPEC {
    const RESET_VALUE: Self::Ux = 0;
}
