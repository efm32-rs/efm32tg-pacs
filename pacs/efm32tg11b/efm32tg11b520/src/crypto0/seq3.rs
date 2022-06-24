#[doc = "Register `SEQ3` reader"]
pub struct R(crate::R<SEQ3_SPEC>);
impl core::ops::Deref for R {
    type Target = crate::R<SEQ3_SPEC>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
impl From<crate::R<SEQ3_SPEC>> for R {
    #[inline(always)]
    fn from(reader: crate::R<SEQ3_SPEC>) -> Self {
        R(reader)
    }
}
#[doc = "Register `SEQ3` writer"]
pub struct W(crate::W<SEQ3_SPEC>);
impl core::ops::Deref for W {
    type Target = crate::W<SEQ3_SPEC>;
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
impl From<crate::W<SEQ3_SPEC>> for W {
    #[inline(always)]
    fn from(writer: crate::W<SEQ3_SPEC>) -> Self {
        W(writer)
    }
}
#[doc = "Field `INSTR12` reader - Sequence Instruction 12"]
pub type INSTR12_R = crate::FieldReader<u8, u8>;
#[doc = "Field `INSTR12` writer - Sequence Instruction 12"]
pub type INSTR12_W<'a> = crate::FieldWriter<'a, u32, SEQ3_SPEC, u8, u8, 8, 0>;
#[doc = "Field `INSTR13` reader - Sequence Instruction 13"]
pub type INSTR13_R = crate::FieldReader<u8, u8>;
#[doc = "Field `INSTR13` writer - Sequence Instruction 13"]
pub type INSTR13_W<'a> = crate::FieldWriter<'a, u32, SEQ3_SPEC, u8, u8, 8, 8>;
#[doc = "Field `INSTR14` reader - Sequence Instruction 14"]
pub type INSTR14_R = crate::FieldReader<u8, u8>;
#[doc = "Field `INSTR14` writer - Sequence Instruction 14"]
pub type INSTR14_W<'a> = crate::FieldWriter<'a, u32, SEQ3_SPEC, u8, u8, 8, 16>;
#[doc = "Field `INSTR15` reader - Sequence Instruction 15"]
pub type INSTR15_R = crate::FieldReader<u8, u8>;
#[doc = "Field `INSTR15` writer - Sequence Instruction 15"]
pub type INSTR15_W<'a> = crate::FieldWriter<'a, u32, SEQ3_SPEC, u8, u8, 8, 24>;
impl R {
    #[doc = "Bits 0:7 - Sequence Instruction 12"]
    #[inline(always)]
    pub fn instr12(&self) -> INSTR12_R {
        INSTR12_R::new((self.bits & 0xff) as u8)
    }
    #[doc = "Bits 8:15 - Sequence Instruction 13"]
    #[inline(always)]
    pub fn instr13(&self) -> INSTR13_R {
        INSTR13_R::new(((self.bits >> 8) & 0xff) as u8)
    }
    #[doc = "Bits 16:23 - Sequence Instruction 14"]
    #[inline(always)]
    pub fn instr14(&self) -> INSTR14_R {
        INSTR14_R::new(((self.bits >> 16) & 0xff) as u8)
    }
    #[doc = "Bits 24:31 - Sequence Instruction 15"]
    #[inline(always)]
    pub fn instr15(&self) -> INSTR15_R {
        INSTR15_R::new(((self.bits >> 24) & 0xff) as u8)
    }
}
impl W {
    #[doc = "Bits 0:7 - Sequence Instruction 12"]
    #[inline(always)]
    pub fn instr12(&mut self) -> INSTR12_W {
        INSTR12_W::new(self)
    }
    #[doc = "Bits 8:15 - Sequence Instruction 13"]
    #[inline(always)]
    pub fn instr13(&mut self) -> INSTR13_W {
        INSTR13_W::new(self)
    }
    #[doc = "Bits 16:23 - Sequence Instruction 14"]
    #[inline(always)]
    pub fn instr14(&mut self) -> INSTR14_W {
        INSTR14_W::new(self)
    }
    #[doc = "Bits 24:31 - Sequence Instruction 15"]
    #[inline(always)]
    pub fn instr15(&mut self) -> INSTR15_W {
        INSTR15_W::new(self)
    }
    #[doc = "Writes raw bits to the register."]
    #[inline(always)]
    pub unsafe fn bits(&mut self, bits: u32) -> &mut Self {
        self.0.bits(bits);
        self
    }
}
#[doc = "Sequence Register 3\n\nThis register you can [`read`](crate::generic::Reg::read), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [seq3](index.html) module"]
pub struct SEQ3_SPEC;
impl crate::RegisterSpec for SEQ3_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [seq3::R](R) reader structure"]
impl crate::Readable for SEQ3_SPEC {
    type Reader = R;
}
#[doc = "`write(|w| ..)` method takes [seq3::W](W) writer structure"]
impl crate::Writable for SEQ3_SPEC {
    type Writer = W;
}
#[doc = "`reset()` method sets SEQ3 to value 0"]
impl crate::Resettable for SEQ3_SPEC {
    #[inline(always)]
    fn reset_value() -> Self::Ux {
        0
    }
}
