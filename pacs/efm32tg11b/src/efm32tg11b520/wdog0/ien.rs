#[doc = "Register `IEN` reader"]
pub struct R(crate::R<IEN_SPEC>);
impl core::ops::Deref for R {
    type Target = crate::R<IEN_SPEC>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
impl From<crate::R<IEN_SPEC>> for R {
    #[inline(always)]
    fn from(reader: crate::R<IEN_SPEC>) -> Self {
        R(reader)
    }
}
#[doc = "Register `IEN` writer"]
pub struct W(crate::W<IEN_SPEC>);
impl core::ops::Deref for W {
    type Target = crate::W<IEN_SPEC>;
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
impl From<crate::W<IEN_SPEC>> for W {
    #[inline(always)]
    fn from(writer: crate::W<IEN_SPEC>) -> Self {
        W(writer)
    }
}
#[doc = "Field `TOUT` reader - TOUT Interrupt Enable"]
pub type TOUT_R = crate::BitReader<bool>;
#[doc = "Field `TOUT` writer - TOUT Interrupt Enable"]
pub type TOUT_W<'a> = crate::BitWriter<'a, u32, IEN_SPEC, bool, 0>;
#[doc = "Field `WARN` reader - WARN Interrupt Enable"]
pub type WARN_R = crate::BitReader<bool>;
#[doc = "Field `WARN` writer - WARN Interrupt Enable"]
pub type WARN_W<'a> = crate::BitWriter<'a, u32, IEN_SPEC, bool, 1>;
#[doc = "Field `WIN` reader - WIN Interrupt Enable"]
pub type WIN_R = crate::BitReader<bool>;
#[doc = "Field `WIN` writer - WIN Interrupt Enable"]
pub type WIN_W<'a> = crate::BitWriter<'a, u32, IEN_SPEC, bool, 2>;
#[doc = "Field `PEM0` reader - PEM0 Interrupt Enable"]
pub type PEM0_R = crate::BitReader<bool>;
#[doc = "Field `PEM0` writer - PEM0 Interrupt Enable"]
pub type PEM0_W<'a> = crate::BitWriter<'a, u32, IEN_SPEC, bool, 3>;
#[doc = "Field `PEM1` reader - PEM1 Interrupt Enable"]
pub type PEM1_R = crate::BitReader<bool>;
#[doc = "Field `PEM1` writer - PEM1 Interrupt Enable"]
pub type PEM1_W<'a> = crate::BitWriter<'a, u32, IEN_SPEC, bool, 4>;
impl R {
    #[doc = "Bit 0 - TOUT Interrupt Enable"]
    #[inline(always)]
    pub fn tout(&self) -> TOUT_R {
        TOUT_R::new((self.bits & 1) != 0)
    }
    #[doc = "Bit 1 - WARN Interrupt Enable"]
    #[inline(always)]
    pub fn warn(&self) -> WARN_R {
        WARN_R::new(((self.bits >> 1) & 1) != 0)
    }
    #[doc = "Bit 2 - WIN Interrupt Enable"]
    #[inline(always)]
    pub fn win(&self) -> WIN_R {
        WIN_R::new(((self.bits >> 2) & 1) != 0)
    }
    #[doc = "Bit 3 - PEM0 Interrupt Enable"]
    #[inline(always)]
    pub fn pem0(&self) -> PEM0_R {
        PEM0_R::new(((self.bits >> 3) & 1) != 0)
    }
    #[doc = "Bit 4 - PEM1 Interrupt Enable"]
    #[inline(always)]
    pub fn pem1(&self) -> PEM1_R {
        PEM1_R::new(((self.bits >> 4) & 1) != 0)
    }
}
impl W {
    #[doc = "Bit 0 - TOUT Interrupt Enable"]
    #[inline(always)]
    pub fn tout(&mut self) -> TOUT_W {
        TOUT_W::new(self)
    }
    #[doc = "Bit 1 - WARN Interrupt Enable"]
    #[inline(always)]
    pub fn warn(&mut self) -> WARN_W {
        WARN_W::new(self)
    }
    #[doc = "Bit 2 - WIN Interrupt Enable"]
    #[inline(always)]
    pub fn win(&mut self) -> WIN_W {
        WIN_W::new(self)
    }
    #[doc = "Bit 3 - PEM0 Interrupt Enable"]
    #[inline(always)]
    pub fn pem0(&mut self) -> PEM0_W {
        PEM0_W::new(self)
    }
    #[doc = "Bit 4 - PEM1 Interrupt Enable"]
    #[inline(always)]
    pub fn pem1(&mut self) -> PEM1_W {
        PEM1_W::new(self)
    }
    #[doc = "Writes raw bits to the register."]
    #[inline(always)]
    pub unsafe fn bits(&mut self, bits: u32) -> &mut Self {
        self.0.bits(bits);
        self
    }
}
#[doc = "Interrupt Enable Register\n\nThis register you can [`read`](crate::generic::Reg::read), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [ien](index.html) module"]
pub struct IEN_SPEC;
impl crate::RegisterSpec for IEN_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [ien::R](R) reader structure"]
impl crate::Readable for IEN_SPEC {
    type Reader = R;
}
#[doc = "`write(|w| ..)` method takes [ien::W](W) writer structure"]
impl crate::Writable for IEN_SPEC {
    type Writer = W;
}
#[doc = "`reset()` method sets IEN to value 0"]
impl crate::Resettable for IEN_SPEC {
    #[inline(always)]
    fn reset_value() -> Self::Ux {
        0
    }
}
