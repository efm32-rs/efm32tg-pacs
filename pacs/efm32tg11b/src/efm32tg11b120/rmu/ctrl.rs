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
#[doc = "Field `WDOGRMODE` reader - WDOG Reset Mode"]
pub type WDOGRMODE_R = crate::FieldReader<u8, WDOGRMODE_A>;
#[doc = "WDOG Reset Mode\n\nValue on reset: 4"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
#[repr(u8)]
pub enum WDOGRMODE_A {
    #[doc = "0: Reset request is blocked. This disable bit is redundant with enable/disable bit in WDOG"]
    DISABLED = 0,
    #[doc = "1: The CRYOTIMER, DEBUGGER, RTCC, are not reset."]
    LIMITED = 1,
    #[doc = "2: The CRYOTIMER, DEBUGGER are not reset. RTCC is reset."]
    EXTENDED = 2,
    #[doc = "4: The entire device is reset except some EMU and RMU registers."]
    FULL = 4,
}
impl From<WDOGRMODE_A> for u8 {
    #[inline(always)]
    fn from(variant: WDOGRMODE_A) -> Self {
        variant as _
    }
}
impl WDOGRMODE_R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> Option<WDOGRMODE_A> {
        match self.bits {
            0 => Some(WDOGRMODE_A::DISABLED),
            1 => Some(WDOGRMODE_A::LIMITED),
            2 => Some(WDOGRMODE_A::EXTENDED),
            4 => Some(WDOGRMODE_A::FULL),
            _ => None,
        }
    }
    #[doc = "Checks if the value of the field is `DISABLED`"]
    #[inline(always)]
    pub fn is_disabled(&self) -> bool {
        *self == WDOGRMODE_A::DISABLED
    }
    #[doc = "Checks if the value of the field is `LIMITED`"]
    #[inline(always)]
    pub fn is_limited(&self) -> bool {
        *self == WDOGRMODE_A::LIMITED
    }
    #[doc = "Checks if the value of the field is `EXTENDED`"]
    #[inline(always)]
    pub fn is_extended(&self) -> bool {
        *self == WDOGRMODE_A::EXTENDED
    }
    #[doc = "Checks if the value of the field is `FULL`"]
    #[inline(always)]
    pub fn is_full(&self) -> bool {
        *self == WDOGRMODE_A::FULL
    }
}
#[doc = "Field `WDOGRMODE` writer - WDOG Reset Mode"]
pub type WDOGRMODE_W<'a, const O: u8> =
    crate::FieldWriter<'a, u32, CTRL_SPEC, u8, WDOGRMODE_A, 3, O>;
