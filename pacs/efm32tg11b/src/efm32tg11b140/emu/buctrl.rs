#[doc = "Register `BUCTRL` reader"]
pub struct R(crate::R<BUCTRL_SPEC>);
impl core::ops::Deref for R {
    type Target = crate::R<BUCTRL_SPEC>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
impl From<crate::R<BUCTRL_SPEC>> for R {
    #[inline(always)]
    fn from(reader: crate::R<BUCTRL_SPEC>) -> Self {
        R(reader)
    }
}
#[doc = "Register `BUCTRL` writer"]
pub struct W(crate::W<BUCTRL_SPEC>);
impl core::ops::Deref for W {
    type Target = crate::W<BUCTRL_SPEC>;
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
impl From<crate::W<BUCTRL_SPEC>> for W {
    #[inline(always)]
    fn from(writer: crate::W<BUCTRL_SPEC>) -> Self {
        W(writer)
    }
}
#[doc = "Field `EN` reader - Enable Backup Mode"]
pub type EN_R = crate::BitReader<bool>;
#[doc = "Field `EN` writer - Enable Backup Mode"]
pub type EN_W<'a> = crate::BitWriter<'a, u32, BUCTRL_SPEC, bool, 0>;
#[doc = "Field `STATEN` reader - Enable Backup Mode Status Export"]
pub type STATEN_R = crate::BitReader<bool>;
#[doc = "Field `STATEN` writer - Enable Backup Mode Status Export"]
pub type STATEN_W<'a> = crate::BitWriter<'a, u32, BUCTRL_SPEC, bool, 1>;
#[doc = "Field `BUVINPROBEEN` reader - Enable BU_VIN Probing"]
pub type BUVINPROBEEN_R = crate::BitReader<bool>;
#[doc = "Field `BUVINPROBEEN` writer - Enable BU_VIN Probing"]
pub type BUVINPROBEEN_W<'a> = crate::BitWriter<'a, u32, BUCTRL_SPEC, bool, 2>;
#[doc = "BU_VOUT Resistor Select\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq)]
#[repr(u8)]
pub enum VOUTRES_A {
    #[doc = "0: BU_VOUT is not connected"]
    DIS = 0,
    #[doc = "1: Enable weak switch between BU_VOUT and backup domain power supply."]
    WEAK = 1,
    #[doc = "2: Enable medium switch between BU_VOUT and backup domain power supply."]
    MED = 2,
    #[doc = "3: Enable strong switch between BU_VOUT and backup domain power supply."]
    STRONG = 3,
}
impl From<VOUTRES_A> for u8 {
    #[inline(always)]
    fn from(variant: VOUTRES_A) -> Self {
        variant as _
    }
}
#[doc = "Field `VOUTRES` reader - BU_VOUT Resistor Select"]
pub type VOUTRES_R = crate::FieldReader<u8, VOUTRES_A>;
impl VOUTRES_R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> VOUTRES_A {
        match self.bits {
            0 => VOUTRES_A::DIS,
            1 => VOUTRES_A::WEAK,
            2 => VOUTRES_A::MED,
            3 => VOUTRES_A::STRONG,
            _ => unreachable!(),
        }
    }
    #[doc = "Checks if the value of the field is `DIS`"]
    #[inline(always)]
    pub fn is_dis(&self) -> bool {
        *self == VOUTRES_A::DIS
    }
    #[doc = "Checks if the value of the field is `WEAK`"]
    #[inline(always)]
    pub fn is_weak(&self) -> bool {
        *self == VOUTRES_A::WEAK
    }
    #[doc = "Checks if the value of the field is `MED`"]
    #[inline(always)]
    pub fn is_med(&self) -> bool {
        *self == VOUTRES_A::MED
    }
    #[doc = "Checks if the value of the field is `STRONG`"]
    #[inline(always)]
    pub fn is_strong(&self) -> bool {
        *self == VOUTRES_A::STRONG
    }
}
#[doc = "Field `VOUTRES` writer - BU_VOUT Resistor Select"]
pub type VOUTRES_W<'a> = crate::FieldWriterSafe<'a, u32, BUCTRL_SPEC, u8, VOUTRES_A, 2, 8>;
impl<'a> VOUTRES_W<'a> {
    #[doc = "BU_VOUT is not connected"]
    #[inline(always)]
    pub fn dis(self) -> &'a mut W {
        self.variant(VOUTRES_A::DIS)
    }
    #[doc = "Enable weak switch between BU_VOUT and backup domain power supply."]
    #[inline(always)]
    pub fn weak(self) -> &'a mut W {
        self.variant(VOUTRES_A::WEAK)
    }
    #[doc = "Enable medium switch between BU_VOUT and backup domain power supply."]
    #[inline(always)]
    pub fn med(self) -> &'a mut W {
        self.variant(VOUTRES_A::MED)
    }
    #[doc = "Enable strong switch between BU_VOUT and backup domain power supply."]
    #[inline(always)]
    pub fn strong(self) -> &'a mut W {
        self.variant(VOUTRES_A::STRONG)
    }
}
#[doc = "Power Domain Resistor Select\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq)]
#[repr(u8)]
pub enum PWRRES_A {
    #[doc = "0: Main power and backup power connected with RES0 series resistance."]
    RES0 = 0,
    #[doc = "1: Main power and backup power connected with RES1 series resistance."]
    RES1 = 1,
    #[doc = "2: Main power and backup power connected with RES2 series resistance."]
    RES2 = 2,
    #[doc = "3: Main power and backup power connected with RES3 series resistance."]
    RES3 = 3,
}
impl From<PWRRES_A> for u8 {
    #[inline(always)]
    fn from(variant: PWRRES_A) -> Self {
        variant as _
    }
}
#[doc = "Field `PWRRES` reader - Power Domain Resistor Select"]
pub type PWRRES_R = crate::FieldReader<u8, PWRRES_A>;
impl PWRRES_R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> PWRRES_A {
        match self.bits {
            0 => PWRRES_A::RES0,
            1 => PWRRES_A::RES1,
            2 => PWRRES_A::RES2,
            3 => PWRRES_A::RES3,
            _ => unreachable!(),
        }
    }
    #[doc = "Checks if the value of the field is `RES0`"]
    #[inline(always)]
    pub fn is_res0(&self) -> bool {
        *self == PWRRES_A::RES0
    }
    #[doc = "Checks if the value of the field is `RES1`"]
    #[inline(always)]
    pub fn is_res1(&self) -> bool {
        *self == PWRRES_A::RES1
    }
    #[doc = "Checks if the value of the field is `RES2`"]
    #[inline(always)]
    pub fn is_res2(&self) -> bool {
        *self == PWRRES_A::RES2
    }
    #[doc = "Checks if the value of the field is `RES3`"]
    #[inline(always)]
    pub fn is_res3(&self) -> bool {
        *self == PWRRES_A::RES3
    }
}
#[doc = "Field `PWRRES` writer - Power Domain Resistor Select"]
pub type PWRRES_W<'a> = crate::FieldWriterSafe<'a, u32, BUCTRL_SPEC, u8, PWRRES_A, 2, 12>;
impl<'a> PWRRES_W<'a> {
    #[doc = "Main power and backup power connected with RES0 series resistance."]
    #[inline(always)]
    pub fn res0(self) -> &'a mut W {
        self.variant(PWRRES_A::RES0)
    }
    #[doc = "Main power and backup power connected with RES1 series resistance."]
    #[inline(always)]
    pub fn res1(self) -> &'a mut W {
        self.variant(PWRRES_A::RES1)
    }
    #[doc = "Main power and backup power connected with RES2 series resistance."]
    #[inline(always)]
    pub fn res2(self) -> &'a mut W {
        self.variant(PWRRES_A::RES2)
    }
    #[doc = "Main power and backup power connected with RES3 series resistance."]
    #[inline(always)]
    pub fn res3(self) -> &'a mut W {
        self.variant(PWRRES_A::RES3)
    }
}
#[doc = "Power Connection Configuration in Backup Mode\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq)]
#[repr(u8)]
pub enum BUACTPWRCON_A {
    #[doc = "0: No connection."]
    NONE = 0,
    #[doc = "1: Main power and backup power are connected through a diode, allowing current to flow from backup power source to main power source, but not the other way."]
    MAINBU = 1,
    #[doc = "2: Main power and backup power are connected through a diode, allowing current to flow from main power source to backup power source, but not the other way."]
    BUMAIN = 2,
    #[doc = "3: Main power and backup power are connected without diode."]
    NODIODE = 3,
}
impl From<BUACTPWRCON_A> for u8 {
    #[inline(always)]
    fn from(variant: BUACTPWRCON_A) -> Self {
        variant as _
    }
}
#[doc = "Field `BUACTPWRCON` reader - Power Connection Configuration in Backup Mode"]
pub type BUACTPWRCON_R = crate::FieldReader<u8, BUACTPWRCON_A>;
impl BUACTPWRCON_R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> BUACTPWRCON_A {
        match self.bits {
            0 => BUACTPWRCON_A::NONE,
            1 => BUACTPWRCON_A::MAINBU,
            2 => BUACTPWRCON_A::BUMAIN,
            3 => BUACTPWRCON_A::NODIODE,
            _ => unreachable!(),
        }
    }
    #[doc = "Checks if the value of the field is `NONE`"]
    #[inline(always)]
    pub fn is_none(&self) -> bool {
        *self == BUACTPWRCON_A::NONE
    }
    #[doc = "Checks if the value of the field is `MAINBU`"]
    #[inline(always)]
    pub fn is_mainbu(&self) -> bool {
        *self == BUACTPWRCON_A::MAINBU
    }
    #[doc = "Checks if the value of the field is `BUMAIN`"]
    #[inline(always)]
    pub fn is_bumain(&self) -> bool {
        *self == BUACTPWRCON_A::BUMAIN
    }
    #[doc = "Checks if the value of the field is `NODIODE`"]
    #[inline(always)]
    pub fn is_nodiode(&self) -> bool {
        *self == BUACTPWRCON_A::NODIODE
    }
}
#[doc = "Field `BUACTPWRCON` writer - Power Connection Configuration in Backup Mode"]
pub type BUACTPWRCON_W<'a> = crate::FieldWriterSafe<'a, u32, BUCTRL_SPEC, u8, BUACTPWRCON_A, 2, 16>;
impl<'a> BUACTPWRCON_W<'a> {
    #[doc = "No connection."]
    #[inline(always)]
    pub fn none(self) -> &'a mut W {
        self.variant(BUACTPWRCON_A::NONE)
    }
    #[doc = "Main power and backup power are connected through a diode, allowing current to flow from backup power source to main power source, but not the other way."]
    #[inline(always)]
    pub fn mainbu(self) -> &'a mut W {
        self.variant(BUACTPWRCON_A::MAINBU)
    }
    #[doc = "Main power and backup power are connected through a diode, allowing current to flow from main power source to backup power source, but not the other way."]
    #[inline(always)]
    pub fn bumain(self) -> &'a mut W {
        self.variant(BUACTPWRCON_A::BUMAIN)
    }
    #[doc = "Main power and backup power are connected without diode."]
    #[inline(always)]
    pub fn nodiode(self) -> &'a mut W {
        self.variant(BUACTPWRCON_A::NODIODE)
    }
}
#[doc = "Power Connection Configuration When Not in Backup Mode\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq)]
#[repr(u8)]
pub enum BUINACTPWRCON_A {
    #[doc = "0: No connection."]
    NONE = 0,
    #[doc = "1: Main power and backup power are connected through a diode, allowing current to flow from main power source to backup power source, but not the other way."]
    MAINBU = 1,
    #[doc = "2: Main power and backup power are connected through a diode, allowing current to flow from backup power source to main power source, but not the other way."]
    BUMAIN = 2,
    #[doc = "3: Main power and backup power are connected without diode."]
    NODIODE = 3,
}
impl From<BUINACTPWRCON_A> for u8 {
    #[inline(always)]
    fn from(variant: BUINACTPWRCON_A) -> Self {
        variant as _
    }
}
#[doc = "Field `BUINACTPWRCON` reader - Power Connection Configuration When Not in Backup Mode"]
pub type BUINACTPWRCON_R = crate::FieldReader<u8, BUINACTPWRCON_A>;
impl BUINACTPWRCON_R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> BUINACTPWRCON_A {
        match self.bits {
            0 => BUINACTPWRCON_A::NONE,
            1 => BUINACTPWRCON_A::MAINBU,
            2 => BUINACTPWRCON_A::BUMAIN,
            3 => BUINACTPWRCON_A::NODIODE,
            _ => unreachable!(),
        }
    }
    #[doc = "Checks if the value of the field is `NONE`"]
    #[inline(always)]
    pub fn is_none(&self) -> bool {
        *self == BUINACTPWRCON_A::NONE
    }
    #[doc = "Checks if the value of the field is `MAINBU`"]
    #[inline(always)]
    pub fn is_mainbu(&self) -> bool {
        *self == BUINACTPWRCON_A::MAINBU
    }
    #[doc = "Checks if the value of the field is `BUMAIN`"]
    #[inline(always)]
    pub fn is_bumain(&self) -> bool {
        *self == BUINACTPWRCON_A::BUMAIN
    }
    #[doc = "Checks if the value of the field is `NODIODE`"]
    #[inline(always)]
    pub fn is_nodiode(&self) -> bool {
        *self == BUINACTPWRCON_A::NODIODE
    }
}
#[doc = "Field `BUINACTPWRCON` writer - Power Connection Configuration When Not in Backup Mode"]
pub type BUINACTPWRCON_W<'a> =
    crate::FieldWriterSafe<'a, u32, BUCTRL_SPEC, u8, BUINACTPWRCON_A, 2, 20>;
