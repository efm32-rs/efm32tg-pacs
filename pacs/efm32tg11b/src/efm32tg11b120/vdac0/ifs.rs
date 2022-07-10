#[doc = "Register `IFS` writer"]
pub struct W(crate::W<IFS_SPEC>);
impl core::ops::Deref for W {
    type Target = crate::W<IFS_SPEC>;
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
impl From<crate::W<IFS_SPEC>> for W {
    #[inline(always)]
    fn from(writer: crate::W<IFS_SPEC>) -> Self {
        W(writer)
    }
}
#[doc = "Field `CH0CD` writer - Set CH0CD Interrupt Flag"]
pub type CH0CD_W<'a> = crate::BitWriter<'a, u32, IFS_SPEC, bool, 0>;
#[doc = "Field `CH1CD` writer - Set CH1CD Interrupt Flag"]
pub type CH1CD_W<'a> = crate::BitWriter<'a, u32, IFS_SPEC, bool, 1>;
#[doc = "Field `CH0OF` writer - Set CH0OF Interrupt Flag"]
pub type CH0OF_W<'a> = crate::BitWriter<'a, u32, IFS_SPEC, bool, 2>;
#[doc = "Field `CH1OF` writer - Set CH1OF Interrupt Flag"]
pub type CH1OF_W<'a> = crate::BitWriter<'a, u32, IFS_SPEC, bool, 3>;
#[doc = "Field `CH0UF` writer - Set CH0UF Interrupt Flag"]
pub type CH0UF_W<'a> = crate::BitWriter<'a, u32, IFS_SPEC, bool, 4>;
#[doc = "Field `CH1UF` writer - Set CH1UF Interrupt Flag"]
pub type CH1UF_W<'a> = crate::BitWriter<'a, u32, IFS_SPEC, bool, 5>;
#[doc = "Field `EM23ERR` writer - Set EM23ERR Interrupt Flag"]
pub type EM23ERR_W<'a> = crate::BitWriter<'a, u32, IFS_SPEC, bool, 15>;
#[doc = "Field `OPA0APORTCONFLICT` writer - Set OPA0APORTCONFLICT Interrupt Flag"]
pub type OPA0APORTCONFLICT_W<'a> = crate::BitWriter<'a, u32, IFS_SPEC, bool, 16>;
#[doc = "Field `OPA1APORTCONFLICT` writer - Set OPA1APORTCONFLICT Interrupt Flag"]
pub type OPA1APORTCONFLICT_W<'a> = crate::BitWriter<'a, u32, IFS_SPEC, bool, 17>;
#[doc = "Field `OPA2APORTCONFLICT` writer - Set OPA2APORTCONFLICT Interrupt Flag"]
pub type OPA2APORTCONFLICT_W<'a> = crate::BitWriter<'a, u32, IFS_SPEC, bool, 18>;
#[doc = "Field `OPA3APORTCONFLICT` writer - Set OPA3APORTCONFLICT Interrupt Flag"]
pub type OPA3APORTCONFLICT_W<'a> = crate::BitWriter<'a, u32, IFS_SPEC, bool, 19>;
#[doc = "Field `OPA0PRSTIMEDERR` writer - Set OPA0PRSTIMEDERR Interrupt Flag"]
pub type OPA0PRSTIMEDERR_W<'a> = crate::BitWriter<'a, u32, IFS_SPEC, bool, 20>;
#[doc = "Field `OPA1PRSTIMEDERR` writer - Set OPA1PRSTIMEDERR Interrupt Flag"]
pub type OPA1PRSTIMEDERR_W<'a> = crate::BitWriter<'a, u32, IFS_SPEC, bool, 21>;
#[doc = "Field `OPA2PRSTIMEDERR` writer - Set OPA2PRSTIMEDERR Interrupt Flag"]
pub type OPA2PRSTIMEDERR_W<'a> = crate::BitWriter<'a, u32, IFS_SPEC, bool, 22>;
#[doc = "Field `OPA3PRSTIMEDERR` writer - Set OPA3PRSTIMEDERR Interrupt Flag"]
pub type OPA3PRSTIMEDERR_W<'a> = crate::BitWriter<'a, u32, IFS_SPEC, bool, 23>;
#[doc = "Field `OPA0OUTVALID` writer - Set OPA0OUTVALID Interrupt Flag"]
pub type OPA0OUTVALID_W<'a> = crate::BitWriter<'a, u32, IFS_SPEC, bool, 28>;
#[doc = "Field `OPA1OUTVALID` writer - Set OPA1OUTVALID Interrupt Flag"]
pub type OPA1OUTVALID_W<'a> = crate::BitWriter<'a, u32, IFS_SPEC, bool, 29>;
#[doc = "Field `OPA2OUTVALID` writer - Set OPA2OUTVALID Interrupt Flag"]
pub type OPA2OUTVALID_W<'a> = crate::BitWriter<'a, u32, IFS_SPEC, bool, 30>;
#[doc = "Field `OPA3OUTVALID` writer - Set OPA3OUTVALID Interrupt Flag"]
pub type OPA3OUTVALID_W<'a> = crate::BitWriter<'a, u32, IFS_SPEC, bool, 31>;
impl W {
    #[doc = "Bit 0 - Set CH0CD Interrupt Flag"]
    #[inline(always)]
    pub fn ch0cd(&mut self) -> CH0CD_W {
        CH0CD_W::new(self)
    }
    #[doc = "Bit 1 - Set CH1CD Interrupt Flag"]
    #[inline(always)]
    pub fn ch1cd(&mut self) -> CH1CD_W {
        CH1CD_W::new(self)
    }
    #[doc = "Bit 2 - Set CH0OF Interrupt Flag"]
    #[inline(always)]
    pub fn ch0of(&mut self) -> CH0OF_W {
        CH0OF_W::new(self)
    }
    #[doc = "Bit 3 - Set CH1OF Interrupt Flag"]
    #[inline(always)]
    pub fn ch1of(&mut self) -> CH1OF_W {
        CH1OF_W::new(self)
    }
    #[doc = "Bit 4 - Set CH0UF Interrupt Flag"]
    #[inline(always)]
    pub fn ch0uf(&mut self) -> CH0UF_W {
        CH0UF_W::new(self)
    }
    #[doc = "Bit 5 - Set CH1UF Interrupt Flag"]
    #[inline(always)]
    pub fn ch1uf(&mut self) -> CH1UF_W {
        CH1UF_W::new(self)
    }
    #[doc = "Bit 15 - Set EM23ERR Interrupt Flag"]
    #[inline(always)]
    pub fn em23err(&mut self) -> EM23ERR_W {
        EM23ERR_W::new(self)
    }
    #[doc = "Bit 16 - Set OPA0APORTCONFLICT Interrupt Flag"]
    #[inline(always)]
    pub fn opa0aportconflict(&mut self) -> OPA0APORTCONFLICT_W {
        OPA0APORTCONFLICT_W::new(self)
    }
    #[doc = "Bit 17 - Set OPA1APORTCONFLICT Interrupt Flag"]
    #[inline(always)]
    pub fn opa1aportconflict(&mut self) -> OPA1APORTCONFLICT_W {
        OPA1APORTCONFLICT_W::new(self)
    }
    #[doc = "Bit 18 - Set OPA2APORTCONFLICT Interrupt Flag"]
    #[inline(always)]
    pub fn opa2aportconflict(&mut self) -> OPA2APORTCONFLICT_W {
        OPA2APORTCONFLICT_W::new(self)
    }
    #[doc = "Bit 19 - Set OPA3APORTCONFLICT Interrupt Flag"]
    #[inline(always)]
    pub fn opa3aportconflict(&mut self) -> OPA3APORTCONFLICT_W {
        OPA3APORTCONFLICT_W::new(self)
    }
    #[doc = "Bit 20 - Set OPA0PRSTIMEDERR Interrupt Flag"]
    #[inline(always)]
    pub fn opa0prstimederr(&mut self) -> OPA0PRSTIMEDERR_W {
        OPA0PRSTIMEDERR_W::new(self)
    }
    #[doc = "Bit 21 - Set OPA1PRSTIMEDERR Interrupt Flag"]
    #[inline(always)]
    pub fn opa1prstimederr(&mut self) -> OPA1PRSTIMEDERR_W {
        OPA1PRSTIMEDERR_W::new(self)
    }
    #[doc = "Bit 22 - Set OPA2PRSTIMEDERR Interrupt Flag"]
    #[inline(always)]
    pub fn opa2prstimederr(&mut self) -> OPA2PRSTIMEDERR_W {
        OPA2PRSTIMEDERR_W::new(self)
    }
    #[doc = "Bit 23 - Set OPA3PRSTIMEDERR Interrupt Flag"]
    #[inline(always)]
    pub fn opa3prstimederr(&mut self) -> OPA3PRSTIMEDERR_W {
        OPA3PRSTIMEDERR_W::new(self)
    }
    #[doc = "Bit 28 - Set OPA0OUTVALID Interrupt Flag"]
    #[inline(always)]
    pub fn opa0outvalid(&mut self) -> OPA0OUTVALID_W {
        OPA0OUTVALID_W::new(self)
    }
    #[doc = "Bit 29 - Set OPA1OUTVALID Interrupt Flag"]
    #[inline(always)]
    pub fn opa1outvalid(&mut self) -> OPA1OUTVALID_W {
        OPA1OUTVALID_W::new(self)
    }
    #[doc = "Bit 30 - Set OPA2OUTVALID Interrupt Flag"]
    #[inline(always)]
    pub fn opa2outvalid(&mut self) -> OPA2OUTVALID_W {
        OPA2OUTVALID_W::new(self)
    }
    #[doc = "Bit 31 - Set OPA3OUTVALID Interrupt Flag"]
    #[inline(always)]
    pub fn opa3outvalid(&mut self) -> OPA3OUTVALID_W {
        OPA3OUTVALID_W::new(self)
    }
    #[doc = "Writes raw bits to the register."]
    #[inline(always)]
    pub unsafe fn bits(&mut self, bits: u32) -> &mut Self {
        self.0.bits(bits);
        self
    }
}
#[doc = "Interrupt Flag Set Register\n\nThis register you can [`write_with_zero`](crate::generic::Reg::write_with_zero), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [ifs](index.html) module"]
pub struct IFS_SPEC;
impl crate::RegisterSpec for IFS_SPEC {
    type Ux = u32;
}
#[doc = "`write(|w| ..)` method takes [ifs::W](W) writer structure"]
impl crate::Writable for IFS_SPEC {
    type Writer = W;
}
#[doc = "`reset()` method sets IFS to value 0"]
impl crate::Resettable for IFS_SPEC {
    #[inline(always)]
    fn reset_value() -> Self::Ux {
        0
    }
}
