#[doc = "Register `EM23PERNORETAINSTATUS` reader"]
pub struct R(crate::R<EM23PERNORETAINSTATUS_SPEC>);
impl core::ops::Deref for R {
    type Target = crate::R<EM23PERNORETAINSTATUS_SPEC>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
impl From<crate::R<EM23PERNORETAINSTATUS_SPEC>> for R {
    #[inline(always)]
    fn from(reader: crate::R<EM23PERNORETAINSTATUS_SPEC>) -> Self {
        R(reader)
    }
}
#[doc = "Field `ACMP0LOCKED` reader - Indicates If ACMP0 Powered Down During EM23"]
pub type ACMP0LOCKED_R = crate::BitReader<bool>;
#[doc = "Field `ACMP1LOCKED` reader - Indicates If ACMP1 Powered Down During EM23"]
pub type ACMP1LOCKED_R = crate::BitReader<bool>;
#[doc = "Field `PCNT0LOCKED` reader - Indicates If PCNT0 Powered Down During EM23"]
pub type PCNT0LOCKED_R = crate::BitReader<bool>;
#[doc = "Field `I2C0LOCKED` reader - Indicates If I2C0 Powered Down During EM23"]
pub type I2C0LOCKED_R = crate::BitReader<bool>;
#[doc = "Field `I2C1LOCKED` reader - Indicates If I2C1 Powered Down During EM23"]
pub type I2C1LOCKED_R = crate::BitReader<bool>;
#[doc = "Field `DAC0LOCKED` reader - Indicates If DAC0 Powered Down During EM23"]
pub type DAC0LOCKED_R = crate::BitReader<bool>;
#[doc = "Field `ADC0LOCKED` reader - Indicates If ADC0 Powered Down During EM23"]
pub type ADC0LOCKED_R = crate::BitReader<bool>;
#[doc = "Field `LETIMER0LOCKED` reader - Indicates If LETIMER0 Powered Down During EM23"]
pub type LETIMER0LOCKED_R = crate::BitReader<bool>;
#[doc = "Field `WDOG0LOCKED` reader - Indicates If WDOG0 Powered Down During EM23"]
pub type WDOG0LOCKED_R = crate::BitReader<bool>;
#[doc = "Field `LESENSE0LOCKED` reader - Indicates If LESENSE0 Powered Down During EM23"]
pub type LESENSE0LOCKED_R = crate::BitReader<bool>;
#[doc = "Field `CSENLOCKED` reader - Indicates If CSEN Powered Down During EM23"]
pub type CSENLOCKED_R = crate::BitReader<bool>;
#[doc = "Field `LEUART0LOCKED` reader - Indicates If LEUART0 Powered Down During EM23"]
pub type LEUART0LOCKED_R = crate::BitReader<bool>;
#[doc = "Field `LCDLOCKED` reader - Indicates If LCD Powered Down During EM23"]
pub type LCDLOCKED_R = crate::BitReader<bool>;
impl R {
    #[doc = "Bit 0 - Indicates If ACMP0 Powered Down During EM23"]
    #[inline(always)]
    pub fn acmp0locked(&self) -> ACMP0LOCKED_R {
        ACMP0LOCKED_R::new((self.bits & 1) != 0)
    }
    #[doc = "Bit 1 - Indicates If ACMP1 Powered Down During EM23"]
    #[inline(always)]
    pub fn acmp1locked(&self) -> ACMP1LOCKED_R {
        ACMP1LOCKED_R::new(((self.bits >> 1) & 1) != 0)
    }
    #[doc = "Bit 2 - Indicates If PCNT0 Powered Down During EM23"]
    #[inline(always)]
    pub fn pcnt0locked(&self) -> PCNT0LOCKED_R {
        PCNT0LOCKED_R::new(((self.bits >> 2) & 1) != 0)
    }
    #[doc = "Bit 5 - Indicates If I2C0 Powered Down During EM23"]
    #[inline(always)]
    pub fn i2c0locked(&self) -> I2C0LOCKED_R {
        I2C0LOCKED_R::new(((self.bits >> 5) & 1) != 0)
    }
    #[doc = "Bit 6 - Indicates If I2C1 Powered Down During EM23"]
    #[inline(always)]
    pub fn i2c1locked(&self) -> I2C1LOCKED_R {
        I2C1LOCKED_R::new(((self.bits >> 6) & 1) != 0)
    }
    #[doc = "Bit 7 - Indicates If DAC0 Powered Down During EM23"]
    #[inline(always)]
    pub fn dac0locked(&self) -> DAC0LOCKED_R {
        DAC0LOCKED_R::new(((self.bits >> 7) & 1) != 0)
    }
    #[doc = "Bit 9 - Indicates If ADC0 Powered Down During EM23"]
    #[inline(always)]
    pub fn adc0locked(&self) -> ADC0LOCKED_R {
        ADC0LOCKED_R::new(((self.bits >> 9) & 1) != 0)
    }
    #[doc = "Bit 10 - Indicates If LETIMER0 Powered Down During EM23"]
    #[inline(always)]
    pub fn letimer0locked(&self) -> LETIMER0LOCKED_R {
        LETIMER0LOCKED_R::new(((self.bits >> 10) & 1) != 0)
    }
    #[doc = "Bit 11 - Indicates If WDOG0 Powered Down During EM23"]
    #[inline(always)]
    pub fn wdog0locked(&self) -> WDOG0LOCKED_R {
        WDOG0LOCKED_R::new(((self.bits >> 11) & 1) != 0)
    }
    #[doc = "Bit 13 - Indicates If LESENSE0 Powered Down During EM23"]
    #[inline(always)]
    pub fn lesense0locked(&self) -> LESENSE0LOCKED_R {
        LESENSE0LOCKED_R::new(((self.bits >> 13) & 1) != 0)
    }
    #[doc = "Bit 14 - Indicates If CSEN Powered Down During EM23"]
    #[inline(always)]
    pub fn csenlocked(&self) -> CSENLOCKED_R {
        CSENLOCKED_R::new(((self.bits >> 14) & 1) != 0)
    }
    #[doc = "Bit 15 - Indicates If LEUART0 Powered Down During EM23"]
    #[inline(always)]
    pub fn leuart0locked(&self) -> LEUART0LOCKED_R {
        LEUART0LOCKED_R::new(((self.bits >> 15) & 1) != 0)
    }
    #[doc = "Bit 17 - Indicates If LCD Powered Down During EM23"]
    #[inline(always)]
    pub fn lcdlocked(&self) -> LCDLOCKED_R {
        LCDLOCKED_R::new(((self.bits >> 17) & 1) != 0)
    }
}
#[doc = "Status Indicating If Peripherals Were Powered Down in EM23, Subsequently Locking Access to It\n\nThis register you can [`read`](crate::generic::Reg::read). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [em23pernoretainstatus](index.html) module"]
pub struct EM23PERNORETAINSTATUS_SPEC;
impl crate::RegisterSpec for EM23PERNORETAINSTATUS_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [em23pernoretainstatus::R](R) reader structure"]
impl crate::Readable for EM23PERNORETAINSTATUS_SPEC {
    type Reader = R;
}
#[doc = "`reset()` method sets EM23PERNORETAINSTATUS to value 0"]
impl crate::Resettable for EM23PERNORETAINSTATUS_SPEC {
    const RESET_VALUE: Self::Ux = 0;
}
