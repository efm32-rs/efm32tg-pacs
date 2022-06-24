#[doc = "Register `SEQ1` reader"]
pub struct R(crate::R<SEQ1_SPEC>);
impl core::ops::Deref for R {
    type Target = crate::R<SEQ1_SPEC>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
impl From<crate::R<SEQ1_SPEC>> for R {
    #[inline(always)]
    fn from(reader: crate::R<SEQ1_SPEC>) -> Self {
        R(reader)
    }
}
#[doc = "Register `SEQ1` writer"]
pub struct W(crate::W<SEQ1_SPEC>);
impl core::ops::Deref for W {
    type Target = crate::W<SEQ1_SPEC>;
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
impl From<crate::W<SEQ1_SPEC>> for W {
    #[inline(always)]
    fn from(writer: crate::W<SEQ1_SPEC>) -> Self {
        W(writer)
    }
}
#[doc = "Field `INSTR4` reader - Sequence Instruction 4"]
pub type INSTR4_R = crate::FieldReader<u8, u8>;
#[doc = "Field `INSTR4` writer - Sequence Instruction 4"]
pub type INSTR4_W<'a> = crate::FieldWriter<'a, u32, SEQ1_SPEC, u8, u8, 8, 0>;
#[doc = "Field `INSTR5` reader - Sequence Instruction 5"]
pub type INSTR5_R = crate::FieldReader<u8, u8>;
#[doc = "Field `INSTR5` writer - Sequence Instruction 5"]
pub type INSTR5_W<'a> = crate::FieldWriter<'a, u32, SEQ1_SPEC, u8, u8, 8, 8>;
#[doc = "Field `INSTR6` reader - Sequence Instruction 6"]
pub type INSTR6_R = crate::FieldReader<u8, u8>;
#[doc = "Field `INSTR6` writer - Sequence Instruction 6"]
pub type INSTR6_W<'a> = crate::FieldWriter<'a, u32, SEQ1_SPEC, u8, u8, 8, 16>;
#[doc = "Field `INSTR7` reader - Sequence Instruction 7"]
pub type INSTR7_R = crate::FieldReader<u8, u8>;
#[doc = "Field `INSTR7` writer - Sequence Instruction 7"]
pub type INSTR7_W<'a> = crate::FieldWriter<'a, u32, SEQ1_SPEC, u8, u8, 8, 24>;
impl R {
    #[doc = "Bits 0:7 - Sequence Instruction 4"]
    #[inline(always)]
    pub fn instr4(&self) -> INSTR4_R {
        INSTR4_R::new((self.bits & 0xff) as u8)
    }
    #[doc = "Bits 8:15 - Sequence Instruction 5"]
    #[inline(always)]
    pub fn instr5(&self) -> INSTR5_R {
        INSTR5_R::new(((self.bits >> 8) & 0xff) as u8)
    }
    #[doc = "Bits 16:23 - Sequence Instruction 6"]
    #[inline(always)]
    pub fn instr6(&self) -> INSTR6_R {
        INSTR6_R::new(((self.bits >> 16) & 0xff) as u8)
    }
    #[doc = "Bits 24:31 - Sequence Instruction 7"]
    #[inline(always)]
    pub fn instr7(&self) -> INSTR7_R {
        INSTR7_R::new(((self.bits >> 24) & 0xff) as u8)
    }
}
impl W {
    #[doc = "Bits 0:7 - Sequence Instruction 4"]
    #[inline(always)]
    pub fn instr4(&mut self) -> INSTR4_W {
        INSTR4_W::new(self)
    }
    #[doc = "Bits 8:15 - Sequence Instruction 5"]
    #[inline(always)]
    pub fn instr5(&mut self) -> INSTR5_W {
        INSTR5_W::new(self)
    }
    #[doc = "Bits 16:23 - Sequence Instruction 6"]
    #[inline(always)]
    pub fn instr6(&mut self) -> INSTR6_W {
        INSTR6_W::new(self)
    }
    #[doc = "Bits 24:31 - Sequence Instruction 7"]
    #[inline(always)]
    pub fn instr7(&mut self) -> INSTR7_W {
        INSTR7_W::new(self)
    }
    #[doc = "Writes raw bits to the register."]
    #[inline(always)]
    pub unsafe fn bits(&mut self, bits: u32) -> &mut Self {
        self.0.bits(bits);
        self
    }
}
#[doc = "Sequence Register 1\n\nThis register you can [`read`](crate::generic::Reg::read), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [seq1](index.html) module"]
pub struct SEQ1_SPEC;
impl crate::RegisterSpec for SEQ1_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [seq1::R](R) reader structure"]
impl crate::Readable for SEQ1_SPEC {
    type Reader = R;
}
#[doc = "`write(|w| ..)` method takes [seq1::W](W) writer structure"]
impl crate::Writable for SEQ1_SPEC {
    type Writer = W;
}
#[doc = "`reset()` method sets SEQ1 to value 0"]
impl crate::Resettable for SEQ1_SPEC {
    #[inline(always)]
    fn reset_value() -> Self::Ux {
        0
    }
}
