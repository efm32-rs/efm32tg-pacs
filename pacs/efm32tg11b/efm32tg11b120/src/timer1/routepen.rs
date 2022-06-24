#[doc = "Register `ROUTEPEN` reader"]
pub struct R(crate::R<ROUTEPEN_SPEC>);
impl core::ops::Deref for R {
    type Target = crate::R<ROUTEPEN_SPEC>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
impl From<crate::R<ROUTEPEN_SPEC>> for R {
    #[inline(always)]
    fn from(reader: crate::R<ROUTEPEN_SPEC>) -> Self {
        R(reader)
    }
}
#[doc = "Register `ROUTEPEN` writer"]
pub struct W(crate::W<ROUTEPEN_SPEC>);
impl core::ops::Deref for W {
    type Target = crate::W<ROUTEPEN_SPEC>;
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
impl From<crate::W<ROUTEPEN_SPEC>> for W {
    #[inline(always)]
    fn from(writer: crate::W<ROUTEPEN_SPEC>) -> Self {
        W(writer)
    }
}
#[doc = "Field `CC0PEN` reader - CC Channel 0 Pin Enable"]
pub type CC0PEN_R = crate::BitReader<bool>;
#[doc = "Field `CC0PEN` writer - CC Channel 0 Pin Enable"]
pub type CC0PEN_W<'a> = crate::BitWriter<'a, u32, ROUTEPEN_SPEC, bool, 0>;
#[doc = "Field `CC1PEN` reader - CC Channel 1 Pin Enable"]
pub type CC1PEN_R = crate::BitReader<bool>;
#[doc = "Field `CC1PEN` writer - CC Channel 1 Pin Enable"]
pub type CC1PEN_W<'a> = crate::BitWriter<'a, u32, ROUTEPEN_SPEC, bool, 1>;
#[doc = "Field `CC2PEN` reader - CC Channel 2 Pin Enable"]
pub type CC2PEN_R = crate::BitReader<bool>;
#[doc = "Field `CC2PEN` writer - CC Channel 2 Pin Enable"]
pub type CC2PEN_W<'a> = crate::BitWriter<'a, u32, ROUTEPEN_SPEC, bool, 2>;
#[doc = "Field `CC3PEN` reader - CC Channel 3 Pin Enable"]
pub type CC3PEN_R = crate::BitReader<bool>;
#[doc = "Field `CC3PEN` writer - CC Channel 3 Pin Enable"]
pub type CC3PEN_W<'a> = crate::BitWriter<'a, u32, ROUTEPEN_SPEC, bool, 3>;
#[doc = "Field `CDTI0PEN` reader - CC Channel 0 Complementary Dead-Time Insertion Pin Enable"]
pub type CDTI0PEN_R = crate::BitReader<bool>;
#[doc = "Field `CDTI0PEN` writer - CC Channel 0 Complementary Dead-Time Insertion Pin Enable"]
pub type CDTI0PEN_W<'a> = crate::BitWriter<'a, u32, ROUTEPEN_SPEC, bool, 8>;
#[doc = "Field `CDTI1PEN` reader - CC Channel 1 Complementary Dead-Time Insertion Pin Enable"]
pub type CDTI1PEN_R = crate::BitReader<bool>;
#[doc = "Field `CDTI1PEN` writer - CC Channel 1 Complementary Dead-Time Insertion Pin Enable"]
pub type CDTI1PEN_W<'a> = crate::BitWriter<'a, u32, ROUTEPEN_SPEC, bool, 9>;
#[doc = "Field `CDTI2PEN` reader - CC Channel 2 Complementary Dead-Time Insertion Pin Enable"]
pub type CDTI2PEN_R = crate::BitReader<bool>;
#[doc = "Field `CDTI2PEN` writer - CC Channel 2 Complementary Dead-Time Insertion Pin Enable"]
pub type CDTI2PEN_W<'a> = crate::BitWriter<'a, u32, ROUTEPEN_SPEC, bool, 10>;
impl R {
    #[doc = "Bit 0 - CC Channel 0 Pin Enable"]
    #[inline(always)]
    pub fn cc0pen(&self) -> CC0PEN_R {
        CC0PEN_R::new((self.bits & 1) != 0)
    }
    #[doc = "Bit 1 - CC Channel 1 Pin Enable"]
    #[inline(always)]
    pub fn cc1pen(&self) -> CC1PEN_R {
        CC1PEN_R::new(((self.bits >> 1) & 1) != 0)
    }
    #[doc = "Bit 2 - CC Channel 2 Pin Enable"]
    #[inline(always)]
    pub fn cc2pen(&self) -> CC2PEN_R {
        CC2PEN_R::new(((self.bits >> 2) & 1) != 0)
    }
    #[doc = "Bit 3 - CC Channel 3 Pin Enable"]
    #[inline(always)]
    pub fn cc3pen(&self) -> CC3PEN_R {
        CC3PEN_R::new(((self.bits >> 3) & 1) != 0)
    }
    #[doc = "Bit 8 - CC Channel 0 Complementary Dead-Time Insertion Pin Enable"]
    #[inline(always)]
    pub fn cdti0pen(&self) -> CDTI0PEN_R {
        CDTI0PEN_R::new(((self.bits >> 8) & 1) != 0)
    }
    #[doc = "Bit 9 - CC Channel 1 Complementary Dead-Time Insertion Pin Enable"]
    #[inline(always)]
    pub fn cdti1pen(&self) -> CDTI1PEN_R {
        CDTI1PEN_R::new(((self.bits >> 9) & 1) != 0)
    }
    #[doc = "Bit 10 - CC Channel 2 Complementary Dead-Time Insertion Pin Enable"]
    #[inline(always)]
    pub fn cdti2pen(&self) -> CDTI2PEN_R {
        CDTI2PEN_R::new(((self.bits >> 10) & 1) != 0)
    }
}
impl W {
    #[doc = "Bit 0 - CC Channel 0 Pin Enable"]
    #[inline(always)]
    pub fn cc0pen(&mut self) -> CC0PEN_W {
        CC0PEN_W::new(self)
    }
    #[doc = "Bit 1 - CC Channel 1 Pin Enable"]
    #[inline(always)]
    pub fn cc1pen(&mut self) -> CC1PEN_W {
        CC1PEN_W::new(self)
    }
    #[doc = "Bit 2 - CC Channel 2 Pin Enable"]
    #[inline(always)]
    pub fn cc2pen(&mut self) -> CC2PEN_W {
        CC2PEN_W::new(self)
    }
    #[doc = "Bit 3 - CC Channel 3 Pin Enable"]
    #[inline(always)]
    pub fn cc3pen(&mut self) -> CC3PEN_W {
        CC3PEN_W::new(self)
    }
    #[doc = "Bit 8 - CC Channel 0 Complementary Dead-Time Insertion Pin Enable"]
    #[inline(always)]
    pub fn cdti0pen(&mut self) -> CDTI0PEN_W {
        CDTI0PEN_W::new(self)
    }
    #[doc = "Bit 9 - CC Channel 1 Complementary Dead-Time Insertion Pin Enable"]
    #[inline(always)]
    pub fn cdti1pen(&mut self) -> CDTI1PEN_W {
        CDTI1PEN_W::new(self)
    }
    #[doc = "Bit 10 - CC Channel 2 Complementary Dead-Time Insertion Pin Enable"]
    #[inline(always)]
    pub fn cdti2pen(&mut self) -> CDTI2PEN_W {
        CDTI2PEN_W::new(self)
    }
    #[doc = "Writes raw bits to the register."]
    #[inline(always)]
    pub unsafe fn bits(&mut self, bits: u32) -> &mut Self {
        self.0.bits(bits);
        self
    }
}
#[doc = "I/O Routing Pin Enable Register\n\nThis register you can [`read`](crate::generic::Reg::read), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [routepen](index.html) module"]
pub struct ROUTEPEN_SPEC;
impl crate::RegisterSpec for ROUTEPEN_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [routepen::R](R) reader structure"]
impl crate::Readable for ROUTEPEN_SPEC {
    type Reader = R;
}
#[doc = "`write(|w| ..)` method takes [routepen::W](W) writer structure"]
impl crate::Writable for ROUTEPEN_SPEC {
    type Writer = W;
}
#[doc = "`reset()` method sets ROUTEPEN to value 0"]
impl crate::Resettable for ROUTEPEN_SPEC {
    #[inline(always)]
    fn reset_value() -> Self::Ux {
        0
    }
}
