#[doc = "Register `CMD` reader"]
pub struct R(crate::R<CMD_SPEC>);
impl core::ops::Deref for R {
    type Target = crate::R<CMD_SPEC>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
impl From<crate::R<CMD_SPEC>> for R {
    #[inline(always)]
    fn from(reader: crate::R<CMD_SPEC>) -> Self {
        R(reader)
    }
}
#[doc = "Register `CMD` writer"]
pub struct W(crate::W<CMD_SPEC>);
impl core::ops::Deref for W {
    type Target = crate::W<CMD_SPEC>;
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
impl From<crate::W<CMD_SPEC>> for W {
    #[inline(always)]
    fn from(writer: crate::W<CMD_SPEC>) -> Self {
        W(writer)
    }
}
#[doc = "Field `INSTR` reader - Execute Instruction"]
pub type INSTR_R = crate::FieldReader<u8, u8>;
#[doc = "Field `INSTR` writer - Execute Instruction"]
pub type INSTR_W<'a> = crate::FieldWriter<'a, u32, CMD_SPEC, u8, u8, 8, 0>;
#[doc = "Field `SEQSTART` writer - Encryption/Decryption SEQUENCE Start"]
pub type SEQSTART_W<'a> = crate::BitWriter<'a, u32, CMD_SPEC, bool, 9>;
#[doc = "Field `SEQSTOP` writer - Sequence Stop"]
pub type SEQSTOP_W<'a> = crate::BitWriter<'a, u32, CMD_SPEC, bool, 10>;
#[doc = "Field `SEQSTEP` writer - Sequence Step"]
pub type SEQSTEP_W<'a> = crate::BitWriter<'a, u32, CMD_SPEC, bool, 11>;
impl R {
    #[doc = "Bits 0:7 - Execute Instruction"]
    #[inline(always)]
    pub fn instr(&self) -> INSTR_R {
        INSTR_R::new((self.bits & 0xff) as u8)
    }
}
impl W {
    #[doc = "Bits 0:7 - Execute Instruction"]
    #[inline(always)]
    pub fn instr(&mut self) -> INSTR_W {
        INSTR_W::new(self)
    }
    #[doc = "Bit 9 - Encryption/Decryption SEQUENCE Start"]
    #[inline(always)]
    pub fn seqstart(&mut self) -> SEQSTART_W {
        SEQSTART_W::new(self)
    }
    #[doc = "Bit 10 - Sequence Stop"]
    #[inline(always)]
    pub fn seqstop(&mut self) -> SEQSTOP_W {
        SEQSTOP_W::new(self)
    }
    #[doc = "Bit 11 - Sequence Step"]
    #[inline(always)]
    pub fn seqstep(&mut self) -> SEQSTEP_W {
        SEQSTEP_W::new(self)
    }
    #[doc = "Writes raw bits to the register."]
    #[inline(always)]
    pub unsafe fn bits(&mut self, bits: u32) -> &mut Self {
        self.0.bits(bits);
        self
    }
}
#[doc = "Command Register\n\nThis register you can [`read`](crate::generic::Reg::read), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [cmd](index.html) module"]
pub struct CMD_SPEC;
impl crate::RegisterSpec for CMD_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [cmd::R](R) reader structure"]
impl crate::Readable for CMD_SPEC {
    type Reader = R;
}
#[doc = "`write(|w| ..)` method takes [cmd::W](W) writer structure"]
impl crate::Writable for CMD_SPEC {
    type Writer = W;
}
#[doc = "`reset()` method sets CMD to value 0"]
impl crate::Resettable for CMD_SPEC {
    #[inline(always)]
    fn reset_value() -> Self::Ux {
        0
    }
}
