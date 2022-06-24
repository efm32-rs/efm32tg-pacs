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
#[doc = "Field `SINGLE` reader - SINGLE Interrupt Enable"]
pub type SINGLE_R = crate::BitReader<bool>;
#[doc = "Field `SINGLE` writer - SINGLE Interrupt Enable"]
pub type SINGLE_W<'a> = crate::BitWriter<'a, u32, IEN_SPEC, bool, 0>;
#[doc = "Field `SCAN` reader - SCAN Interrupt Enable"]
pub type SCAN_R = crate::BitReader<bool>;
#[doc = "Field `SCAN` writer - SCAN Interrupt Enable"]
pub type SCAN_W<'a> = crate::BitWriter<'a, u32, IEN_SPEC, bool, 1>;
#[doc = "Field `SINGLEOF` reader - SINGLEOF Interrupt Enable"]
pub type SINGLEOF_R = crate::BitReader<bool>;
#[doc = "Field `SINGLEOF` writer - SINGLEOF Interrupt Enable"]
pub type SINGLEOF_W<'a> = crate::BitWriter<'a, u32, IEN_SPEC, bool, 8>;
#[doc = "Field `SCANOF` reader - SCANOF Interrupt Enable"]
pub type SCANOF_R = crate::BitReader<bool>;
#[doc = "Field `SCANOF` writer - SCANOF Interrupt Enable"]
pub type SCANOF_W<'a> = crate::BitWriter<'a, u32, IEN_SPEC, bool, 9>;
#[doc = "Field `SINGLEUF` reader - SINGLEUF Interrupt Enable"]
pub type SINGLEUF_R = crate::BitReader<bool>;
#[doc = "Field `SINGLEUF` writer - SINGLEUF Interrupt Enable"]
pub type SINGLEUF_W<'a> = crate::BitWriter<'a, u32, IEN_SPEC, bool, 10>;
#[doc = "Field `SCANUF` reader - SCANUF Interrupt Enable"]
pub type SCANUF_R = crate::BitReader<bool>;
#[doc = "Field `SCANUF` writer - SCANUF Interrupt Enable"]
pub type SCANUF_W<'a> = crate::BitWriter<'a, u32, IEN_SPEC, bool, 11>;
#[doc = "Field `SINGLECMP` reader - SINGLECMP Interrupt Enable"]
pub type SINGLECMP_R = crate::BitReader<bool>;
#[doc = "Field `SINGLECMP` writer - SINGLECMP Interrupt Enable"]
pub type SINGLECMP_W<'a> = crate::BitWriter<'a, u32, IEN_SPEC, bool, 16>;
#[doc = "Field `SCANCMP` reader - SCANCMP Interrupt Enable"]
pub type SCANCMP_R = crate::BitReader<bool>;
#[doc = "Field `SCANCMP` writer - SCANCMP Interrupt Enable"]
pub type SCANCMP_W<'a> = crate::BitWriter<'a, u32, IEN_SPEC, bool, 17>;
#[doc = "Field `VREFOV` reader - VREFOV Interrupt Enable"]
pub type VREFOV_R = crate::BitReader<bool>;
#[doc = "Field `VREFOV` writer - VREFOV Interrupt Enable"]
pub type VREFOV_W<'a> = crate::BitWriter<'a, u32, IEN_SPEC, bool, 24>;
#[doc = "Field `PROGERR` reader - PROGERR Interrupt Enable"]
pub type PROGERR_R = crate::BitReader<bool>;
#[doc = "Field `PROGERR` writer - PROGERR Interrupt Enable"]
pub type PROGERR_W<'a> = crate::BitWriter<'a, u32, IEN_SPEC, bool, 25>;
#[doc = "Field `SCANEXTPEND` reader - SCANEXTPEND Interrupt Enable"]
pub type SCANEXTPEND_R = crate::BitReader<bool>;
#[doc = "Field `SCANEXTPEND` writer - SCANEXTPEND Interrupt Enable"]
pub type SCANEXTPEND_W<'a> = crate::BitWriter<'a, u32, IEN_SPEC, bool, 26>;
#[doc = "Field `SCANPEND` reader - SCANPEND Interrupt Enable"]
pub type SCANPEND_R = crate::BitReader<bool>;
#[doc = "Field `SCANPEND` writer - SCANPEND Interrupt Enable"]
pub type SCANPEND_W<'a> = crate::BitWriter<'a, u32, IEN_SPEC, bool, 27>;
#[doc = "Field `PRSTIMEDERR` reader - PRSTIMEDERR Interrupt Enable"]
pub type PRSTIMEDERR_R = crate::BitReader<bool>;
#[doc = "Field `PRSTIMEDERR` writer - PRSTIMEDERR Interrupt Enable"]
pub type PRSTIMEDERR_W<'a> = crate::BitWriter<'a, u32, IEN_SPEC, bool, 28>;
#[doc = "Field `EM23ERR` reader - EM23ERR Interrupt Enable"]
pub type EM23ERR_R = crate::BitReader<bool>;
#[doc = "Field `EM23ERR` writer - EM23ERR Interrupt Enable"]
pub type EM23ERR_W<'a> = crate::BitWriter<'a, u32, IEN_SPEC, bool, 29>;
impl R {
    #[doc = "Bit 0 - SINGLE Interrupt Enable"]
    #[inline(always)]
    pub fn single(&self) -> SINGLE_R {
        SINGLE_R::new((self.bits & 1) != 0)
    }
    #[doc = "Bit 1 - SCAN Interrupt Enable"]
    #[inline(always)]
    pub fn scan(&self) -> SCAN_R {
        SCAN_R::new(((self.bits >> 1) & 1) != 0)
    }
    #[doc = "Bit 8 - SINGLEOF Interrupt Enable"]
    #[inline(always)]
    pub fn singleof(&self) -> SINGLEOF_R {
        SINGLEOF_R::new(((self.bits >> 8) & 1) != 0)
    }
    #[doc = "Bit 9 - SCANOF Interrupt Enable"]
    #[inline(always)]
    pub fn scanof(&self) -> SCANOF_R {
        SCANOF_R::new(((self.bits >> 9) & 1) != 0)
    }
    #[doc = "Bit 10 - SINGLEUF Interrupt Enable"]
    #[inline(always)]
    pub fn singleuf(&self) -> SINGLEUF_R {
        SINGLEUF_R::new(((self.bits >> 10) & 1) != 0)
    }
    #[doc = "Bit 11 - SCANUF Interrupt Enable"]
    #[inline(always)]
    pub fn scanuf(&self) -> SCANUF_R {
        SCANUF_R::new(((self.bits >> 11) & 1) != 0)
    }
    #[doc = "Bit 16 - SINGLECMP Interrupt Enable"]
    #[inline(always)]
    pub fn singlecmp(&self) -> SINGLECMP_R {
        SINGLECMP_R::new(((self.bits >> 16) & 1) != 0)
    }
    #[doc = "Bit 17 - SCANCMP Interrupt Enable"]
    #[inline(always)]
    pub fn scancmp(&self) -> SCANCMP_R {
        SCANCMP_R::new(((self.bits >> 17) & 1) != 0)
    }
    #[doc = "Bit 24 - VREFOV Interrupt Enable"]
    #[inline(always)]
    pub fn vrefov(&self) -> VREFOV_R {
        VREFOV_R::new(((self.bits >> 24) & 1) != 0)
    }
    #[doc = "Bit 25 - PROGERR Interrupt Enable"]
    #[inline(always)]
    pub fn progerr(&self) -> PROGERR_R {
        PROGERR_R::new(((self.bits >> 25) & 1) != 0)
    }
    #[doc = "Bit 26 - SCANEXTPEND Interrupt Enable"]
    #[inline(always)]
    pub fn scanextpend(&self) -> SCANEXTPEND_R {
        SCANEXTPEND_R::new(((self.bits >> 26) & 1) != 0)
    }
    #[doc = "Bit 27 - SCANPEND Interrupt Enable"]
    #[inline(always)]
    pub fn scanpend(&self) -> SCANPEND_R {
        SCANPEND_R::new(((self.bits >> 27) & 1) != 0)
    }
    #[doc = "Bit 28 - PRSTIMEDERR Interrupt Enable"]
    #[inline(always)]
    pub fn prstimederr(&self) -> PRSTIMEDERR_R {
        PRSTIMEDERR_R::new(((self.bits >> 28) & 1) != 0)
    }
    #[doc = "Bit 29 - EM23ERR Interrupt Enable"]
    #[inline(always)]
    pub fn em23err(&self) -> EM23ERR_R {
        EM23ERR_R::new(((self.bits >> 29) & 1) != 0)
    }
}
impl W {
    #[doc = "Bit 0 - SINGLE Interrupt Enable"]
    #[inline(always)]
    pub fn single(&mut self) -> SINGLE_W {
        SINGLE_W::new(self)
    }
    #[doc = "Bit 1 - SCAN Interrupt Enable"]
    #[inline(always)]
    pub fn scan(&mut self) -> SCAN_W {
        SCAN_W::new(self)
    }
    #[doc = "Bit 8 - SINGLEOF Interrupt Enable"]
    #[inline(always)]
    pub fn singleof(&mut self) -> SINGLEOF_W {
        SINGLEOF_W::new(self)
    }
    #[doc = "Bit 9 - SCANOF Interrupt Enable"]
    #[inline(always)]
    pub fn scanof(&mut self) -> SCANOF_W {
        SCANOF_W::new(self)
    }
    #[doc = "Bit 10 - SINGLEUF Interrupt Enable"]
    #[inline(always)]
    pub fn singleuf(&mut self) -> SINGLEUF_W {
        SINGLEUF_W::new(self)
    }
    #[doc = "Bit 11 - SCANUF Interrupt Enable"]
    #[inline(always)]
    pub fn scanuf(&mut self) -> SCANUF_W {
        SCANUF_W::new(self)
    }
    #[doc = "Bit 16 - SINGLECMP Interrupt Enable"]
    #[inline(always)]
    pub fn singlecmp(&mut self) -> SINGLECMP_W {
        SINGLECMP_W::new(self)
    }
    #[doc = "Bit 17 - SCANCMP Interrupt Enable"]
    #[inline(always)]
    pub fn scancmp(&mut self) -> SCANCMP_W {
        SCANCMP_W::new(self)
    }
    #[doc = "Bit 24 - VREFOV Interrupt Enable"]
    #[inline(always)]
    pub fn vrefov(&mut self) -> VREFOV_W {
        VREFOV_W::new(self)
    }
    #[doc = "Bit 25 - PROGERR Interrupt Enable"]
    #[inline(always)]
    pub fn progerr(&mut self) -> PROGERR_W {
        PROGERR_W::new(self)
    }
    #[doc = "Bit 26 - SCANEXTPEND Interrupt Enable"]
    #[inline(always)]
    pub fn scanextpend(&mut self) -> SCANEXTPEND_W {
        SCANEXTPEND_W::new(self)
    }
    #[doc = "Bit 27 - SCANPEND Interrupt Enable"]
    #[inline(always)]
    pub fn scanpend(&mut self) -> SCANPEND_W {
        SCANPEND_W::new(self)
    }
    #[doc = "Bit 28 - PRSTIMEDERR Interrupt Enable"]
    #[inline(always)]
    pub fn prstimederr(&mut self) -> PRSTIMEDERR_W {
        PRSTIMEDERR_W::new(self)
    }
    #[doc = "Bit 29 - EM23ERR Interrupt Enable"]
    #[inline(always)]
    pub fn em23err(&mut self) -> EM23ERR_W {
        EM23ERR_W::new(self)
    }
    #[doc = "Writes raw bits to the register."]
    #[inline(always)]
    pub unsafe fn bits(&mut self, bits: u32) -> &mut Self {
        self.0.bits(bits);
        self
    }
}
#[doc = "Interrupt Enable Register\n\nThis register you can [`read`](crate::generic::Reg::read), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [ien](index.html) module"]
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
