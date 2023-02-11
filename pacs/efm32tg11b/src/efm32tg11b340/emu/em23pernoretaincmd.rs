#[doc = "Register `EM23PERNORETAINCMD` writer"]
pub struct W(crate::W<EM23PERNORETAINCMD_SPEC>);
impl core::ops::Deref for W {
    type Target = crate::W<EM23PERNORETAINCMD_SPEC>;
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
impl From<crate::W<EM23PERNORETAINCMD_SPEC>> for W {
    #[inline(always)]
    fn from(writer: crate::W<EM23PERNORETAINCMD_SPEC>) -> Self {
        W(writer)
    }
}
#[doc = "Field `ACMP0UNLOCK` writer - Clears Status Bit of ACMP0 and Unlocks Access to It"]
pub type ACMP0UNLOCK_W<'a, const O: u8> =
    crate::BitWriter<'a, u32, EM23PERNORETAINCMD_SPEC, bool, O>;
#[doc = "Field `ACMP1UNLOCK` writer - Clears Status Bit of ACMP1 and Unlocks Access to It"]
pub type ACMP1UNLOCK_W<'a, const O: u8> =
    crate::BitWriter<'a, u32, EM23PERNORETAINCMD_SPEC, bool, O>;
#[doc = "Field `PCNT0UNLOCK` writer - Clears Status Bit of PCNT0 and Unlocks Access to It"]
pub type PCNT0UNLOCK_W<'a, const O: u8> =
    crate::BitWriter<'a, u32, EM23PERNORETAINCMD_SPEC, bool, O>;
#[doc = "Field `I2C0UNLOCK` writer - Clears Status Bit of I2C0 and Unlocks Access to It"]
pub type I2C0UNLOCK_W<'a, const O: u8> =
    crate::BitWriter<'a, u32, EM23PERNORETAINCMD_SPEC, bool, O>;
#[doc = "Field `I2C1UNLOCK` writer - Clears Status Bit of I2C1 and Unlocks Access to It"]
pub type I2C1UNLOCK_W<'a, const O: u8> =
    crate::BitWriter<'a, u32, EM23PERNORETAINCMD_SPEC, bool, O>;
#[doc = "Field `DAC0UNLOCK` writer - Clears Status Bit of DAC0 and Unlocks Access to It"]
pub type DAC0UNLOCK_W<'a, const O: u8> =
    crate::BitWriter<'a, u32, EM23PERNORETAINCMD_SPEC, bool, O>;
#[doc = "Field `ADC0UNLOCK` writer - Clears Status Bit of ADC0 and Unlocks Access to It"]
pub type ADC0UNLOCK_W<'a, const O: u8> =
    crate::BitWriter<'a, u32, EM23PERNORETAINCMD_SPEC, bool, O>;
#[doc = "Field `LETIMER0UNLOCK` writer - Clears Status Bit of LETIMER0 and Unlocks Access to It"]
pub type LETIMER0UNLOCK_W<'a, const O: u8> =
    crate::BitWriter<'a, u32, EM23PERNORETAINCMD_SPEC, bool, O>;
#[doc = "Field `WDOG0UNLOCK` writer - Clears Status Bit of WDOG0 and Unlocks Access to It"]
pub type WDOG0UNLOCK_W<'a, const O: u8> =
    crate::BitWriter<'a, u32, EM23PERNORETAINCMD_SPEC, bool, O>;
#[doc = "Field `LESENSE0UNLOCK` writer - Clears Status Bit of LESENSE0 and Unlocks Access to It"]
pub type LESENSE0UNLOCK_W<'a, const O: u8> =
    crate::BitWriter<'a, u32, EM23PERNORETAINCMD_SPEC, bool, O>;
#[doc = "Field `CSENUNLOCK` writer - Clears Status Bit of CSEN and Unlocks Access to It"]
pub type CSENUNLOCK_W<'a, const O: u8> =
    crate::BitWriter<'a, u32, EM23PERNORETAINCMD_SPEC, bool, O>;
#[doc = "Field `LEUART0UNLOCK` writer - Clears Status Bit of LEUART0 and Unlocks Access to It"]
pub type LEUART0UNLOCK_W<'a, const O: u8> =
    crate::BitWriter<'a, u32, EM23PERNORETAINCMD_SPEC, bool, O>;
#[doc = "Field `LCDUNLOCK` writer - Clears Status Bit of LCD and Unlocks Access to It"]
pub type LCDUNLOCK_W<'a, const O: u8> = crate::BitWriter<'a, u32, EM23PERNORETAINCMD_SPEC, bool, O>;
impl W {
    #[doc = "Bit 0 - Clears Status Bit of ACMP0 and Unlocks Access to It"]
    #[inline(always)]
    #[must_use]
    pub fn acmp0unlock(&mut self) -> ACMP0UNLOCK_W<0> {
        ACMP0UNLOCK_W::new(self)
    }
    #[doc = "Bit 1 - Clears Status Bit of ACMP1 and Unlocks Access to It"]
    #[inline(always)]
    #[must_use]
    pub fn acmp1unlock(&mut self) -> ACMP1UNLOCK_W<1> {
        ACMP1UNLOCK_W::new(self)
    }
    #[doc = "Bit 2 - Clears Status Bit of PCNT0 and Unlocks Access to It"]
    #[inline(always)]
    #[must_use]
    pub fn pcnt0unlock(&mut self) -> PCNT0UNLOCK_W<2> {
        PCNT0UNLOCK_W::new(self)
    }
    #[doc = "Bit 5 - Clears Status Bit of I2C0 and Unlocks Access to It"]
    #[inline(always)]
    #[must_use]
    pub fn i2c0unlock(&mut self) -> I2C0UNLOCK_W<5> {
        I2C0UNLOCK_W::new(self)
    }
    #[doc = "Bit 6 - Clears Status Bit of I2C1 and Unlocks Access to It"]
    #[inline(always)]
    #[must_use]
    pub fn i2c1unlock(&mut self) -> I2C1UNLOCK_W<6> {
        I2C1UNLOCK_W::new(self)
    }
    #[doc = "Bit 7 - Clears Status Bit of DAC0 and Unlocks Access to It"]
    #[inline(always)]
    #[must_use]
    pub fn dac0unlock(&mut self) -> DAC0UNLOCK_W<7> {
        DAC0UNLOCK_W::new(self)
    }
    #[doc = "Bit 9 - Clears Status Bit of ADC0 and Unlocks Access to It"]
    #[inline(always)]
    #[must_use]
    pub fn adc0unlock(&mut self) -> ADC0UNLOCK_W<9> {
        ADC0UNLOCK_W::new(self)
    }
    #[doc = "Bit 10 - Clears Status Bit of LETIMER0 and Unlocks Access to It"]
    #[inline(always)]
    #[must_use]
    pub fn letimer0unlock(&mut self) -> LETIMER0UNLOCK_W<10> {
        LETIMER0UNLOCK_W::new(self)
    }
    #[doc = "Bit 11 - Clears Status Bit of WDOG0 and Unlocks Access to It"]
    #[inline(always)]
    #[must_use]
    pub fn wdog0unlock(&mut self) -> WDOG0UNLOCK_W<11> {
        WDOG0UNLOCK_W::new(self)
    }
    #[doc = "Bit 13 - Clears Status Bit of LESENSE0 and Unlocks Access to It"]
    #[inline(always)]
    #[must_use]
    pub fn lesense0unlock(&mut self) -> LESENSE0UNLOCK_W<13> {
        LESENSE0UNLOCK_W::new(self)
    }
    #[doc = "Bit 14 - Clears Status Bit of CSEN and Unlocks Access to It"]
    #[inline(always)]
    #[must_use]
    pub fn csenunlock(&mut self) -> CSENUNLOCK_W<14> {
        CSENUNLOCK_W::new(self)
    }
    #[doc = "Bit 15 - Clears Status Bit of LEUART0 and Unlocks Access to It"]
    #[inline(always)]
    #[must_use]
    pub fn leuart0unlock(&mut self) -> LEUART0UNLOCK_W<15> {
        LEUART0UNLOCK_W::new(self)
    }
    #[doc = "Bit 17 - Clears Status Bit of LCD and Unlocks Access to It"]
    #[inline(always)]
    #[must_use]
    pub fn lcdunlock(&mut self) -> LCDUNLOCK_W<17> {
        LCDUNLOCK_W::new(self)
    }
    #[doc = "Writes raw bits to the register."]
    #[inline(always)]
    pub unsafe fn bits(&mut self, bits: u32) -> &mut Self {
        self.0.bits(bits);
        self
    }
}
#[doc = "Clears Corresponding Bits in EM23PERNORETAINSTATUS Unlocking Access to Peripheral\n\nThis register you can [`write_with_zero`](crate::generic::Reg::write_with_zero), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [em23pernoretaincmd](index.html) module"]
pub struct EM23PERNORETAINCMD_SPEC;
impl crate::RegisterSpec for EM23PERNORETAINCMD_SPEC {
    type Ux = u32;
}
#[doc = "`write(|w| ..)` method takes [em23pernoretaincmd::W](W) writer structure"]
impl crate::Writable for EM23PERNORETAINCMD_SPEC {
    type Writer = W;
    const ZERO_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
}
#[doc = "`reset()` method sets EM23PERNORETAINCMD to value 0"]
impl crate::Resettable for EM23PERNORETAINCMD_SPEC {
    const RESET_VALUE: Self::Ux = 0;
}
