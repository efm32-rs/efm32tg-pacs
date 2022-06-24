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
#[doc = "Field `OF` reader - OF Interrupt Enable"]
pub type OF_R = crate::BitReader<bool>;
#[doc = "Field `OF` writer - OF Interrupt Enable"]
pub type OF_W<'a> = crate::BitWriter<'a, u32, IEN_SPEC, bool, 0>;
#[doc = "Field `UF` reader - UF Interrupt Enable"]
pub type UF_R = crate::BitReader<bool>;
#[doc = "Field `UF` writer - UF Interrupt Enable"]
pub type UF_W<'a> = crate::BitWriter<'a, u32, IEN_SPEC, bool, 1>;
#[doc = "Field `DIRCHG` reader - DIRCHG Interrupt Enable"]
pub type DIRCHG_R = crate::BitReader<bool>;
#[doc = "Field `DIRCHG` writer - DIRCHG Interrupt Enable"]
pub type DIRCHG_W<'a> = crate::BitWriter<'a, u32, IEN_SPEC, bool, 2>;
#[doc = "Field `CC0` reader - CC0 Interrupt Enable"]
pub type CC0_R = crate::BitReader<bool>;
#[doc = "Field `CC0` writer - CC0 Interrupt Enable"]
pub type CC0_W<'a> = crate::BitWriter<'a, u32, IEN_SPEC, bool, 4>;
#[doc = "Field `CC1` reader - CC1 Interrupt Enable"]
pub type CC1_R = crate::BitReader<bool>;
#[doc = "Field `CC1` writer - CC1 Interrupt Enable"]
pub type CC1_W<'a> = crate::BitWriter<'a, u32, IEN_SPEC, bool, 5>;
#[doc = "Field `CC2` reader - CC2 Interrupt Enable"]
pub type CC2_R = crate::BitReader<bool>;
#[doc = "Field `CC2` writer - CC2 Interrupt Enable"]
pub type CC2_W<'a> = crate::BitWriter<'a, u32, IEN_SPEC, bool, 6>;
#[doc = "Field `CC3` reader - CC3 Interrupt Enable"]
pub type CC3_R = crate::BitReader<bool>;
#[doc = "Field `CC3` writer - CC3 Interrupt Enable"]
pub type CC3_W<'a> = crate::BitWriter<'a, u32, IEN_SPEC, bool, 7>;
#[doc = "Field `ICBOF0` reader - ICBOF0 Interrupt Enable"]
pub type ICBOF0_R = crate::BitReader<bool>;
#[doc = "Field `ICBOF0` writer - ICBOF0 Interrupt Enable"]
pub type ICBOF0_W<'a> = crate::BitWriter<'a, u32, IEN_SPEC, bool, 8>;
#[doc = "Field `ICBOF1` reader - ICBOF1 Interrupt Enable"]
pub type ICBOF1_R = crate::BitReader<bool>;
#[doc = "Field `ICBOF1` writer - ICBOF1 Interrupt Enable"]
pub type ICBOF1_W<'a> = crate::BitWriter<'a, u32, IEN_SPEC, bool, 9>;
#[doc = "Field `ICBOF2` reader - ICBOF2 Interrupt Enable"]
pub type ICBOF2_R = crate::BitReader<bool>;
#[doc = "Field `ICBOF2` writer - ICBOF2 Interrupt Enable"]
pub type ICBOF2_W<'a> = crate::BitWriter<'a, u32, IEN_SPEC, bool, 10>;
#[doc = "Field `ICBOF3` reader - ICBOF3 Interrupt Enable"]
pub type ICBOF3_R = crate::BitReader<bool>;
#[doc = "Field `ICBOF3` writer - ICBOF3 Interrupt Enable"]
pub type ICBOF3_W<'a> = crate::BitWriter<'a, u32, IEN_SPEC, bool, 11>;
impl R {
    #[doc = "Bit 0 - OF Interrupt Enable"]
    #[inline(always)]
    pub fn of(&self) -> OF_R {
        OF_R::new((self.bits & 1) != 0)
    }
    #[doc = "Bit 1 - UF Interrupt Enable"]
    #[inline(always)]
    pub fn uf(&self) -> UF_R {
        UF_R::new(((self.bits >> 1) & 1) != 0)
    }
    #[doc = "Bit 2 - DIRCHG Interrupt Enable"]
    #[inline(always)]
    pub fn dirchg(&self) -> DIRCHG_R {
        DIRCHG_R::new(((self.bits >> 2) & 1) != 0)
    }
    #[doc = "Bit 4 - CC0 Interrupt Enable"]
    #[inline(always)]
    pub fn cc0(&self) -> CC0_R {
        CC0_R::new(((self.bits >> 4) & 1) != 0)
    }
    #[doc = "Bit 5 - CC1 Interrupt Enable"]
    #[inline(always)]
    pub fn cc1(&self) -> CC1_R {
        CC1_R::new(((self.bits >> 5) & 1) != 0)
    }
    #[doc = "Bit 6 - CC2 Interrupt Enable"]
    #[inline(always)]
    pub fn cc2(&self) -> CC2_R {
        CC2_R::new(((self.bits >> 6) & 1) != 0)
    }
    #[doc = "Bit 7 - CC3 Interrupt Enable"]
    #[inline(always)]
    pub fn cc3(&self) -> CC3_R {
        CC3_R::new(((self.bits >> 7) & 1) != 0)
    }
    #[doc = "Bit 8 - ICBOF0 Interrupt Enable"]
    #[inline(always)]
    pub fn icbof0(&self) -> ICBOF0_R {
        ICBOF0_R::new(((self.bits >> 8) & 1) != 0)
    }
    #[doc = "Bit 9 - ICBOF1 Interrupt Enable"]
    #[inline(always)]
    pub fn icbof1(&self) -> ICBOF1_R {
        ICBOF1_R::new(((self.bits >> 9) & 1) != 0)
    }
    #[doc = "Bit 10 - ICBOF2 Interrupt Enable"]
    #[inline(always)]
    pub fn icbof2(&self) -> ICBOF2_R {
        ICBOF2_R::new(((self.bits >> 10) & 1) != 0)
    }
    #[doc = "Bit 11 - ICBOF3 Interrupt Enable"]
    #[inline(always)]
    pub fn icbof3(&self) -> ICBOF3_R {
        ICBOF3_R::new(((self.bits >> 11) & 1) != 0)
    }
}
impl W {
    #[doc = "Bit 0 - OF Interrupt Enable"]
    #[inline(always)]
    pub fn of(&mut self) -> OF_W {
        OF_W::new(self)
    }
    #[doc = "Bit 1 - UF Interrupt Enable"]
    #[inline(always)]
    pub fn uf(&mut self) -> UF_W {
        UF_W::new(self)
    }
    #[doc = "Bit 2 - DIRCHG Interrupt Enable"]
    #[inline(always)]
    pub fn dirchg(&mut self) -> DIRCHG_W {
        DIRCHG_W::new(self)
    }
    #[doc = "Bit 4 - CC0 Interrupt Enable"]
    #[inline(always)]
    pub fn cc0(&mut self) -> CC0_W {
        CC0_W::new(self)
    }
    #[doc = "Bit 5 - CC1 Interrupt Enable"]
    #[inline(always)]
    pub fn cc1(&mut self) -> CC1_W {
        CC1_W::new(self)
    }
    #[doc = "Bit 6 - CC2 Interrupt Enable"]
    #[inline(always)]
    pub fn cc2(&mut self) -> CC2_W {
        CC2_W::new(self)
    }
    #[doc = "Bit 7 - CC3 Interrupt Enable"]
    #[inline(always)]
    pub fn cc3(&mut self) -> CC3_W {
        CC3_W::new(self)
    }
    #[doc = "Bit 8 - ICBOF0 Interrupt Enable"]
    #[inline(always)]
    pub fn icbof0(&mut self) -> ICBOF0_W {
        ICBOF0_W::new(self)
    }
    #[doc = "Bit 9 - ICBOF1 Interrupt Enable"]
    #[inline(always)]
    pub fn icbof1(&mut self) -> ICBOF1_W {
        ICBOF1_W::new(self)
    }
    #[doc = "Bit 10 - ICBOF2 Interrupt Enable"]
    #[inline(always)]
    pub fn icbof2(&mut self) -> ICBOF2_W {
        ICBOF2_W::new(self)
    }
    #[doc = "Bit 11 - ICBOF3 Interrupt Enable"]
    #[inline(always)]
    pub fn icbof3(&mut self) -> ICBOF3_W {
        ICBOF3_W::new(self)
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
