#[doc = "Register `DMCFG` reader"]
pub struct R(crate::R<DMCFG_SPEC>);
impl core::ops::Deref for R {
    type Target = crate::R<DMCFG_SPEC>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
impl From<crate::R<DMCFG_SPEC>> for R {
    #[inline(always)]
    fn from(reader: crate::R<DMCFG_SPEC>) -> Self {
        R(reader)
    }
}
#[doc = "Register `DMCFG` writer"]
pub struct W(crate::W<DMCFG_SPEC>);
impl core::ops::Deref for W {
    type Target = crate::W<DMCFG_SPEC>;
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
impl From<crate::W<DMCFG_SPEC>> for W {
    #[inline(always)]
    fn from(writer: crate::W<DMCFG_SPEC>) -> Self {
        W(writer)
    }
}
#[doc = "Field `DMG` reader - Delta Modulator Gain Step"]
pub type DMG_R = crate::FieldReader<u8, u8>;
#[doc = "Field `DMG` writer - Delta Modulator Gain Step"]
pub type DMG_W<'a> = crate::FieldWriter<'a, u32, DMCFG_SPEC, u8, u8, 8, 0>;
#[doc = "Field `DMR` reader - Delta Modulator Gain Reduction Interval"]
pub type DMR_R = crate::FieldReader<u8, u8>;
#[doc = "Field `DMR` writer - Delta Modulator Gain Reduction Interval"]
pub type DMR_W<'a> = crate::FieldWriter<'a, u32, DMCFG_SPEC, u8, u8, 4, 8>;
#[doc = "Field `DMCR` reader - Delta Modulator Conversion Rate"]
pub type DMCR_R = crate::FieldReader<u8, u8>;
#[doc = "Field `DMCR` writer - Delta Modulator Conversion Rate"]
pub type DMCR_W<'a> = crate::FieldWriter<'a, u32, DMCFG_SPEC, u8, u8, 4, 16>;
#[doc = "Delta Modulator Conversion Resolution.\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq)]
#[repr(u8)]
pub enum CRMODE_A {
    #[doc = "0: 10-bit delta modulator"]
    DM10 = 0,
    #[doc = "1: 12-bit delta modulator"]
    DM12 = 1,
    #[doc = "2: 14-bit delta modulator"]
    DM14 = 2,
    #[doc = "3: 16-bit delta modulator"]
    DM16 = 3,
}
impl From<CRMODE_A> for u8 {
    #[inline(always)]
    fn from(variant: CRMODE_A) -> Self {
        variant as _
    }
}
#[doc = "Field `CRMODE` reader - Delta Modulator Conversion Resolution."]
pub type CRMODE_R = crate::FieldReader<u8, CRMODE_A>;
impl CRMODE_R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> CRMODE_A {
        match self.bits {
            0 => CRMODE_A::DM10,
            1 => CRMODE_A::DM12,
            2 => CRMODE_A::DM14,
            3 => CRMODE_A::DM16,
            _ => unreachable!(),
        }
    }
    #[doc = "Checks if the value of the field is `DM10`"]
    #[inline(always)]
    pub fn is_dm10(&self) -> bool {
        *self == CRMODE_A::DM10
    }
    #[doc = "Checks if the value of the field is `DM12`"]
    #[inline(always)]
    pub fn is_dm12(&self) -> bool {
        *self == CRMODE_A::DM12
    }
    #[doc = "Checks if the value of the field is `DM14`"]
    #[inline(always)]
    pub fn is_dm14(&self) -> bool {
        *self == CRMODE_A::DM14
    }
    #[doc = "Checks if the value of the field is `DM16`"]
    #[inline(always)]
    pub fn is_dm16(&self) -> bool {
        *self == CRMODE_A::DM16
    }
}
#[doc = "Field `CRMODE` writer - Delta Modulator Conversion Resolution."]
pub type CRMODE_W<'a> = crate::FieldWriterSafe<'a, u32, DMCFG_SPEC, u8, CRMODE_A, 2, 20>;
impl<'a> CRMODE_W<'a> {
    #[doc = "10-bit delta modulator"]
    #[inline(always)]
    pub fn dm10(self) -> &'a mut W {
        self.variant(CRMODE_A::DM10)
    }
    #[doc = "12-bit delta modulator"]
    #[inline(always)]
    pub fn dm12(self) -> &'a mut W {
        self.variant(CRMODE_A::DM12)
    }
    #[doc = "14-bit delta modulator"]
    #[inline(always)]
    pub fn dm14(self) -> &'a mut W {
        self.variant(CRMODE_A::DM14)
    }
    #[doc = "16-bit delta modulator"]
    #[inline(always)]
    pub fn dm16(self) -> &'a mut W {
        self.variant(CRMODE_A::DM16)
    }
}
#[doc = "Field `DMGRDIS` reader - Delta Modulation Gain Step Reduction Disable"]
pub type DMGRDIS_R = crate::BitReader<bool>;
#[doc = "Field `DMGRDIS` writer - Delta Modulation Gain Step Reduction Disable"]
pub type DMGRDIS_W<'a> = crate::BitWriter<'a, u32, DMCFG_SPEC, bool, 28>;
impl R {
    #[doc = "Bits 0:7 - Delta Modulator Gain Step"]
    #[inline(always)]
    pub fn dmg(&self) -> DMG_R {
        DMG_R::new((self.bits & 0xff) as u8)
    }
    #[doc = "Bits 8:11 - Delta Modulator Gain Reduction Interval"]
    #[inline(always)]
    pub fn dmr(&self) -> DMR_R {
        DMR_R::new(((self.bits >> 8) & 0x0f) as u8)
    }
    #[doc = "Bits 16:19 - Delta Modulator Conversion Rate"]
    #[inline(always)]
    pub fn dmcr(&self) -> DMCR_R {
        DMCR_R::new(((self.bits >> 16) & 0x0f) as u8)
    }
    #[doc = "Bits 20:21 - Delta Modulator Conversion Resolution."]
    #[inline(always)]
    pub fn crmode(&self) -> CRMODE_R {
        CRMODE_R::new(((self.bits >> 20) & 3) as u8)
    }
    #[doc = "Bit 28 - Delta Modulation Gain Step Reduction Disable"]
    #[inline(always)]
    pub fn dmgrdis(&self) -> DMGRDIS_R {
        DMGRDIS_R::new(((self.bits >> 28) & 1) != 0)
    }
}
impl W {
    #[doc = "Bits 0:7 - Delta Modulator Gain Step"]
    #[inline(always)]
    pub fn dmg(&mut self) -> DMG_W {
        DMG_W::new(self)
    }
    #[doc = "Bits 8:11 - Delta Modulator Gain Reduction Interval"]
    #[inline(always)]
    pub fn dmr(&mut self) -> DMR_W {
        DMR_W::new(self)
    }
    #[doc = "Bits 16:19 - Delta Modulator Conversion Rate"]
    #[inline(always)]
    pub fn dmcr(&mut self) -> DMCR_W {
        DMCR_W::new(self)
    }
    #[doc = "Bits 20:21 - Delta Modulator Conversion Resolution."]
    #[inline(always)]
    pub fn crmode(&mut self) -> CRMODE_W {
        CRMODE_W::new(self)
    }
    #[doc = "Bit 28 - Delta Modulation Gain Step Reduction Disable"]
    #[inline(always)]
    pub fn dmgrdis(&mut self) -> DMGRDIS_W {
        DMGRDIS_W::new(self)
    }
    #[doc = "Writes raw bits to the register."]
    #[inline(always)]
    pub unsafe fn bits(&mut self, bits: u32) -> &mut Self {
        self.0.bits(bits);
        self
    }
}
#[doc = "Delta Modulation Configuration\n\nThis register you can [`read`](crate::generic::Reg::read), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [dmcfg](index.html) module"]
pub struct DMCFG_SPEC;
impl crate::RegisterSpec for DMCFG_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [dmcfg::R](R) reader structure"]
impl crate::Readable for DMCFG_SPEC {
    type Reader = R;
}
#[doc = "`write(|w| ..)` method takes [dmcfg::W](W) writer structure"]
impl crate::Writable for DMCFG_SPEC {
    type Writer = W;
}
#[doc = "`reset()` method sets DMCFG to value 0"]
impl crate::Resettable for DMCFG_SPEC {
    #[inline(always)]
    fn reset_value() -> Self::Ux {
        0
    }
}
