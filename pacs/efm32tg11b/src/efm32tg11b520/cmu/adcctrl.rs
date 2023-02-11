#[doc = "Register `ADCCTRL` reader"]
pub struct R(crate::R<ADCCTRL_SPEC>);
impl core::ops::Deref for R {
    type Target = crate::R<ADCCTRL_SPEC>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
impl From<crate::R<ADCCTRL_SPEC>> for R {
    #[inline(always)]
    fn from(reader: crate::R<ADCCTRL_SPEC>) -> Self {
        R(reader)
    }
}
#[doc = "Register `ADCCTRL` writer"]
pub struct W(crate::W<ADCCTRL_SPEC>);
impl core::ops::Deref for W {
    type Target = crate::W<ADCCTRL_SPEC>;
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
impl From<crate::W<ADCCTRL_SPEC>> for W {
    #[inline(always)]
    fn from(writer: crate::W<ADCCTRL_SPEC>) -> Self {
        W(writer)
    }
}
#[doc = "Field `ADC0CLKDIV` reader - ADC0 Clock Prescaler"]
pub type ADC0CLKDIV_R = crate::FieldReader<u8, ADC0CLKDIV_A>;
#[doc = "ADC0 Clock Prescaler\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
#[repr(u8)]
pub enum ADC0CLKDIV_A {
    #[doc = "0: `0`"]
    NODIVISION = 0,
}
impl From<ADC0CLKDIV_A> for u8 {
    #[inline(always)]
    fn from(variant: ADC0CLKDIV_A) -> Self {
        variant as _
    }
}
impl ADC0CLKDIV_R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> Option<ADC0CLKDIV_A> {
        match self.bits {
            0 => Some(ADC0CLKDIV_A::NODIVISION),
            _ => None,
        }
    }
    #[doc = "Checks if the value of the field is `NODIVISION`"]
    #[inline(always)]
    pub fn is_nodivision(&self) -> bool {
        *self == ADC0CLKDIV_A::NODIVISION
    }
}
#[doc = "Field `ADC0CLKDIV` writer - ADC0 Clock Prescaler"]
pub type ADC0CLKDIV_W<'a, const O: u8> =
    crate::FieldWriter<'a, u32, ADCCTRL_SPEC, u8, ADC0CLKDIV_A, 2, O>;
impl<'a, const O: u8> ADC0CLKDIV_W<'a, O> {
    #[doc = "`0`"]
    #[inline(always)]
    pub fn nodivision(self) -> &'a mut W {
        self.variant(ADC0CLKDIV_A::NODIVISION)
    }
}
#[doc = "Field `ADC0CLKSEL` reader - ADC0 Clock Select"]
pub type ADC0CLKSEL_R = crate::FieldReader<u8, ADC0CLKSEL_A>;
#[doc = "ADC0 Clock Select\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
#[repr(u8)]
pub enum ADC0CLKSEL_A {
    #[doc = "0: ADC0 is not clocked"]
    DISABLED = 0,
    #[doc = "1: AUXHFRCO is clocking ADC0"]
    AUXHFRCO = 1,
    #[doc = "2: HFXO is clocking ADC0"]
    HFXO = 2,
    #[doc = "3: HFSRCCLK is clocking ADC0"]
    HFSRCCLK = 3,
}
impl From<ADC0CLKSEL_A> for u8 {
    #[inline(always)]
    fn from(variant: ADC0CLKSEL_A) -> Self {
        variant as _
    }
}
impl ADC0CLKSEL_R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> ADC0CLKSEL_A {
        match self.bits {
            0 => ADC0CLKSEL_A::DISABLED,
            1 => ADC0CLKSEL_A::AUXHFRCO,
            2 => ADC0CLKSEL_A::HFXO,
            3 => ADC0CLKSEL_A::HFSRCCLK,
            _ => unreachable!(),
        }
    }
    #[doc = "Checks if the value of the field is `DISABLED`"]
    #[inline(always)]
    pub fn is_disabled(&self) -> bool {
        *self == ADC0CLKSEL_A::DISABLED
    }
    #[doc = "Checks if the value of the field is `AUXHFRCO`"]
    #[inline(always)]
    pub fn is_auxhfrco(&self) -> bool {
        *self == ADC0CLKSEL_A::AUXHFRCO
    }
    #[doc = "Checks if the value of the field is `HFXO`"]
    #[inline(always)]
    pub fn is_hfxo(&self) -> bool {
        *self == ADC0CLKSEL_A::HFXO
    }
    #[doc = "Checks if the value of the field is `HFSRCCLK`"]
    #[inline(always)]
    pub fn is_hfsrcclk(&self) -> bool {
        *self == ADC0CLKSEL_A::HFSRCCLK
    }
}
#[doc = "Field `ADC0CLKSEL` writer - ADC0 Clock Select"]
pub type ADC0CLKSEL_W<'a, const O: u8> =
    crate::FieldWriterSafe<'a, u32, ADCCTRL_SPEC, u8, ADC0CLKSEL_A, 2, O>;