impl<'a, const O: u8> WDOGRMODE_W<'a, O> {
    #[doc = "Reset request is blocked. This disable bit is redundant with enable/disable bit in WDOG"]
    #[inline(always)]
    pub fn disabled(self) -> &'a mut W {
        self.variant(WDOGRMODE_A::DISABLED)
    }
    #[doc = "The CRYOTIMER, DEBUGGER, RTCC, are not reset."]
    #[inline(always)]
    pub fn limited(self) -> &'a mut W {
        self.variant(WDOGRMODE_A::LIMITED)
    }
    #[doc = "The CRYOTIMER, DEBUGGER are not reset. RTCC is reset."]
    #[inline(always)]
    pub fn extended(self) -> &'a mut W {
        self.variant(WDOGRMODE_A::EXTENDED)
    }
    #[doc = "The entire device is reset except some EMU and RMU registers."]
    #[inline(always)]
    pub fn full(self) -> &'a mut W {
        self.variant(WDOGRMODE_A::FULL)
    }
}
#[doc = "Field `LOCKUPRMODE` reader - Core LOCKUP Reset Mode"]
pub type LOCKUPRMODE_R = crate::FieldReader<u8, LOCKUPRMODE_A>;
#[doc = "Core LOCKUP Reset Mode\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
#[repr(u8)]
pub enum LOCKUPRMODE_A {
    #[doc = "0: Reset request is blocked."]
    DISABLED = 0,
    #[doc = "1: The CRYOTIMER, DEBUGGER, RTCC, are not reset."]
    LIMITED = 1,
    #[doc = "2: The CRYOTIMER, DEBUGGER are not reset. RTCC is reset."]
    EXTENDED = 2,
    #[doc = "4: The entire device is reset except some EMU and RMU registers."]
    FULL = 4,
}
impl From<LOCKUPRMODE_A> for u8 {
    #[inline(always)]
    fn from(variant: LOCKUPRMODE_A) -> Self {
        variant as _
    }
}
impl LOCKUPRMODE_R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> Option<LOCKUPRMODE_A> {
        match self.bits {
            0 => Some(LOCKUPRMODE_A::DISABLED),
            1 => Some(LOCKUPRMODE_A::LIMITED),
            2 => Some(LOCKUPRMODE_A::EXTENDED),
            4 => Some(LOCKUPRMODE_A::FULL),
            _ => None,
        }
    }
    #[doc = "Checks if the value of the field is `DISABLED`"]
    #[inline(always)]
    pub fn is_disabled(&self) -> bool {
        *self == LOCKUPRMODE_A::DISABLED
    }
    #[doc = "Checks if the value of the field is `LIMITED`"]
    #[inline(always)]
    pub fn is_limited(&self) -> bool {
        *self == LOCKUPRMODE_A::LIMITED
    }
    #[doc = "Checks if the value of the field is `EXTENDED`"]
    #[inline(always)]
    pub fn is_extended(&self) -> bool {
        *self == LOCKUPRMODE_A::EXTENDED
    }
    #[doc = "Checks if the value of the field is `FULL`"]
    #[inline(always)]
    pub fn is_full(&self) -> bool {
        *self == LOCKUPRMODE_A::FULL
    }
}
#[doc = "Field `LOCKUPRMODE` writer - Core LOCKUP Reset Mode"]
pub type LOCKUPRMODE_W<'a, const O: u8> =
    crate::FieldWriter<'a, u32, CTRL_SPEC, u8, LOCKUPRMODE_A, 3, O>;
