#[doc = "Register `DCDCLPEM01CFG` reader"]
pub struct R(crate::R<DCDCLPEM01CFG_SPEC>);
impl core::ops::Deref for R {
    type Target = crate::R<DCDCLPEM01CFG_SPEC>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
impl From<crate::R<DCDCLPEM01CFG_SPEC>> for R {
    #[inline(always)]
    fn from(reader: crate::R<DCDCLPEM01CFG_SPEC>) -> Self {
        R(reader)
    }
}
#[doc = "Register `DCDCLPEM01CFG` writer"]
pub struct W(crate::W<DCDCLPEM01CFG_SPEC>);
impl core::ops::Deref for W {
    type Target = crate::W<DCDCLPEM01CFG_SPEC>;
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
impl From<crate::W<DCDCLPEM01CFG_SPEC>> for W {
    #[inline(always)]
    fn from(writer: crate::W<DCDCLPEM01CFG_SPEC>) -> Self {
        W(writer)
    }
}
#[doc = "LP Mode Comparator Bias Selection for EM01\n\nValue on reset: 3"]
#[derive(Clone, Copy, Debug, PartialEq)]
#[repr(u8)]
pub enum LPCMPBIASEM01_A {
    #[doc = "0: Maximum load current less than 75uA."]
    BIAS0 = 0,
    #[doc = "1: Maximum load current less than 500uA."]
    BIAS1 = 1,
    #[doc = "2: Maximum load current less than 2.5mA."]
    BIAS2 = 2,
    #[doc = "3: Maximum load current less than 10mA."]
    BIAS3 = 3,
}
impl From<LPCMPBIASEM01_A> for u8 {
    #[inline(always)]
    fn from(variant: LPCMPBIASEM01_A) -> Self {
        variant as _
    }
}
#[doc = "Field `LPCMPBIASEM01` reader - LP Mode Comparator Bias Selection for EM01"]
pub type LPCMPBIASEM01_R = crate::FieldReader<u8, LPCMPBIASEM01_A>;
impl LPCMPBIASEM01_R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> LPCMPBIASEM01_A {
        match self.bits {
            0 => LPCMPBIASEM01_A::BIAS0,
            1 => LPCMPBIASEM01_A::BIAS1,
            2 => LPCMPBIASEM01_A::BIAS2,
            3 => LPCMPBIASEM01_A::BIAS3,
            _ => unreachable!(),
        }
    }
    #[doc = "Checks if the value of the field is `BIAS0`"]
    #[inline(always)]
    pub fn is_bias0(&self) -> bool {
        *self == LPCMPBIASEM01_A::BIAS0
    }
    #[doc = "Checks if the value of the field is `BIAS1`"]
    #[inline(always)]
    pub fn is_bias1(&self) -> bool {
        *self == LPCMPBIASEM01_A::BIAS1
    }
    #[doc = "Checks if the value of the field is `BIAS2`"]
    #[inline(always)]
    pub fn is_bias2(&self) -> bool {
        *self == LPCMPBIASEM01_A::BIAS2
    }
    #[doc = "Checks if the value of the field is `BIAS3`"]
    #[inline(always)]
    pub fn is_bias3(&self) -> bool {
        *self == LPCMPBIASEM01_A::BIAS3
    }
}
#[doc = "Field `LPCMPBIASEM01` writer - LP Mode Comparator Bias Selection for EM01"]
pub type LPCMPBIASEM01_W<'a> =
    crate::FieldWriterSafe<'a, u32, DCDCLPEM01CFG_SPEC, u8, LPCMPBIASEM01_A, 2, 8>;
impl<'a> LPCMPBIASEM01_W<'a> {
    #[doc = "Maximum load current less than 75uA."]
    #[inline(always)]
    pub fn bias0(self) -> &'a mut W {
        self.variant(LPCMPBIASEM01_A::BIAS0)
    }
    #[doc = "Maximum load current less than 500uA."]
    #[inline(always)]
    pub fn bias1(self) -> &'a mut W {
        self.variant(LPCMPBIASEM01_A::BIAS1)
    }
    #[doc = "Maximum load current less than 2.5mA."]
    #[inline(always)]
    pub fn bias2(self) -> &'a mut W {
        self.variant(LPCMPBIASEM01_A::BIAS2)
    }
    #[doc = "Maximum load current less than 10mA."]
    #[inline(always)]
    pub fn bias3(self) -> &'a mut W {
        self.variant(LPCMPBIASEM01_A::BIAS3)
    }
}
#[doc = "Field `LPCMPHYSSELEM01` reader - LP Mode Hysteresis Selection for EM01"]
pub type LPCMPHYSSELEM01_R = crate::FieldReader<u8, u8>;
#[doc = "Field `LPCMPHYSSELEM01` writer - LP Mode Hysteresis Selection for EM01"]
pub type LPCMPHYSSELEM01_W<'a> = crate::FieldWriter<'a, u32, DCDCLPEM01CFG_SPEC, u8, u8, 4, 12>;
impl R {
    #[doc = "Bits 8:9 - LP Mode Comparator Bias Selection for EM01"]
    #[inline(always)]
    pub fn lpcmpbiasem01(&self) -> LPCMPBIASEM01_R {
        LPCMPBIASEM01_R::new(((self.bits >> 8) & 3) as u8)
    }
    #[doc = "Bits 12:15 - LP Mode Hysteresis Selection for EM01"]
    #[inline(always)]
    pub fn lpcmphysselem01(&self) -> LPCMPHYSSELEM01_R {
        LPCMPHYSSELEM01_R::new(((self.bits >> 12) & 0x0f) as u8)
    }
}
impl W {
    #[doc = "Bits 8:9 - LP Mode Comparator Bias Selection for EM01"]
    #[inline(always)]
    pub fn lpcmpbiasem01(&mut self) -> LPCMPBIASEM01_W {
        LPCMPBIASEM01_W::new(self)
    }
    #[doc = "Bits 12:15 - LP Mode Hysteresis Selection for EM01"]
    #[inline(always)]
    pub fn lpcmphysselem01(&mut self) -> LPCMPHYSSELEM01_W {
        LPCMPHYSSELEM01_W::new(self)
    }
    #[doc = "Writes raw bits to the register."]
    #[inline(always)]
    pub unsafe fn bits(&mut self, bits: u32) -> &mut Self {
        self.0.bits(bits);
        self
    }
}
#[doc = "Configuration Bits for Low Power Mode to Be Applied During EM01, This Field is Only Relevant If LP Mode is Used in EM01\n\nThis register you can [`read`](crate::generic::Reg::read), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [dcdclpem01cfg](index.html) module"]
pub struct DCDCLPEM01CFG_SPEC;
impl crate::RegisterSpec for DCDCLPEM01CFG_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [dcdclpem01cfg::R](R) reader structure"]
impl crate::Readable for DCDCLPEM01CFG_SPEC {
    type Reader = R;
}
#[doc = "`write(|w| ..)` method takes [dcdclpem01cfg::W](W) writer structure"]
impl crate::Writable for DCDCLPEM01CFG_SPEC {
    type Writer = W;
}
#[doc = "`reset()` method sets DCDCLPEM01CFG to value 0x0300"]
impl crate::Resettable for DCDCLPEM01CFG_SPEC {
    #[inline(always)]
    fn reset_value() -> Self::Ux {
        0x0300
    }
}
