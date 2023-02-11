#[doc = "Register `MASTER` reader"]
pub struct R(crate::R<MASTER_SPEC>);
impl core::ops::Deref for R {
    type Target = crate::R<MASTER_SPEC>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
impl From<crate::R<MASTER_SPEC>> for R {
    #[inline(always)]
    fn from(reader: crate::R<MASTER_SPEC>) -> Self {
        R(reader)
    }
}
#[doc = "Register `MASTER` writer"]
pub struct W(crate::W<MASTER_SPEC>);
impl core::ops::Deref for W {
    type Target = crate::W<MASTER_SPEC>;
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
impl From<crate::W<MASTER_SPEC>> for W {
    #[inline(always)]
    fn from(writer: crate::W<MASTER_SPEC>) -> Self {
        W(writer)
    }
}
#[doc = "Field `MASK` reader - This value determines the maximum size of the trace buffer in SRAM."]
pub type MASK_R = crate::FieldReader<u8, u8>;
#[doc = "Field `MASK` writer - This value determines the maximum size of the trace buffer in SRAM."]
pub type MASK_W<'a, const O: u8> = crate::FieldWriter<'a, u32, MASTER_SPEC, u8, u8, 5, O>;
#[doc = "Field `TSTARTEN` reader - Trace start input enable."]
pub type TSTARTEN_R = crate::BitReader<bool>;
#[doc = "Field `TSTARTEN` writer - Trace start input enable."]
pub type TSTARTEN_W<'a, const O: u8> = crate::BitWriter<'a, u32, MASTER_SPEC, bool, O>;
#[doc = "Field `TSTOPEN` reader - Trace stop input enable."]
pub type TSTOPEN_R = crate::BitReader<bool>;
#[doc = "Field `TSTOPEN` writer - Trace stop input enable."]
pub type TSTOPEN_W<'a, const O: u8> = crate::BitWriter<'a, u32, MASTER_SPEC, bool, O>;
#[doc = "Field `HALTREQ` reader - Halt request bit."]
pub type HALTREQ_R = crate::BitReader<bool>;
#[doc = "Field `HALTREQ` writer - Halt request bit."]
pub type HALTREQ_W<'a, const O: u8> = crate::BitWriter<'a, u32, MASTER_SPEC, bool, O>;
#[doc = "Field `EN` reader - Main trace enable bit."]
pub type EN_R = crate::BitReader<bool>;
#[doc = "Field `EN` writer - Main trace enable bit."]
pub type EN_W<'a, const O: u8> = crate::BitWriter<'a, u32, MASTER_SPEC, bool, O>;
impl R {
    #[doc = "Bits 0:4 - This value determines the maximum size of the trace buffer in SRAM."]
    #[inline(always)]
    pub fn mask(&self) -> MASK_R {
        MASK_R::new((self.bits & 0x1f) as u8)
    }
    #[doc = "Bit 5 - Trace start input enable."]
    #[inline(always)]
    pub fn tstarten(&self) -> TSTARTEN_R {
        TSTARTEN_R::new(((self.bits >> 5) & 1) != 0)
    }
    #[doc = "Bit 6 - Trace stop input enable."]
    #[inline(always)]
    pub fn tstopen(&self) -> TSTOPEN_R {
        TSTOPEN_R::new(((self.bits >> 6) & 1) != 0)
    }
    #[doc = "Bit 9 - Halt request bit."]
    #[inline(always)]
    pub fn haltreq(&self) -> HALTREQ_R {
        HALTREQ_R::new(((self.bits >> 9) & 1) != 0)
    }
    #[doc = "Bit 31 - Main trace enable bit."]
    #[inline(always)]
    pub fn en(&self) -> EN_R {
        EN_R::new(((self.bits >> 31) & 1) != 0)
    }
}
impl W {
    #[doc = "Bits 0:4 - This value determines the maximum size of the trace buffer in SRAM."]
    #[inline(always)]
    #[must_use]
    pub fn mask(&mut self) -> MASK_W<0> {
        MASK_W::new(self)
    }
    #[doc = "Bit 5 - Trace start input enable."]
    #[inline(always)]
    #[must_use]
    pub fn tstarten(&mut self) -> TSTARTEN_W<5> {
        TSTARTEN_W::new(self)
    }
    #[doc = "Bit 6 - Trace stop input enable."]
    #[inline(always)]
    #[must_use]
    pub fn tstopen(&mut self) -> TSTOPEN_W<6> {
        TSTOPEN_W::new(self)
    }
    #[doc = "Bit 9 - Halt request bit."]
    #[inline(always)]
    #[must_use]
    pub fn haltreq(&mut self) -> HALTREQ_W<9> {
        HALTREQ_W::new(self)
    }
    #[doc = "Bit 31 - Main trace enable bit."]
    #[inline(always)]
    #[must_use]
    pub fn en(&mut self) -> EN_W<31> {
        EN_W::new(self)
    }
    #[doc = "Writes raw bits to the register."]
    #[inline(always)]
    pub unsafe fn bits(&mut self, bits: u32) -> &mut Self {
        self.0.bits(bits);
        self
    }
}
#[doc = "MTB Trace Control Register\n\nThis register you can [`read`](crate::generic::Reg::read), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [master](index.html) module"]
pub struct MASTER_SPEC;
impl crate::RegisterSpec for MASTER_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [master::R](R) reader structure"]
impl crate::Readable for MASTER_SPEC {
    type Reader = R;
}
#[doc = "`write(|w| ..)` method takes [master::W](W) writer structure"]
impl crate::Writable for MASTER_SPEC {
    type Writer = W;
    const ZERO_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
}
#[doc = "`reset()` method sets MASTER to value 0"]
impl crate::Resettable for MASTER_SPEC {
    const RESET_VALUE: Self::Ux = 0;
}
