#[doc = "Register `DCDCMISCCTRL` reader"]
pub struct R(crate::R<DCDCMISCCTRL_SPEC>);
impl core::ops::Deref for R {
    type Target = crate::R<DCDCMISCCTRL_SPEC>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
impl From<crate::R<DCDCMISCCTRL_SPEC>> for R {
    #[inline(always)]
    fn from(reader: crate::R<DCDCMISCCTRL_SPEC>) -> Self {
        R(reader)
    }
}
#[doc = "Register `DCDCMISCCTRL` writer"]
pub struct W(crate::W<DCDCMISCCTRL_SPEC>);
impl core::ops::Deref for W {
    type Target = crate::W<DCDCMISCCTRL_SPEC>;
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
impl From<crate::W<DCDCMISCCTRL_SPEC>> for W {
    #[inline(always)]
    fn from(writer: crate::W<DCDCMISCCTRL_SPEC>) -> Self {
        W(writer)
    }
}
#[doc = "Field `LNFORCECCM` reader - Force DCDC Into CCM Mode in Low Noise Operation"]
pub type LNFORCECCM_R = crate::BitReader<bool>;
#[doc = "Field `LNFORCECCM` writer - Force DCDC Into CCM Mode in Low Noise Operation"]
pub type LNFORCECCM_W<'a, const O: u8> = crate::BitWriter<'a, u32, DCDCMISCCTRL_SPEC, bool, O>;
#[doc = "Field `LPCMPHYSDIS` reader - Disable LP Mode Hysteresis in the State Machine Control"]
pub type LPCMPHYSDIS_R = crate::BitReader<bool>;
#[doc = "Field `LPCMPHYSDIS` writer - Disable LP Mode Hysteresis in the State Machine Control"]
pub type LPCMPHYSDIS_W<'a, const O: u8> = crate::BitWriter<'a, u32, DCDCMISCCTRL_SPEC, bool, O>;
#[doc = "Field `LPCMPHYSHI` reader - Comparator Threshold on the High Side"]
pub type LPCMPHYSHI_R = crate::BitReader<bool>;
#[doc = "Field `LPCMPHYSHI` writer - Comparator Threshold on the High Side"]
pub type LPCMPHYSHI_W<'a, const O: u8> = crate::BitWriter<'a, u32, DCDCMISCCTRL_SPEC, bool, O>;
#[doc = "Field `LNFORCECCMIMM` reader - Force DCDC Into CCM Mode Immediately, Based on LNFORCECCM"]
pub type LNFORCECCMIMM_R = crate::BitReader<bool>;
#[doc = "Field `LNFORCECCMIMM` writer - Force DCDC Into CCM Mode Immediately, Based on LNFORCECCM"]
pub type LNFORCECCMIMM_W<'a, const O: u8> = crate::BitWriter<'a, u32, DCDCMISCCTRL_SPEC, bool, O>;
#[doc = "Field `PFETCNT` reader - PFET Switch Number Selection"]
pub type PFETCNT_R = crate::FieldReader<u8, u8>;
#[doc = "Field `PFETCNT` writer - PFET Switch Number Selection"]
pub type PFETCNT_W<'a, const O: u8> = crate::FieldWriter<'a, u32, DCDCMISCCTRL_SPEC, u8, u8, 4, O>;
#[doc = "Field `NFETCNT` reader - NFET Switch Number Selection"]
pub type NFETCNT_R = crate::FieldReader<u8, u8>;
#[doc = "Field `NFETCNT` writer - NFET Switch Number Selection"]
pub type NFETCNT_W<'a, const O: u8> = crate::FieldWriter<'a, u32, DCDCMISCCTRL_SPEC, u8, u8, 4, O>;
#[doc = "Field `BYPLIMSEL` reader - Current Limit in Bypass Mode"]
pub type BYPLIMSEL_R = crate::FieldReader<u8, u8>;
#[doc = "Field `BYPLIMSEL` writer - Current Limit in Bypass Mode"]
pub type BYPLIMSEL_W<'a, const O: u8> =
    crate::FieldWriter<'a, u32, DCDCMISCCTRL_SPEC, u8, u8, 4, O>;
#[doc = "Field `LPCLIMILIMSEL` reader - Current Limit Level Selection for Current Limiter in LP Mode"]
pub type LPCLIMILIMSEL_R = crate::FieldReader<u8, u8>;
#[doc = "Field `LPCLIMILIMSEL` writer - Current Limit Level Selection for Current Limiter in LP Mode"]
pub type LPCLIMILIMSEL_W<'a, const O: u8> =
    crate::FieldWriter<'a, u32, DCDCMISCCTRL_SPEC, u8, u8, 3, O>;
#[doc = "Field `LNCLIMILIMSEL` reader - Current Limit Level Selection for Current Limiter in LN Mode"]
pub type LNCLIMILIMSEL_R = crate::FieldReader<u8, u8>;
#[doc = "Field `LNCLIMILIMSEL` writer - Current Limit Level Selection for Current Limiter in LN Mode"]
pub type LNCLIMILIMSEL_W<'a, const O: u8> =
    crate::FieldWriter<'a, u32, DCDCMISCCTRL_SPEC, u8, u8, 3, O>;
#[doc = "Field `LPCMPBIASEM234H` reader - LP Mode Comparator Bias Selection for EM23 or EM4H"]
pub type LPCMPBIASEM234H_R = crate::FieldReader<u8, LPCMPBIASEM234H_A>;
#[doc = "LP Mode Comparator Bias Selection for EM23 or EM4H\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
#[repr(u8)]
pub enum LPCMPBIASEM234H_A {
    #[doc = "0: Maximum load current less than 75uA."]
    BIAS0 = 0,
    #[doc = "1: Maximum load current less than 500uA."]
    BIAS1 = 1,
    #[doc = "2: Maximum load current less than 2.5mA."]
    BIAS2 = 2,
    #[doc = "3: Maximum load current less than 10mA."]
    BIAS3 = 3,
}
impl From<LPCMPBIASEM234H_A> for u8 {
    #[inline(always)]
    fn from(variant: LPCMPBIASEM234H_A) -> Self {
        variant as _
    }
}
impl LPCMPBIASEM234H_R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> LPCMPBIASEM234H_A {
        match self.bits {
            0 => LPCMPBIASEM234H_A::BIAS0,
            1 => LPCMPBIASEM234H_A::BIAS1,
            2 => LPCMPBIASEM234H_A::BIAS2,
            3 => LPCMPBIASEM234H_A::BIAS3,
            _ => unreachable!(),
        }
    }
    #[doc = "Checks if the value of the field is `BIAS0`"]
    #[inline(always)]
    pub fn is_bias0(&self) -> bool {
        *self == LPCMPBIASEM234H_A::BIAS0
    }
    #[doc = "Checks if the value of the field is `BIAS1`"]
    #[inline(always)]
    pub fn is_bias1(&self) -> bool {
        *self == LPCMPBIASEM234H_A::BIAS1
    }
    #[doc = "Checks if the value of the field is `BIAS2`"]
    #[inline(always)]
    pub fn is_bias2(&self) -> bool {
        *self == LPCMPBIASEM234H_A::BIAS2
    }
    #[doc = "Checks if the value of the field is `BIAS3`"]
    #[inline(always)]
    pub fn is_bias3(&self) -> bool {
        *self == LPCMPBIASEM234H_A::BIAS3
    }
}
#[doc = "Field `LPCMPBIASEM234H` writer - LP Mode Comparator Bias Selection for EM23 or EM4H"]
pub type LPCMPBIASEM234H_W<'a, const O: u8> =
    crate::FieldWriterSafe<'a, u32, DCDCMISCCTRL_SPEC, u8, LPCMPBIASEM234H_A, 2, O>;
impl<'a, const O: u8> LPCMPBIASEM234H_W<'a, O> {
    #[doc = "Maximum load current less than 75uA."]
    #[inline(always)]
    pub fn bias0(self) -> &'a mut W {
        self.variant(LPCMPBIASEM234H_A::BIAS0)
    }
    #[doc = "Maximum load current less than 500uA."]
    #[inline(always)]
    pub fn bias1(self) -> &'a mut W {
        self.variant(LPCMPBIASEM234H_A::BIAS1)
    }
    #[doc = "Maximum load current less than 2.5mA."]
    #[inline(always)]
    pub fn bias2(self) -> &'a mut W {
        self.variant(LPCMPBIASEM234H_A::BIAS2)
    }
    #[doc = "Maximum load current less than 10mA."]
    #[inline(always)]
    pub fn bias3(self) -> &'a mut W {
        self.variant(LPCMPBIASEM234H_A::BIAS3)
    }
}
impl R {
    #[doc = "Bit 0 - Force DCDC Into CCM Mode in Low Noise Operation"]
    #[inline(always)]
    pub fn lnforceccm(&self) -> LNFORCECCM_R {
        LNFORCECCM_R::new((self.bits & 1) != 0)
    }
    #[doc = "Bit 1 - Disable LP Mode Hysteresis in the State Machine Control"]
    #[inline(always)]
    pub fn lpcmphysdis(&self) -> LPCMPHYSDIS_R {
        LPCMPHYSDIS_R::new(((self.bits >> 1) & 1) != 0)
    }
    #[doc = "Bit 2 - Comparator Threshold on the High Side"]
    #[inline(always)]
    pub fn lpcmphyshi(&self) -> LPCMPHYSHI_R {
        LPCMPHYSHI_R::new(((self.bits >> 2) & 1) != 0)
    }
    #[doc = "Bit 5 - Force DCDC Into CCM Mode Immediately, Based on LNFORCECCM"]
    #[inline(always)]
    pub fn lnforceccmimm(&self) -> LNFORCECCMIMM_R {
        LNFORCECCMIMM_R::new(((self.bits >> 5) & 1) != 0)
    }
    #[doc = "Bits 8:11 - PFET Switch Number Selection"]
    #[inline(always)]
    pub fn pfetcnt(&self) -> PFETCNT_R {
        PFETCNT_R::new(((self.bits >> 8) & 0x0f) as u8)
    }
    #[doc = "Bits 12:15 - NFET Switch Number Selection"]
    #[inline(always)]
    pub fn nfetcnt(&self) -> NFETCNT_R {
        NFETCNT_R::new(((self.bits >> 12) & 0x0f) as u8)
    }
    #[doc = "Bits 16:19 - Current Limit in Bypass Mode"]
    #[inline(always)]
    pub fn byplimsel(&self) -> BYPLIMSEL_R {
        BYPLIMSEL_R::new(((self.bits >> 16) & 0x0f) as u8)
    }
    #[doc = "Bits 20:22 - Current Limit Level Selection for Current Limiter in LP Mode"]
    #[inline(always)]
    pub fn lpclimilimsel(&self) -> LPCLIMILIMSEL_R {
        LPCLIMILIMSEL_R::new(((self.bits >> 20) & 7) as u8)
    }
    #[doc = "Bits 24:26 - Current Limit Level Selection for Current Limiter in LN Mode"]
    #[inline(always)]
    pub fn lnclimilimsel(&self) -> LNCLIMILIMSEL_R {
        LNCLIMILIMSEL_R::new(((self.bits >> 24) & 7) as u8)
    }
    #[doc = "Bits 28:29 - LP Mode Comparator Bias Selection for EM23 or EM4H"]
    #[inline(always)]
    pub fn lpcmpbiasem234h(&self) -> LPCMPBIASEM234H_R {
        LPCMPBIASEM234H_R::new(((self.bits >> 28) & 3) as u8)
    }
}
impl W {
    #[doc = "Bit 0 - Force DCDC Into CCM Mode in Low Noise Operation"]
    #[inline(always)]
    #[must_use]
    pub fn lnforceccm(&mut self) -> LNFORCECCM_W<0> {
        LNFORCECCM_W::new(self)
    }
    #[doc = "Bit 1 - Disable LP Mode Hysteresis in the State Machine Control"]
    #[inline(always)]
    #[must_use]
    pub fn lpcmphysdis(&mut self) -> LPCMPHYSDIS_W<1> {
        LPCMPHYSDIS_W::new(self)
    }
    #[doc = "Bit 2 - Comparator Threshold on the High Side"]
    #[inline(always)]
    #[must_use]
    pub fn lpcmphyshi(&mut self) -> LPCMPHYSHI_W<2> {
        LPCMPHYSHI_W::new(self)
    }
    #[doc = "Bit 5 - Force DCDC Into CCM Mode Immediately, Based on LNFORCECCM"]
    #[inline(always)]
    #[must_use]
    pub fn lnforceccmimm(&mut self) -> LNFORCECCMIMM_W<5> {
        LNFORCECCMIMM_W::new(self)
    }
    #[doc = "Bits 8:11 - PFET Switch Number Selection"]
    #[inline(always)]
    #[must_use]
    pub fn pfetcnt(&mut self) -> PFETCNT_W<8> {
        PFETCNT_W::new(self)
    }
    #[doc = "Bits 12:15 - NFET Switch Number Selection"]
    #[inline(always)]
    #[must_use]
    pub fn nfetcnt(&mut self) -> NFETCNT_W<12> {
        NFETCNT_W::new(self)
    }
    #[doc = "Bits 16:19 - Current Limit in Bypass Mode"]
    #[inline(always)]
    #[must_use]
    pub fn byplimsel(&mut self) -> BYPLIMSEL_W<16> {
        BYPLIMSEL_W::new(self)
    }
    #[doc = "Bits 20:22 - Current Limit Level Selection for Current Limiter in LP Mode"]
    #[inline(always)]
    #[must_use]
    pub fn lpclimilimsel(&mut self) -> LPCLIMILIMSEL_W<20> {
        LPCLIMILIMSEL_W::new(self)
    }
    #[doc = "Bits 24:26 - Current Limit Level Selection for Current Limiter in LN Mode"]
    #[inline(always)]
    #[must_use]
    pub fn lnclimilimsel(&mut self) -> LNCLIMILIMSEL_W<24> {
        LNCLIMILIMSEL_W::new(self)
    }
    #[doc = "Bits 28:29 - LP Mode Comparator Bias Selection for EM23 or EM4H"]
    #[inline(always)]
    #[must_use]
    pub fn lpcmpbiasem234h(&mut self) -> LPCMPBIASEM234H_W<28> {
        LPCMPBIASEM234H_W::new(self)
    }
    #[doc = "Writes raw bits to the register."]
    #[inline(always)]
    pub unsafe fn bits(&mut self, bits: u32) -> &mut Self {
        self.0.bits(bits);
        self
    }
}
#[doc = "DCDC Miscellaneous Control Register\n\nThis register you can [`read`](crate::generic::Reg::read), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [dcdcmiscctrl](index.html) module"]
pub struct DCDCMISCCTRL_SPEC;
impl crate::RegisterSpec for DCDCMISCCTRL_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [dcdcmiscctrl::R](R) reader structure"]
impl crate::Readable for DCDCMISCCTRL_SPEC {
    type Reader = R;
}
#[doc = "`write(|w| ..)` method takes [dcdcmiscctrl::W](W) writer structure"]
impl crate::Writable for DCDCMISCCTRL_SPEC {
    type Writer = W;
    const ZERO_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
}
#[doc = "`reset()` method sets DCDCMISCCTRL to value 0x0310_7706"]
impl crate::Resettable for DCDCMISCCTRL_SPEC {
    const RESET_VALUE: Self::Ux = 0x0310_7706;
}
