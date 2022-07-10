#[doc = "Register `ST28_TCONFA` reader"]
pub struct R(crate::R<ST28_TCONFA_SPEC>);
impl core::ops::Deref for R {
    type Target = crate::R<ST28_TCONFA_SPEC>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
impl From<crate::R<ST28_TCONFA_SPEC>> for R {
    #[inline(always)]
    fn from(reader: crate::R<ST28_TCONFA_SPEC>) -> Self {
        R(reader)
    }
}
#[doc = "Register `ST28_TCONFA` writer"]
pub struct W(crate::W<ST28_TCONFA_SPEC>);
impl core::ops::Deref for W {
    type Target = crate::W<ST28_TCONFA_SPEC>;
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
impl From<crate::W<ST28_TCONFA_SPEC>> for W {
    #[inline(always)]
    fn from(writer: crate::W<ST28_TCONFA_SPEC>) -> Self {
        W(writer)
    }
}
#[doc = "Field `COMP` reader - Sensor Compare Value"]
pub type COMP_R = crate::FieldReader<u8, u8>;
#[doc = "Field `COMP` writer - Sensor Compare Value"]
pub type COMP_W<'a> = crate::FieldWriter<'a, u32, ST28_TCONFA_SPEC, u8, u8, 4, 0>;
#[doc = "Field `MASK` reader - Sensor Mask"]
pub type MASK_R = crate::FieldReader<u8, u8>;
#[doc = "Field `MASK` writer - Sensor Mask"]
pub type MASK_W<'a> = crate::FieldWriter<'a, u32, ST28_TCONFA_SPEC, u8, u8, 4, 4>;
#[doc = "Field `NEXTSTATE` reader - Next State Index"]
pub type NEXTSTATE_R = crate::FieldReader<u8, u8>;
#[doc = "Field `NEXTSTATE` writer - Next State Index"]
pub type NEXTSTATE_W<'a> = crate::FieldWriter<'a, u32, ST28_TCONFA_SPEC, u8, u8, 5, 8>;
#[doc = "Field `CHAIN` reader - Enable State Descriptor Chaining"]
pub type CHAIN_R = crate::BitReader<bool>;
#[doc = "Field `CHAIN` writer - Enable State Descriptor Chaining"]
pub type CHAIN_W<'a> = crate::BitWriter<'a, u32, ST28_TCONFA_SPEC, bool, 14>;
#[doc = "Field `SETIF` reader - Set Interrupt Flag Enable"]
pub type SETIF_R = crate::BitReader<bool>;
#[doc = "Field `SETIF` writer - Set Interrupt Flag Enable"]
pub type SETIF_W<'a> = crate::BitWriter<'a, u32, ST28_TCONFA_SPEC, bool, 15>;
#[doc = "Field `PRSACT` reader - Configure Transition Action"]
pub type PRSACT_R = crate::FieldReader<u8, u8>;
#[doc = "Field `PRSACT` writer - Configure Transition Action"]
pub type PRSACT_W<'a> = crate::FieldWriter<'a, u32, ST28_TCONFA_SPEC, u8, u8, 3, 16>;
impl R {
    #[doc = "Bits 0:3 - Sensor Compare Value"]
    #[inline(always)]
    pub fn comp(&self) -> COMP_R {
        COMP_R::new((self.bits & 0x0f) as u8)
    }
    #[doc = "Bits 4:7 - Sensor Mask"]
    #[inline(always)]
    pub fn mask(&self) -> MASK_R {
        MASK_R::new(((self.bits >> 4) & 0x0f) as u8)
    }
    #[doc = "Bits 8:12 - Next State Index"]
    #[inline(always)]
    pub fn nextstate(&self) -> NEXTSTATE_R {
        NEXTSTATE_R::new(((self.bits >> 8) & 0x1f) as u8)
    }
    #[doc = "Bit 14 - Enable State Descriptor Chaining"]
    #[inline(always)]
    pub fn chain(&self) -> CHAIN_R {
        CHAIN_R::new(((self.bits >> 14) & 1) != 0)
    }
    #[doc = "Bit 15 - Set Interrupt Flag Enable"]
    #[inline(always)]
    pub fn setif(&self) -> SETIF_R {
        SETIF_R::new(((self.bits >> 15) & 1) != 0)
    }
    #[doc = "Bits 16:18 - Configure Transition Action"]
    #[inline(always)]
    pub fn prsact(&self) -> PRSACT_R {
        PRSACT_R::new(((self.bits >> 16) & 7) as u8)
    }
}
impl W {
    #[doc = "Bits 0:3 - Sensor Compare Value"]
    #[inline(always)]
    pub fn comp(&mut self) -> COMP_W {
        COMP_W::new(self)
    }
    #[doc = "Bits 4:7 - Sensor Mask"]
    #[inline(always)]
    pub fn mask(&mut self) -> MASK_W {
        MASK_W::new(self)
    }
    #[doc = "Bits 8:12 - Next State Index"]
    #[inline(always)]
    pub fn nextstate(&mut self) -> NEXTSTATE_W {
        NEXTSTATE_W::new(self)
    }
    #[doc = "Bit 14 - Enable State Descriptor Chaining"]
    #[inline(always)]
    pub fn chain(&mut self) -> CHAIN_W {
        CHAIN_W::new(self)
    }
    #[doc = "Bit 15 - Set Interrupt Flag Enable"]
    #[inline(always)]
    pub fn setif(&mut self) -> SETIF_W {
        SETIF_W::new(self)
    }
    #[doc = "Bits 16:18 - Configure Transition Action"]
    #[inline(always)]
    pub fn prsact(&mut self) -> PRSACT_W {
        PRSACT_W::new(self)
    }
    #[doc = "Writes raw bits to the register."]
    #[inline(always)]
    pub unsafe fn bits(&mut self, bits: u32) -> &mut Self {
        self.0.bits(bits);
        self
    }
}
#[doc = "State Transition Configuration a\n\nThis register you can [`read`](crate::generic::Reg::read), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [st28_tconfa](index.html) module"]
pub struct ST28_TCONFA_SPEC;
impl crate::RegisterSpec for ST28_TCONFA_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [st28_tconfa::R](R) reader structure"]
impl crate::Readable for ST28_TCONFA_SPEC {
    type Reader = R;
}
#[doc = "`write(|w| ..)` method takes [st28_tconfa::W](W) writer structure"]
impl crate::Writable for ST28_TCONFA_SPEC {
    type Writer = W;
}
#[doc = "`reset()` method sets ST28_TCONFA to value 0"]
impl crate::Resettable for ST28_TCONFA_SPEC {
    #[inline(always)]
    fn reset_value() -> Self::Ux {
        0
    }
}
