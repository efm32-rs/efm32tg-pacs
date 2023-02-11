#[doc = "Register `EM4CTRL` reader"]
pub struct R(crate::R<EM4CTRL_SPEC>);
impl core::ops::Deref for R {
    type Target = crate::R<EM4CTRL_SPEC>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
impl From<crate::R<EM4CTRL_SPEC>> for R {
    #[inline(always)]
    fn from(reader: crate::R<EM4CTRL_SPEC>) -> Self {
        R(reader)
    }
}
#[doc = "Register `EM4CTRL` writer"]
pub struct W(crate::W<EM4CTRL_SPEC>);
impl core::ops::Deref for W {
    type Target = crate::W<EM4CTRL_SPEC>;
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
impl From<crate::W<EM4CTRL_SPEC>> for W {
    #[inline(always)]
    fn from(writer: crate::W<EM4CTRL_SPEC>) -> Self {
        W(writer)
    }
}
#[doc = "Field `EM4STATE` reader - Energy Mode 4 State"]
pub type EM4STATE_R = crate::BitReader<bool>;
#[doc = "Field `EM4STATE` writer - Energy Mode 4 State"]
pub type EM4STATE_W<'a, const O: u8> = crate::BitWriter<'a, u32, EM4CTRL_SPEC, bool, O>;
#[doc = "Field `RETAINLFRCO` reader - LFRCO Retain During EM4"]
pub type RETAINLFRCO_R = crate::BitReader<bool>;
#[doc = "Field `RETAINLFRCO` writer - LFRCO Retain During EM4"]
pub type RETAINLFRCO_W<'a, const O: u8> = crate::BitWriter<'a, u32, EM4CTRL_SPEC, bool, O>;
#[doc = "Field `RETAINLFXO` reader - LFXO Retain During EM4"]
pub type RETAINLFXO_R = crate::BitReader<bool>;
#[doc = "Field `RETAINLFXO` writer - LFXO Retain During EM4"]
pub type RETAINLFXO_W<'a, const O: u8> = crate::BitWriter<'a, u32, EM4CTRL_SPEC, bool, O>;
#[doc = "Field `RETAINULFRCO` reader - ULFRCO Retain During EM4S"]
pub type RETAINULFRCO_R = crate::BitReader<bool>;
#[doc = "Field `RETAINULFRCO` writer - ULFRCO Retain During EM4S"]
pub type RETAINULFRCO_W<'a, const O: u8> = crate::BitWriter<'a, u32, EM4CTRL_SPEC, bool, O>;
#[doc = "Field `EM4IORETMODE` reader - EM4 IO Retention Disable"]
pub type EM4IORETMODE_R = crate::FieldReader<u8, EM4IORETMODE_A>;
#[doc = "EM4 IO Retention Disable\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
#[repr(u8)]
pub enum EM4IORETMODE_A {
    #[doc = "0: No Retention: Pads enter reset state when entering EM4"]
    DISABLE = 0,
    #[doc = "1: Retention through EM4: Pads enter reset state when exiting EM4"]
    EM4EXIT = 1,
    #[doc = "2: Retention through EM4 and Wakeup: software writes UNLATCH register to remove retention"]
    SWUNLATCH = 2,
}
impl From<EM4IORETMODE_A> for u8 {
    #[inline(always)]
    fn from(variant: EM4IORETMODE_A) -> Self {
        variant as _
    }
}
impl EM4IORETMODE_R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> Option<EM4IORETMODE_A> {
        match self.bits {
            0 => Some(EM4IORETMODE_A::DISABLE),
            1 => Some(EM4IORETMODE_A::EM4EXIT),
            2 => Some(EM4IORETMODE_A::SWUNLATCH),
            _ => None,
        }
    }
    #[doc = "Checks if the value of the field is `DISABLE`"]
    #[inline(always)]
    pub fn is_disable(&self) -> bool {
        *self == EM4IORETMODE_A::DISABLE
    }
    #[doc = "Checks if the value of the field is `EM4EXIT`"]
    #[inline(always)]
    pub fn is_em4exit(&self) -> bool {
        *self == EM4IORETMODE_A::EM4EXIT
    }
    #[doc = "Checks if the value of the field is `SWUNLATCH`"]
    #[inline(always)]
    pub fn is_swunlatch(&self) -> bool {
        *self == EM4IORETMODE_A::SWUNLATCH
    }
}
#[doc = "Field `EM4IORETMODE` writer - EM4 IO Retention Disable"]
pub type EM4IORETMODE_W<'a, const O: u8> =
    crate::FieldWriter<'a, u32, EM4CTRL_SPEC, u8, EM4IORETMODE_A, 2, O>;
impl<'a, const O: u8> EM4IORETMODE_W<'a, O> {
    #[doc = "No Retention: Pads enter reset state when entering EM4"]
    #[inline(always)]
    pub fn disable(self) -> &'a mut W {
        self.variant(EM4IORETMODE_A::DISABLE)
    }
    #[doc = "Retention through EM4: Pads enter reset state when exiting EM4"]
    #[inline(always)]
    pub fn em4exit(self) -> &'a mut W {
        self.variant(EM4IORETMODE_A::EM4EXIT)
    }
    #[doc = "Retention through EM4 and Wakeup: software writes UNLATCH register to remove retention"]
    #[inline(always)]
    pub fn swunlatch(self) -> &'a mut W {
        self.variant(EM4IORETMODE_A::SWUNLATCH)
    }
}
#[doc = "Field `EM4ENTRY` writer - Energy Mode 4 Entry"]
pub type EM4ENTRY_W<'a, const O: u8> = crate::FieldWriter<'a, u32, EM4CTRL_SPEC, u8, u8, 2, O>;
impl R {
    #[doc = "Bit 0 - Energy Mode 4 State"]
    #[inline(always)]
    pub fn em4state(&self) -> EM4STATE_R {
        EM4STATE_R::new((self.bits & 1) != 0)
    }
    #[doc = "Bit 1 - LFRCO Retain During EM4"]
    #[inline(always)]
    pub fn retainlfrco(&self) -> RETAINLFRCO_R {
        RETAINLFRCO_R::new(((self.bits >> 1) & 1) != 0)
    }
    #[doc = "Bit 2 - LFXO Retain During EM4"]
    #[inline(always)]
    pub fn retainlfxo(&self) -> RETAINLFXO_R {
        RETAINLFXO_R::new(((self.bits >> 2) & 1) != 0)
    }
    #[doc = "Bit 3 - ULFRCO Retain During EM4S"]
    #[inline(always)]
    pub fn retainulfrco(&self) -> RETAINULFRCO_R {
        RETAINULFRCO_R::new(((self.bits >> 3) & 1) != 0)
    }
    #[doc = "Bits 4:5 - EM4 IO Retention Disable"]
    #[inline(always)]
    pub fn em4ioretmode(&self) -> EM4IORETMODE_R {
        EM4IORETMODE_R::new(((self.bits >> 4) & 3) as u8)
    }
}
impl W {
    #[doc = "Bit 0 - Energy Mode 4 State"]
    #[inline(always)]
    #[must_use]
    pub fn em4state(&mut self) -> EM4STATE_W<0> {
        EM4STATE_W::new(self)
    }
    #[doc = "Bit 1 - LFRCO Retain During EM4"]
    #[inline(always)]
    #[must_use]
    pub fn retainlfrco(&mut self) -> RETAINLFRCO_W<1> {
        RETAINLFRCO_W::new(self)
    }
    #[doc = "Bit 2 - LFXO Retain During EM4"]
    #[inline(always)]
    #[must_use]
    pub fn retainlfxo(&mut self) -> RETAINLFXO_W<2> {
        RETAINLFXO_W::new(self)
    }
    #[doc = "Bit 3 - ULFRCO Retain During EM4S"]
    #[inline(always)]
    #[must_use]
    pub fn retainulfrco(&mut self) -> RETAINULFRCO_W<3> {
        RETAINULFRCO_W::new(self)
    }
    #[doc = "Bits 4:5 - EM4 IO Retention Disable"]
    #[inline(always)]
    #[must_use]
    pub fn em4ioretmode(&mut self) -> EM4IORETMODE_W<4> {
        EM4IORETMODE_W::new(self)
    }
    #[doc = "Bits 16:17 - Energy Mode 4 Entry"]
    #[inline(always)]
    #[must_use]
    pub fn em4entry(&mut self) -> EM4ENTRY_W<16> {
        EM4ENTRY_W::new(self)
    }
    #[doc = "Writes raw bits to the register."]
    #[inline(always)]
    pub unsafe fn bits(&mut self, bits: u32) -> &mut Self {
        self.0.bits(bits);
        self
    }
}
#[doc = "EM4 Control Register\n\nThis register you can [`read`](crate::generic::Reg::read), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [em4ctrl](index.html) module"]
pub struct EM4CTRL_SPEC;
impl crate::RegisterSpec for EM4CTRL_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [em4ctrl::R](R) reader structure"]
impl crate::Readable for EM4CTRL_SPEC {
    type Reader = R;
}
#[doc = "`write(|w| ..)` method takes [em4ctrl::W](W) writer structure"]
impl crate::Writable for EM4CTRL_SPEC {
    type Writer = W;
    const ZERO_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
}
#[doc = "`reset()` method sets EM4CTRL to value 0"]
impl crate::Resettable for EM4CTRL_SPEC {
    const RESET_VALUE: Self::Ux = 0;
}
