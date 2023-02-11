#[doc = "Register `MIR1_CMDMASK` reader"]
pub struct R(crate::R<MIR1_CMDMASK_SPEC>);
impl core::ops::Deref for R {
    type Target = crate::R<MIR1_CMDMASK_SPEC>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
impl From<crate::R<MIR1_CMDMASK_SPEC>> for R {
    #[inline(always)]
    fn from(reader: crate::R<MIR1_CMDMASK_SPEC>) -> Self {
        R(reader)
    }
}
#[doc = "Register `MIR1_CMDMASK` writer"]
pub struct W(crate::W<MIR1_CMDMASK_SPEC>);
impl core::ops::Deref for W {
    type Target = crate::W<MIR1_CMDMASK_SPEC>;
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
impl From<crate::W<MIR1_CMDMASK_SPEC>> for W {
    #[inline(always)]
    fn from(writer: crate::W<MIR1_CMDMASK_SPEC>) -> Self {
        W(writer)
    }
}
#[doc = "Field `DATAB` reader - CC Channel Mode"]
pub type DATAB_R = crate::BitReader<bool>;
#[doc = "Field `DATAB` writer - CC Channel Mode"]
pub type DATAB_W<'a, const O: u8> = crate::BitWriter<'a, u32, MIR1_CMDMASK_SPEC, bool, O>;
#[doc = "Field `DATAA` reader - Access Data Bytes 0-3"]
pub type DATAA_R = crate::BitReader<bool>;
#[doc = "Field `DATAA` writer - Access Data Bytes 0-3"]
pub type DATAA_W<'a, const O: u8> = crate::BitWriter<'a, u32, MIR1_CMDMASK_SPEC, bool, O>;
#[doc = "Field `TXRQSTNEWDAT` reader - Transmission Request Bit/ New Data Bit"]
pub type TXRQSTNEWDAT_R = crate::BitReader<bool>;
#[doc = "Field `TXRQSTNEWDAT` writer - Transmission Request Bit/ New Data Bit"]
pub type TXRQSTNEWDAT_W<'a, const O: u8> = crate::BitWriter<'a, u32, MIR1_CMDMASK_SPEC, bool, O>;
#[doc = "Field `CLRINTPND` reader - Clear Interrupt Pending Bit"]
pub type CLRINTPND_R = crate::BitReader<bool>;
#[doc = "Field `CLRINTPND` writer - Clear Interrupt Pending Bit"]
pub type CLRINTPND_W<'a, const O: u8> = crate::BitWriter<'a, u32, MIR1_CMDMASK_SPEC, bool, O>;
#[doc = "Field `CONTROL` reader - Access Control Bits"]
pub type CONTROL_R = crate::BitReader<bool>;
#[doc = "Field `CONTROL` writer - Access Control Bits"]
pub type CONTROL_W<'a, const O: u8> = crate::BitWriter<'a, u32, MIR1_CMDMASK_SPEC, bool, O>;
#[doc = "Field `ARBACC` reader - Access Arbitration Bits"]
pub type ARBACC_R = crate::BitReader<bool>;
#[doc = "Field `ARBACC` writer - Access Arbitration Bits"]
pub type ARBACC_W<'a, const O: u8> = crate::BitWriter<'a, u32, MIR1_CMDMASK_SPEC, bool, O>;
#[doc = "Field `MASKACC` reader - Access Mask Bits"]
pub type MASKACC_R = crate::BitReader<bool>;
#[doc = "Field `MASKACC` writer - Access Mask Bits"]
pub type MASKACC_W<'a, const O: u8> = crate::BitWriter<'a, u32, MIR1_CMDMASK_SPEC, bool, O>;
#[doc = "Field `WRRD` reader - Write/Read RAM"]
pub type WRRD_R = crate::BitReader<bool>;
#[doc = "Field `WRRD` writer - Write/Read RAM"]
pub type WRRD_W<'a, const O: u8> = crate::BitWriter<'a, u32, MIR1_CMDMASK_SPEC, bool, O>;
impl R {
    #[doc = "Bit 0 - CC Channel Mode"]
    #[inline(always)]
    pub fn datab(&self) -> DATAB_R {
        DATAB_R::new((self.bits & 1) != 0)
    }
    #[doc = "Bit 1 - Access Data Bytes 0-3"]
    #[inline(always)]
    pub fn dataa(&self) -> DATAA_R {
        DATAA_R::new(((self.bits >> 1) & 1) != 0)
    }
    #[doc = "Bit 2 - Transmission Request Bit/ New Data Bit"]
    #[inline(always)]
    pub fn txrqstnewdat(&self) -> TXRQSTNEWDAT_R {
        TXRQSTNEWDAT_R::new(((self.bits >> 2) & 1) != 0)
    }
    #[doc = "Bit 3 - Clear Interrupt Pending Bit"]
    #[inline(always)]
    pub fn clrintpnd(&self) -> CLRINTPND_R {
        CLRINTPND_R::new(((self.bits >> 3) & 1) != 0)
    }
    #[doc = "Bit 4 - Access Control Bits"]
    #[inline(always)]
    pub fn control(&self) -> CONTROL_R {
        CONTROL_R::new(((self.bits >> 4) & 1) != 0)
    }
    #[doc = "Bit 5 - Access Arbitration Bits"]
    #[inline(always)]
    pub fn arbacc(&self) -> ARBACC_R {
        ARBACC_R::new(((self.bits >> 5) & 1) != 0)
    }
    #[doc = "Bit 6 - Access Mask Bits"]
    #[inline(always)]
    pub fn maskacc(&self) -> MASKACC_R {
        MASKACC_R::new(((self.bits >> 6) & 1) != 0)
    }
    #[doc = "Bit 7 - Write/Read RAM"]
    #[inline(always)]
    pub fn wrrd(&self) -> WRRD_R {
        WRRD_R::new(((self.bits >> 7) & 1) != 0)
    }
}
impl W {
    #[doc = "Bit 0 - CC Channel Mode"]
    #[inline(always)]
    #[must_use]
    pub fn datab(&mut self) -> DATAB_W<0> {
        DATAB_W::new(self)
    }
    #[doc = "Bit 1 - Access Data Bytes 0-3"]
    #[inline(always)]
    #[must_use]
    pub fn dataa(&mut self) -> DATAA_W<1> {
        DATAA_W::new(self)
    }
    #[doc = "Bit 2 - Transmission Request Bit/ New Data Bit"]
    #[inline(always)]
    #[must_use]
    pub fn txrqstnewdat(&mut self) -> TXRQSTNEWDAT_W<2> {
        TXRQSTNEWDAT_W::new(self)
    }
    #[doc = "Bit 3 - Clear Interrupt Pending Bit"]
    #[inline(always)]
    #[must_use]
    pub fn clrintpnd(&mut self) -> CLRINTPND_W<3> {
        CLRINTPND_W::new(self)
    }
    #[doc = "Bit 4 - Access Control Bits"]
    #[inline(always)]
    #[must_use]
    pub fn control(&mut self) -> CONTROL_W<4> {
        CONTROL_W::new(self)
    }
    #[doc = "Bit 5 - Access Arbitration Bits"]
    #[inline(always)]
    #[must_use]
    pub fn arbacc(&mut self) -> ARBACC_W<5> {
        ARBACC_W::new(self)
    }
    #[doc = "Bit 6 - Access Mask Bits"]
    #[inline(always)]
    #[must_use]
    pub fn maskacc(&mut self) -> MASKACC_W<6> {
        MASKACC_W::new(self)
    }
    #[doc = "Bit 7 - Write/Read RAM"]
    #[inline(always)]
    #[must_use]
    pub fn wrrd(&mut self) -> WRRD_W<7> {
        WRRD_W::new(self)
    }
    #[doc = "Writes raw bits to the register."]
    #[inline(always)]
    pub unsafe fn bits(&mut self, bits: u32) -> &mut Self {
        self.0.bits(bits);
        self
    }
}
#[doc = "Interface Command Mask Register\n\nThis register you can [`read`](crate::generic::Reg::read), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [mir1_cmdmask](index.html) module"]
pub struct MIR1_CMDMASK_SPEC;
impl crate::RegisterSpec for MIR1_CMDMASK_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [mir1_cmdmask::R](R) reader structure"]
impl crate::Readable for MIR1_CMDMASK_SPEC {
    type Reader = R;
}
#[doc = "`write(|w| ..)` method takes [mir1_cmdmask::W](W) writer structure"]
impl crate::Writable for MIR1_CMDMASK_SPEC {
    type Writer = W;
    const ZERO_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
}
#[doc = "`reset()` method sets MIR1_CMDMASK to value 0"]
impl crate::Resettable for MIR1_CMDMASK_SPEC {
    const RESET_VALUE: Self::Ux = 0;
}
