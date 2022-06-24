#[doc = "Register `CTRL` reader"]
pub struct R(crate::R<CTRL_SPEC>);
impl core::ops::Deref for R {
    type Target = crate::R<CTRL_SPEC>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
impl From<crate::R<CTRL_SPEC>> for R {
    #[inline(always)]
    fn from(reader: crate::R<CTRL_SPEC>) -> Self {
        R(reader)
    }
}
#[doc = "Register `CTRL` writer"]
pub struct W(crate::W<CTRL_SPEC>);
impl core::ops::Deref for W {
    type Target = crate::W<CTRL_SPEC>;
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
impl From<crate::W<CTRL_SPEC>> for W {
    #[inline(always)]
    fn from(writer: crate::W<CTRL_SPEC>) -> Self {
        W(writer)
    }
}
#[doc = "Field `INIT` reader - Initialize"]
pub type INIT_R = crate::BitReader<bool>;
#[doc = "Field `INIT` writer - Initialize"]
pub type INIT_W<'a> = crate::BitWriter<'a, u32, CTRL_SPEC, bool, 0>;
#[doc = "Field `IE` reader - Module Interrupt Enable"]
pub type IE_R = crate::BitReader<bool>;
#[doc = "Field `IE` writer - Module Interrupt Enable"]
pub type IE_W<'a> = crate::BitWriter<'a, u32, CTRL_SPEC, bool, 1>;
#[doc = "Field `SIE` reader - Status Change Interrupt Enable"]
pub type SIE_R = crate::BitReader<bool>;
#[doc = "Field `SIE` writer - Status Change Interrupt Enable"]
pub type SIE_W<'a> = crate::BitWriter<'a, u32, CTRL_SPEC, bool, 2>;
#[doc = "Field `EIE` reader - Error Interrupt Enable"]
pub type EIE_R = crate::BitReader<bool>;
#[doc = "Field `EIE` writer - Error Interrupt Enable"]
pub type EIE_W<'a> = crate::BitWriter<'a, u32, CTRL_SPEC, bool, 3>;
#[doc = "Field `DAR` reader - Disable Automatic Retransmission"]
pub type DAR_R = crate::BitReader<bool>;
#[doc = "Field `DAR` writer - Disable Automatic Retransmission"]
pub type DAR_W<'a> = crate::BitWriter<'a, u32, CTRL_SPEC, bool, 5>;
#[doc = "Field `CCE` reader - Configuration Change Enable"]
pub type CCE_R = crate::BitReader<bool>;
#[doc = "Field `CCE` writer - Configuration Change Enable"]
pub type CCE_W<'a> = crate::BitWriter<'a, u32, CTRL_SPEC, bool, 6>;
#[doc = "Field `TEST` reader - Test Mode Enable Write"]
pub type TEST_R = crate::BitReader<bool>;
#[doc = "Field `TEST` writer - Test Mode Enable Write"]
pub type TEST_W<'a> = crate::BitWriter<'a, u32, CTRL_SPEC, bool, 7>;
impl R {
    #[doc = "Bit 0 - Initialize"]
    #[inline(always)]
    pub fn init(&self) -> INIT_R {
        INIT_R::new((self.bits & 1) != 0)
    }
    #[doc = "Bit 1 - Module Interrupt Enable"]
    #[inline(always)]
    pub fn ie(&self) -> IE_R {
        IE_R::new(((self.bits >> 1) & 1) != 0)
    }
    #[doc = "Bit 2 - Status Change Interrupt Enable"]
    #[inline(always)]
    pub fn sie(&self) -> SIE_R {
        SIE_R::new(((self.bits >> 2) & 1) != 0)
    }
    #[doc = "Bit 3 - Error Interrupt Enable"]
    #[inline(always)]
    pub fn eie(&self) -> EIE_R {
        EIE_R::new(((self.bits >> 3) & 1) != 0)
    }
    #[doc = "Bit 5 - Disable Automatic Retransmission"]
    #[inline(always)]
    pub fn dar(&self) -> DAR_R {
        DAR_R::new(((self.bits >> 5) & 1) != 0)
    }
    #[doc = "Bit 6 - Configuration Change Enable"]
    #[inline(always)]
    pub fn cce(&self) -> CCE_R {
        CCE_R::new(((self.bits >> 6) & 1) != 0)
    }
    #[doc = "Bit 7 - Test Mode Enable Write"]
    #[inline(always)]
    pub fn test(&self) -> TEST_R {
        TEST_R::new(((self.bits >> 7) & 1) != 0)
    }
}
impl W {
    #[doc = "Bit 0 - Initialize"]
    #[inline(always)]
    pub fn init(&mut self) -> INIT_W {
        INIT_W::new(self)
    }
    #[doc = "Bit 1 - Module Interrupt Enable"]
    #[inline(always)]
    pub fn ie(&mut self) -> IE_W {
        IE_W::new(self)
    }
    #[doc = "Bit 2 - Status Change Interrupt Enable"]
    #[inline(always)]
    pub fn sie(&mut self) -> SIE_W {
        SIE_W::new(self)
    }
    #[doc = "Bit 3 - Error Interrupt Enable"]
    #[inline(always)]
    pub fn eie(&mut self) -> EIE_W {
        EIE_W::new(self)
    }
    #[doc = "Bit 5 - Disable Automatic Retransmission"]
    #[inline(always)]
    pub fn dar(&mut self) -> DAR_W {
        DAR_W::new(self)
    }
    #[doc = "Bit 6 - Configuration Change Enable"]
    #[inline(always)]
    pub fn cce(&mut self) -> CCE_W {
        CCE_W::new(self)
    }
    #[doc = "Bit 7 - Test Mode Enable Write"]
    #[inline(always)]
    pub fn test(&mut self) -> TEST_W {
        TEST_W::new(self)
    }
    #[doc = "Writes raw bits to the register."]
    #[inline(always)]
    pub unsafe fn bits(&mut self, bits: u32) -> &mut Self {
        self.0.bits(bits);
        self
    }
}
#[doc = "Control Register\n\nThis register you can [`read`](crate::generic::Reg::read), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [ctrl](index.html) module"]
pub struct CTRL_SPEC;
impl crate::RegisterSpec for CTRL_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [ctrl::R](R) reader structure"]
impl crate::Readable for CTRL_SPEC {
    type Reader = R;
}
#[doc = "`write(|w| ..)` method takes [ctrl::W](W) writer structure"]
impl crate::Writable for CTRL_SPEC {
    type Writer = W;
}
#[doc = "`reset()` method sets CTRL to value 0x01"]
impl crate::Resettable for CTRL_SPEC {
    #[inline(always)]
    fn reset_value() -> Self::Ux {
        0x01
    }
}
