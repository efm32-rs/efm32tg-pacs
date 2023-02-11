#[doc = "Register `IFC` writer"]
pub struct W(crate::W<IFC_SPEC>);
impl core::ops::Deref for W {
    type Target = crate::W<IFC_SPEC>;
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
impl From<crate::W<IFC_SPEC>> for W {
    #[inline(always)]
    fn from(writer: crate::W<IFC_SPEC>) -> Self {
        W(writer)
    }
}
#[doc = "Field `CH0CD` writer - Clear CH0CD Interrupt Flag"]
pub type CH0CD_W<'a, const O: u8> = crate::BitWriter<'a, u32, IFC_SPEC, bool, O>;
#[doc = "Field `CH1CD` writer - Clear CH1CD Interrupt Flag"]
pub type CH1CD_W<'a, const O: u8> = crate::BitWriter<'a, u32, IFC_SPEC, bool, O>;
#[doc = "Field `CH0OF` writer - Clear CH0OF Interrupt Flag"]
pub type CH0OF_W<'a, const O: u8> = crate::BitWriter<'a, u32, IFC_SPEC, bool, O>;
#[doc = "Field `CH1OF` writer - Clear CH1OF Interrupt Flag"]
pub type CH1OF_W<'a, const O: u8> = crate::BitWriter<'a, u32, IFC_SPEC, bool, O>;
#[doc = "Field `CH0UF` writer - Clear CH0UF Interrupt Flag"]
pub type CH0UF_W<'a, const O: u8> = crate::BitWriter<'a, u32, IFC_SPEC, bool, O>;
#[doc = "Field `CH1UF` writer - Clear CH1UF Interrupt Flag"]
pub type CH1UF_W<'a, const O: u8> = crate::BitWriter<'a, u32, IFC_SPEC, bool, O>;
#[doc = "Field `EM23ERR` writer - Clear EM23ERR Interrupt Flag"]
pub type EM23ERR_W<'a, const O: u8> = crate::BitWriter<'a, u32, IFC_SPEC, bool, O>;
#[doc = "Field `OPA0APORTCONFLICT` writer - Clear OPA0APORTCONFLICT Interrupt Flag"]
pub type OPA0APORTCONFLICT_W<'a, const O: u8> = crate::BitWriter<'a, u32, IFC_SPEC, bool, O>;
#[doc = "Field `OPA1APORTCONFLICT` writer - Clear OPA1APORTCONFLICT Interrupt Flag"]
pub type OPA1APORTCONFLICT_W<'a, const O: u8> = crate::BitWriter<'a, u32, IFC_SPEC, bool, O>;
#[doc = "Field `OPA2APORTCONFLICT` writer - Clear OPA2APORTCONFLICT Interrupt Flag"]
pub type OPA2APORTCONFLICT_W<'a, const O: u8> = crate::BitWriter<'a, u32, IFC_SPEC, bool, O>;
#[doc = "Field `OPA3APORTCONFLICT` writer - Clear OPA3APORTCONFLICT Interrupt Flag"]
pub type OPA3APORTCONFLICT_W<'a, const O: u8> = crate::BitWriter<'a, u32, IFC_SPEC, bool, O>;
#[doc = "Field `OPA0PRSTIMEDERR` writer - Clear OPA0PRSTIMEDERR Interrupt Flag"]
pub type OPA0PRSTIMEDERR_W<'a, const O: u8> = crate::BitWriter<'a, u32, IFC_SPEC, bool, O>;
#[doc = "Field `OPA1PRSTIMEDERR` writer - Clear OPA1PRSTIMEDERR Interrupt Flag"]
pub type OPA1PRSTIMEDERR_W<'a, const O: u8> = crate::BitWriter<'a, u32, IFC_SPEC, bool, O>;
#[doc = "Field `OPA2PRSTIMEDERR` writer - Clear OPA2PRSTIMEDERR Interrupt Flag"]
pub type OPA2PRSTIMEDERR_W<'a, const O: u8> = crate::BitWriter<'a, u32, IFC_SPEC, bool, O>;
#[doc = "Field `OPA3PRSTIMEDERR` writer - Clear OPA3PRSTIMEDERR Interrupt Flag"]
pub type OPA3PRSTIMEDERR_W<'a, const O: u8> = crate::BitWriter<'a, u32, IFC_SPEC, bool, O>;
#[doc = "Field `OPA0OUTVALID` writer - Clear OPA0OUTVALID Interrupt Flag"]
pub type OPA0OUTVALID_W<'a, const O: u8> = crate::BitWriter<'a, u32, IFC_SPEC, bool, O>;
#[doc = "Field `OPA1OUTVALID` writer - Clear OPA1OUTVALID Interrupt Flag"]
pub type OPA1OUTVALID_W<'a, const O: u8> = crate::BitWriter<'a, u32, IFC_SPEC, bool, O>;
#[doc = "Field `OPA2OUTVALID` writer - Clear OPA2OUTVALID Interrupt Flag"]
pub type OPA2OUTVALID_W<'a, const O: u8> = crate::BitWriter<'a, u32, IFC_SPEC, bool, O>;
#[doc = "Field `OPA3OUTVALID` writer - Clear OPA3OUTVALID Interrupt Flag"]
pub type OPA3OUTVALID_W<'a, const O: u8> = crate::BitWriter<'a, u32, IFC_SPEC, bool, O>;
impl W {
    #[doc = "Bit 0 - Clear CH0CD Interrupt Flag"]
    #[inline(always)]
    #[must_use]
    pub fn ch0cd(&mut self) -> CH0CD_W<0> {
        CH0CD_W::new(self)
    }
    #[doc = "Bit 1 - Clear CH1CD Interrupt Flag"]
    #[inline(always)]
    #[must_use]
    pub fn ch1cd(&mut self) -> CH1CD_W<1> {
        CH1CD_W::new(self)
    }
    #[doc = "Bit 2 - Clear CH0OF Interrupt Flag"]
    #[inline(always)]
    #[must_use]
    pub fn ch0of(&mut self) -> CH0OF_W<2> {
        CH0OF_W::new(self)
    }
    #[doc = "Bit 3 - Clear CH1OF Interrupt Flag"]
    #[inline(always)]
    #[must_use]
    pub fn ch1of(&mut self) -> CH1OF_W<3> {
        CH1OF_W::new(self)
    }
    #[doc = "Bit 4 - Clear CH0UF Interrupt Flag"]
    #[inline(always)]
    #[must_use]
    pub fn ch0uf(&mut self) -> CH0UF_W<4> {
        CH0UF_W::new(self)
    }
    #[doc = "Bit 5 - Clear CH1UF Interrupt Flag"]
    #[inline(always)]
    #[must_use]
    pub fn ch1uf(&mut self) -> CH1UF_W<5> {
        CH1UF_W::new(self)
    }
    #[doc = "Bit 15 - Clear EM23ERR Interrupt Flag"]
    #[inline(always)]
    #[must_use]
    pub fn em23err(&mut self) -> EM23ERR_W<15> {
        EM23ERR_W::new(self)
    }
    #[doc = "Bit 16 - Clear OPA0APORTCONFLICT Interrupt Flag"]
    #[inline(always)]
    #[must_use]
    pub fn opa0aportconflict(&mut self) -> OPA0APORTCONFLICT_W<16> {
        OPA0APORTCONFLICT_W::new(self)
    }
    #[doc = "Bit 17 - Clear OPA1APORTCONFLICT Interrupt Flag"]
    #[inline(always)]
    #[must_use]
    pub fn opa1aportconflict(&mut self) -> OPA1APORTCONFLICT_W<17> {
        OPA1APORTCONFLICT_W::new(self)
    }
    #[doc = "Bit 18 - Clear OPA2APORTCONFLICT Interrupt Flag"]
    #[inline(always)]
    #[must_use]
    pub fn opa2aportconflict(&mut self) -> OPA2APORTCONFLICT_W<18> {
        OPA2APORTCONFLICT_W::new(self)
    }
    #[doc = "Bit 19 - Clear OPA3APORTCONFLICT Interrupt Flag"]
    #[inline(always)]
    #[must_use]
    pub fn opa3aportconflict(&mut self) -> OPA3APORTCONFLICT_W<19> {
        OPA3APORTCONFLICT_W::new(self)
    }
    #[doc = "Bit 20 - Clear OPA0PRSTIMEDERR Interrupt Flag"]
    #[inline(always)]
    #[must_use]
    pub fn opa0prstimederr(&mut self) -> OPA0PRSTIMEDERR_W<20> {
        OPA0PRSTIMEDERR_W::new(self)
    }
    #[doc = "Bit 21 - Clear OPA1PRSTIMEDERR Interrupt Flag"]
    #[inline(always)]
    #[must_use]
    pub fn opa1prstimederr(&mut self) -> OPA1PRSTIMEDERR_W<21> {
        OPA1PRSTIMEDERR_W::new(self)
    }
    #[doc = "Bit 22 - Clear OPA2PRSTIMEDERR Interrupt Flag"]
    #[inline(always)]
    #[must_use]
    pub fn opa2prstimederr(&mut self) -> OPA2PRSTIMEDERR_W<22> {
        OPA2PRSTIMEDERR_W::new(self)
    }
    #[doc = "Bit 23 - Clear OPA3PRSTIMEDERR Interrupt Flag"]
    #[inline(always)]
    #[must_use]
    pub fn opa3prstimederr(&mut self) -> OPA3PRSTIMEDERR_W<23> {
        OPA3PRSTIMEDERR_W::new(self)
    }
    #[doc = "Bit 28 - Clear OPA0OUTVALID Interrupt Flag"]
    #[inline(always)]
    #[must_use]
    pub fn opa0outvalid(&mut self) -> OPA0OUTVALID_W<28> {
        OPA0OUTVALID_W::new(self)
    }
    #[doc = "Bit 29 - Clear OPA1OUTVALID Interrupt Flag"]
    #[inline(always)]
    #[must_use]
    pub fn opa1outvalid(&mut self) -> OPA1OUTVALID_W<29> {
        OPA1OUTVALID_W::new(self)
    }
    #[doc = "Bit 30 - Clear OPA2OUTVALID Interrupt Flag"]
    #[inline(always)]
    #[must_use]
    pub fn opa2outvalid(&mut self) -> OPA2OUTVALID_W<30> {
        OPA2OUTVALID_W::new(self)
    }
    #[doc = "Bit 31 - Clear OPA3OUTVALID Interrupt Flag"]
    #[inline(always)]
    #[must_use]
    pub fn opa3outvalid(&mut self) -> OPA3OUTVALID_W<31> {
        OPA3OUTVALID_W::new(self)
    }
    #[doc = "Writes raw bits to the register."]
    #[inline(always)]
    pub unsafe fn bits(&mut self, bits: u32) -> &mut Self {
        self.0.bits(bits);
        self
    }
}
#[doc = "Interrupt Flag Clear Register\n\nThis register you can [`write_with_zero`](crate::generic::Reg::write_with_zero), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [ifc](index.html) module"]
pub struct IFC_SPEC;
impl crate::RegisterSpec for IFC_SPEC {
    type Ux = u32;
}
#[doc = "`write(|w| ..)` method takes [ifc::W](W) writer structure"]
impl crate::Writable for IFC_SPEC {
    type Writer = W;
    const ZERO_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
}
#[doc = "`reset()` method sets IFC to value 0"]
impl crate::Resettable for IFC_SPEC {
    const RESET_VALUE: Self::Ux = 0;
}
