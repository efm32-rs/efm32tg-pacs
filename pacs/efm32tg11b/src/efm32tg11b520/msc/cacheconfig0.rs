#[doc = "Register `CACHECONFIG0` reader"]
pub struct R(crate::R<CACHECONFIG0_SPEC>);
impl core::ops::Deref for R {
    type Target = crate::R<CACHECONFIG0_SPEC>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
impl From<crate::R<CACHECONFIG0_SPEC>> for R {
    #[inline(always)]
    fn from(reader: crate::R<CACHECONFIG0_SPEC>) -> Self {
        R(reader)
    }
}
#[doc = "Register `CACHECONFIG0` writer"]
pub struct W(crate::W<CACHECONFIG0_SPEC>);
impl core::ops::Deref for W {
    type Target = crate::W<CACHECONFIG0_SPEC>;
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
impl From<crate::W<CACHECONFIG0_SPEC>> for W {
    #[inline(always)]
    fn from(writer: crate::W<CACHECONFIG0_SPEC>) -> Self {
        W(writer)
    }
}
#[doc = "Field `CACHELPLEVEL` reader - Instruction Cache Low-Power Level"]
pub type CACHELPLEVEL_R = crate::FieldReader<u8, CACHELPLEVEL_A>;
#[doc = "Instruction Cache Low-Power Level\n\nValue on reset: 3"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
#[repr(u8)]
pub enum CACHELPLEVEL_A {
    #[doc = "0: Base instruction cache functionality."]
    BASE = 0,
    #[doc = "1: Advanced buffering mode, where the cache uses the fetch pattern to predict highly accessed data and store it in low-energy memory."]
    ADVANCED = 1,
    #[doc = "3: Minimum activity mode, which allows the cache to minimize activity in logic that it predicts has a low probability being used. This mode can introduce wait-states into the instruction fetch stream when the cache exits one of its low-activity states. The number of wait-states introduced is small, but users running with 0-wait-state memory and wishing to reduce the variability that the cache might introduce with additional wait-states may wish to lower the cache low-power level. Note, this mode includes the advanced buffering mode functionality."]
    MINACTIVITY = 3,
}
impl From<CACHELPLEVEL_A> for u8 {
    #[inline(always)]
    fn from(variant: CACHELPLEVEL_A) -> Self {
        variant as _
    }
}
impl CACHELPLEVEL_R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> Option<CACHELPLEVEL_A> {
        match self.bits {
            0 => Some(CACHELPLEVEL_A::BASE),
            1 => Some(CACHELPLEVEL_A::ADVANCED),
            3 => Some(CACHELPLEVEL_A::MINACTIVITY),
            _ => None,
        }
    }
    #[doc = "Checks if the value of the field is `BASE`"]
    #[inline(always)]
    pub fn is_base(&self) -> bool {
        *self == CACHELPLEVEL_A::BASE
    }
    #[doc = "Checks if the value of the field is `ADVANCED`"]
    #[inline(always)]
    pub fn is_advanced(&self) -> bool {
        *self == CACHELPLEVEL_A::ADVANCED
    }
    #[doc = "Checks if the value of the field is `MINACTIVITY`"]
    #[inline(always)]
    pub fn is_minactivity(&self) -> bool {
        *self == CACHELPLEVEL_A::MINACTIVITY
    }
}
#[doc = "Field `CACHELPLEVEL` writer - Instruction Cache Low-Power Level"]
pub type CACHELPLEVEL_W<'a, const O: u8> =
    crate::FieldWriter<'a, u32, CACHECONFIG0_SPEC, u8, CACHELPLEVEL_A, 2, O>;
impl<'a, const O: u8> CACHELPLEVEL_W<'a, O> {
    #[doc = "Base instruction cache functionality."]
    #[inline(always)]
    pub fn base(self) -> &'a mut W {
        self.variant(CACHELPLEVEL_A::BASE)
    }
    #[doc = "Advanced buffering mode, where the cache uses the fetch pattern to predict highly accessed data and store it in low-energy memory."]
    #[inline(always)]
    pub fn advanced(self) -> &'a mut W {
        self.variant(CACHELPLEVEL_A::ADVANCED)
    }
    #[doc = "Minimum activity mode, which allows the cache to minimize activity in logic that it predicts has a low probability being used. This mode can introduce wait-states into the instruction fetch stream when the cache exits one of its low-activity states. The number of wait-states introduced is small, but users running with 0-wait-state memory and wishing to reduce the variability that the cache might introduce with additional wait-states may wish to lower the cache low-power level. Note, this mode includes the advanced buffering mode functionality."]
    #[inline(always)]
    pub fn minactivity(self) -> &'a mut W {
        self.variant(CACHELPLEVEL_A::MINACTIVITY)
    }
}
impl R {
    #[doc = "Bits 0:1 - Instruction Cache Low-Power Level"]
    #[inline(always)]
    pub fn cachelplevel(&self) -> CACHELPLEVEL_R {
        CACHELPLEVEL_R::new((self.bits & 3) as u8)
    }
}
impl W {
    #[doc = "Bits 0:1 - Instruction Cache Low-Power Level"]
    #[inline(always)]
    #[must_use]
    pub fn cachelplevel(&mut self) -> CACHELPLEVEL_W<0> {
        CACHELPLEVEL_W::new(self)
    }
    #[doc = "Writes raw bits to the register."]
    #[inline(always)]
    pub unsafe fn bits(&mut self, bits: u32) -> &mut Self {
        self.0.bits(bits);
        self
    }
}
#[doc = "Cache Configuration Register 0\n\nThis register you can [`read`](crate::generic::Reg::read), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [cacheconfig0](index.html) module"]
pub struct CACHECONFIG0_SPEC;
impl crate::RegisterSpec for CACHECONFIG0_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [cacheconfig0::R](R) reader structure"]
impl crate::Readable for CACHECONFIG0_SPEC {
    type Reader = R;
}
#[doc = "`write(|w| ..)` method takes [cacheconfig0::W](W) writer structure"]
impl crate::Writable for CACHECONFIG0_SPEC {
    type Writer = W;
    const ZERO_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
}
#[doc = "`reset()` method sets CACHECONFIG0 to value 0x03"]
impl crate::Resettable for CACHECONFIG0_SPEC {
    const RESET_VALUE: Self::Ux = 0x03;
}
