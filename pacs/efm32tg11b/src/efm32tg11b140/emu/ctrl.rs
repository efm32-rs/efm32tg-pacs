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
#[doc = "Field `EM2BLOCK` reader - Energy Mode 2 Block"]
pub type EM2BLOCK_R = crate::BitReader<bool>;
#[doc = "Field `EM2BLOCK` writer - Energy Mode 2 Block"]
pub type EM2BLOCK_W<'a, const O: u8> = crate::BitWriter<'a, u32, CTRL_SPEC, bool, O>;
#[doc = "Field `EM2BODDIS` reader - Disable BOD in EM2"]
pub type EM2BODDIS_R = crate::BitReader<bool>;
#[doc = "Field `EM2BODDIS` writer - Disable BOD in EM2"]
pub type EM2BODDIS_W<'a, const O: u8> = crate::BitWriter<'a, u32, CTRL_SPEC, bool, O>;
#[doc = "Field `EM01LD` reader - Reserved for internal use. Do not change."]
pub type EM01LD_R = crate::BitReader<bool>;
#[doc = "Field `EM01LD` writer - Reserved for internal use. Do not change."]
pub type EM01LD_W<'a, const O: u8> = crate::BitWriter<'a, u32, CTRL_SPEC, bool, O>;
#[doc = "Field `EM23VSCALEAUTOWSEN` reader - Automatically Configures Flash and Frequency to Wakeup From EM2 or EM3 at Low Voltage"]
pub type EM23VSCALEAUTOWSEN_R = crate::BitReader<bool>;
#[doc = "Field `EM23VSCALEAUTOWSEN` writer - Automatically Configures Flash and Frequency to Wakeup From EM2 or EM3 at Low Voltage"]
pub type EM23VSCALEAUTOWSEN_W<'a, const O: u8> = crate::BitWriter<'a, u32, CTRL_SPEC, bool, O>;
#[doc = "Field `EM23VSCALE` reader - EM23 Voltage Scale"]
pub type EM23VSCALE_R = crate::FieldReader<u8, EM23VSCALE_A>;
#[doc = "EM23 Voltage Scale\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
#[repr(u8)]
pub enum EM23VSCALE_A {
    #[doc = "0: Voltage Scale Level 2"]
    VSCALE2 = 0,
    #[doc = "2: Voltage Scale Level 0"]
    VSCALE0 = 2,
    #[doc = "3: RESV"]
    RESV = 3,
}
impl From<EM23VSCALE_A> for u8 {
    #[inline(always)]
    fn from(variant: EM23VSCALE_A) -> Self {
        variant as _
    }
}
impl EM23VSCALE_R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> Option<EM23VSCALE_A> {
        match self.bits {
            0 => Some(EM23VSCALE_A::VSCALE2),
            2 => Some(EM23VSCALE_A::VSCALE0),
            3 => Some(EM23VSCALE_A::RESV),
            _ => None,
        }
    }
    #[doc = "Checks if the value of the field is `VSCALE2`"]
    #[inline(always)]
    pub fn is_vscale2(&self) -> bool {
        *self == EM23VSCALE_A::VSCALE2
    }
    #[doc = "Checks if the value of the field is `VSCALE0`"]
    #[inline(always)]
    pub fn is_vscale0(&self) -> bool {
        *self == EM23VSCALE_A::VSCALE0
    }
    #[doc = "Checks if the value of the field is `RESV`"]
    #[inline(always)]
    pub fn is_resv(&self) -> bool {
        *self == EM23VSCALE_A::RESV
    }
}
#[doc = "Field `EM23VSCALE` writer - EM23 Voltage Scale"]
pub type EM23VSCALE_W<'a, const O: u8> =
    crate::FieldWriter<'a, u32, CTRL_SPEC, u8, EM23VSCALE_A, 2, O>;
impl<'a, const O: u8> EM23VSCALE_W<'a, O> {
    #[doc = "Voltage Scale Level 2"]
    #[inline(always)]
    pub fn vscale2(self) -> &'a mut W {
        self.variant(EM23VSCALE_A::VSCALE2)
    }
    #[doc = "Voltage Scale Level 0"]
    #[inline(always)]
    pub fn vscale0(self) -> &'a mut W {
        self.variant(EM23VSCALE_A::VSCALE0)
    }
    #[doc = "RESV"]
    #[inline(always)]
    pub fn resv(self) -> &'a mut W {
        self.variant(EM23VSCALE_A::RESV)
    }
}
#[doc = "Field `EM4HVSCALE` reader - EM4H Voltage Scale"]
pub type EM4HVSCALE_R = crate::FieldReader<u8, EM4HVSCALE_A>;
#[doc = "EM4H Voltage Scale\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
#[repr(u8)]
pub enum EM4HVSCALE_A {
    #[doc = "0: Voltage Scale Level 2"]
    VSCALE2 = 0,
    #[doc = "2: Voltage Scale Level 0"]
    VSCALE0 = 2,
    #[doc = "3: RESV"]
    RESV = 3,
}
impl From<EM4HVSCALE_A> for u8 {
    #[inline(always)]
    fn from(variant: EM4HVSCALE_A) -> Self {
        variant as _
    }
}
impl EM4HVSCALE_R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> Option<EM4HVSCALE_A> {
        match self.bits {
            0 => Some(EM4HVSCALE_A::VSCALE2),
            2 => Some(EM4HVSCALE_A::VSCALE0),
            3 => Some(EM4HVSCALE_A::RESV),
            _ => None,
        }
    }
    #[doc = "Checks if the value of the field is `VSCALE2`"]
    #[inline(always)]
    pub fn is_vscale2(&self) -> bool {
        *self == EM4HVSCALE_A::VSCALE2
    }
    #[doc = "Checks if the value of the field is `VSCALE0`"]
    #[inline(always)]
    pub fn is_vscale0(&self) -> bool {
        *self == EM4HVSCALE_A::VSCALE0
    }
    #[doc = "Checks if the value of the field is `RESV`"]
    #[inline(always)]
    pub fn is_resv(&self) -> bool {
        *self == EM4HVSCALE_A::RESV
    }
}
#[doc = "Field `EM4HVSCALE` writer - EM4H Voltage Scale"]
pub type EM4HVSCALE_W<'a, const O: u8> =
    crate::FieldWriter<'a, u32, CTRL_SPEC, u8, EM4HVSCALE_A, 2, O>;
