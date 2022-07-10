#[doc = "Register `TIME` reader"]
pub struct R(crate::R<TIME_SPEC>);
impl core::ops::Deref for R {
    type Target = crate::R<TIME_SPEC>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
impl From<crate::R<TIME_SPEC>> for R {
    #[inline(always)]
    fn from(reader: crate::R<TIME_SPEC>) -> Self {
        R(reader)
    }
}
#[doc = "Register `TIME` writer"]
pub struct W(crate::W<TIME_SPEC>);
impl core::ops::Deref for W {
    type Target = crate::W<TIME_SPEC>;
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
impl From<crate::W<TIME_SPEC>> for W {
    #[inline(always)]
    fn from(writer: crate::W<TIME_SPEC>) -> Self {
        W(writer)
    }
}
#[doc = "Field `SECU` reader - Seconds, Units"]
pub type SECU_R = crate::FieldReader<u8, u8>;
#[doc = "Field `SECU` writer - Seconds, Units"]
pub type SECU_W<'a> = crate::FieldWriter<'a, u32, TIME_SPEC, u8, u8, 4, 0>;
#[doc = "Field `SECT` reader - Seconds, Tens"]
pub type SECT_R = crate::FieldReader<u8, u8>;
#[doc = "Field `SECT` writer - Seconds, Tens"]
pub type SECT_W<'a> = crate::FieldWriter<'a, u32, TIME_SPEC, u8, u8, 3, 4>;
#[doc = "Field `MINU` reader - Minutes, Units"]
pub type MINU_R = crate::FieldReader<u8, u8>;
#[doc = "Field `MINU` writer - Minutes, Units"]
pub type MINU_W<'a> = crate::FieldWriter<'a, u32, TIME_SPEC, u8, u8, 4, 8>;
#[doc = "Field `MINT` reader - Minutes, Tens"]
pub type MINT_R = crate::FieldReader<u8, u8>;
#[doc = "Field `MINT` writer - Minutes, Tens"]
pub type MINT_W<'a> = crate::FieldWriter<'a, u32, TIME_SPEC, u8, u8, 3, 12>;
#[doc = "Field `HOURU` reader - Hours, Units"]
pub type HOURU_R = crate::FieldReader<u8, u8>;
#[doc = "Field `HOURU` writer - Hours, Units"]
pub type HOURU_W<'a> = crate::FieldWriter<'a, u32, TIME_SPEC, u8, u8, 4, 16>;
#[doc = "Field `HOURT` reader - Hours, Tens"]
pub type HOURT_R = crate::FieldReader<u8, u8>;
#[doc = "Field `HOURT` writer - Hours, Tens"]
pub type HOURT_W<'a> = crate::FieldWriter<'a, u32, TIME_SPEC, u8, u8, 2, 20>;
impl R {
    #[doc = "Bits 0:3 - Seconds, Units"]
    #[inline(always)]
    pub fn secu(&self) -> SECU_R {
        SECU_R::new((self.bits & 0x0f) as u8)
    }
    #[doc = "Bits 4:6 - Seconds, Tens"]
    #[inline(always)]
    pub fn sect(&self) -> SECT_R {
        SECT_R::new(((self.bits >> 4) & 7) as u8)
    }
    #[doc = "Bits 8:11 - Minutes, Units"]
    #[inline(always)]
    pub fn minu(&self) -> MINU_R {
        MINU_R::new(((self.bits >> 8) & 0x0f) as u8)
    }
    #[doc = "Bits 12:14 - Minutes, Tens"]
    #[inline(always)]
    pub fn mint(&self) -> MINT_R {
        MINT_R::new(((self.bits >> 12) & 7) as u8)
    }
    #[doc = "Bits 16:19 - Hours, Units"]
    #[inline(always)]
    pub fn houru(&self) -> HOURU_R {
        HOURU_R::new(((self.bits >> 16) & 0x0f) as u8)
    }
    #[doc = "Bits 20:21 - Hours, Tens"]
    #[inline(always)]
    pub fn hourt(&self) -> HOURT_R {
        HOURT_R::new(((self.bits >> 20) & 3) as u8)
    }
}
impl W {
    #[doc = "Bits 0:3 - Seconds, Units"]
    #[inline(always)]
    pub fn secu(&mut self) -> SECU_W {
        SECU_W::new(self)
    }
    #[doc = "Bits 4:6 - Seconds, Tens"]
    #[inline(always)]
    pub fn sect(&mut self) -> SECT_W {
        SECT_W::new(self)
    }
    #[doc = "Bits 8:11 - Minutes, Units"]
    #[inline(always)]
    pub fn minu(&mut self) -> MINU_W {
        MINU_W::new(self)
    }
    #[doc = "Bits 12:14 - Minutes, Tens"]
    #[inline(always)]
    pub fn mint(&mut self) -> MINT_W {
        MINT_W::new(self)
    }
    #[doc = "Bits 16:19 - Hours, Units"]
    #[inline(always)]
    pub fn houru(&mut self) -> HOURU_W {
        HOURU_W::new(self)
    }
    #[doc = "Bits 20:21 - Hours, Tens"]
    #[inline(always)]
    pub fn hourt(&mut self) -> HOURT_W {
        HOURT_W::new(self)
    }
    #[doc = "Writes raw bits to the register."]
    #[inline(always)]
    pub unsafe fn bits(&mut self, bits: u32) -> &mut Self {
        self.0.bits(bits);
        self
    }
}
#[doc = "Time of Day Register\n\nThis register you can [`read`](crate::generic::Reg::read), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [time](index.html) module"]
pub struct TIME_SPEC;
impl crate::RegisterSpec for TIME_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [time::R](R) reader structure"]
impl crate::Readable for TIME_SPEC {
    type Reader = R;
}
#[doc = "`write(|w| ..)` method takes [time::W](W) writer structure"]
impl crate::Writable for TIME_SPEC {
    type Writer = W;
}
#[doc = "`reset()` method sets TIME to value 0"]
impl crate::Resettable for TIME_SPEC {
    #[inline(always)]
    fn reset_value() -> Self::Ux {
        0
    }
}
