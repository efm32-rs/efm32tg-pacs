#[doc = "Register `MIR0_CMDREQ` reader"]
pub struct R(crate::R<MIR0_CMDREQ_SPEC>);
impl core::ops::Deref for R {
    type Target = crate::R<MIR0_CMDREQ_SPEC>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
impl From<crate::R<MIR0_CMDREQ_SPEC>> for R {
    #[inline(always)]
    fn from(reader: crate::R<MIR0_CMDREQ_SPEC>) -> Self {
        R(reader)
    }
}
#[doc = "Register `MIR0_CMDREQ` writer"]
pub struct W(crate::W<MIR0_CMDREQ_SPEC>);
impl core::ops::Deref for W {
    type Target = crate::W<MIR0_CMDREQ_SPEC>;
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
impl From<crate::W<MIR0_CMDREQ_SPEC>> for W {
    #[inline(always)]
    fn from(writer: crate::W<MIR0_CMDREQ_SPEC>) -> Self {
        W(writer)
    }
}
#[doc = "Field `MSGNUM` reader - Message Number"]
pub type MSGNUM_R = crate::FieldReader<u8, u8>;
#[doc = "Field `MSGNUM` writer - Message Number"]
pub type MSGNUM_W<'a> = crate::FieldWriter<'a, u32, MIR0_CMDREQ_SPEC, u8, u8, 6, 0>;
#[doc = "Field `BUSY` reader - Busy Flag"]
pub type BUSY_R = crate::BitReader<bool>;
impl R {
    #[doc = "Bits 0:5 - Message Number"]
    #[inline(always)]
    pub fn msgnum(&self) -> MSGNUM_R {
        MSGNUM_R::new((self.bits & 0x3f) as u8)
    }
    #[doc = "Bit 15 - Busy Flag"]
    #[inline(always)]
    pub fn busy(&self) -> BUSY_R {
        BUSY_R::new(((self.bits >> 15) & 1) != 0)
    }
}
impl W {
    #[doc = "Bits 0:5 - Message Number"]
    #[inline(always)]
    pub fn msgnum(&mut self) -> MSGNUM_W {
        MSGNUM_W::new(self)
    }
    #[doc = "Writes raw bits to the register."]
    #[inline(always)]
    pub unsafe fn bits(&mut self, bits: u32) -> &mut Self {
        self.0.bits(bits);
        self
    }
}
#[doc = "Interface Command Request Register\n\nThis register you can [`read`](crate::generic::Reg::read), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [mir0_cmdreq](index.html) module"]
pub struct MIR0_CMDREQ_SPEC;
impl crate::RegisterSpec for MIR0_CMDREQ_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [mir0_cmdreq::R](R) reader structure"]
impl crate::Readable for MIR0_CMDREQ_SPEC {
    type Reader = R;
}
#[doc = "`write(|w| ..)` method takes [mir0_cmdreq::W](W) writer structure"]
impl crate::Writable for MIR0_CMDREQ_SPEC {
    type Writer = W;
}
#[doc = "`reset()` method sets MIR0_CMDREQ to value 0x01"]
impl crate::Resettable for MIR0_CMDREQ_SPEC {
    #[inline(always)]
    fn reset_value() -> Self::Ux {
        0x01
    }
}
