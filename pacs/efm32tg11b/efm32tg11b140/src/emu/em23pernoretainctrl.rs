#[doc = "Register `EM23PERNORETAINCTRL` reader"]
pub struct R(crate::R<EM23PERNORETAINCTRL_SPEC>);
impl core::ops::Deref for R {
    type Target = crate::R<EM23PERNORETAINCTRL_SPEC>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
impl From<crate::R<EM23PERNORETAINCTRL_SPEC>> for R {
    #[inline(always)]
    fn from(reader: crate::R<EM23PERNORETAINCTRL_SPEC>) -> Self {
        R(reader)
    }
}
#[doc = "Register `EM23PERNORETAINCTRL` writer"]
pub struct W(crate::W<EM23PERNORETAINCTRL_SPEC>);
impl core::ops::Deref for W {
    type Target = crate::W<EM23PERNORETAINCTRL_SPEC>;
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
impl From<crate::W<EM23PERNORETAINCTRL_SPEC>> for W {
    #[inline(always)]
    fn from(writer: crate::W<EM23PERNORETAINCTRL_SPEC>) -> Self {
        W(writer)
    }
}
#[doc = "Field `ACMP0DIS` reader - Allow Power Down of ACMP0 During EM23"]
pub type ACMP0DIS_R = crate::BitReader<bool>;
#[doc = "Field `ACMP0DIS` writer - Allow Power Down of ACMP0 During EM23"]
pub type ACMP0DIS_W<'a> = crate::BitWriter<'a, u32, EM23PERNORETAINCTRL_SPEC, bool, 0>;
#[doc = "Field `ACMP1DIS` reader - Allow Power Down of ACMP1 During EM23"]
pub type ACMP1DIS_R = crate::BitReader<bool>;
#[doc = "Field `ACMP1DIS` writer - Allow Power Down of ACMP1 During EM23"]
pub type ACMP1DIS_W<'a> = crate::BitWriter<'a, u32, EM23PERNORETAINCTRL_SPEC, bool, 1>;
#[doc = "Field `PCNT0DIS` reader - Allow Power Down of PCNT0 During EM23"]
pub type PCNT0DIS_R = crate::BitReader<bool>;
#[doc = "Field `PCNT0DIS` writer - Allow Power Down of PCNT0 During EM23"]
pub type PCNT0DIS_W<'a> = crate::BitWriter<'a, u32, EM23PERNORETAINCTRL_SPEC, bool, 2>;
#[doc = "Field `I2C0DIS` reader - Allow Power Down of I2C0 During EM23"]
pub type I2C0DIS_R = crate::BitReader<bool>;
#[doc = "Field `I2C0DIS` writer - Allow Power Down of I2C0 During EM23"]
pub type I2C0DIS_W<'a> = crate::BitWriter<'a, u32, EM23PERNORETAINCTRL_SPEC, bool, 5>;
#[doc = "Field `I2C1DIS` reader - Allow Power Down of I2C1 During EM23"]
pub type I2C1DIS_R = crate::BitReader<bool>;
#[doc = "Field `I2C1DIS` writer - Allow Power Down of I2C1 During EM23"]
pub type I2C1DIS_W<'a> = crate::BitWriter<'a, u32, EM23PERNORETAINCTRL_SPEC, bool, 6>;
#[doc = "Field `VDAC0DIS` reader - Allow Power Down of DAC0 During EM23"]
pub type VDAC0DIS_R = crate::BitReader<bool>;
#[doc = "Field `VDAC0DIS` writer - Allow Power Down of DAC0 During EM23"]
pub type VDAC0DIS_W<'a> = crate::BitWriter<'a, u32, EM23PERNORETAINCTRL_SPEC, bool, 7>;
#[doc = "Field `ADC0DIS` reader - Allow Power Down of ADC0 During EM23"]
pub type ADC0DIS_R = crate::BitReader<bool>;
#[doc = "Field `ADC0DIS` writer - Allow Power Down of ADC0 During EM23"]
pub type ADC0DIS_W<'a> = crate::BitWriter<'a, u32, EM23PERNORETAINCTRL_SPEC, bool, 9>;
#[doc = "Field `LETIMER0DIS` reader - Allow Power Down of LETIMER0 During EM23"]
pub type LETIMER0DIS_R = crate::BitReader<bool>;
#[doc = "Field `LETIMER0DIS` writer - Allow Power Down of LETIMER0 During EM23"]
pub type LETIMER0DIS_W<'a> = crate::BitWriter<'a, u32, EM23PERNORETAINCTRL_SPEC, bool, 10>;
#[doc = "Field `WDOG0DIS` reader - Allow Power Down of WDOG0 During EM23"]
pub type WDOG0DIS_R = crate::BitReader<bool>;
#[doc = "Field `WDOG0DIS` writer - Allow Power Down of WDOG0 During EM23"]
pub type WDOG0DIS_W<'a> = crate::BitWriter<'a, u32, EM23PERNORETAINCTRL_SPEC, bool, 11>;
#[doc = "Field `LESENSE0DIS` reader - Allow Power Down of LESENSE0 During EM23"]
pub type LESENSE0DIS_R = crate::BitReader<bool>;
#[doc = "Field `LESENSE0DIS` writer - Allow Power Down of LESENSE0 During EM23"]
pub type LESENSE0DIS_W<'a> = crate::BitWriter<'a, u32, EM23PERNORETAINCTRL_SPEC, bool, 13>;
#[doc = "Field `CSENDIS` reader - Allow Power Down of CSEN During EM23"]
pub type CSENDIS_R = crate::BitReader<bool>;
#[doc = "Field `CSENDIS` writer - Allow Power Down of CSEN During EM23"]
pub type CSENDIS_W<'a> = crate::BitWriter<'a, u32, EM23PERNORETAINCTRL_SPEC, bool, 14>;
#[doc = "Field `LEUART0DIS` reader - Allow Power Down of LEUART0 During EM23"]
pub type LEUART0DIS_R = crate::BitReader<bool>;
#[doc = "Field `LEUART0DIS` writer - Allow Power Down of LEUART0 During EM23"]
pub type LEUART0DIS_W<'a> = crate::BitWriter<'a, u32, EM23PERNORETAINCTRL_SPEC, bool, 15>;
#[doc = "Field `LCDDIS` reader - Allow Power Down of LCD During EM23"]
pub type LCDDIS_R = crate::BitReader<bool>;
#[doc = "Field `LCDDIS` writer - Allow Power Down of LCD During EM23"]
pub type LCDDIS_W<'a> = crate::BitWriter<'a, u32, EM23PERNORETAINCTRL_SPEC, bool, 17>;
impl R {
    #[doc = "Bit 0 - Allow Power Down of ACMP0 During EM23"]
    #[inline(always)]
    pub fn acmp0dis(&self) -> ACMP0DIS_R {
        ACMP0DIS_R::new((self.bits & 1) != 0)
    }
    #[doc = "Bit 1 - Allow Power Down of ACMP1 During EM23"]
    #[inline(always)]
    pub fn acmp1dis(&self) -> ACMP1DIS_R {
        ACMP1DIS_R::new(((self.bits >> 1) & 1) != 0)
    }
    #[doc = "Bit 2 - Allow Power Down of PCNT0 During EM23"]
    #[inline(always)]
    pub fn pcnt0dis(&self) -> PCNT0DIS_R {
        PCNT0DIS_R::new(((self.bits >> 2) & 1) != 0)
    }
    #[doc = "Bit 5 - Allow Power Down of I2C0 During EM23"]
    #[inline(always)]
    pub fn i2c0dis(&self) -> I2C0DIS_R {
        I2C0DIS_R::new(((self.bits >> 5) & 1) != 0)
    }
    #[doc = "Bit 6 - Allow Power Down of I2C1 During EM23"]
    #[inline(always)]
    pub fn i2c1dis(&self) -> I2C1DIS_R {
        I2C1DIS_R::new(((self.bits >> 6) & 1) != 0)
    }
    #[doc = "Bit 7 - Allow Power Down of DAC0 During EM23"]
    #[inline(always)]
    pub fn vdac0dis(&self) -> VDAC0DIS_R {
        VDAC0DIS_R::new(((self.bits >> 7) & 1) != 0)
    }
    #[doc = "Bit 9 - Allow Power Down of ADC0 During EM23"]
    #[inline(always)]
    pub fn adc0dis(&self) -> ADC0DIS_R {
        ADC0DIS_R::new(((self.bits >> 9) & 1) != 0)
    }
    #[doc = "Bit 10 - Allow Power Down of LETIMER0 During EM23"]
    #[inline(always)]
    pub fn letimer0dis(&self) -> LETIMER0DIS_R {
        LETIMER0DIS_R::new(((self.bits >> 10) & 1) != 0)
    }
    #[doc = "Bit 11 - Allow Power Down of WDOG0 During EM23"]
    #[inline(always)]
    pub fn wdog0dis(&self) -> WDOG0DIS_R {
        WDOG0DIS_R::new(((self.bits >> 11) & 1) != 0)
    }
    #[doc = "Bit 13 - Allow Power Down of LESENSE0 During EM23"]
    #[inline(always)]
    pub fn lesense0dis(&self) -> LESENSE0DIS_R {
        LESENSE0DIS_R::new(((self.bits >> 13) & 1) != 0)
    }
    #[doc = "Bit 14 - Allow Power Down of CSEN During EM23"]
    #[inline(always)]
    pub fn csendis(&self) -> CSENDIS_R {
        CSENDIS_R::new(((self.bits >> 14) & 1) != 0)
    }
    #[doc = "Bit 15 - Allow Power Down of LEUART0 During EM23"]
    #[inline(always)]
    pub fn leuart0dis(&self) -> LEUART0DIS_R {
        LEUART0DIS_R::new(((self.bits >> 15) & 1) != 0)
    }
    #[doc = "Bit 17 - Allow Power Down of LCD During EM23"]
    #[inline(always)]
    pub fn lcddis(&self) -> LCDDIS_R {
        LCDDIS_R::new(((self.bits >> 17) & 1) != 0)
    }
}
impl W {
    #[doc = "Bit 0 - Allow Power Down of ACMP0 During EM23"]
    #[inline(always)]
    pub fn acmp0dis(&mut self) -> ACMP0DIS_W {
        ACMP0DIS_W::new(self)
    }
    #[doc = "Bit 1 - Allow Power Down of ACMP1 During EM23"]
    #[inline(always)]
    pub fn acmp1dis(&mut self) -> ACMP1DIS_W {
        ACMP1DIS_W::new(self)
    }
    #[doc = "Bit 2 - Allow Power Down of PCNT0 During EM23"]
    #[inline(always)]
    pub fn pcnt0dis(&mut self) -> PCNT0DIS_W {
        PCNT0DIS_W::new(self)
    }
    #[doc = "Bit 5 - Allow Power Down of I2C0 During EM23"]
    #[inline(always)]
    pub fn i2c0dis(&mut self) -> I2C0DIS_W {
        I2C0DIS_W::new(self)
    }
    #[doc = "Bit 6 - Allow Power Down of I2C1 During EM23"]
    #[inline(always)]
    pub fn i2c1dis(&mut self) -> I2C1DIS_W {
        I2C1DIS_W::new(self)
    }
    #[doc = "Bit 7 - Allow Power Down of DAC0 During EM23"]
    #[inline(always)]
    pub fn vdac0dis(&mut self) -> VDAC0DIS_W {
        VDAC0DIS_W::new(self)
    }
    #[doc = "Bit 9 - Allow Power Down of ADC0 During EM23"]
    #[inline(always)]
    pub fn adc0dis(&mut self) -> ADC0DIS_W {
        ADC0DIS_W::new(self)
    }
    #[doc = "Bit 10 - Allow Power Down of LETIMER0 During EM23"]
    #[inline(always)]
    pub fn letimer0dis(&mut self) -> LETIMER0DIS_W {
        LETIMER0DIS_W::new(self)
    }
    #[doc = "Bit 11 - Allow Power Down of WDOG0 During EM23"]
    #[inline(always)]
    pub fn wdog0dis(&mut self) -> WDOG0DIS_W {
        WDOG0DIS_W::new(self)
    }
    #[doc = "Bit 13 - Allow Power Down of LESENSE0 During EM23"]
    #[inline(always)]
    pub fn lesense0dis(&mut self) -> LESENSE0DIS_W {
        LESENSE0DIS_W::new(self)
    }
    #[doc = "Bit 14 - Allow Power Down of CSEN During EM23"]
    #[inline(always)]
    pub fn csendis(&mut self) -> CSENDIS_W {
        CSENDIS_W::new(self)
    }
    #[doc = "Bit 15 - Allow Power Down of LEUART0 During EM23"]
    #[inline(always)]
    pub fn leuart0dis(&mut self) -> LEUART0DIS_W {
        LEUART0DIS_W::new(self)
    }
    #[doc = "Bit 17 - Allow Power Down of LCD During EM23"]
    #[inline(always)]
    pub fn lcddis(&mut self) -> LCDDIS_W {
        LCDDIS_W::new(self)
    }
    #[doc = "Writes raw bits to the register."]
    #[inline(always)]
    pub unsafe fn bits(&mut self, bits: u32) -> &mut Self {
        self.0.bits(bits);
        self
    }
}
#[doc = "When Set Corresponding Peripherals May Get Powered Down in EM23\n\nThis register you can [`read`](crate::generic::Reg::read), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [em23pernoretainctrl](index.html) module"]
pub struct EM23PERNORETAINCTRL_SPEC;
impl crate::RegisterSpec for EM23PERNORETAINCTRL_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [em23pernoretainctrl::R](R) reader structure"]
impl crate::Readable for EM23PERNORETAINCTRL_SPEC {
    type Reader = R;
}
#[doc = "`write(|w| ..)` method takes [em23pernoretainctrl::W](W) writer structure"]
impl crate::Writable for EM23PERNORETAINCTRL_SPEC {
    type Writer = W;
}
#[doc = "`reset()` method sets EM23PERNORETAINCTRL to value 0"]
impl crate::Resettable for EM23PERNORETAINCTRL_SPEC {
    #[inline(always)]
    fn reset_value() -> Self::Ux {
        0
    }
}