impl<'a, const O: u8> ADC0CLKSEL_W<'a, O> {
    #[doc = "ADC0 is not clocked"]
    #[inline(always)]
    pub fn disabled(self) -> &'a mut W {
        self.variant(ADC0CLKSEL_A::DISABLED)
    }
    #[doc = "AUXHFRCO is clocking ADC0"]
    #[inline(always)]
    pub fn auxhfrco(self) -> &'a mut W {
        self.variant(ADC0CLKSEL_A::AUXHFRCO)
    }
    #[doc = "HFXO is clocking ADC0"]
    #[inline(always)]
    pub fn hfxo(self) -> &'a mut W {
        self.variant(ADC0CLKSEL_A::HFXO)
    }
    #[doc = "HFSRCCLK is clocking ADC0"]
    #[inline(always)]
    pub fn hfsrcclk(self) -> &'a mut W {
        self.variant(ADC0CLKSEL_A::HFSRCCLK)
    }
}
#[doc = "Field `ADC0CLKINV` reader - Invert Clock Selected By ADC0CLKSEL"]
pub type ADC0CLKINV_R = crate::BitReader<bool>;
#[doc = "Field `ADC0CLKINV` writer - Invert Clock Selected By ADC0CLKSEL"]
pub type ADC0CLKINV_W<'a, const O: u8> = crate::BitWriter<'a, u32, ADCCTRL_SPEC, bool, O>;
impl R {
    #[doc = "Bits 0:1 - ADC0 Clock Prescaler"]
    #[inline(always)]
    pub fn adc0clkdiv(&self) -> ADC0CLKDIV_R {
        ADC0CLKDIV_R::new((self.bits & 3) as u8)
    }
    #[doc = "Bits 4:5 - ADC0 Clock Select"]
    #[inline(always)]
    pub fn adc0clksel(&self) -> ADC0CLKSEL_R {
        ADC0CLKSEL_R::new(((self.bits >> 4) & 3) as u8)
    }
    #[doc = "Bit 8 - Invert Clock Selected By ADC0CLKSEL"]
    #[inline(always)]
    pub fn adc0clkinv(&self) -> ADC0CLKINV_R {
        ADC0CLKINV_R::new(((self.bits >> 8) & 1) != 0)
    }
}
impl W {
    #[doc = "Bits 0:1 - ADC0 Clock Prescaler"]
    #[inline(always)]
    #[must_use]
    pub fn adc0clkdiv(&mut self) -> ADC0CLKDIV_W<0> {
        ADC0CLKDIV_W::new(self)
    }
    #[doc = "Bits 4:5 - ADC0 Clock Select"]
    #[inline(always)]
    #[must_use]
    pub fn adc0clksel(&mut self) -> ADC0CLKSEL_W<4> {
        ADC0CLKSEL_W::new(self)
    }
    #[doc = "Bit 8 - Invert Clock Selected By ADC0CLKSEL"]
    #[inline(always)]
    #[must_use]
    pub fn adc0clkinv(&mut self) -> ADC0CLKINV_W<8> {
        ADC0CLKINV_W::new(self)
    }
    #[doc = "Writes raw bits to the register."]
    #[inline(always)]
    pub unsafe fn bits(&mut self, bits: u32) -> &mut Self {
        self.0.bits(bits);
        self
    }
}
#[doc = "ADC Control Register\n\nThis register you can [`read`](crate::generic::Reg::read), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [adcctrl](index.html) module"]
pub struct ADCCTRL_SPEC;
impl crate::RegisterSpec for ADCCTRL_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [adcctrl::R](R) reader structure"]
impl crate::Readable for ADCCTRL_SPEC {
    type Reader = R;
}
#[doc = "`write(|w| ..)` method takes [adcctrl::W](W) writer structure"]
impl crate::Writable for ADCCTRL_SPEC {
    type Writer = W;
    const ZERO_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
}
#[doc = "`reset()` method sets ADCCTRL to value 0"]
impl crate::Resettable for ADCCTRL_SPEC {
    const RESET_VALUE: Self::Ux = 0;
}
