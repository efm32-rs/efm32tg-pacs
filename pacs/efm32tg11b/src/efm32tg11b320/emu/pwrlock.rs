#[doc = "Register `PWRLOCK` reader"]
pub struct R(crate::R<PWRLOCK_SPEC>);
impl core::ops::Deref for R {
    type Target = crate::R<PWRLOCK_SPEC>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
impl From<crate::R<PWRLOCK_SPEC>> for R {
    #[inline(always)]
    fn from(reader: crate::R<PWRLOCK_SPEC>) -> Self {
        R(reader)
    }
}
#[doc = "Register `PWRLOCK` writer"]
pub struct W(crate::W<PWRLOCK_SPEC>);
impl core::ops::Deref for W {
    type Target = crate::W<PWRLOCK_SPEC>;
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
impl From<crate::W<PWRLOCK_SPEC>> for W {
    #[inline(always)]
    fn from(writer: crate::W<PWRLOCK_SPEC>) -> Self {
        W(writer)
    }
}
#[doc = "Field `LOCKKEY` reader - Regulator and Supply Configuration Lock Key"]
pub type LOCKKEY_R = crate::FieldReader<u16, LOCKKEY_A>;
#[doc = "Regulator and Supply Configuration Lock Key\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
#[repr(u16)]
pub enum LOCKKEY_A {
    #[doc = "0: `0`"]
    UNLOCKED = 0,
    #[doc = "1: `1`"]
    LOCKED = 1,
}
impl From<LOCKKEY_A> for u16 {
    #[inline(always)]
    fn from(variant: LOCKKEY_A) -> Self {
        variant as _
    }
}
impl LOCKKEY_R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> Option<LOCKKEY_A> {
        match self.bits {
            0 => Some(LOCKKEY_A::UNLOCKED),
            1 => Some(LOCKKEY_A::LOCKED),
            _ => None,
        }
    }
    #[doc = "Checks if the value of the field is `UNLOCKED`"]
    #[inline(always)]
    pub fn is_unlocked(&self) -> bool {
        *self == LOCKKEY_A::UNLOCKED
    }
    #[doc = "Checks if the value of the field is `LOCKED`"]
    #[inline(always)]
    pub fn is_locked(&self) -> bool {
        *self == LOCKKEY_A::LOCKED
    }
}
#[doc = "Field `LOCKKEY` writer - Regulator and Supply Configuration Lock Key"]
pub type LOCKKEY_W<'a, const O: u8> =
    crate::FieldWriter<'a, u32, PWRLOCK_SPEC, u16, LOCKKEY_A, 16, O>;
impl<'a, const O: u8> LOCKKEY_W<'a, O> {
    #[doc = "`0`"]
    #[inline(always)]
    pub fn unlocked(self) -> &'a mut W {
        self.variant(LOCKKEY_A::UNLOCKED)
    }
    #[doc = "`1`"]
    #[inline(always)]
    pub fn locked(self) -> &'a mut W {
        self.variant(LOCKKEY_A::LOCKED)
    }
}
impl R {
    #[doc = "Bits 0:15 - Regulator and Supply Configuration Lock Key"]
    #[inline(always)]
    pub fn lockkey(&self) -> LOCKKEY_R {
        LOCKKEY_R::new((self.bits & 0xffff) as u16)
    }
}
impl W {
    #[doc = "Bits 0:15 - Regulator and Supply Configuration Lock Key"]
    #[inline(always)]
    #[must_use]
    pub fn lockkey(&mut self) -> LOCKKEY_W<0> {
        LOCKKEY_W::new(self)
    }
    #[doc = "Writes raw bits to the register."]
    #[inline(always)]
    pub unsafe fn bits(&mut self, bits: u32) -> &mut Self {
        self.0.bits(bits);
        self
    }
}
#[doc = "Regulator and Supply Lock Register\n\nThis register you can [`read`](crate::generic::Reg::read), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [pwrlock](index.html) module"]
pub struct PWRLOCK_SPEC;
impl crate::RegisterSpec for PWRLOCK_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [pwrlock::R](R) reader structure"]
impl crate::Readable for PWRLOCK_SPEC {
    type Reader = R;
}
#[doc = "`write(|w| ..)` method takes [pwrlock::W](W) writer structure"]
impl crate::Writable for PWRLOCK_SPEC {
    type Writer = W;
    const ZERO_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
}
#[doc = "`reset()` method sets PWRLOCK to value 0"]
impl crate::Resettable for PWRLOCK_SPEC {
    const RESET_VALUE: Self::Ux = 0;
}
