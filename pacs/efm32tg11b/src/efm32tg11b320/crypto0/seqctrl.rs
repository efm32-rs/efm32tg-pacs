#[doc = "Register `SEQCTRL` reader"]
pub struct R(crate::R<SEQCTRL_SPEC>);
impl core::ops::Deref for R {
    type Target = crate::R<SEQCTRL_SPEC>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
impl From<crate::R<SEQCTRL_SPEC>> for R {
    #[inline(always)]
    fn from(reader: crate::R<SEQCTRL_SPEC>) -> Self {
        R(reader)
    }
}
#[doc = "Register `SEQCTRL` writer"]
pub struct W(crate::W<SEQCTRL_SPEC>);
impl core::ops::Deref for W {
    type Target = crate::W<SEQCTRL_SPEC>;
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
impl From<crate::W<SEQCTRL_SPEC>> for W {
    #[inline(always)]
    fn from(writer: crate::W<SEQCTRL_SPEC>) -> Self {
        W(writer)
    }
}
#[doc = "Field `LENGTHA` reader - Buffer Length a in Bytes"]
pub type LENGTHA_R = crate::FieldReader<u16, u16>;
#[doc = "Field `LENGTHA` writer - Buffer Length a in Bytes"]
pub type LENGTHA_W<'a, const O: u8> = crate::FieldWriter<'a, u32, SEQCTRL_SPEC, u16, u16, 14, O>;
#[doc = "Field `BLOCKSIZE` reader - Size of Data Blocks"]
pub type BLOCKSIZE_R = crate::FieldReader<u8, BLOCKSIZE_A>;
#[doc = "Size of Data Blocks\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
#[repr(u8)]
pub enum BLOCKSIZE_A {
    #[doc = "0: A block is 16 bytes long"]
    _16BYTES = 0,
    #[doc = "1: A block is 32 bytes long"]
    _32BYTES = 1,
    #[doc = "2: A block is 64 bytes long"]
    _64BYTES = 2,
}
impl From<BLOCKSIZE_A> for u8 {
    #[inline(always)]
    fn from(variant: BLOCKSIZE_A) -> Self {
        variant as _
    }
}
impl BLOCKSIZE_R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> Option<BLOCKSIZE_A> {
        match self.bits {
            0 => Some(BLOCKSIZE_A::_16BYTES),
            1 => Some(BLOCKSIZE_A::_32BYTES),
            2 => Some(BLOCKSIZE_A::_64BYTES),
            _ => None,
        }
    }
    #[doc = "Checks if the value of the field is `_16BYTES`"]
    #[inline(always)]
    pub fn is_16bytes(&self) -> bool {
        *self == BLOCKSIZE_A::_16BYTES
    }
    #[doc = "Checks if the value of the field is `_32BYTES`"]
    #[inline(always)]
    pub fn is_32bytes(&self) -> bool {
        *self == BLOCKSIZE_A::_32BYTES
    }
    #[doc = "Checks if the value of the field is `_64BYTES`"]
    #[inline(always)]
    pub fn is_64bytes(&self) -> bool {
        *self == BLOCKSIZE_A::_64BYTES
    }
}
#[doc = "Field `BLOCKSIZE` writer - Size of Data Blocks"]
pub type BLOCKSIZE_W<'a, const O: u8> =
    crate::FieldWriter<'a, u32, SEQCTRL_SPEC, u8, BLOCKSIZE_A, 2, O>;
impl<'a, const O: u8> BLOCKSIZE_W<'a, O> {
    #[doc = "A block is 16 bytes long"]
    #[inline(always)]
    pub fn _16bytes(self) -> &'a mut W {
        self.variant(BLOCKSIZE_A::_16BYTES)
    }
    #[doc = "A block is 32 bytes long"]
    #[inline(always)]
    pub fn _32bytes(self) -> &'a mut W {
        self.variant(BLOCKSIZE_A::_32BYTES)
    }
    #[doc = "A block is 64 bytes long"]
    #[inline(always)]
    pub fn _64bytes(self) -> &'a mut W {
        self.variant(BLOCKSIZE_A::_64BYTES)
    }
}
#[doc = "Field `DMA0SKIP` reader - DMA0 Skip"]
pub type DMA0SKIP_R = crate::FieldReader<u8, u8>;
#[doc = "Field `DMA0SKIP` writer - DMA0 Skip"]
pub type DMA0SKIP_W<'a, const O: u8> = crate::FieldWriter<'a, u32, SEQCTRL_SPEC, u8, u8, 2, O>;
#[doc = "Field `DMA1SKIP` reader - DMA1 Skip"]
pub type DMA1SKIP_R = crate::FieldReader<u8, u8>;
#[doc = "Field `DMA1SKIP` writer - DMA1 Skip"]
pub type DMA1SKIP_W<'a, const O: u8> = crate::FieldWriter<'a, u32, SEQCTRL_SPEC, u8, u8, 2, O>;
#[doc = "Field `DMA0PRESA` reader - DMA0 Preserve a"]
pub type DMA0PRESA_R = crate::BitReader<bool>;
#[doc = "Field `DMA0PRESA` writer - DMA0 Preserve a"]
pub type DMA0PRESA_W<'a, const O: u8> = crate::BitWriter<'a, u32, SEQCTRL_SPEC, bool, O>;
#[doc = "Field `DMA1PRESA` reader - DMA1 Preserve a"]
pub type DMA1PRESA_R = crate::BitReader<bool>;
#[doc = "Field `DMA1PRESA` writer - DMA1 Preserve a"]
pub type DMA1PRESA_W<'a, const O: u8> = crate::BitWriter<'a, u32, SEQCTRL_SPEC, bool, O>;
#[doc = "Field `HALT` reader - Halt Sequence"]
pub type HALT_R = crate::BitReader<bool>;
#[doc = "Field `HALT` writer - Halt Sequence"]
pub type HALT_W<'a, const O: u8> = crate::BitWriter<'a, u32, SEQCTRL_SPEC, bool, O>;
impl R {
    #[doc = "Bits 0:13 - Buffer Length a in Bytes"]
    #[inline(always)]
    pub fn lengtha(&self) -> LENGTHA_R {
        LENGTHA_R::new((self.bits & 0x3fff) as u16)
    }
    #[doc = "Bits 20:21 - Size of Data Blocks"]
    #[inline(always)]
    pub fn blocksize(&self) -> BLOCKSIZE_R {
        BLOCKSIZE_R::new(((self.bits >> 20) & 3) as u8)
    }
    #[doc = "Bits 24:25 - DMA0 Skip"]
    #[inline(always)]
    pub fn dma0skip(&self) -> DMA0SKIP_R {
        DMA0SKIP_R::new(((self.bits >> 24) & 3) as u8)
    }
    #[doc = "Bits 26:27 - DMA1 Skip"]
    #[inline(always)]
    pub fn dma1skip(&self) -> DMA1SKIP_R {
        DMA1SKIP_R::new(((self.bits >> 26) & 3) as u8)
    }
    #[doc = "Bit 28 - DMA0 Preserve a"]
    #[inline(always)]
    pub fn dma0presa(&self) -> DMA0PRESA_R {
        DMA0PRESA_R::new(((self.bits >> 28) & 1) != 0)
    }
    #[doc = "Bit 29 - DMA1 Preserve a"]
    #[inline(always)]
    pub fn dma1presa(&self) -> DMA1PRESA_R {
        DMA1PRESA_R::new(((self.bits >> 29) & 1) != 0)
    }
    #[doc = "Bit 31 - Halt Sequence"]
    #[inline(always)]
    pub fn halt(&self) -> HALT_R {
        HALT_R::new(((self.bits >> 31) & 1) != 0)
    }
}
impl W {
    #[doc = "Bits 0:13 - Buffer Length a in Bytes"]
    #[inline(always)]
    #[must_use]
    pub fn lengtha(&mut self) -> LENGTHA_W<0> {
        LENGTHA_W::new(self)
    }
    #[doc = "Bits 20:21 - Size of Data Blocks"]
    #[inline(always)]
    #[must_use]
    pub fn blocksize(&mut self) -> BLOCKSIZE_W<20> {
        BLOCKSIZE_W::new(self)
    }
    #[doc = "Bits 24:25 - DMA0 Skip"]
    #[inline(always)]
    #[must_use]
    pub fn dma0skip(&mut self) -> DMA0SKIP_W<24> {
        DMA0SKIP_W::new(self)
    }
    #[doc = "Bits 26:27 - DMA1 Skip"]
    #[inline(always)]
    #[must_use]
    pub fn dma1skip(&mut self) -> DMA1SKIP_W<26> {
        DMA1SKIP_W::new(self)
    }
    #[doc = "Bit 28 - DMA0 Preserve a"]
    #[inline(always)]
    #[must_use]
    pub fn dma0presa(&mut self) -> DMA0PRESA_W<28> {
        DMA0PRESA_W::new(self)
    }
    #[doc = "Bit 29 - DMA1 Preserve a"]
    #[inline(always)]
    #[must_use]
    pub fn dma1presa(&mut self) -> DMA1PRESA_W<29> {
        DMA1PRESA_W::new(self)
    }
    #[doc = "Bit 31 - Halt Sequence"]
    #[inline(always)]
    #[must_use]
    pub fn halt(&mut self) -> HALT_W<31> {
        HALT_W::new(self)
    }
    #[doc = "Writes raw bits to the register."]
    #[inline(always)]
    pub unsafe fn bits(&mut self, bits: u32) -> &mut Self {
        self.0.bits(bits);
        self
    }
}
#[doc = "Sequence Control\n\nThis register you can [`read`](crate::generic::Reg::read), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [seqctrl](index.html) module"]
pub struct SEQCTRL_SPEC;
impl crate::RegisterSpec for SEQCTRL_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [seqctrl::R](R) reader structure"]
impl crate::Readable for SEQCTRL_SPEC {
    type Reader = R;
}
#[doc = "`write(|w| ..)` method takes [seqctrl::W](W) writer structure"]
impl crate::Writable for SEQCTRL_SPEC {
    type Writer = W;
    const ZERO_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
}
#[doc = "`reset()` method sets SEQCTRL to value 0"]
impl crate::Resettable for SEQCTRL_SPEC {
    const RESET_VALUE: Self::Ux = 0;
}
