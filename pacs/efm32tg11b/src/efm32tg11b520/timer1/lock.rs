#[doc = "Register `LOCK` reader"]
pub struct R(crate::R<LOCK_SPEC>);
impl core::ops::Deref for R {
    type Target = crate::R<LOCK_SPEC>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
impl From<crate::R<LOCK_SPEC>> for R {
    #[inline(always)]
    fn from(reader: crate::R<LOCK_SPEC>) -> Self {
        R(reader)
    }
}
#[doc = "Register `LOCK` writer"]
pub struct W(crate::W<LOCK_SPEC>);
impl core::ops::Deref for W {
    type Target = crate::W<LOCK_SPEC>;
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
impl From<crate::W<LOCK_SPEC>> for W {
    #[inline(always)]
    fn from(writer: crate::W<LOCK_SPEC>) -> Self {
        W(writer)
    }
}
#[doc = "Field `TIMERLOCKKEY` reader - Timer Lock Key"]
pub type TIMERLOCKKEY_R = crate::FieldReader<u16, TIMERLOCKKEY_A>;
#[doc = "Timer Lock Key\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
#[repr(u16)]
pub enum TIMERLOCKKEY_A {
    #[doc = "0: `0`"]
    UNLOCKED = 0,
    #[doc = "1: `1`"]
    LOCKED = 1,
}
impl From<TIMERLOCKKEY_A> for u16 {
    #[inline(always)]
    fn from(variant: TIMERLOCKKEY_A) -> Self {
        variant as _
    }
}
impl TIMERLOCKKEY_R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> Option<TIMERLOCKKEY_A> {
        match self.bits {
            0 => Some(TIMERLOCKKEY_A::UNLOCKED),
            1 => Some(TIMERLOCKKEY_A::LOCKED),
            _ => None,
        }
    }
    #[doc = "Checks if the value of the field is `UNLOCKED`"]
    #[inline(always)]
    pub fn is_unlocked(&self) -> bool {
        *self == TIMERLOCKKEY_A::UNLOCKED
    }
    #[doc = "Checks if the value of the field is `LOCKED`"]
    #[inline(always)]
    pub fn is_locked(&self) -> bool {
        *self == TIMERLOCKKEY_A::LOCKED
    }
}
#[doc = "Field `TIMERLOCKKEY` writer - Timer Lock Key"]
pub type TIMERLOCKKEY_W<'a, const O: u8> =
    crate::FieldWriter<'a, u32, LOCK_SPEC, u16, TIMERLOCKKEY_A, 16, O>;
impl<'a, const O: u8> TIMERLOCKKEY_W<'a, O> {
    #[doc = "`0`"]
    #[inline(always)]
    pub fn unlocked(self) -> &'a mut W {
        self.variant(TIMERLOCKKEY_A::UNLOCKED)
    }
    #[doc = "`1`"]
    #[inline(always)]
    pub fn locked(self) -> &'a mut W {
        self.variant(TIMERLOCKKEY_A::LOCKED)
    }
}
impl R {
    #[doc = "Bits 0:15 - Timer Lock Key"]
    #[inline(always)]
    pub fn timerlockkey(&self) -> TIMERLOCKKEY_R {
        TIMERLOCKKEY_R::new((self.bits & 0xffff) as u16)
    }
}
impl W {
    #[doc = "Bits 0:15 - Timer Lock Key"]
    #[inline(always)]
    #[must_use]
    pub fn timerlockkey(&mut self) -> TIMERLOCKKEY_W<0> {
        TIMERLOCKKEY_W::new(self)
    }
    #[doc = "Writes raw bits to the register."]
    #[inline(always)]
    pub unsafe fn bits(&mut self, bits: u32) -> &mut Self {
        self.0.bits(bits);
        self
    }
}
#[doc = "TIMER Configuration Lock Register\n\nThis register you can [`read`](crate::generic::Reg::read), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [lock](index.html) module"]
pub struct LOCK_SPEC;
impl crate::RegisterSpec for LOCK_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [lock::R](R) reader structure"]
impl crate::Readable for LOCK_SPEC {
    type Reader = R;
}
#[doc = "`write(|w| ..)` method takes [lock::W](W) writer structure"]
impl crate::Writable for LOCK_SPEC {
    type Writer = W;
    const ZERO_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
}
#[doc = "`reset()` method sets LOCK to value 0"]
impl crate::Resettable for LOCK_SPEC {
    const RESET_VALUE: Self::Ux = 0;
}
