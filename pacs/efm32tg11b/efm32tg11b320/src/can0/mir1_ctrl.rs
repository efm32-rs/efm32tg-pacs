#[doc = "Register `MIR1_CTRL` reader"]
pub struct R(crate::R<MIR1_CTRL_SPEC>);
impl core::ops::Deref for R {
    type Target = crate::R<MIR1_CTRL_SPEC>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
impl From<crate::R<MIR1_CTRL_SPEC>> for R {
    #[inline(always)]
    fn from(reader: crate::R<MIR1_CTRL_SPEC>) -> Self {
        R(reader)
    }
}
#[doc = "Register `MIR1_CTRL` writer"]
pub struct W(crate::W<MIR1_CTRL_SPEC>);
impl core::ops::Deref for W {
    type Target = crate::W<MIR1_CTRL_SPEC>;
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
impl From<crate::W<MIR1_CTRL_SPEC>> for W {
    #[inline(always)]
    fn from(writer: crate::W<MIR1_CTRL_SPEC>) -> Self {
        W(writer)
    }
}
#[doc = "Field `DLC` reader - Data Length Code"]
pub type DLC_R = crate::FieldReader<u8, u8>;
#[doc = "Field `DLC` writer - Data Length Code"]
pub type DLC_W<'a> = crate::FieldWriter<'a, u32, MIR1_CTRL_SPEC, u8, u8, 4, 0>;
#[doc = "Field `EOB` reader - End of Buffer"]
pub type EOB_R = crate::BitReader<bool>;
#[doc = "Field `EOB` writer - End of Buffer"]
pub type EOB_W<'a> = crate::BitWriter<'a, u32, MIR1_CTRL_SPEC, bool, 7>;
#[doc = "Field `TXRQST` reader - Transmit Request"]
pub type TXRQST_R = crate::BitReader<bool>;
#[doc = "Field `TXRQST` writer - Transmit Request"]
pub type TXRQST_W<'a> = crate::BitWriter<'a, u32, MIR1_CTRL_SPEC, bool, 8>;
#[doc = "Field `RMTEN` reader - Remote Enable"]
pub type RMTEN_R = crate::BitReader<bool>;
#[doc = "Field `RMTEN` writer - Remote Enable"]
pub type RMTEN_W<'a> = crate::BitWriter<'a, u32, MIR1_CTRL_SPEC, bool, 9>;
#[doc = "Field `RXIE` reader - Receive Interrupt Enable"]
pub type RXIE_R = crate::BitReader<bool>;
#[doc = "Field `RXIE` writer - Receive Interrupt Enable"]
pub type RXIE_W<'a> = crate::BitWriter<'a, u32, MIR1_CTRL_SPEC, bool, 10>;
#[doc = "Field `TXIE` reader - Transmit Interrupt Enable"]
pub type TXIE_R = crate::BitReader<bool>;
#[doc = "Field `TXIE` writer - Transmit Interrupt Enable"]
pub type TXIE_W<'a> = crate::BitWriter<'a, u32, MIR1_CTRL_SPEC, bool, 11>;
#[doc = "Field `UMASK` reader - Use Acceptance Mask"]
pub type UMASK_R = crate::BitReader<bool>;
#[doc = "Field `UMASK` writer - Use Acceptance Mask"]
pub type UMASK_W<'a> = crate::BitWriter<'a, u32, MIR1_CTRL_SPEC, bool, 12>;
#[doc = "Field `INTPND` reader - Interrupt Pending"]
pub type INTPND_R = crate::BitReader<bool>;
#[doc = "Field `INTPND` writer - Interrupt Pending"]
pub type INTPND_W<'a> = crate::BitWriter<'a, u32, MIR1_CTRL_SPEC, bool, 13>;
#[doc = "Field `MESSAGEOF` reader - Message Lost (only Valid for Message Objects With Direction = Receive)"]
pub type MESSAGEOF_R = crate::BitReader<bool>;
#[doc = "Field `MESSAGEOF` writer - Message Lost (only Valid for Message Objects With Direction = Receive)"]
pub type MESSAGEOF_W<'a> = crate::BitWriter<'a, u32, MIR1_CTRL_SPEC, bool, 14>;
#[doc = "Field `DATAVALID` reader - New Data"]
pub type DATAVALID_R = crate::BitReader<bool>;
#[doc = "Field `DATAVALID` writer - New Data"]
pub type DATAVALID_W<'a> = crate::BitWriter<'a, u32, MIR1_CTRL_SPEC, bool, 15>;
impl R {
    #[doc = "Bits 0:3 - Data Length Code"]
    #[inline(always)]
    pub fn dlc(&self) -> DLC_R {
        DLC_R::new((self.bits & 0x0f) as u8)
    }
    #[doc = "Bit 7 - End of Buffer"]
    #[inline(always)]
    pub fn eob(&self) -> EOB_R {
        EOB_R::new(((self.bits >> 7) & 1) != 0)
    }
    #[doc = "Bit 8 - Transmit Request"]
    #[inline(always)]
    pub fn txrqst(&self) -> TXRQST_R {
        TXRQST_R::new(((self.bits >> 8) & 1) != 0)
    }
    #[doc = "Bit 9 - Remote Enable"]
    #[inline(always)]
    pub fn rmten(&self) -> RMTEN_R {
        RMTEN_R::new(((self.bits >> 9) & 1) != 0)
    }
    #[doc = "Bit 10 - Receive Interrupt Enable"]
    #[inline(always)]
    pub fn rxie(&self) -> RXIE_R {
        RXIE_R::new(((self.bits >> 10) & 1) != 0)
    }
    #[doc = "Bit 11 - Transmit Interrupt Enable"]
    #[inline(always)]
    pub fn txie(&self) -> TXIE_R {
        TXIE_R::new(((self.bits >> 11) & 1) != 0)
    }
    #[doc = "Bit 12 - Use Acceptance Mask"]
    #[inline(always)]
    pub fn umask(&self) -> UMASK_R {
        UMASK_R::new(((self.bits >> 12) & 1) != 0)
    }
    #[doc = "Bit 13 - Interrupt Pending"]
    #[inline(always)]
    pub fn intpnd(&self) -> INTPND_R {
        INTPND_R::new(((self.bits >> 13) & 1) != 0)
    }
    #[doc = "Bit 14 - Message Lost (only Valid for Message Objects With Direction = Receive)"]
    #[inline(always)]
    pub fn messageof(&self) -> MESSAGEOF_R {
        MESSAGEOF_R::new(((self.bits >> 14) & 1) != 0)
    }
    #[doc = "Bit 15 - New Data"]
    #[inline(always)]
    pub fn datavalid(&self) -> DATAVALID_R {
        DATAVALID_R::new(((self.bits >> 15) & 1) != 0)
    }
}
impl W {
    #[doc = "Bits 0:3 - Data Length Code"]
    #[inline(always)]
    pub fn dlc(&mut self) -> DLC_W {
        DLC_W::new(self)
    }
    #[doc = "Bit 7 - End of Buffer"]
    #[inline(always)]
    pub fn eob(&mut self) -> EOB_W {
        EOB_W::new(self)
    }
    #[doc = "Bit 8 - Transmit Request"]
    #[inline(always)]
    pub fn txrqst(&mut self) -> TXRQST_W {
        TXRQST_W::new(self)
    }
    #[doc = "Bit 9 - Remote Enable"]
    #[inline(always)]
    pub fn rmten(&mut self) -> RMTEN_W {
        RMTEN_W::new(self)
    }
    #[doc = "Bit 10 - Receive Interrupt Enable"]
    #[inline(always)]
    pub fn rxie(&mut self) -> RXIE_W {
        RXIE_W::new(self)
    }
    #[doc = "Bit 11 - Transmit Interrupt Enable"]
    #[inline(always)]
    pub fn txie(&mut self) -> TXIE_W {
        TXIE_W::new(self)
    }
    #[doc = "Bit 12 - Use Acceptance Mask"]
    #[inline(always)]
    pub fn umask(&mut self) -> UMASK_W {
        UMASK_W::new(self)
    }
    #[doc = "Bit 13 - Interrupt Pending"]
    #[inline(always)]
    pub fn intpnd(&mut self) -> INTPND_W {
        INTPND_W::new(self)
    }
    #[doc = "Bit 14 - Message Lost (only Valid for Message Objects With Direction = Receive)"]
    #[inline(always)]
    pub fn messageof(&mut self) -> MESSAGEOF_W {
        MESSAGEOF_W::new(self)
    }
    #[doc = "Bit 15 - New Data"]
    #[inline(always)]
    pub fn datavalid(&mut self) -> DATAVALID_W {
        DATAVALID_W::new(self)
    }
    #[doc = "Writes raw bits to the register."]
    #[inline(always)]
    pub unsafe fn bits(&mut self, bits: u32) -> &mut Self {
        self.0.bits(bits);
        self
    }
}
#[doc = "Interface Message Control Register\n\nThis register you can [`read`](crate::generic::Reg::read), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [mir1_ctrl](index.html) module"]
pub struct MIR1_CTRL_SPEC;
impl crate::RegisterSpec for MIR1_CTRL_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [mir1_ctrl::R](R) reader structure"]
impl crate::Readable for MIR1_CTRL_SPEC {
    type Reader = R;
}
#[doc = "`write(|w| ..)` method takes [mir1_ctrl::W](W) writer structure"]
impl crate::Writable for MIR1_CTRL_SPEC {
    type Writer = W;
}
#[doc = "`reset()` method sets MIR1_CTRL to value 0"]
impl crate::Resettable for MIR1_CTRL_SPEC {
    #[inline(always)]
    fn reset_value() -> Self::Ux {
        0
    }
}
