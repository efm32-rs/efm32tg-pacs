#[doc = "Register `CTRL` reader"]
pub struct R(crate::R<CTRL_SPEC>);
impl core::ops::Deref for R {
    type Target = crate::R<CTRL_SPEC>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
impl From<crate::R<CTRL_SPEC>> for R {
    #[inline(always)]
    fn from(reader: crate::R<CTRL_SPEC>) -> Self {
        R(reader)
    }
}
#[doc = "Register `CTRL` writer"]
pub struct W(crate::W<CTRL_SPEC>);
impl core::ops::Deref for W {
    type Target = crate::W<CTRL_SPEC>;
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
impl From<crate::W<CTRL_SPEC>> for W {
    #[inline(always)]
    fn from(writer: crate::W<CTRL_SPEC>) -> Self {
        W(writer)
    }
}
#[doc = "Field `SYNCPRSSETEN` reader - Synchronization PRS Set Enable"]
pub type SYNCPRSSETEN_R = crate::FieldReader<u8, u8>;
#[doc = "Field `SYNCPRSSETEN` writer - Synchronization PRS Set Enable"]
pub type SYNCPRSSETEN_W<'a, const O: u8> = crate::FieldWriter<'a, u32, CTRL_SPEC, u8, u8, 8, O>;
#[doc = "Field `SYNCPRSCLREN` reader - Synchronization PRS Clear Enable"]
pub type SYNCPRSCLREN_R = crate::FieldReader<u8, u8>;
#[doc = "Field `SYNCPRSCLREN` writer - Synchronization PRS Clear Enable"]
pub type SYNCPRSCLREN_W<'a, const O: u8> = crate::FieldWriter<'a, u32, CTRL_SPEC, u8, u8, 8, O>;
#[doc = "Field `NUMFIXED` reader - Number of Fixed Priority Channels"]
pub type NUMFIXED_R = crate::FieldReader<u8, u8>;
#[doc = "Field `NUMFIXED` writer - Number of Fixed Priority Channels"]
pub type NUMFIXED_W<'a, const O: u8> = crate::FieldWriter<'a, u32, CTRL_SPEC, u8, u8, 3, O>;
impl R {
    #[doc = "Bits 0:7 - Synchronization PRS Set Enable"]
    #[inline(always)]
    pub fn syncprsseten(&self) -> SYNCPRSSETEN_R {
        SYNCPRSSETEN_R::new((self.bits & 0xff) as u8)
    }
    #[doc = "Bits 8:15 - Synchronization PRS Clear Enable"]
    #[inline(always)]
    pub fn syncprsclren(&self) -> SYNCPRSCLREN_R {
        SYNCPRSCLREN_R::new(((self.bits >> 8) & 0xff) as u8)
    }
    #[doc = "Bits 24:26 - Number of Fixed Priority Channels"]
    #[inline(always)]
    pub fn numfixed(&self) -> NUMFIXED_R {
        NUMFIXED_R::new(((self.bits >> 24) & 7) as u8)
    }
}
impl W {
    #[doc = "Bits 0:7 - Synchronization PRS Set Enable"]
    #[inline(always)]
    #[must_use]
    pub fn syncprsseten(&mut self) -> SYNCPRSSETEN_W<0> {
        SYNCPRSSETEN_W::new(self)
    }
    #[doc = "Bits 8:15 - Synchronization PRS Clear Enable"]
    #[inline(always)]
    #[must_use]
    pub fn syncprsclren(&mut self) -> SYNCPRSCLREN_W<8> {
        SYNCPRSCLREN_W::new(self)
    }
    #[doc = "Bits 24:26 - Number of Fixed Priority Channels"]
    #[inline(always)]
    #[must_use]
    pub fn numfixed(&mut self) -> NUMFIXED_W<24> {
        NUMFIXED_W::new(self)
    }
    #[doc = "Writes raw bits to the register."]
    #[inline(always)]
    pub unsafe fn bits(&mut self, bits: u32) -> &mut Self {
        self.0.bits(bits);
        self
    }
}
#[doc = "DMA Control Register\n\nThis register you can [`read`](crate::generic::Reg::read), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [ctrl](index.html) module"]
pub struct CTRL_SPEC;
impl crate::RegisterSpec for CTRL_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [ctrl::R](R) reader structure"]
impl crate::Readable for CTRL_SPEC {
    type Reader = R;
}
#[doc = "`write(|w| ..)` method takes [ctrl::W](W) writer structure"]
impl crate::Writable for CTRL_SPEC {
    type Writer = W;
    const ZERO_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
}
#[doc = "`reset()` method sets CTRL to value 0x0700_0000"]
impl crate::Resettable for CTRL_SPEC {
    const RESET_VALUE: Self::Ux = 0x0700_0000;
}
