#[doc = "Register `APORTMASTERDIS` reader"]
pub struct R(crate::R<APORTMASTERDIS_SPEC>);
impl core::ops::Deref for R {
    type Target = crate::R<APORTMASTERDIS_SPEC>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
impl From<crate::R<APORTMASTERDIS_SPEC>> for R {
    #[inline(always)]
    fn from(reader: crate::R<APORTMASTERDIS_SPEC>) -> Self {
        R(reader)
    }
}
#[doc = "Register `APORTMASTERDIS` writer"]
pub struct W(crate::W<APORTMASTERDIS_SPEC>);
impl core::ops::Deref for W {
    type Target = crate::W<APORTMASTERDIS_SPEC>;
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
impl From<crate::W<APORTMASTERDIS_SPEC>> for W {
    #[inline(always)]
    fn from(writer: crate::W<APORTMASTERDIS_SPEC>) -> Self {
        W(writer)
    }
}
#[doc = "Field `APORT1XMASTERDIS` reader - APORT1X Master Disable"]
pub type APORT1XMASTERDIS_R = crate::BitReader<bool>;
#[doc = "Field `APORT1XMASTERDIS` writer - APORT1X Master Disable"]
pub type APORT1XMASTERDIS_W<'a, const O: u8> =
    crate::BitWriter<'a, u32, APORTMASTERDIS_SPEC, bool, O>;
#[doc = "Field `APORT1YMASTERDIS` reader - APORT1Y Master Disable"]
pub type APORT1YMASTERDIS_R = crate::BitReader<bool>;
#[doc = "Field `APORT1YMASTERDIS` writer - APORT1Y Master Disable"]
pub type APORT1YMASTERDIS_W<'a, const O: u8> =
    crate::BitWriter<'a, u32, APORTMASTERDIS_SPEC, bool, O>;
#[doc = "Field `APORT2XMASTERDIS` reader - APORT2X Master Disable"]
pub type APORT2XMASTERDIS_R = crate::BitReader<bool>;
#[doc = "Field `APORT2XMASTERDIS` writer - APORT2X Master Disable"]
pub type APORT2XMASTERDIS_W<'a, const O: u8> =
    crate::BitWriter<'a, u32, APORTMASTERDIS_SPEC, bool, O>;
#[doc = "Field `APORT2YMASTERDIS` reader - APORT2Y Master Disable"]
pub type APORT2YMASTERDIS_R = crate::BitReader<bool>;
#[doc = "Field `APORT2YMASTERDIS` writer - APORT2Y Master Disable"]
pub type APORT2YMASTERDIS_W<'a, const O: u8> =
    crate::BitWriter<'a, u32, APORTMASTERDIS_SPEC, bool, O>;
#[doc = "Field `APORT3XMASTERDIS` reader - APORT3X Master Disable"]
pub type APORT3XMASTERDIS_R = crate::BitReader<bool>;
#[doc = "Field `APORT3XMASTERDIS` writer - APORT3X Master Disable"]
pub type APORT3XMASTERDIS_W<'a, const O: u8> =
    crate::BitWriter<'a, u32, APORTMASTERDIS_SPEC, bool, O>;
#[doc = "Field `APORT3YMASTERDIS` reader - APORT3Y Master Disable"]
pub type APORT3YMASTERDIS_R = crate::BitReader<bool>;
#[doc = "Field `APORT3YMASTERDIS` writer - APORT3Y Master Disable"]
pub type APORT3YMASTERDIS_W<'a, const O: u8> =
    crate::BitWriter<'a, u32, APORTMASTERDIS_SPEC, bool, O>;
#[doc = "Field `APORT4XMASTERDIS` reader - APORT4X Master Disable"]
pub type APORT4XMASTERDIS_R = crate::BitReader<bool>;
#[doc = "Field `APORT4XMASTERDIS` writer - APORT4X Master Disable"]
pub type APORT4XMASTERDIS_W<'a, const O: u8> =
    crate::BitWriter<'a, u32, APORTMASTERDIS_SPEC, bool, O>;
#[doc = "Field `APORT4YMASTERDIS` reader - APORT4Y Master Disable"]
pub type APORT4YMASTERDIS_R = crate::BitReader<bool>;
#[doc = "Field `APORT4YMASTERDIS` writer - APORT4Y Master Disable"]
pub type APORT4YMASTERDIS_W<'a, const O: u8> =
    crate::BitWriter<'a, u32, APORTMASTERDIS_SPEC, bool, O>;
impl R {
    #[doc = "Bit 2 - APORT1X Master Disable"]
    #[inline(always)]
    pub fn aport1xmasterdis(&self) -> APORT1XMASTERDIS_R {
        APORT1XMASTERDIS_R::new(((self.bits >> 2) & 1) != 0)
    }
    #[doc = "Bit 3 - APORT1Y Master Disable"]
    #[inline(always)]
    pub fn aport1ymasterdis(&self) -> APORT1YMASTERDIS_R {
        APORT1YMASTERDIS_R::new(((self.bits >> 3) & 1) != 0)
    }
    #[doc = "Bit 4 - APORT2X Master Disable"]
    #[inline(always)]
    pub fn aport2xmasterdis(&self) -> APORT2XMASTERDIS_R {
        APORT2XMASTERDIS_R::new(((self.bits >> 4) & 1) != 0)
    }
    #[doc = "Bit 5 - APORT2Y Master Disable"]
    #[inline(always)]
    pub fn aport2ymasterdis(&self) -> APORT2YMASTERDIS_R {
        APORT2YMASTERDIS_R::new(((self.bits >> 5) & 1) != 0)
    }
    #[doc = "Bit 6 - APORT3X Master Disable"]
    #[inline(always)]
    pub fn aport3xmasterdis(&self) -> APORT3XMASTERDIS_R {
        APORT3XMASTERDIS_R::new(((self.bits >> 6) & 1) != 0)
    }
    #[doc = "Bit 7 - APORT3Y Master Disable"]
    #[inline(always)]
    pub fn aport3ymasterdis(&self) -> APORT3YMASTERDIS_R {
        APORT3YMASTERDIS_R::new(((self.bits >> 7) & 1) != 0)
    }
    #[doc = "Bit 8 - APORT4X Master Disable"]
    #[inline(always)]
    pub fn aport4xmasterdis(&self) -> APORT4XMASTERDIS_R {
        APORT4XMASTERDIS_R::new(((self.bits >> 8) & 1) != 0)
    }
    #[doc = "Bit 9 - APORT4Y Master Disable"]
    #[inline(always)]
    pub fn aport4ymasterdis(&self) -> APORT4YMASTERDIS_R {
        APORT4YMASTERDIS_R::new(((self.bits >> 9) & 1) != 0)
    }
}
impl W {
    #[doc = "Bit 2 - APORT1X Master Disable"]
    #[inline(always)]
    #[must_use]
    pub fn aport1xmasterdis(&mut self) -> APORT1XMASTERDIS_W<2> {
        APORT1XMASTERDIS_W::new(self)
    }
    #[doc = "Bit 3 - APORT1Y Master Disable"]
    #[inline(always)]
    #[must_use]
    pub fn aport1ymasterdis(&mut self) -> APORT1YMASTERDIS_W<3> {
        APORT1YMASTERDIS_W::new(self)
    }
    #[doc = "Bit 4 - APORT2X Master Disable"]
    #[inline(always)]
    #[must_use]
    pub fn aport2xmasterdis(&mut self) -> APORT2XMASTERDIS_W<4> {
        APORT2XMASTERDIS_W::new(self)
    }
    #[doc = "Bit 5 - APORT2Y Master Disable"]
    #[inline(always)]
    #[must_use]
    pub fn aport2ymasterdis(&mut self) -> APORT2YMASTERDIS_W<5> {
        APORT2YMASTERDIS_W::new(self)
    }
    #[doc = "Bit 6 - APORT3X Master Disable"]
    #[inline(always)]
    #[must_use]
    pub fn aport3xmasterdis(&mut self) -> APORT3XMASTERDIS_W<6> {
        APORT3XMASTERDIS_W::new(self)
    }
    #[doc = "Bit 7 - APORT3Y Master Disable"]
    #[inline(always)]
    #[must_use]
    pub fn aport3ymasterdis(&mut self) -> APORT3YMASTERDIS_W<7> {
        APORT3YMASTERDIS_W::new(self)
    }
    #[doc = "Bit 8 - APORT4X Master Disable"]
    #[inline(always)]
    #[must_use]
    pub fn aport4xmasterdis(&mut self) -> APORT4XMASTERDIS_W<8> {
        APORT4XMASTERDIS_W::new(self)
    }
    #[doc = "Bit 9 - APORT4Y Master Disable"]
    #[inline(always)]
    #[must_use]
    pub fn aport4ymasterdis(&mut self) -> APORT4YMASTERDIS_W<9> {
        APORT4YMASTERDIS_W::new(self)
    }
    #[doc = "Writes raw bits to the register."]
    #[inline(always)]
    pub unsafe fn bits(&mut self, bits: u32) -> &mut Self {
        self.0.bits(bits);
        self
    }
}
#[doc = "APORT Bus Master Disable Register\n\nThis register you can [`read`](crate::generic::Reg::read), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [aportmasterdis](index.html) module"]
pub struct APORTMASTERDIS_SPEC;
impl crate::RegisterSpec for APORTMASTERDIS_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [aportmasterdis::R](R) reader structure"]
impl crate::Readable for APORTMASTERDIS_SPEC {
    type Reader = R;
}
#[doc = "`write(|w| ..)` method takes [aportmasterdis::W](W) writer structure"]
impl crate::Writable for APORTMASTERDIS_SPEC {
    type Writer = W;
    const ZERO_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
}
#[doc = "`reset()` method sets APORTMASTERDIS to value 0"]
impl crate::Resettable for APORTMASTERDIS_SPEC {
    const RESET_VALUE: Self::Ux = 0;
}