impl<'a, const O: u8> LOCKUPRMODE_W<'a, O> {
    #[doc = "Reset request is blocked."]
    #[inline(always)]
    pub fn disabled(self) -> &'a mut W {
        self.variant(LOCKUPRMODE_A::DISABLED)
    }
    #[doc = "The CRYOTIMER, DEBUGGER, RTCC, are not reset."]
    #[inline(always)]
    pub fn limited(self) -> &'a mut W {
        self.variant(LOCKUPRMODE_A::LIMITED)
    }
    #[doc = "The CRYOTIMER, DEBUGGER are not reset. RTCC is reset."]
    #[inline(always)]
    pub fn extended(self) -> &'a mut W {
        self.variant(LOCKUPRMODE_A::EXTENDED)
    }
    #[doc = "The entire device is reset except some EMU and RMU registers."]
    #[inline(always)]
    pub fn full(self) -> &'a mut W {
        self.variant(LOCKUPRMODE_A::FULL)
    }
}
#[doc = "Field `SYSRMODE` reader - Core Sysreset Reset Mode"]
pub type SYSRMODE_R = crate::FieldReader<u8, SYSRMODE_A>;
#[doc = "Core Sysreset Reset Mode\n\nValue on reset: 2"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
#[repr(u8)]
pub enum SYSRMODE_A {
    #[doc = "0: Reset request is blocked."]
    DISABLED = 0,
    #[doc = "1: The CRYOTIMER, DEBUGGER, RTCC, are not reset."]
    LIMITED = 1,
    #[doc = "2: The CRYOTIMER, DEBUGGER are not reset. RTCC is reset."]
    EXTENDED = 2,
    #[doc = "4: The entire device is reset except some EMU and RMU registers."]
    FULL = 4,
}
impl From<SYSRMODE_A> for u8 {
    #[inline(always)]
    fn from(variant: SYSRMODE_A) -> Self {
        variant as _
    }
}
impl SYSRMODE_R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> Option<SYSRMODE_A> {
        match self.bits {
            0 => Some(SYSRMODE_A::DISABLED),
            1 => Some(SYSRMODE_A::LIMITED),
            2 => Some(SYSRMODE_A::EXTENDED),
            4 => Some(SYSRMODE_A::FULL),
            _ => None,
        }
    }
    #[doc = "Checks if the value of the field is `DISABLED`"]
    #[inline(always)]
    pub fn is_disabled(&self) -> bool {
        *self == SYSRMODE_A::DISABLED
    }
    #[doc = "Checks if the value of the field is `LIMITED`"]
    #[inline(always)]
    pub fn is_limited(&self) -> bool {
        *self == SYSRMODE_A::LIMITED
    }
    #[doc = "Checks if the value of the field is `EXTENDED`"]
    #[inline(always)]
    pub fn is_extended(&self) -> bool {
        *self == SYSRMODE_A::EXTENDED
    }
    #[doc = "Checks if the value of the field is `FULL`"]
    #[inline(always)]
    pub fn is_full(&self) -> bool {
        *self == SYSRMODE_A::FULL
    }
}
#[doc = "Field `SYSRMODE` writer - Core Sysreset Reset Mode"]
pub type SYSRMODE_W<'a, const O: u8> = crate::FieldWriter<'a, u32, CTRL_SPEC, u8, SYSRMODE_A, 3, O>;
impl<'a, const O: u8> SYSRMODE_W<'a, O> {
    #[doc = "Reset request is blocked."]
    #[inline(always)]
    pub fn disabled(self) -> &'a mut W {
        self.variant(SYSRMODE_A::DISABLED)
    }
    #[doc = "The CRYOTIMER, DEBUGGER, RTCC, are not reset."]
    #[inline(always)]
    pub fn limited(self) -> &'a mut W {
        self.variant(SYSRMODE_A::LIMITED)
    }
    #[doc = "The CRYOTIMER, DEBUGGER are not reset. RTCC is reset."]
    #[inline(always)]
    pub fn extended(self) -> &'a mut W {
        self.variant(SYSRMODE_A::EXTENDED)
    }
    #[doc = "The entire device is reset except some EMU and RMU registers."]
    #[inline(always)]
    pub fn full(self) -> &'a mut W {
        self.variant(SYSRMODE_A::FULL)
    }
}
#[doc = "Field `PINRMODE` reader - PIN Reset Mode"]
pub type PINRMODE_R = crate::FieldReader<u8, PINRMODE_A>;
#[doc = "PIN Reset Mode\n\nValue on reset: 4"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
#[repr(u8)]
pub enum PINRMODE_A {
    #[doc = "0: Reset request is blocked."]
    DISABLED = 0,
    #[doc = "1: The CRYOTIMER, DEBUGGER, RTCC, are not reset."]
    LIMITED = 1,
    #[doc = "2: The CRYOTIMER, DEBUGGER are not reset. RTCC is reset."]
    EXTENDED = 2,
    #[doc = "4: The entire device is reset except some EMU and RMU registers."]
    FULL = 4,
}
impl From<PINRMODE_A> for u8 {
    #[inline(always)]
    fn from(variant: PINRMODE_A) -> Self {
        variant as _
    }
}
impl PINRMODE_R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> Option<PINRMODE_A> {
        match self.bits {
            0 => Some(PINRMODE_A::DISABLED),
            1 => Some(PINRMODE_A::LIMITED),
            2 => Some(PINRMODE_A::EXTENDED),
            4 => Some(PINRMODE_A::FULL),
            _ => None,
        }
    }
    #[doc = "Checks if the value of the field is `DISABLED`"]
    #[inline(always)]
    pub fn is_disabled(&self) -> bool {
        *self == PINRMODE_A::DISABLED
    }
    #[doc = "Checks if the value of the field is `LIMITED`"]
    #[inline(always)]
    pub fn is_limited(&self) -> bool {
        *self == PINRMODE_A::LIMITED
    }
    #[doc = "Checks if the value of the field is `EXTENDED`"]
    #[inline(always)]
    pub fn is_extended(&self) -> bool {
        *self == PINRMODE_A::EXTENDED
    }
    #[doc = "Checks if the value of the field is `FULL`"]
    #[inline(always)]
    pub fn is_full(&self) -> bool {
        *self == PINRMODE_A::FULL
    }
}
#[doc = "Field `PINRMODE` writer - PIN Reset Mode"]
pub type PINRMODE_W<'a, const O: u8> = crate::FieldWriter<'a, u32, CTRL_SPEC, u8, PINRMODE_A, 3, O>;
impl<'a, const O: u8> PINRMODE_W<'a, O> {
    #[doc = "Reset request is blocked."]
    #[inline(always)]
    pub fn disabled(self) -> &'a mut W {
        self.variant(PINRMODE_A::DISABLED)
    }
    #[doc = "The CRYOTIMER, DEBUGGER, RTCC, are not reset."]
    #[inline(always)]
    pub fn limited(self) -> &'a mut W {
        self.variant(PINRMODE_A::LIMITED)
    }
    #[doc = "The CRYOTIMER, DEBUGGER are not reset. RTCC is reset."]
    #[inline(always)]
    pub fn extended(self) -> &'a mut W {
        self.variant(PINRMODE_A::EXTENDED)
    }
    #[doc = "The entire device is reset except some EMU and RMU registers."]
    #[inline(always)]
    pub fn full(self) -> &'a mut W {
        self.variant(PINRMODE_A::FULL)
    }
}
#[doc = "Field `RESETSTATE` reader - System Software Reset State"]
pub type RESETSTATE_R = crate::FieldReader<u8, u8>;
#[doc = "Field `RESETSTATE` writer - System Software Reset State"]
pub type RESETSTATE_W<'a, const O: u8> = crate::FieldWriter<'a, u32, CTRL_SPEC, u8, u8, 2, O>;
impl R {
    #[doc = "Bits 0:2 - WDOG Reset Mode"]
    #[inline(always)]
    pub fn wdogrmode(&self) -> WDOGRMODE_R {
        WDOGRMODE_R::new((self.bits & 7) as u8)
    }
    #[doc = "Bits 4:6 - Core LOCKUP Reset Mode"]
    #[inline(always)]
    pub fn lockuprmode(&self) -> LOCKUPRMODE_R {
        LOCKUPRMODE_R::new(((self.bits >> 4) & 7) as u8)
    }
    #[doc = "Bits 8:10 - Core Sysreset Reset Mode"]
    #[inline(always)]
    pub fn sysrmode(&self) -> SYSRMODE_R {
        SYSRMODE_R::new(((self.bits >> 8) & 7) as u8)
    }
    #[doc = "Bits 12:14 - PIN Reset Mode"]
    #[inline(always)]
    pub fn pinrmode(&self) -> PINRMODE_R {
        PINRMODE_R::new(((self.bits >> 12) & 7) as u8)
    }
    #[doc = "Bits 24:25 - System Software Reset State"]
    #[inline(always)]
    pub fn resetstate(&self) -> RESETSTATE_R {
        RESETSTATE_R::new(((self.bits >> 24) & 3) as u8)
    }
}
impl W {
    #[doc = "Bits 0:2 - WDOG Reset Mode"]
    #[inline(always)]
    #[must_use]
    pub fn wdogrmode(&mut self) -> WDOGRMODE_W<0> {
        WDOGRMODE_W::new(self)
    }
    #[doc = "Bits 4:6 - Core LOCKUP Reset Mode"]
    #[inline(always)]
    #[must_use]
    pub fn lockuprmode(&mut self) -> LOCKUPRMODE_W<4> {
        LOCKUPRMODE_W::new(self)
    }
    #[doc = "Bits 8:10 - Core Sysreset Reset Mode"]
    #[inline(always)]
    #[must_use]
    pub fn sysrmode(&mut self) -> SYSRMODE_W<8> {
        SYSRMODE_W::new(self)
    }
    #[doc = "Bits 12:14 - PIN Reset Mode"]
    #[inline(always)]
    #[must_use]
    pub fn pinrmode(&mut self) -> PINRMODE_W<12> {
        PINRMODE_W::new(self)
    }
    #[doc = "Bits 24:25 - System Software Reset State"]
    #[inline(always)]
    #[must_use]
    pub fn resetstate(&mut self) -> RESETSTATE_W<24> {
        RESETSTATE_W::new(self)
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
#[doc = "`reset()` method sets CTRL to value 0x4204"]
impl crate::Resettable for CTRL_SPEC {
    const RESET_VALUE: Self::Ux = 0x4204;
}
