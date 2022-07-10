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
#[doc = "Field `TXC` writer - Clear TXC Interrupt Flag"]
pub type TXC_W<'a> = crate::BitWriter<'a, u32, IFC_SPEC, bool, 0>;
#[doc = "Field `RXFULL` writer - Clear RXFULL Interrupt Flag"]
pub type RXFULL_W<'a> = crate::BitWriter<'a, u32, IFC_SPEC, bool, 3>;
#[doc = "Field `RXOF` writer - Clear RXOF Interrupt Flag"]
pub type RXOF_W<'a> = crate::BitWriter<'a, u32, IFC_SPEC, bool, 4>;
#[doc = "Field `RXUF` writer - Clear RXUF Interrupt Flag"]
pub type RXUF_W<'a> = crate::BitWriter<'a, u32, IFC_SPEC, bool, 5>;
#[doc = "Field `TXOF` writer - Clear TXOF Interrupt Flag"]
pub type TXOF_W<'a> = crate::BitWriter<'a, u32, IFC_SPEC, bool, 6>;
#[doc = "Field `TXUF` writer - Clear TXUF Interrupt Flag"]
pub type TXUF_W<'a> = crate::BitWriter<'a, u32, IFC_SPEC, bool, 7>;
#[doc = "Field `PERR` writer - Clear PERR Interrupt Flag"]
pub type PERR_W<'a> = crate::BitWriter<'a, u32, IFC_SPEC, bool, 8>;
#[doc = "Field `FERR` writer - Clear FERR Interrupt Flag"]
pub type FERR_W<'a> = crate::BitWriter<'a, u32, IFC_SPEC, bool, 9>;
#[doc = "Field `MPAF` writer - Clear MPAF Interrupt Flag"]
pub type MPAF_W<'a> = crate::BitWriter<'a, u32, IFC_SPEC, bool, 10>;
#[doc = "Field `SSM` writer - Clear SSM Interrupt Flag"]
pub type SSM_W<'a> = crate::BitWriter<'a, u32, IFC_SPEC, bool, 11>;
#[doc = "Field `CCF` writer - Clear CCF Interrupt Flag"]
pub type CCF_W<'a> = crate::BitWriter<'a, u32, IFC_SPEC, bool, 12>;
#[doc = "Field `TXIDLE` writer - Clear TXIDLE Interrupt Flag"]
pub type TXIDLE_W<'a> = crate::BitWriter<'a, u32, IFC_SPEC, bool, 13>;
#[doc = "Field `TCMP0` writer - Clear TCMP0 Interrupt Flag"]
pub type TCMP0_W<'a> = crate::BitWriter<'a, u32, IFC_SPEC, bool, 14>;
#[doc = "Field `TCMP1` writer - Clear TCMP1 Interrupt Flag"]
pub type TCMP1_W<'a> = crate::BitWriter<'a, u32, IFC_SPEC, bool, 15>;
#[doc = "Field `TCMP2` writer - Clear TCMP2 Interrupt Flag"]
pub type TCMP2_W<'a> = crate::BitWriter<'a, u32, IFC_SPEC, bool, 16>;
impl W {
    #[doc = "Bit 0 - Clear TXC Interrupt Flag"]
    #[inline(always)]
    pub fn txc(&mut self) -> TXC_W {
        TXC_W::new(self)
    }
    #[doc = "Bit 3 - Clear RXFULL Interrupt Flag"]
    #[inline(always)]
    pub fn rxfull(&mut self) -> RXFULL_W {
        RXFULL_W::new(self)
    }
    #[doc = "Bit 4 - Clear RXOF Interrupt Flag"]
    #[inline(always)]
    pub fn rxof(&mut self) -> RXOF_W {
        RXOF_W::new(self)
    }
    #[doc = "Bit 5 - Clear RXUF Interrupt Flag"]
    #[inline(always)]
    pub fn rxuf(&mut self) -> RXUF_W {
        RXUF_W::new(self)
    }
    #[doc = "Bit 6 - Clear TXOF Interrupt Flag"]
    #[inline(always)]
    pub fn txof(&mut self) -> TXOF_W {
        TXOF_W::new(self)
    }
    #[doc = "Bit 7 - Clear TXUF Interrupt Flag"]
    #[inline(always)]
    pub fn txuf(&mut self) -> TXUF_W {
        TXUF_W::new(self)
    }
    #[doc = "Bit 8 - Clear PERR Interrupt Flag"]
    #[inline(always)]
    pub fn perr(&mut self) -> PERR_W {
        PERR_W::new(self)
    }
    #[doc = "Bit 9 - Clear FERR Interrupt Flag"]
    #[inline(always)]
    pub fn ferr(&mut self) -> FERR_W {
        FERR_W::new(self)
    }
    #[doc = "Bit 10 - Clear MPAF Interrupt Flag"]
    #[inline(always)]
    pub fn mpaf(&mut self) -> MPAF_W {
        MPAF_W::new(self)
    }
    #[doc = "Bit 11 - Clear SSM Interrupt Flag"]
    #[inline(always)]
    pub fn ssm(&mut self) -> SSM_W {
        SSM_W::new(self)
    }
    #[doc = "Bit 12 - Clear CCF Interrupt Flag"]
    #[inline(always)]
    pub fn ccf(&mut self) -> CCF_W {
        CCF_W::new(self)
    }
    #[doc = "Bit 13 - Clear TXIDLE Interrupt Flag"]
    #[inline(always)]
    pub fn txidle(&mut self) -> TXIDLE_W {
        TXIDLE_W::new(self)
    }
    #[doc = "Bit 14 - Clear TCMP0 Interrupt Flag"]
    #[inline(always)]
    pub fn tcmp0(&mut self) -> TCMP0_W {
        TCMP0_W::new(self)
    }
    #[doc = "Bit 15 - Clear TCMP1 Interrupt Flag"]
    #[inline(always)]
    pub fn tcmp1(&mut self) -> TCMP1_W {
        TCMP1_W::new(self)
    }
    #[doc = "Bit 16 - Clear TCMP2 Interrupt Flag"]
    #[inline(always)]
    pub fn tcmp2(&mut self) -> TCMP2_W {
        TCMP2_W::new(self)
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
}
#[doc = "`reset()` method sets IFC to value 0"]
impl crate::Resettable for IFC_SPEC {
    #[inline(always)]
    fn reset_value() -> Self::Ux {
        0
    }
}
