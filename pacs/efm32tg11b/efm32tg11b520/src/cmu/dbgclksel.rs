#[doc = "Register `DBGCLKSEL` reader"]
pub struct R(crate::R<DBGCLKSEL_SPEC>);
impl core::ops::Deref for R {
    type Target = crate::R<DBGCLKSEL_SPEC>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
impl From<crate::R<DBGCLKSEL_SPEC>> for R {
    #[inline(always)]
    fn from(reader: crate::R<DBGCLKSEL_SPEC>) -> Self {
        R(reader)
    }
}
#[doc = "Register `DBGCLKSEL` writer"]
pub struct W(crate::W<DBGCLKSEL_SPEC>);
impl core::ops::Deref for W {
    type Target = crate::W<DBGCLKSEL_SPEC>;
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
impl From<crate::W<DBGCLKSEL_SPEC>> for W {
    #[inline(always)]
    fn from(writer: crate::W<DBGCLKSEL_SPEC>) -> Self {
        W(writer)
    }
}
#[doc = "Debug Trace Clock\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq)]
#[repr(u8)]
pub enum DBG_A {
    #[doc = "0: AUXHFRCO is the debug trace clock"]
    AUXHFRCO = 0,
    #[doc = "1: HFCLK is the debug trace clock"]
    HFCLK = 1,
    #[doc = "2: HFRCO divided by 2 is the debug trace clock"]
    HFRCODIV2 = 2,
}
impl From<DBG_A> for u8 {
    #[inline(always)]
    fn from(variant: DBG_A) -> Self {
        variant as _
    }
}
#[doc = "Field `DBG` reader - Debug Trace Clock"]
pub type DBG_R = crate::FieldReader<u8, DBG_A>;
impl DBG_R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> Option<DBG_A> {
        match self.bits {
            0 => Some(DBG_A::AUXHFRCO),
            1 => Some(DBG_A::HFCLK),
            2 => Some(DBG_A::HFRCODIV2),
            _ => None,
        }
    }
    #[doc = "Checks if the value of the field is `AUXHFRCO`"]
    #[inline(always)]
    pub fn is_auxhfrco(&self) -> bool {
        *self == DBG_A::AUXHFRCO
    }
    #[doc = "Checks if the value of the field is `HFCLK`"]
    #[inline(always)]
    pub fn is_hfclk(&self) -> bool {
        *self == DBG_A::HFCLK
    }
    #[doc = "Checks if the value of the field is `HFRCODIV2`"]
    #[inline(always)]
    pub fn is_hfrcodiv2(&self) -> bool {
        *self == DBG_A::HFRCODIV2
    }
}
#[doc = "Field `DBG` writer - Debug Trace Clock"]
pub type DBG_W<'a> = crate::FieldWriter<'a, u32, DBGCLKSEL_SPEC, u8, DBG_A, 2, 0>;
impl<'a> DBG_W<'a> {
    #[doc = "AUXHFRCO is the debug trace clock"]
    #[inline(always)]
    pub fn auxhfrco(self) -> &'a mut W {
        self.variant(DBG_A::AUXHFRCO)
    }
    #[doc = "HFCLK is the debug trace clock"]
    #[inline(always)]
    pub fn hfclk(self) -> &'a mut W {
        self.variant(DBG_A::HFCLK)
    }
    #[doc = "HFRCO divided by 2 is the debug trace clock"]
    #[inline(always)]
    pub fn hfrcodiv2(self) -> &'a mut W {
        self.variant(DBG_A::HFRCODIV2)
    }
}
impl R {
    #[doc = "Bits 0:1 - Debug Trace Clock"]
    #[inline(always)]
    pub fn dbg(&self) -> DBG_R {
        DBG_R::new((self.bits & 3) as u8)
    }
}
impl W {
    #[doc = "Bits 0:1 - Debug Trace Clock"]
    #[inline(always)]
    pub fn dbg(&mut self) -> DBG_W {
        DBG_W::new(self)
    }
    #[doc = "Writes raw bits to the register."]
    #[inline(always)]
    pub unsafe fn bits(&mut self, bits: u32) -> &mut Self {
        self.0.bits(bits);
        self
    }
}
#[doc = "Debug Trace Clock Select\n\nThis register you can [`read`](crate::generic::Reg::read), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [dbgclksel](index.html) module"]
pub struct DBGCLKSEL_SPEC;
impl crate::RegisterSpec for DBGCLKSEL_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [dbgclksel::R](R) reader structure"]
impl crate::Readable for DBGCLKSEL_SPEC {
    type Reader = R;
}
#[doc = "`write(|w| ..)` method takes [dbgclksel::W](W) writer structure"]
impl crate::Writable for DBGCLKSEL_SPEC {
    type Writer = W;
}
#[doc = "`reset()` method sets DBGCLKSEL to value 0"]
impl crate::Resettable for DBGCLKSEL_SPEC {
    #[inline(always)]
    fn reset_value() -> Self::Ux {
        0
    }
}
