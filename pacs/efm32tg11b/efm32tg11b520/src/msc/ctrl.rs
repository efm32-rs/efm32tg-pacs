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
#[doc = "Field `ADDRFAULTEN` reader - Invalid Address Bus Fault Response Enable"]
pub type ADDRFAULTEN_R = crate::BitReader<bool>;
#[doc = "Field `ADDRFAULTEN` writer - Invalid Address Bus Fault Response Enable"]
pub type ADDRFAULTEN_W<'a> = crate::BitWriter<'a, u32, CTRL_SPEC, bool, 0>;
#[doc = "Field `CLKDISFAULTEN` reader - Clock-disabled Bus Fault Response Enable"]
pub type CLKDISFAULTEN_R = crate::BitReader<bool>;
#[doc = "Field `CLKDISFAULTEN` writer - Clock-disabled Bus Fault Response Enable"]
pub type CLKDISFAULTEN_W<'a> = crate::BitWriter<'a, u32, CTRL_SPEC, bool, 1>;
#[doc = "Field `PWRUPONDEMAND` reader - Power Up on Demand During Wake Up"]
pub type PWRUPONDEMAND_R = crate::BitReader<bool>;
#[doc = "Field `PWRUPONDEMAND` writer - Power Up on Demand During Wake Up"]
pub type PWRUPONDEMAND_W<'a> = crate::BitWriter<'a, u32, CTRL_SPEC, bool, 2>;
#[doc = "Field `IFCREADCLEAR` reader - IFC Read Clears IF"]
pub type IFCREADCLEAR_R = crate::BitReader<bool>;
#[doc = "Field `IFCREADCLEAR` writer - IFC Read Clears IF"]
pub type IFCREADCLEAR_W<'a> = crate::BitWriter<'a, u32, CTRL_SPEC, bool, 3>;
#[doc = "Field `TIMEOUTFAULTEN` reader - Timeout Bus Fault Response Enable"]
pub type TIMEOUTFAULTEN_R = crate::BitReader<bool>;
#[doc = "Field `TIMEOUTFAULTEN` writer - Timeout Bus Fault Response Enable"]
pub type TIMEOUTFAULTEN_W<'a> = crate::BitWriter<'a, u32, CTRL_SPEC, bool, 4>;
impl R {
    #[doc = "Bit 0 - Invalid Address Bus Fault Response Enable"]
    #[inline(always)]
    pub fn addrfaulten(&self) -> ADDRFAULTEN_R {
        ADDRFAULTEN_R::new((self.bits & 1) != 0)
    }
    #[doc = "Bit 1 - Clock-disabled Bus Fault Response Enable"]
    #[inline(always)]
    pub fn clkdisfaulten(&self) -> CLKDISFAULTEN_R {
        CLKDISFAULTEN_R::new(((self.bits >> 1) & 1) != 0)
    }
    #[doc = "Bit 2 - Power Up on Demand During Wake Up"]
    #[inline(always)]
    pub fn pwrupondemand(&self) -> PWRUPONDEMAND_R {
        PWRUPONDEMAND_R::new(((self.bits >> 2) & 1) != 0)
    }
    #[doc = "Bit 3 - IFC Read Clears IF"]
    #[inline(always)]
    pub fn ifcreadclear(&self) -> IFCREADCLEAR_R {
        IFCREADCLEAR_R::new(((self.bits >> 3) & 1) != 0)
    }
    #[doc = "Bit 4 - Timeout Bus Fault Response Enable"]
    #[inline(always)]
    pub fn timeoutfaulten(&self) -> TIMEOUTFAULTEN_R {
        TIMEOUTFAULTEN_R::new(((self.bits >> 4) & 1) != 0)
    }
}
impl W {
    #[doc = "Bit 0 - Invalid Address Bus Fault Response Enable"]
    #[inline(always)]
    pub fn addrfaulten(&mut self) -> ADDRFAULTEN_W {
        ADDRFAULTEN_W::new(self)
    }
    #[doc = "Bit 1 - Clock-disabled Bus Fault Response Enable"]
    #[inline(always)]
    pub fn clkdisfaulten(&mut self) -> CLKDISFAULTEN_W {
        CLKDISFAULTEN_W::new(self)
    }
    #[doc = "Bit 2 - Power Up on Demand During Wake Up"]
    #[inline(always)]
    pub fn pwrupondemand(&mut self) -> PWRUPONDEMAND_W {
        PWRUPONDEMAND_W::new(self)
    }
    #[doc = "Bit 3 - IFC Read Clears IF"]
    #[inline(always)]
    pub fn ifcreadclear(&mut self) -> IFCREADCLEAR_W {
        IFCREADCLEAR_W::new(self)
    }
    #[doc = "Bit 4 - Timeout Bus Fault Response Enable"]
    #[inline(always)]
    pub fn timeoutfaulten(&mut self) -> TIMEOUTFAULTEN_W {
        TIMEOUTFAULTEN_W::new(self)
    }
    #[doc = "Writes raw bits to the register."]
    #[inline(always)]
    pub unsafe fn bits(&mut self, bits: u32) -> &mut Self {
        self.0.bits(bits);
        self
    }
}
#[doc = "Memory System Control Register\n\nThis register you can [`read`](crate::generic::Reg::read), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [ctrl](index.html) module"]
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
