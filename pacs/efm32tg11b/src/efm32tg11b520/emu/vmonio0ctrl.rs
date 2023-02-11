#[doc = "Register `VMONIO0CTRL` reader"]
pub struct R(crate::R<VMONIO0CTRL_SPEC>);
impl core::ops::Deref for R {
    type Target = crate::R<VMONIO0CTRL_SPEC>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
impl From<crate::R<VMONIO0CTRL_SPEC>> for R {
    #[inline(always)]
    fn from(reader: crate::R<VMONIO0CTRL_SPEC>) -> Self {
        R(reader)
    }
}
#[doc = "Register `VMONIO0CTRL` writer"]
pub struct W(crate::W<VMONIO0CTRL_SPEC>);
impl core::ops::Deref for W {
    type Target = crate::W<VMONIO0CTRL_SPEC>;
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
impl From<crate::W<VMONIO0CTRL_SPEC>> for W {
    #[inline(always)]
    fn from(writer: crate::W<VMONIO0CTRL_SPEC>) -> Self {
        W(writer)
    }
}
#[doc = "Field `EN` reader - Enable"]
pub type EN_R = crate::BitReader<bool>;
#[doc = "Field `EN` writer - Enable"]
pub type EN_W<'a, const O: u8> = crate::BitWriter<'a, u32, VMONIO0CTRL_SPEC, bool, O>;
#[doc = "Field `RISEWU` reader - Rise Wakeup"]
pub type RISEWU_R = crate::BitReader<bool>;
#[doc = "Field `RISEWU` writer - Rise Wakeup"]
pub type RISEWU_W<'a, const O: u8> = crate::BitWriter<'a, u32, VMONIO0CTRL_SPEC, bool, O>;
#[doc = "Field `FALLWU` reader - Fall Wakeup"]
pub type FALLWU_R = crate::BitReader<bool>;
#[doc = "Field `FALLWU` writer - Fall Wakeup"]
pub type FALLWU_W<'a, const O: u8> = crate::BitWriter<'a, u32, VMONIO0CTRL_SPEC, bool, O>;
#[doc = "Field `RETDIS` reader - EM4 IO0 Retention Disable"]
pub type RETDIS_R = crate::BitReader<bool>;
#[doc = "Field `RETDIS` writer - EM4 IO0 Retention Disable"]
pub type RETDIS_W<'a, const O: u8> = crate::BitWriter<'a, u32, VMONIO0CTRL_SPEC, bool, O>;
#[doc = "Field `THRESFINE` reader - Threshold Fine Adjust"]
pub type THRESFINE_R = crate::FieldReader<u8, u8>;
#[doc = "Field `THRESFINE` writer - Threshold Fine Adjust"]
pub type THRESFINE_W<'a, const O: u8> = crate::FieldWriter<'a, u32, VMONIO0CTRL_SPEC, u8, u8, 4, O>;
#[doc = "Field `THRESCOARSE` reader - Threshold Coarse Adjust"]
pub type THRESCOARSE_R = crate::FieldReader<u8, u8>;
#[doc = "Field `THRESCOARSE` writer - Threshold Coarse Adjust"]
pub type THRESCOARSE_W<'a, const O: u8> =
    crate::FieldWriter<'a, u32, VMONIO0CTRL_SPEC, u8, u8, 4, O>;
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
    #[doc = "Bit 4 - EM4 IO0 Retention Disable"]
    #[inline(always)]
    pub fn retdis(&self) -> RETDIS_R {
        RETDIS_R::new(((self.bits >> 4) & 1) != 0)
    }
    #[doc = "Bits 8:11 - Threshold Fine Adjust"]
    #[inline(always)]
    pub fn thresfine(&self) -> THRESFINE_R {
        THRESFINE_R::new(((self.bits >> 8) & 0x0f) as u8)
    }
    #[doc = "Bits 12:15 - Threshold Coarse Adjust"]
    #[inline(always)]
    pub fn threscoarse(&self) -> THRESCOARSE_R {
        THRESCOARSE_R::new(((self.bits >> 12) & 0x0f) as u8)
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
    #[doc = "Bit 4 - EM4 IO0 Retention Disable"]
    #[inline(always)]
    #[must_use]
    pub fn retdis(&mut self) -> RETDIS_W<4> {
        RETDIS_W::new(self)
    }
    #[doc = "Bits 8:11 - Threshold Fine Adjust"]
    #[inline(always)]
    #[must_use]
    pub fn thresfine(&mut self) -> THRESFINE_W<8> {
        THRESFINE_W::new(self)
    }
    #[doc = "Bits 12:15 - Threshold Coarse Adjust"]
    #[inline(always)]
    #[must_use]
    pub fn threscoarse(&mut self) -> THRESCOARSE_W<12> {
        THRESCOARSE_W::new(self)
    }
    #[doc = "Writes raw bits to the register."]
    #[inline(always)]
    pub unsafe fn bits(&mut self, bits: u32) -> &mut Self {
        self.0.bits(bits);
        self
    }
}
#[doc = "VMON IOVDD0 Channel Control\n\nThis register you can [`read`](crate::generic::Reg::read), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [vmonio0ctrl](index.html) module"]
pub struct VMONIO0CTRL_SPEC;
impl crate::RegisterSpec for VMONIO0CTRL_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [vmonio0ctrl::R](R) reader structure"]
impl crate::Readable for VMONIO0CTRL_SPEC {
    type Reader = R;
}
#[doc = "`write(|w| ..)` method takes [vmonio0ctrl::W](W) writer structure"]
impl crate::Writable for VMONIO0CTRL_SPEC {
    type Writer = W;
    const ZERO_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
}
#[doc = "`reset()` method sets VMONIO0CTRL to value 0"]
impl crate::Resettable for VMONIO0CTRL_SPEC {
    const RESET_VALUE: Self::Ux = 0;
}
