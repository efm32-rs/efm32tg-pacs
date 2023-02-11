#[doc = "Register `VMONAVDDCTRL` reader"]
pub struct R(crate::R<VMONAVDDCTRL_SPEC>);
impl core::ops::Deref for R {
    type Target = crate::R<VMONAVDDCTRL_SPEC>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
impl From<crate::R<VMONAVDDCTRL_SPEC>> for R {
    #[inline(always)]
    fn from(reader: crate::R<VMONAVDDCTRL_SPEC>) -> Self {
        R(reader)
    }
}
#[doc = "Register `VMONAVDDCTRL` writer"]
pub struct W(crate::W<VMONAVDDCTRL_SPEC>);
impl core::ops::Deref for W {
    type Target = crate::W<VMONAVDDCTRL_SPEC>;
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
impl From<crate::W<VMONAVDDCTRL_SPEC>> for W {
    #[inline(always)]
    fn from(writer: crate::W<VMONAVDDCTRL_SPEC>) -> Self {
        W(writer)
    }
}
#[doc = "Field `EN` reader - Enable"]
pub type EN_R = crate::BitReader<bool>;
#[doc = "Field `EN` writer - Enable"]
pub type EN_W<'a, const O: u8> = crate::BitWriter<'a, u32, VMONAVDDCTRL_SPEC, bool, O>;
#[doc = "Field `RISEWU` reader - Rise Wakeup"]
pub type RISEWU_R = crate::BitReader<bool>;
#[doc = "Field `RISEWU` writer - Rise Wakeup"]
pub type RISEWU_W<'a, const O: u8> = crate::BitWriter<'a, u32, VMONAVDDCTRL_SPEC, bool, O>;
#[doc = "Field `FALLWU` reader - Fall Wakeup"]
pub type FALLWU_R = crate::BitReader<bool>;
#[doc = "Field `FALLWU` writer - Fall Wakeup"]
pub type FALLWU_W<'a, const O: u8> = crate::BitWriter<'a, u32, VMONAVDDCTRL_SPEC, bool, O>;
#[doc = "Field `FALLTHRESFINE` reader - Falling Threshold Fine Adjust"]
pub type FALLTHRESFINE_R = crate::FieldReader<u8, u8>;
#[doc = "Field `FALLTHRESFINE` writer - Falling Threshold Fine Adjust"]
pub type FALLTHRESFINE_W<'a, const O: u8> =
    crate::FieldWriter<'a, u32, VMONAVDDCTRL_SPEC, u8, u8, 4, O>;
#[doc = "Field `FALLTHRESCOARSE` reader - Falling Threshold Coarse Adjust"]
pub type FALLTHRESCOARSE_R = crate::FieldReader<u8, u8>;
#[doc = "Field `FALLTHRESCOARSE` writer - Falling Threshold Coarse Adjust"]
pub type FALLTHRESCOARSE_W<'a, const O: u8> =
    crate::FieldWriter<'a, u32, VMONAVDDCTRL_SPEC, u8, u8, 4, O>;
#[doc = "Field `RISETHRESFINE` reader - Rising Threshold Fine Adjust"]
pub type RISETHRESFINE_R = crate::FieldReader<u8, u8>;
#[doc = "Field `RISETHRESFINE` writer - Rising Threshold Fine Adjust"]
pub type RISETHRESFINE_W<'a, const O: u8> =
    crate::FieldWriter<'a, u32, VMONAVDDCTRL_SPEC, u8, u8, 4, O>;
#[doc = "Field `RISETHRESCOARSE` reader - Rising Threshold Coarse Adjust"]
pub type RISETHRESCOARSE_R = crate::FieldReader<u8, u8>;
#[doc = "Field `RISETHRESCOARSE` writer - Rising Threshold Coarse Adjust"]
pub type RISETHRESCOARSE_W<'a, const O: u8> =
    crate::FieldWriter<'a, u32, VMONAVDDCTRL_SPEC, u8, u8, 4, O>;
impl R {
    #[doc = "Bit 0 - Enable"]
    #[inline(always)]
    pub fn en(&self) -> EN_R {
        EN_R::new((self.bits & 1) != 0)
    }
    #[doc = "Bit 2 - Rise Wakeup"]
    #[inline(always)]
    pub fn risewu(&self) -> RISEWU_R {
        RISEWU_R::new(((self.bits >> 2) & 1) != 0)
    }
    #[doc = "Bit 3 - Fall Wakeup"]
    #[inline(always)]
    pub fn fallwu(&self) -> FALLWU_R {
        FALLWU_R::new(((self.bits >> 3) & 1) != 0)
    }
    #[doc = "Bits 8:11 - Falling Threshold Fine Adjust"]
    #[inline(always)]
    pub fn fallthresfine(&self) -> FALLTHRESFINE_R {
        FALLTHRESFINE_R::new(((self.bits >> 8) & 0x0f) as u8)
    }
    #[doc = "Bits 12:15 - Falling Threshold Coarse Adjust"]
    #[inline(always)]
    pub fn fallthrescoarse(&self) -> FALLTHRESCOARSE_R {
        FALLTHRESCOARSE_R::new(((self.bits >> 12) & 0x0f) as u8)
    }
    #[doc = "Bits 16:19 - Rising Threshold Fine Adjust"]
    #[inline(always)]
    pub fn risethresfine(&self) -> RISETHRESFINE_R {
        RISETHRESFINE_R::new(((self.bits >> 16) & 0x0f) as u8)
    }
    #[doc = "Bits 20:23 - Rising Threshold Coarse Adjust"]
    #[inline(always)]
    pub fn risethrescoarse(&self) -> RISETHRESCOARSE_R {
        RISETHRESCOARSE_R::new(((self.bits >> 20) & 0x0f) as u8)
    }
}
impl W {
    #[doc = "Bit 0 - Enable"]
    #[inline(always)]
    #[must_use]
    pub fn en(&mut self) -> EN_W<0> {
        EN_W::new(self)
    }
    #[doc = "Bit 2 - Rise Wakeup"]
    #[inline(always)]
    #[must_use]
    pub fn risewu(&mut self) -> RISEWU_W<2> {
        RISEWU_W::new(self)
    }
    #[doc = "Bit 3 - Fall Wakeup"]
    #[inline(always)]
    #[must_use]
    pub fn fallwu(&mut self) -> FALLWU_W<3> {
        FALLWU_W::new(self)
    }
    #[doc = "Bits 8:11 - Falling Threshold Fine Adjust"]
    #[inline(always)]
    #[must_use]
    pub fn fallthresfine(&mut self) -> FALLTHRESFINE_W<8> {
        FALLTHRESFINE_W::new(self)
    }
    #[doc = "Bits 12:15 - Falling Threshold Coarse Adjust"]
    #[inline(always)]
    #[must_use]
    pub fn fallthrescoarse(&mut self) -> FALLTHRESCOARSE_W<12> {
        FALLTHRESCOARSE_W::new(self)
    }
    #[doc = "Bits 16:19 - Rising Threshold Fine Adjust"]
    #[inline(always)]
    #[must_use]
    pub fn risethresfine(&mut self) -> RISETHRESFINE_W<16> {
        RISETHRESFINE_W::new(self)
    }
    #[doc = "Bits 20:23 - Rising Threshold Coarse Adjust"]
    #[inline(always)]
    #[must_use]
    pub fn risethrescoarse(&mut self) -> RISETHRESCOARSE_W<20> {
        RISETHRESCOARSE_W::new(self)
    }
    #[doc = "Writes raw bits to the register."]
    #[inline(always)]
    pub unsafe fn bits(&mut self, bits: u32) -> &mut Self {
        self.0.bits(bits);
        self
    }
}
#[doc = "VMON AVDD Channel Control\n\nThis register you can [`read`](crate::generic::Reg::read), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [vmonavddctrl](index.html) module"]
pub struct VMONAVDDCTRL_SPEC;
impl crate::RegisterSpec for VMONAVDDCTRL_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [vmonavddctrl::R](R) reader structure"]
impl crate::Readable for VMONAVDDCTRL_SPEC {
    type Reader = R;
}
#[doc = "`write(|w| ..)` method takes [vmonavddctrl::W](W) writer structure"]
impl crate::Writable for VMONAVDDCTRL_SPEC {
    type Writer = W;
    const ZERO_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
}
#[doc = "`reset()` method sets VMONAVDDCTRL to value 0"]
impl crate::Resettable for VMONAVDDCTRL_SPEC {
    const RESET_VALUE: Self::Ux = 0;
}
