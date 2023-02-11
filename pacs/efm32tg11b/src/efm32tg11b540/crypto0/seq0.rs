#[doc = "Register `SEQ0` reader"]
pub struct R(crate::R<SEQ0_SPEC>);
impl core::ops::Deref for R {
    type Target = crate::R<SEQ0_SPEC>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
impl From<crate::R<SEQ0_SPEC>> for R {
    #[inline(always)]
    fn from(reader: crate::R<SEQ0_SPEC>) -> Self {
        R(reader)
    }
}
#[doc = "Register `SEQ0` writer"]
pub struct W(crate::W<SEQ0_SPEC>);
impl core::ops::Deref for W {
    type Target = crate::W<SEQ0_SPEC>;
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
impl From<crate::W<SEQ0_SPEC>> for W {
    #[inline(always)]
    fn from(writer: crate::W<SEQ0_SPEC>) -> Self {
        W(writer)
    }
}
#[doc = "Field `INSTR0` reader - Sequence Instruction 0"]
pub type INSTR0_R = crate::FieldReader<u8, u8>;
#[doc = "Field `INSTR0` writer - Sequence Instruction 0"]
pub type INSTR0_W<'a, const O: u8> = crate::FieldWriter<'a, u32, SEQ0_SPEC, u8, u8, 8, O>;
#[doc = "Field `INSTR1` reader - Sequence Instruction 1"]
pub type INSTR1_R = crate::FieldReader<u8, u8>;
#[doc = "Field `INSTR1` writer - Sequence Instruction 1"]
pub type INSTR1_W<'a, const O: u8> = crate::FieldWriter<'a, u32, SEQ0_SPEC, u8, u8, 8, O>;
#[doc = "Field `INSTR2` reader - Sequence Instruction 2"]
pub type INSTR2_R = crate::FieldReader<u8, u8>;
#[doc = "Field `INSTR2` writer - Sequence Instruction 2"]
pub type INSTR2_W<'a, const O: u8> = crate::FieldWriter<'a, u32, SEQ0_SPEC, u8, u8, 8, O>;
#[doc = "Field `INSTR3` reader - Sequence Instruction 3"]
pub type INSTR3_R = crate::FieldReader<u8, u8>;
#[doc = "Field `INSTR3` writer - Sequence Instruction 3"]
pub type INSTR3_W<'a, const O: u8> = crate::FieldWriter<'a, u32, SEQ0_SPEC, u8, u8, 8, O>;
impl R {
    #[doc = "Bits 0:7 - Sequence Instruction 0"]
    #[inline(always)]
    pub fn instr0(&self) -> INSTR0_R {
        INSTR0_R::new((self.bits & 0xff) as u8)
    }
    #[doc = "Bits 8:15 - Sequence Instruction 1"]
    #[inline(always)]
    pub fn instr1(&self) -> INSTR1_R {
        INSTR1_R::new(((self.bits >> 8) & 0xff) as u8)
    }
    #[doc = "Bits 16:23 - Sequence Instruction 2"]
    #[inline(always)]
    pub fn instr2(&self) -> INSTR2_R {
        INSTR2_R::new(((self.bits >> 16) & 0xff) as u8)
    }
    #[doc = "Bits 24:31 - Sequence Instruction 3"]
    #[inline(always)]
    pub fn instr3(&self) -> INSTR3_R {
        INSTR3_R::new(((self.bits >> 24) & 0xff) as u8)
    }
}
impl W {
    #[doc = "Bits 0:7 - Sequence Instruction 0"]
    #[inline(always)]
    #[must_use]
    pub fn instr0(&mut self) -> INSTR0_W<0> {
        INSTR0_W::new(self)
    }
    #[doc = "Bits 8:15 - Sequence Instruction 1"]
    #[inline(always)]
    #[must_use]
    pub fn instr1(&mut self) -> INSTR1_W<8> {
        INSTR1_W::new(self)
    }
    #[doc = "Bits 16:23 - Sequence Instruction 2"]
    #[inline(always)]
    #[must_use]
    pub fn instr2(&mut self) -> INSTR2_W<16> {
        INSTR2_W::new(self)
    }
    #[doc = "Bits 24:31 - Sequence Instruction 3"]
    #[inline(always)]
    #[must_use]
    pub fn instr3(&mut self) -> INSTR3_W<24> {
        INSTR3_W::new(self)
    }
    #[doc = "Writes raw bits to the register."]
    #[inline(always)]
    pub unsafe fn bits(&mut self, bits: u32) -> &mut Self {
        self.0.bits(bits);
        self
    }
}
#[doc = "Sequence Register 0\n\nThis register you can [`read`](crate::generic::Reg::read), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [seq0](index.html) module"]
pub struct SEQ0_SPEC;
impl crate::RegisterSpec for SEQ0_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [seq0::R](R) reader structure"]
impl crate::Readable for SEQ0_SPEC {
    type Reader = R;
}
#[doc = "`write(|w| ..)` method takes [seq0::W](W) writer structure"]
impl crate::Writable for SEQ0_SPEC {
    type Writer = W;
    const ZERO_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
}
#[doc = "`reset()` method sets SEQ0 to value 0"]
impl crate::Resettable for SEQ0_SPEC {
    const RESET_VALUE: Self::Ux = 0;
}
