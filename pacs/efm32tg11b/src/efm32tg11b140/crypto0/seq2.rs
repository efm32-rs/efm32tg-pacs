#[doc = "Register `SEQ2` reader"]
pub struct R(crate::R<SEQ2_SPEC>);
impl core::ops::Deref for R {
    type Target = crate::R<SEQ2_SPEC>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
impl From<crate::R<SEQ2_SPEC>> for R {
    #[inline(always)]
    fn from(reader: crate::R<SEQ2_SPEC>) -> Self {
        R(reader)
    }
}
#[doc = "Register `SEQ2` writer"]
pub struct W(crate::W<SEQ2_SPEC>);
impl core::ops::Deref for W {
    type Target = crate::W<SEQ2_SPEC>;
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
impl From<crate::W<SEQ2_SPEC>> for W {
    #[inline(always)]
    fn from(writer: crate::W<SEQ2_SPEC>) -> Self {
        W(writer)
    }
}
#[doc = "Field `INSTR8` reader - Sequence Instruction 8"]
pub type INSTR8_R = crate::FieldReader<u8, u8>;
#[doc = "Field `INSTR8` writer - Sequence Instruction 8"]
pub type INSTR8_W<'a, const O: u8> = crate::FieldWriter<'a, u32, SEQ2_SPEC, u8, u8, 8, O>;
#[doc = "Field `INSTR9` reader - Sequence Instruction 9"]
pub type INSTR9_R = crate::FieldReader<u8, u8>;
#[doc = "Field `INSTR9` writer - Sequence Instruction 9"]
pub type INSTR9_W<'a, const O: u8> = crate::FieldWriter<'a, u32, SEQ2_SPEC, u8, u8, 8, O>;
#[doc = "Field `INSTR10` reader - Sequence Instruction 10"]
pub type INSTR10_R = crate::FieldReader<u8, u8>;
#[doc = "Field `INSTR10` writer - Sequence Instruction 10"]
pub type INSTR10_W<'a, const O: u8> = crate::FieldWriter<'a, u32, SEQ2_SPEC, u8, u8, 8, O>;
#[doc = "Field `INSTR11` reader - Sequence Instruction 11"]
pub type INSTR11_R = crate::FieldReader<u8, u8>;
#[doc = "Field `INSTR11` writer - Sequence Instruction 11"]
pub type INSTR11_W<'a, const O: u8> = crate::FieldWriter<'a, u32, SEQ2_SPEC, u8, u8, 8, O>;
impl R {
    #[doc = "Bits 0:7 - Sequence Instruction 8"]
    #[inline(always)]
    pub fn instr8(&self) -> INSTR8_R {
        INSTR8_R::new((self.bits & 0xff) as u8)
    }
    #[doc = "Bits 8:15 - Sequence Instruction 9"]
    #[inline(always)]
    pub fn instr9(&self) -> INSTR9_R {
        INSTR9_R::new(((self.bits >> 8) & 0xff) as u8)
    }
    #[doc = "Bits 16:23 - Sequence Instruction 10"]
    #[inline(always)]
    pub fn instr10(&self) -> INSTR10_R {
        INSTR10_R::new(((self.bits >> 16) & 0xff) as u8)
    }
    #[doc = "Bits 24:31 - Sequence Instruction 11"]
    #[inline(always)]
    pub fn instr11(&self) -> INSTR11_R {
        INSTR11_R::new(((self.bits >> 24) & 0xff) as u8)
    }
}
impl W {
    #[doc = "Bits 0:7 - Sequence Instruction 8"]
    #[inline(always)]
    #[must_use]
    pub fn instr8(&mut self) -> INSTR8_W<0> {
        INSTR8_W::new(self)
    }
    #[doc = "Bits 8:15 - Sequence Instruction 9"]
    #[inline(always)]
    #[must_use]
    pub fn instr9(&mut self) -> INSTR9_W<8> {
        INSTR9_W::new(self)
    }
    #[doc = "Bits 16:23 - Sequence Instruction 10"]
    #[inline(always)]
    #[must_use]
    pub fn instr10(&mut self) -> INSTR10_W<16> {
        INSTR10_W::new(self)
    }
    #[doc = "Bits 24:31 - Sequence Instruction 11"]
    #[inline(always)]
    #[must_use]
    pub fn instr11(&mut self) -> INSTR11_W<24> {
        INSTR11_W::new(self)
    }
    #[doc = "Writes raw bits to the register."]
    #[inline(always)]
    pub unsafe fn bits(&mut self, bits: u32) -> &mut Self {
        self.0.bits(bits);
        self
    }
}
#[doc = "Sequence Register 2\n\nThis register you can [`read`](crate::generic::Reg::read), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [seq2](index.html) module"]
pub struct SEQ2_SPEC;
impl crate::RegisterSpec for SEQ2_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [seq2::R](R) reader structure"]
impl crate::Readable for SEQ2_SPEC {
    type Reader = R;
}
#[doc = "`write(|w| ..)` method takes [seq2::W](W) writer structure"]
impl crate::Writable for SEQ2_SPEC {
    type Writer = W;
    const ZERO_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
}
#[doc = "`reset()` method sets SEQ2 to value 0"]
impl crate::Resettable for SEQ2_SPEC {
    const RESET_VALUE: Self::Ux = 0;
}
