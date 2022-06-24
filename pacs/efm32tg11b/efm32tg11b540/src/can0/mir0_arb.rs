#[doc = "Register `MIR0_ARB` reader"]
pub struct R(crate::R<MIR0_ARB_SPEC>);
impl core::ops::Deref for R {
    type Target = crate::R<MIR0_ARB_SPEC>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
impl From<crate::R<MIR0_ARB_SPEC>> for R {
    #[inline(always)]
    fn from(reader: crate::R<MIR0_ARB_SPEC>) -> Self {
        R(reader)
    }
}
#[doc = "Register `MIR0_ARB` writer"]
pub struct W(crate::W<MIR0_ARB_SPEC>);
impl core::ops::Deref for W {
    type Target = crate::W<MIR0_ARB_SPEC>;
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
impl From<crate::W<MIR0_ARB_SPEC>> for W {
    #[inline(always)]
    fn from(writer: crate::W<MIR0_ARB_SPEC>) -> Self {
        W(writer)
    }
}
#[doc = "Field `ID` reader - Message Identifier"]
pub type ID_R = crate::FieldReader<u32, u32>;
#[doc = "Field `ID` writer - Message Identifier"]
pub type ID_W<'a> = crate::FieldWriter<'a, u32, MIR0_ARB_SPEC, u32, u32, 29, 0>;
#[doc = "Field `DIR` reader - Message Direction"]
pub type DIR_R = crate::BitReader<bool>;
#[doc = "Field `DIR` writer - Message Direction"]
pub type DIR_W<'a> = crate::BitWriter<'a, u32, MIR0_ARB_SPEC, bool, 29>;
#[doc = "Field `XTD` reader - Extended Identifier"]
pub type XTD_R = crate::BitReader<bool>;
#[doc = "Field `XTD` writer - Extended Identifier"]
pub type XTD_W<'a> = crate::BitWriter<'a, u32, MIR0_ARB_SPEC, bool, 30>;
#[doc = "Field `MSGVAL` reader - Message Valid"]
pub type MSGVAL_R = crate::BitReader<bool>;
#[doc = "Field `MSGVAL` writer - Message Valid"]
pub type MSGVAL_W<'a> = crate::BitWriter<'a, u32, MIR0_ARB_SPEC, bool, 31>;
impl R {
    #[doc = "Bits 0:28 - Message Identifier"]
    #[inline(always)]
    pub fn id(&self) -> ID_R {
        ID_R::new((self.bits & 0x1fff_ffff) as u32)
    }
    #[doc = "Bit 29 - Message Direction"]
    #[inline(always)]
    pub fn dir(&self) -> DIR_R {
        DIR_R::new(((self.bits >> 29) & 1) != 0)
    }
    #[doc = "Bit 30 - Extended Identifier"]
    #[inline(always)]
    pub fn xtd(&self) -> XTD_R {
        XTD_R::new(((self.bits >> 30) & 1) != 0)
    }
    #[doc = "Bit 31 - Message Valid"]
    #[inline(always)]
    pub fn msgval(&self) -> MSGVAL_R {
        MSGVAL_R::new(((self.bits >> 31) & 1) != 0)
    }
}
impl W {
    #[doc = "Bits 0:28 - Message Identifier"]
    #[inline(always)]
    pub fn id(&mut self) -> ID_W {
        ID_W::new(self)
    }
    #[doc = "Bit 29 - Message Direction"]
    #[inline(always)]
    pub fn dir(&mut self) -> DIR_W {
        DIR_W::new(self)
    }
    #[doc = "Bit 30 - Extended Identifier"]
    #[inline(always)]
    pub fn xtd(&mut self) -> XTD_W {
        XTD_W::new(self)
    }
    #[doc = "Bit 31 - Message Valid"]
    #[inline(always)]
    pub fn msgval(&mut self) -> MSGVAL_W {
        MSGVAL_W::new(self)
    }
    #[doc = "Writes raw bits to the register."]
    #[inline(always)]
    pub unsafe fn bits(&mut self, bits: u32) -> &mut Self {
        self.0.bits(bits);
        self
    }
}
#[doc = "Interface Arbitration Register\n\nThis register you can [`read`](crate::generic::Reg::read), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [mir0_arb](index.html) module"]
pub struct MIR0_ARB_SPEC;
impl crate::RegisterSpec for MIR0_ARB_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [mir0_arb::R](R) reader structure"]
impl crate::Readable for MIR0_ARB_SPEC {
    type Reader = R;
}
#[doc = "`write(|w| ..)` method takes [mir0_arb::W](W) writer structure"]
impl crate::Writable for MIR0_ARB_SPEC {
    type Writer = W;
}
#[doc = "`reset()` method sets MIR0_ARB to value 0"]
impl crate::Resettable for MIR0_ARB_SPEC {
    #[inline(always)]
    fn reset_value() -> Self::Ux {
        0
    }
}