impl<'a> BUINACTPWRCON_W<'a> {
    #[doc = "No connection."]
    #[inline(always)]
    pub fn none(self) -> &'a mut W {
        self.variant(BUINACTPWRCON_A::NONE)
    }
    #[doc = "Main power and backup power are connected through a diode, allowing current to flow from main power source to backup power source, but not the other way."]
    #[inline(always)]
    pub fn mainbu(self) -> &'a mut W {
        self.variant(BUINACTPWRCON_A::MAINBU)
    }
    #[doc = "Main power and backup power are connected through a diode, allowing current to flow from backup power source to main power source, but not the other way."]
    #[inline(always)]
    pub fn bumain(self) -> &'a mut W {
        self.variant(BUINACTPWRCON_A::BUMAIN)
    }
    #[doc = "Main power and backup power are connected without diode."]
    #[inline(always)]
    pub fn nodiode(self) -> &'a mut W {
        self.variant(BUINACTPWRCON_A::NODIODE)
    }
}
#[doc = "Field `DISMAXCOMP` reader - Disable MAIN-BU Comparator"]
pub type DISMAXCOMP_R = crate::BitReader<bool>;
#[doc = "Field `DISMAXCOMP` writer - Disable MAIN-BU Comparator"]
pub type DISMAXCOMP_W<'a> = crate::BitWriter<'a, u32, BUCTRL_SPEC, bool, 31>;
impl R {
    #[doc = "Bit 0 - Enable Backup Mode"]
    #[inline(always)]
    pub fn en(&self) -> EN_R {
        EN_R::new((self.bits & 1) != 0)
    }
    #[doc = "Bit 1 - Enable Backup Mode Status Export"]
    #[inline(always)]
    pub fn staten(&self) -> STATEN_R {
        STATEN_R::new(((self.bits >> 1) & 1) != 0)
    }
    #[doc = "Bit 2 - Enable BU_VIN Probing"]
    #[inline(always)]
    pub fn buvinprobeen(&self) -> BUVINPROBEEN_R {
        BUVINPROBEEN_R::new(((self.bits >> 2) & 1) != 0)
    }
    #[doc = "Bits 8:9 - BU_VOUT Resistor Select"]
    #[inline(always)]
    pub fn voutres(&self) -> VOUTRES_R {
        VOUTRES_R::new(((self.bits >> 8) & 3) as u8)
    }
    #[doc = "Bits 12:13 - Power Domain Resistor Select"]
    #[inline(always)]
    pub fn pwrres(&self) -> PWRRES_R {
        PWRRES_R::new(((self.bits >> 12) & 3) as u8)
    }
    #[doc = "Bits 16:17 - Power Connection Configuration in Backup Mode"]
    #[inline(always)]
    pub fn buactpwrcon(&self) -> BUACTPWRCON_R {
        BUACTPWRCON_R::new(((self.bits >> 16) & 3) as u8)
    }
    #[doc = "Bits 20:21 - Power Connection Configuration When Not in Backup Mode"]
    #[inline(always)]
    pub fn buinactpwrcon(&self) -> BUINACTPWRCON_R {
        BUINACTPWRCON_R::new(((self.bits >> 20) & 3) as u8)
    }
    #[doc = "Bit 31 - Disable MAIN-BU Comparator"]
    #[inline(always)]
    pub fn dismaxcomp(&self) -> DISMAXCOMP_R {
        DISMAXCOMP_R::new(((self.bits >> 31) & 1) != 0)
    }
}
impl W {
    #[doc = "Bit 0 - Enable Backup Mode"]
    #[inline(always)]
    pub fn en(&mut self) -> EN_W {
        EN_W::new(self)
    }
    #[doc = "Bit 1 - Enable Backup Mode Status Export"]
    #[inline(always)]
    pub fn staten(&mut self) -> STATEN_W {
        STATEN_W::new(self)
    }
    #[doc = "Bit 2 - Enable BU_VIN Probing"]
    #[inline(always)]
    pub fn buvinprobeen(&mut self) -> BUVINPROBEEN_W {
        BUVINPROBEEN_W::new(self)
    }
    #[doc = "Bits 8:9 - BU_VOUT Resistor Select"]
    #[inline(always)]
    pub fn voutres(&mut self) -> VOUTRES_W {
        VOUTRES_W::new(self)
    }
    #[doc = "Bits 12:13 - Power Domain Resistor Select"]
    #[inline(always)]
    pub fn pwrres(&mut self) -> PWRRES_W {
        PWRRES_W::new(self)
    }
    #[doc = "Bits 16:17 - Power Connection Configuration in Backup Mode"]
    #[inline(always)]
    pub fn buactpwrcon(&mut self) -> BUACTPWRCON_W {
        BUACTPWRCON_W::new(self)
    }
    #[doc = "Bits 20:21 - Power Connection Configuration When Not in Backup Mode"]
    #[inline(always)]
    pub fn buinactpwrcon(&mut self) -> BUINACTPWRCON_W {
        BUINACTPWRCON_W::new(self)
    }
    #[doc = "Bit 31 - Disable MAIN-BU Comparator"]
    #[inline(always)]
    pub fn dismaxcomp(&mut self) -> DISMAXCOMP_W {
        DISMAXCOMP_W::new(self)
    }
    #[doc = "Writes raw bits to the register."]
    #[inline(always)]
    pub unsafe fn bits(&mut self, bits: u32) -> &mut Self {
        self.0.bits(bits);
        self
    }
}
#[doc = "Backup Power Configuration Register\n\nThis register you can [`read`](crate::generic::Reg::read), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [buctrl](index.html) module"]
pub struct BUCTRL_SPEC;
impl crate::RegisterSpec for BUCTRL_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [buctrl::R](R) reader structure"]
impl crate::Readable for BUCTRL_SPEC {
    type Reader = R;
}
#[doc = "`write(|w| ..)` method takes [buctrl::W](W) writer structure"]
impl crate::Writable for BUCTRL_SPEC {
    type Writer = W;
}
#[doc = "`reset()` method sets BUCTRL to value 0"]
impl crate::Resettable for BUCTRL_SPEC {
    #[inline(always)]
    fn reset_value() -> Self::Ux {
        0
    }
}
