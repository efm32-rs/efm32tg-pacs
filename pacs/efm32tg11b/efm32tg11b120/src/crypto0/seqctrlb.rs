#[doc = "Register `SEQCTRLB` reader"]
pub struct R(crate::R<SEQCTRLB_SPEC>);
impl core::ops::Deref for R {
    type Target = crate::R<SEQCTRLB_SPEC>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
impl From<crate::R<SEQCTRLB_SPEC>> for R {
    #[inline(always)]
    fn from(reader: crate::R<SEQCTRLB_SPEC>) -> Self {
        R(reader)
    }
}
#[doc = "Register `SEQCTRLB` writer"]
pub struct W(crate::W<SEQCTRLB_SPEC>);
impl core::ops::Deref for W {
    type Target = crate::W<SEQCTRLB_SPEC>;
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
impl From<crate::W<SEQCTRLB_SPEC>> for W {
    #[inline(always)]
    fn from(writer: crate::W<SEQCTRLB_SPEC>) -> Self {
        W(writer)
    }
}
#[doc = "Field `LENGTHB` reader - Buffer Length B in Bytes"]
pub type LENGTHB_R = crate::FieldReader<u16, u16>;
#[doc = "Field `LENGTHB` writer - Buffer Length B in Bytes"]
pub type LENGTHB_W<'a> = crate::FieldWriter<'a, u32, SEQCTRLB_SPEC, u16, u16, 14, 0>;
#[doc = "Field `DMA0PRESB` reader - DMA0 Preserve B"]
pub type DMA0PRESB_R = crate::BitReader<bool>;
#[doc = "Field `DMA0PRESB` writer - DMA0 Preserve B"]
pub type DMA0PRESB_W<'a> = crate::BitWriter<'a, u32, SEQCTRLB_SPEC, bool, 28>;
#[doc = "Field `DMA1PRESB` reader - DMA1 Preserve B"]
pub type DMA1PRESB_R = crate::BitReader<bool>;
#[doc = "Field `DMA1PRESB` writer - DMA1 Preserve B"]
pub type DMA1PRESB_W<'a> = crate::BitWriter<'a, u32, SEQCTRLB_SPEC, bool, 29>;
impl R {
    #[doc = "Bits 0:13 - Buffer Length B in Bytes"]
    #[inline(always)]
    pub fn lengthb(&self) -> LENGTHB_R {
        LENGTHB_R::new((self.bits & 0x3fff) as u16)
    }
    #[doc = "Bit 28 - DMA0 Preserve B"]
    #[inline(always)]
    pub fn dma0presb(&self) -> DMA0PRESB_R {
        DMA0PRESB_R::new(((self.bits >> 28) & 1) != 0)
    }
    #[doc = "Bit 29 - DMA1 Preserve B"]
    #[inline(always)]
    pub fn dma1presb(&self) -> DMA1PRESB_R {
        DMA1PRESB_R::new(((self.bits >> 29) & 1) != 0)
    }
}
impl W {
    #[doc = "Bits 0:13 - Buffer Length B in Bytes"]
    #[inline(always)]
    pub fn lengthb(&mut self) -> LENGTHB_W {
        LENGTHB_W::new(self)
    }
    #[doc = "Bit 28 - DMA0 Preserve B"]
    #[inline(always)]
    pub fn dma0presb(&mut self) -> DMA0PRESB_W {
        DMA0PRESB_W::new(self)
    }
    #[doc = "Bit 29 - DMA1 Preserve B"]
    #[inline(always)]
    pub fn dma1presb(&mut self) -> DMA1PRESB_W {
        DMA1PRESB_W::new(self)
    }
    #[doc = "Writes raw bits to the register."]
    #[inline(always)]
    pub unsafe fn bits(&mut self, bits: u32) -> &mut Self {
        self.0.bits(bits);
        self
    }
}
#[doc = "Sequence Control B\n\nThis register you can [`read`](crate::generic::Reg::read), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [seqctrlb](index.html) module"]
pub struct SEQCTRLB_SPEC;
impl crate::RegisterSpec for SEQCTRLB_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [seqctrlb::R](R) reader structure"]
impl crate::Readable for SEQCTRLB_SPEC {
    type Reader = R;
}
#[doc = "`write(|w| ..)` method takes [seqctrlb::W](W) writer structure"]
impl crate::Writable for SEQCTRLB_SPEC {
    type Writer = W;
}
#[doc = "`reset()` method sets SEQCTRLB to value 0"]
impl crate::Resettable for SEQCTRLB_SPEC {
    #[inline(always)]
    fn reset_value() -> Self::Ux {
        0
    }
}
