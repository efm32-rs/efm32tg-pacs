#[doc = "Register `IEN` reader"]
pub struct R(crate::R<IEN_SPEC>);
impl core::ops::Deref for R {
    type Target = crate::R<IEN_SPEC>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
impl From<crate::R<IEN_SPEC>> for R {
    #[inline(always)]
    fn from(reader: crate::R<IEN_SPEC>) -> Self {
        R(reader)
    }
}
#[doc = "Register `IEN` writer"]
pub struct W(crate::W<IEN_SPEC>);
impl core::ops::Deref for W {
    type Target = crate::W<IEN_SPEC>;
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
impl From<crate::W<IEN_SPEC>> for W {
    #[inline(always)]
    fn from(writer: crate::W<IEN_SPEC>) -> Self {
        W(writer)
    }
}
#[doc = "Field `CMP` reader - CMP Interrupt Enable"]
pub type CMP_R = crate::BitReader<bool>;
#[doc = "Field `CMP` writer - CMP Interrupt Enable"]
pub type CMP_W<'a> = crate::BitWriter<'a, u32, IEN_SPEC, bool, 0>;
#[doc = "Field `CONV` reader - CONV Interrupt Enable"]
pub type CONV_R = crate::BitReader<bool>;
#[doc = "Field `CONV` writer - CONV Interrupt Enable"]
pub type CONV_W<'a> = crate::BitWriter<'a, u32, IEN_SPEC, bool, 1>;
#[doc = "Field `EOS` reader - EOS Interrupt Enable"]
pub type EOS_R = crate::BitReader<bool>;
#[doc = "Field `EOS` writer - EOS Interrupt Enable"]
pub type EOS_W<'a> = crate::BitWriter<'a, u32, IEN_SPEC, bool, 2>;
#[doc = "Field `DMAOF` reader - DMAOF Interrupt Enable"]
pub type DMAOF_R = crate::BitReader<bool>;
#[doc = "Field `DMAOF` writer - DMAOF Interrupt Enable"]
pub type DMAOF_W<'a> = crate::BitWriter<'a, u32, IEN_SPEC, bool, 3>;
#[doc = "Field `APORTCONFLICT` reader - APORTCONFLICT Interrupt Enable"]
pub type APORTCONFLICT_R = crate::BitReader<bool>;
#[doc = "Field `APORTCONFLICT` writer - APORTCONFLICT Interrupt Enable"]
pub type APORTCONFLICT_W<'a> = crate::BitWriter<'a, u32, IEN_SPEC, bool, 4>;
impl R {
    #[doc = "Bit 0 - CMP Interrupt Enable"]
    #[inline(always)]
    pub fn cmp(&self) -> CMP_R {
        CMP_R::new((self.bits & 1) != 0)
    }
    #[doc = "Bit 1 - CONV Interrupt Enable"]
    #[inline(always)]
    pub fn conv(&self) -> CONV_R {
        CONV_R::new(((self.bits >> 1) & 1) != 0)
    }
    #[doc = "Bit 2 - EOS Interrupt Enable"]
    #[inline(always)]
    pub fn eos(&self) -> EOS_R {
        EOS_R::new(((self.bits >> 2) & 1) != 0)
    }
    #[doc = "Bit 3 - DMAOF Interrupt Enable"]
    #[inline(always)]
    pub fn dmaof(&self) -> DMAOF_R {
        DMAOF_R::new(((self.bits >> 3) & 1) != 0)
    }
    #[doc = "Bit 4 - APORTCONFLICT Interrupt Enable"]
    #[inline(always)]
    pub fn aportconflict(&self) -> APORTCONFLICT_R {
        APORTCONFLICT_R::new(((self.bits >> 4) & 1) != 0)
    }
}
impl W {
    #[doc = "Bit 0 - CMP Interrupt Enable"]
    #[inline(always)]
    pub fn cmp(&mut self) -> CMP_W {
        CMP_W::new(self)
    }
    #[doc = "Bit 1 - CONV Interrupt Enable"]
    #[inline(always)]
    pub fn conv(&mut self) -> CONV_W {
        CONV_W::new(self)
    }
    #[doc = "Bit 2 - EOS Interrupt Enable"]
    #[inline(always)]
    pub fn eos(&mut self) -> EOS_W {
        EOS_W::new(self)
    }
    #[doc = "Bit 3 - DMAOF Interrupt Enable"]
    #[inline(always)]
    pub fn dmaof(&mut self) -> DMAOF_W {
        DMAOF_W::new(self)
    }
    #[doc = "Bit 4 - APORTCONFLICT Interrupt Enable"]
    #[inline(always)]
    pub fn aportconflict(&mut self) -> APORTCONFLICT_W {
        APORTCONFLICT_W::new(self)
    }
    #[doc = "Writes raw bits to the register."]
    #[inline(always)]
    pub unsafe fn bits(&mut self, bits: u32) -> &mut Self {
        self.0.bits(bits);
        self
    }
}
#[doc = "Interrupt Enable\n\nThis register you can [`read`](crate::generic::Reg::read), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [ien](index.html) module"]
pub struct IEN_SPEC;
impl crate::RegisterSpec for IEN_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [ien::R](R) reader structure"]
impl crate::Readable for IEN_SPEC {
    type Reader = R;
}
#[doc = "`write(|w| ..)` method takes [ien::W](W) writer structure"]
impl crate::Writable for IEN_SPEC {
    type Writer = W;
}
#[doc = "`reset()` method sets IEN to value 0"]
impl crate::Resettable for IEN_SPEC {
    #[inline(always)]
    fn reset_value() -> Self::Ux {
        0
    }
}