impl<'a, const O: u8> EM4HVSCALE_W<'a, O> {
    #[doc = "Voltage Scale Level 2"]
    #[inline(always)]
    pub fn vscale2(self) -> &'a mut W {
        self.variant(EM4HVSCALE_A::VSCALE2)
    }
    #[doc = "Voltage Scale Level 0"]
    #[inline(always)]
    pub fn vscale0(self) -> &'a mut W {
        self.variant(EM4HVSCALE_A::VSCALE0)
    }
    #[doc = "RESV"]
    #[inline(always)]
    pub fn resv(self) -> &'a mut W {
        self.variant(EM4HVSCALE_A::RESV)
    }
}
impl R {
    #[doc = "Bit 1 - Energy Mode 2 Block"]
    #[inline(always)]
    pub fn em2block(&self) -> EM2BLOCK_R {
        EM2BLOCK_R::new(((self.bits >> 1) & 1) != 0)
    }
    #[doc = "Bit 2 - Disable BOD in EM2"]
    #[inline(always)]
    pub fn em2boddis(&self) -> EM2BODDIS_R {
        EM2BODDIS_R::new(((self.bits >> 2) & 1) != 0)
    }
    #[doc = "Bit 3 - Reserved for internal use. Do not change."]
    #[inline(always)]
    pub fn em01ld(&self) -> EM01LD_R {
        EM01LD_R::new(((self.bits >> 3) & 1) != 0)
    }
    #[doc = "Bit 4 - Automatically Configures Flash and Frequency to Wakeup From EM2 or EM3 at Low Voltage"]
    #[inline(always)]
    pub fn em23vscaleautowsen(&self) -> EM23VSCALEAUTOWSEN_R {
        EM23VSCALEAUTOWSEN_R::new(((self.bits >> 4) & 1) != 0)
    }
    #[doc = "Bits 8:9 - EM23 Voltage Scale"]
    #[inline(always)]
    pub fn em23vscale(&self) -> EM23VSCALE_R {
        EM23VSCALE_R::new(((self.bits >> 8) & 3) as u8)
    }
    #[doc = "Bits 16:17 - EM4H Voltage Scale"]
    #[inline(always)]
    pub fn em4hvscale(&self) -> EM4HVSCALE_R {
        EM4HVSCALE_R::new(((self.bits >> 16) & 3) as u8)
    }
}
impl W {
    #[doc = "Bit 1 - Energy Mode 2 Block"]
    #[inline(always)]
    #[must_use]
    pub fn em2block(&mut self) -> EM2BLOCK_W<1> {
        EM2BLOCK_W::new(self)
    }
    #[doc = "Bit 2 - Disable BOD in EM2"]
    #[inline(always)]
    #[must_use]
    pub fn em2boddis(&mut self) -> EM2BODDIS_W<2> {
        EM2BODDIS_W::new(self)
    }
    #[doc = "Bit 3 - Reserved for internal use. Do not change."]
    #[inline(always)]
    #[must_use]
    pub fn em01ld(&mut self) -> EM01LD_W<3> {
        EM01LD_W::new(self)
    }
    #[doc = "Bit 4 - Automatically Configures Flash and Frequency to Wakeup From EM2 or EM3 at Low Voltage"]
    #[inline(always)]
    #[must_use]
    pub fn em23vscaleautowsen(&mut self) -> EM23VSCALEAUTOWSEN_W<4> {
        EM23VSCALEAUTOWSEN_W::new(self)
    }
    #[doc = "Bits 8:9 - EM23 Voltage Scale"]
    #[inline(always)]
    #[must_use]
    pub fn em23vscale(&mut self) -> EM23VSCALE_W<8> {
        EM23VSCALE_W::new(self)
    }
    #[doc = "Bits 16:17 - EM4H Voltage Scale"]
    #[inline(always)]
    #[must_use]
    pub fn em4hvscale(&mut self) -> EM4HVSCALE_W<16> {
        EM4HVSCALE_W::new(self)
    }
    #[doc = "Writes raw bits to the register."]
    #[inline(always)]
    pub unsafe fn bits(&mut self, bits: u32) -> &mut Self {
        self.0.bits(bits);
        self
    }
}
#[doc = "Control Register\n\nThis register you can [`read`](crate::generic::Reg::read), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [ctrl](index.html) module"]
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
#[doc = "`reset()` method sets CTRL to value 0"]
impl crate::Resettable for CTRL_SPEC {
    const RESET_VALUE: Self::Ux = 0;
}
