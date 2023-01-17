#[doc = "Register `BOOTLOADERCTRL` reader"]
pub struct R(crate::R<BOOTLOADERCTRL_SPEC>);
impl core::ops::Deref for R {
    type Target = crate::R<BOOTLOADERCTRL_SPEC>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
impl From<crate::R<BOOTLOADERCTRL_SPEC>> for R {
    #[inline(always)]
    fn from(reader: crate::R<BOOTLOADERCTRL_SPEC>) -> Self {
        R(reader)
    }
}
#[doc = "Register `BOOTLOADERCTRL` writer"]
pub struct W(crate::W<BOOTLOADERCTRL_SPEC>);
impl core::ops::Deref for W {
    type Target = crate::W<BOOTLOADERCTRL_SPEC>;
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
impl From<crate::W<BOOTLOADERCTRL_SPEC>> for W {
    #[inline(always)]
    fn from(writer: crate::W<BOOTLOADERCTRL_SPEC>) -> Self {
        W(writer)
    }
}
#[doc = "Field `BLRDIS` reader - Flash Bootloader Read Disable"]
pub type BLRDIS_R = crate::BitReader<bool>;
#[doc = "Field `BLRDIS` writer - Flash Bootloader Read Disable"]
pub type BLRDIS_W<'a, const O: u8> = crate::BitWriter<'a, u32, BOOTLOADERCTRL_SPEC, bool, O>;
#[doc = "Field `BLWDIS` reader - Flash Bootloader Write/Erase Disable"]
pub type BLWDIS_R = crate::BitReader<bool>;
#[doc = "Field `BLWDIS` writer - Flash Bootloader Write/Erase Disable"]
pub type BLWDIS_W<'a, const O: u8> = crate::BitWriter<'a, u32, BOOTLOADERCTRL_SPEC, bool, O>;
impl R {
    #[doc = "Bit 0 - Flash Bootloader Read Disable"]
    #[inline(always)]
    pub fn blrdis(&self) -> BLRDIS_R {
        BLRDIS_R::new((self.bits & 1) != 0)
    }
    #[doc = "Bit 1 - Flash Bootloader Write/Erase Disable"]
    #[inline(always)]
    pub fn blwdis(&self) -> BLWDIS_R {
        BLWDIS_R::new(((self.bits >> 1) & 1) != 0)
    }
}
impl W {
    #[doc = "Bit 0 - Flash Bootloader Read Disable"]
    #[inline(always)]
    #[must_use]
    pub fn blrdis(&mut self) -> BLRDIS_W<0> {
        BLRDIS_W::new(self)
    }
    #[doc = "Bit 1 - Flash Bootloader Write/Erase Disable"]
    #[inline(always)]
    #[must_use]
    pub fn blwdis(&mut self) -> BLWDIS_W<1> {
        BLWDIS_W::new(self)
    }
    #[doc = "Writes raw bits to the register."]
    #[inline(always)]
    pub unsafe fn bits(&mut self, bits: u32) -> &mut Self {
        self.0.bits(bits);
        self
    }
}
#[doc = "Bootloader Read and Write Enable, Write Once Register\n\nThis register you can [`read`](crate::generic::Reg::read), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [bootloaderctrl](index.html) module"]
pub struct BOOTLOADERCTRL_SPEC;
impl crate::RegisterSpec for BOOTLOADERCTRL_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [bootloaderctrl::R](R) reader structure"]
impl crate::Readable for BOOTLOADERCTRL_SPEC {
    type Reader = R;
}
#[doc = "`write(|w| ..)` method takes [bootloaderctrl::W](W) writer structure"]
impl crate::Writable for BOOTLOADERCTRL_SPEC {
    type Writer = W;
    const ZERO_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
}
#[doc = "`reset()` method sets BOOTLOADERCTRL to value 0"]
impl crate::Resettable for BOOTLOADERCTRL_SPEC {
    const RESET_VALUE: Self::Ux = 0;
}
