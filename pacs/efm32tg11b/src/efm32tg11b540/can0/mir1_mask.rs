#[doc = "Register `MIR1_MASK` reader"]
pub struct R(crate::R<MIR1_MASK_SPEC>);
impl core::ops::Deref for R {
    type Target = crate::R<MIR1_MASK_SPEC>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
impl From<crate::R<MIR1_MASK_SPEC>> for R {
    #[inline(always)]
    fn from(reader: crate::R<MIR1_MASK_SPEC>) -> Self {
        R(reader)
    }
}
#[doc = "Register `MIR1_MASK` writer"]
pub struct W(crate::W<MIR1_MASK_SPEC>);
impl core::ops::Deref for W {
    type Target = crate::W<MIR1_MASK_SPEC>;
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
impl From<crate::W<MIR1_MASK_SPEC>> for W {
    #[inline(always)]
    fn from(writer: crate::W<MIR1_MASK_SPEC>) -> Self {
        W(writer)
    }
}
#[doc = "Field `MASK` reader - Identifier Mask"]
pub type MASK_R = crate::FieldReader<u32, u32>;
#[doc = "Field `MASK` writer - Identifier Mask"]
pub type MASK_W<'a, const O: u8> = crate::FieldWriter<'a, u32, MIR1_MASK_SPEC, u32, u32, 29, O>;
#[doc = "Field `MDIR` reader - Mask Message Direction"]
pub type MDIR_R = crate::BitReader<bool>;
#[doc = "Field `MDIR` writer - Mask Message Direction"]
pub type MDIR_W<'a, const O: u8> = crate::BitWriter<'a, u32, MIR1_MASK_SPEC, bool, O>;
#[doc = "Field `MXTD` reader - Mask Extended Identifier"]
pub type MXTD_R = crate::BitReader<bool>;
#[doc = "Field `MXTD` writer - Mask Extended Identifier"]
pub type MXTD_W<'a, const O: u8> = crate::BitWriter<'a, u32, MIR1_MASK_SPEC, bool, O>;
impl R {
    #[doc = "Bits 0:28 - Identifier Mask"]
    #[inline(always)]
    pub fn mask(&self) -> MASK_R {
        MASK_R::new(self.bits & 0x1fff_ffff)
    }
    #[doc = "Bit 30 - Mask Message Direction"]
    #[inline(always)]
    pub fn mdir(&self) -> MDIR_R {
        MDIR_R::new(((self.bits >> 30) & 1) != 0)
    }
    #[doc = "Bit 31 - Mask Extended Identifier"]
    #[inline(always)]
    pub fn mxtd(&self) -> MXTD_R {
        MXTD_R::new(((self.bits >> 31) & 1) != 0)
    }
}
impl W {
    #[doc = "Bits 0:28 - Identifier Mask"]
    #[inline(always)]
    #[must_use]
    pub fn mask(&mut self) -> MASK_W<0> {
        MASK_W::new(self)
    }
    #[doc = "Bit 30 - Mask Message Direction"]
    #[inline(always)]
    #[must_use]
    pub fn mdir(&mut self) -> MDIR_W<30> {
        MDIR_W::new(self)
    }
    #[doc = "Bit 31 - Mask Extended Identifier"]
    #[inline(always)]
    #[must_use]
    pub fn mxtd(&mut self) -> MXTD_W<31> {
        MXTD_W::new(self)
    }
    #[doc = "Writes raw bits to the register."]
    #[inline(always)]
    pub unsafe fn bits(&mut self, bits: u32) -> &mut Self {
        self.0.bits(bits);
        self
    }
}
#[doc = "Interface Mask Register\n\nThis register you can [`read`](crate::generic::Reg::read), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [mir1_mask](index.html) module"]
pub struct MIR1_MASK_SPEC;
impl crate::RegisterSpec for MIR1_MASK_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [mir1_mask::R](R) reader structure"]
impl crate::Readable for MIR1_MASK_SPEC {
    type Reader = R;
}
#[doc = "`write(|w| ..)` method takes [mir1_mask::W](W) writer structure"]
impl crate::Writable for MIR1_MASK_SPEC {
    type Writer = W;
    const ZERO_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
}
#[doc = "`reset()` method sets MIR1_MASK to value 0xdfff_ffff"]
impl crate::Resettable for MIR1_MASK_SPEC {
    const RESET_VALUE: Self::Ux = 0xdfff_ffff;
}
