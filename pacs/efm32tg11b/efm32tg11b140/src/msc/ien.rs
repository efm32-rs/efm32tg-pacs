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
#[doc = "Field `ERASE` reader - ERASE Interrupt Enable"]
pub type ERASE_R = crate::BitReader<bool>;
#[doc = "Field `ERASE` writer - ERASE Interrupt Enable"]
pub type ERASE_W<'a> = crate::BitWriter<'a, u32, IEN_SPEC, bool, 0>;
#[doc = "Field `WRITE` reader - WRITE Interrupt Enable"]
pub type WRITE_R = crate::BitReader<bool>;
#[doc = "Field `WRITE` writer - WRITE Interrupt Enable"]
pub type WRITE_W<'a> = crate::BitWriter<'a, u32, IEN_SPEC, bool, 1>;
#[doc = "Field `CHOF` reader - CHOF Interrupt Enable"]
pub type CHOF_R = crate::BitReader<bool>;
#[doc = "Field `CHOF` writer - CHOF Interrupt Enable"]
pub type CHOF_W<'a> = crate::BitWriter<'a, u32, IEN_SPEC, bool, 2>;
#[doc = "Field `CMOF` reader - CMOF Interrupt Enable"]
pub type CMOF_R = crate::BitReader<bool>;
#[doc = "Field `CMOF` writer - CMOF Interrupt Enable"]
pub type CMOF_W<'a> = crate::BitWriter<'a, u32, IEN_SPEC, bool, 3>;
#[doc = "Field `PWRUPF` reader - PWRUPF Interrupt Enable"]
pub type PWRUPF_R = crate::BitReader<bool>;
#[doc = "Field `PWRUPF` writer - PWRUPF Interrupt Enable"]
pub type PWRUPF_W<'a> = crate::BitWriter<'a, u32, IEN_SPEC, bool, 4>;
#[doc = "Field `ICACHERR` reader - ICACHERR Interrupt Enable"]
pub type ICACHERR_R = crate::BitReader<bool>;
#[doc = "Field `ICACHERR` writer - ICACHERR Interrupt Enable"]
pub type ICACHERR_W<'a> = crate::BitWriter<'a, u32, IEN_SPEC, bool, 5>;
#[doc = "Field `WDATAOV` reader - WDATAOV Interrupt Enable"]
pub type WDATAOV_R = crate::BitReader<bool>;
#[doc = "Field `WDATAOV` writer - WDATAOV Interrupt Enable"]
pub type WDATAOV_W<'a> = crate::BitWriter<'a, u32, IEN_SPEC, bool, 6>;
#[doc = "Field `LVEWRITE` reader - LVEWRITE Interrupt Enable"]
pub type LVEWRITE_R = crate::BitReader<bool>;
#[doc = "Field `LVEWRITE` writer - LVEWRITE Interrupt Enable"]
pub type LVEWRITE_W<'a> = crate::BitWriter<'a, u32, IEN_SPEC, bool, 8>;
impl R {
    #[doc = "Bit 0 - ERASE Interrupt Enable"]
    #[inline(always)]
    pub fn erase(&self) -> ERASE_R {
        ERASE_R::new((self.bits & 1) != 0)
    }
    #[doc = "Bit 1 - WRITE Interrupt Enable"]
    #[inline(always)]
    pub fn write(&self) -> WRITE_R {
        WRITE_R::new(((self.bits >> 1) & 1) != 0)
    }
    #[doc = "Bit 2 - CHOF Interrupt Enable"]
    #[inline(always)]
    pub fn chof(&self) -> CHOF_R {
        CHOF_R::new(((self.bits >> 2) & 1) != 0)
    }
    #[doc = "Bit 3 - CMOF Interrupt Enable"]
    #[inline(always)]
    pub fn cmof(&self) -> CMOF_R {
        CMOF_R::new(((self.bits >> 3) & 1) != 0)
    }
    #[doc = "Bit 4 - PWRUPF Interrupt Enable"]
    #[inline(always)]
    pub fn pwrupf(&self) -> PWRUPF_R {
        PWRUPF_R::new(((self.bits >> 4) & 1) != 0)
    }
    #[doc = "Bit 5 - ICACHERR Interrupt Enable"]
    #[inline(always)]
    pub fn icacherr(&self) -> ICACHERR_R {
        ICACHERR_R::new(((self.bits >> 5) & 1) != 0)
    }
    #[doc = "Bit 6 - WDATAOV Interrupt Enable"]
    #[inline(always)]
    pub fn wdataov(&self) -> WDATAOV_R {
        WDATAOV_R::new(((self.bits >> 6) & 1) != 0)
    }
    #[doc = "Bit 8 - LVEWRITE Interrupt Enable"]
    #[inline(always)]
    pub fn lvewrite(&self) -> LVEWRITE_R {
        LVEWRITE_R::new(((self.bits >> 8) & 1) != 0)
    }
}
impl W {
    #[doc = "Bit 0 - ERASE Interrupt Enable"]
    #[inline(always)]
    pub fn erase(&mut self) -> ERASE_W {
        ERASE_W::new(self)
    }
    #[doc = "Bit 1 - WRITE Interrupt Enable"]
    #[inline(always)]
    pub fn write(&mut self) -> WRITE_W {
        WRITE_W::new(self)
    }
    #[doc = "Bit 2 - CHOF Interrupt Enable"]
    #[inline(always)]
    pub fn chof(&mut self) -> CHOF_W {
        CHOF_W::new(self)
    }
    #[doc = "Bit 3 - CMOF Interrupt Enable"]
    #[inline(always)]
    pub fn cmof(&mut self) -> CMOF_W {
        CMOF_W::new(self)
    }
    #[doc = "Bit 4 - PWRUPF Interrupt Enable"]
    #[inline(always)]
    pub fn pwrupf(&mut self) -> PWRUPF_W {
        PWRUPF_W::new(self)
    }
    #[doc = "Bit 5 - ICACHERR Interrupt Enable"]
    #[inline(always)]
    pub fn icacherr(&mut self) -> ICACHERR_W {
        ICACHERR_W::new(self)
    }
    #[doc = "Bit 6 - WDATAOV Interrupt Enable"]
    #[inline(always)]
    pub fn wdataov(&mut self) -> WDATAOV_W {
        WDATAOV_W::new(self)
    }
    #[doc = "Bit 8 - LVEWRITE Interrupt Enable"]
    #[inline(always)]
    pub fn lvewrite(&mut self) -> LVEWRITE_W {
        LVEWRITE_W::new(self)
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
