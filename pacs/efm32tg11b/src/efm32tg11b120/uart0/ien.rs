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
#[doc = "Field `TXC` reader - TXC Interrupt Enable"]
pub type TXC_R = crate::BitReader<bool>;
#[doc = "Field `TXC` writer - TXC Interrupt Enable"]
pub type TXC_W<'a, const O: u8> = crate::BitWriter<'a, u32, IEN_SPEC, bool, O>;
#[doc = "Field `TXBL` reader - TXBL Interrupt Enable"]
pub type TXBL_R = crate::BitReader<bool>;
#[doc = "Field `TXBL` writer - TXBL Interrupt Enable"]
pub type TXBL_W<'a, const O: u8> = crate::BitWriter<'a, u32, IEN_SPEC, bool, O>;
#[doc = "Field `RXDATAV` reader - RXDATAV Interrupt Enable"]
pub type RXDATAV_R = crate::BitReader<bool>;
#[doc = "Field `RXDATAV` writer - RXDATAV Interrupt Enable"]
pub type RXDATAV_W<'a, const O: u8> = crate::BitWriter<'a, u32, IEN_SPEC, bool, O>;
#[doc = "Field `RXFULL` reader - RXFULL Interrupt Enable"]
pub type RXFULL_R = crate::BitReader<bool>;
#[doc = "Field `RXFULL` writer - RXFULL Interrupt Enable"]
pub type RXFULL_W<'a, const O: u8> = crate::BitWriter<'a, u32, IEN_SPEC, bool, O>;
#[doc = "Field `RXOF` reader - RXOF Interrupt Enable"]
pub type RXOF_R = crate::BitReader<bool>;
#[doc = "Field `RXOF` writer - RXOF Interrupt Enable"]
pub type RXOF_W<'a, const O: u8> = crate::BitWriter<'a, u32, IEN_SPEC, bool, O>;
#[doc = "Field `RXUF` reader - RXUF Interrupt Enable"]
pub type RXUF_R = crate::BitReader<bool>;
#[doc = "Field `RXUF` writer - RXUF Interrupt Enable"]
pub type RXUF_W<'a, const O: u8> = crate::BitWriter<'a, u32, IEN_SPEC, bool, O>;
#[doc = "Field `TXOF` reader - TXOF Interrupt Enable"]
pub type TXOF_R = crate::BitReader<bool>;
#[doc = "Field `TXOF` writer - TXOF Interrupt Enable"]
pub type TXOF_W<'a, const O: u8> = crate::BitWriter<'a, u32, IEN_SPEC, bool, O>;
#[doc = "Field `TXUF` reader - TXUF Interrupt Enable"]
pub type TXUF_R = crate::BitReader<bool>;
#[doc = "Field `TXUF` writer - TXUF Interrupt Enable"]
pub type TXUF_W<'a, const O: u8> = crate::BitWriter<'a, u32, IEN_SPEC, bool, O>;
#[doc = "Field `PERR` reader - PERR Interrupt Enable"]
pub type PERR_R = crate::BitReader<bool>;
#[doc = "Field `PERR` writer - PERR Interrupt Enable"]
pub type PERR_W<'a, const O: u8> = crate::BitWriter<'a, u32, IEN_SPEC, bool, O>;
#[doc = "Field `FERR` reader - FERR Interrupt Enable"]
pub type FERR_R = crate::BitReader<bool>;
#[doc = "Field `FERR` writer - FERR Interrupt Enable"]
pub type FERR_W<'a, const O: u8> = crate::BitWriter<'a, u32, IEN_SPEC, bool, O>;
#[doc = "Field `MPAF` reader - MPAF Interrupt Enable"]
pub type MPAF_R = crate::BitReader<bool>;
#[doc = "Field `MPAF` writer - MPAF Interrupt Enable"]
pub type MPAF_W<'a, const O: u8> = crate::BitWriter<'a, u32, IEN_SPEC, bool, O>;
#[doc = "Field `SSM` reader - SSM Interrupt Enable"]
pub type SSM_R = crate::BitReader<bool>;
#[doc = "Field `SSM` writer - SSM Interrupt Enable"]
pub type SSM_W<'a, const O: u8> = crate::BitWriter<'a, u32, IEN_SPEC, bool, O>;
#[doc = "Field `CCF` reader - CCF Interrupt Enable"]
pub type CCF_R = crate::BitReader<bool>;
#[doc = "Field `CCF` writer - CCF Interrupt Enable"]
pub type CCF_W<'a, const O: u8> = crate::BitWriter<'a, u32, IEN_SPEC, bool, O>;
#[doc = "Field `TXIDLE` reader - TXIDLE Interrupt Enable"]
pub type TXIDLE_R = crate::BitReader<bool>;
#[doc = "Field `TXIDLE` writer - TXIDLE Interrupt Enable"]
pub type TXIDLE_W<'a, const O: u8> = crate::BitWriter<'a, u32, IEN_SPEC, bool, O>;
#[doc = "Field `TCMP0` reader - TCMP0 Interrupt Enable"]
pub type TCMP0_R = crate::BitReader<bool>;
#[doc = "Field `TCMP0` writer - TCMP0 Interrupt Enable"]
pub type TCMP0_W<'a, const O: u8> = crate::BitWriter<'a, u32, IEN_SPEC, bool, O>;
#[doc = "Field `TCMP1` reader - TCMP1 Interrupt Enable"]
pub type TCMP1_R = crate::BitReader<bool>;
#[doc = "Field `TCMP1` writer - TCMP1 Interrupt Enable"]
pub type TCMP1_W<'a, const O: u8> = crate::BitWriter<'a, u32, IEN_SPEC, bool, O>;
#[doc = "Field `TCMP2` reader - TCMP2 Interrupt Enable"]
pub type TCMP2_R = crate::BitReader<bool>;
#[doc = "Field `TCMP2` writer - TCMP2 Interrupt Enable"]
pub type TCMP2_W<'a, const O: u8> = crate::BitWriter<'a, u32, IEN_SPEC, bool, O>;
impl R {
    #[doc = "Bit 0 - TXC Interrupt Enable"]
    #[inline(always)]
    pub fn txc(&self) -> TXC_R {
        TXC_R::new((self.bits & 1) != 0)
    }
    #[doc = "Bit 1 - TXBL Interrupt Enable"]
    #[inline(always)]
    pub fn txbl(&self) -> TXBL_R {
        TXBL_R::new(((self.bits >> 1) & 1) != 0)
    }
    #[doc = "Bit 2 - RXDATAV Interrupt Enable"]
    #[inline(always)]
    pub fn rxdatav(&self) -> RXDATAV_R {
        RXDATAV_R::new(((self.bits >> 2) & 1) != 0)
    }
    #[doc = "Bit 3 - RXFULL Interrupt Enable"]
    #[inline(always)]
    pub fn rxfull(&self) -> RXFULL_R {
        RXFULL_R::new(((self.bits >> 3) & 1) != 0)
    }
    #[doc = "Bit 4 - RXOF Interrupt Enable"]
    #[inline(always)]
    pub fn rxof(&self) -> RXOF_R {
        RXOF_R::new(((self.bits >> 4) & 1) != 0)
    }
    #[doc = "Bit 5 - RXUF Interrupt Enable"]
    #[inline(always)]
    pub fn rxuf(&self) -> RXUF_R {
        RXUF_R::new(((self.bits >> 5) & 1) != 0)
    }
    #[doc = "Bit 6 - TXOF Interrupt Enable"]
    #[inline(always)]
    pub fn txof(&self) -> TXOF_R {
        TXOF_R::new(((self.bits >> 6) & 1) != 0)
    }
    #[doc = "Bit 7 - TXUF Interrupt Enable"]
    #[inline(always)]
    pub fn txuf(&self) -> TXUF_R {
        TXUF_R::new(((self.bits >> 7) & 1) != 0)
    }
    #[doc = "Bit 8 - PERR Interrupt Enable"]
    #[inline(always)]
    pub fn perr(&self) -> PERR_R {
        PERR_R::new(((self.bits >> 8) & 1) != 0)
    }
    #[doc = "Bit 9 - FERR Interrupt Enable"]
    #[inline(always)]
    pub fn ferr(&self) -> FERR_R {
        FERR_R::new(((self.bits >> 9) & 1) != 0)
    }
    #[doc = "Bit 10 - MPAF Interrupt Enable"]
    #[inline(always)]
    pub fn mpaf(&self) -> MPAF_R {
        MPAF_R::new(((self.bits >> 10) & 1) != 0)
    }
    #[doc = "Bit 11 - SSM Interrupt Enable"]
    #[inline(always)]
    pub fn ssm(&self) -> SSM_R {
        SSM_R::new(((self.bits >> 11) & 1) != 0)
    }
    #[doc = "Bit 12 - CCF Interrupt Enable"]
    #[inline(always)]
    pub fn ccf(&self) -> CCF_R {
        CCF_R::new(((self.bits >> 12) & 1) != 0)
    }
    #[doc = "Bit 13 - TXIDLE Interrupt Enable"]
    #[inline(always)]
    pub fn txidle(&self) -> TXIDLE_R {
        TXIDLE_R::new(((self.bits >> 13) & 1) != 0)
    }
    #[doc = "Bit 14 - TCMP0 Interrupt Enable"]
    #[inline(always)]
    pub fn tcmp0(&self) -> TCMP0_R {
        TCMP0_R::new(((self.bits >> 14) & 1) != 0)
    }
    #[doc = "Bit 15 - TCMP1 Interrupt Enable"]
    #[inline(always)]
    pub fn tcmp1(&self) -> TCMP1_R {
        TCMP1_R::new(((self.bits >> 15) & 1) != 0)
    }
    #[doc = "Bit 16 - TCMP2 Interrupt Enable"]
    #[inline(always)]
    pub fn tcmp2(&self) -> TCMP2_R {
        TCMP2_R::new(((self.bits >> 16) & 1) != 0)
    }
}
impl W {
    #[doc = "Bit 0 - TXC Interrupt Enable"]
    #[inline(always)]
    #[must_use]
    pub fn txc(&mut self) -> TXC_W<0> {
        TXC_W::new(self)
    }
    #[doc = "Bit 1 - TXBL Interrupt Enable"]
    #[inline(always)]
    #[must_use]
    pub fn txbl(&mut self) -> TXBL_W<1> {
        TXBL_W::new(self)
    }
    #[doc = "Bit 2 - RXDATAV Interrupt Enable"]
    #[inline(always)]
    #[must_use]
    pub fn rxdatav(&mut self) -> RXDATAV_W<2> {
        RXDATAV_W::new(self)
    }
    #[doc = "Bit 3 - RXFULL Interrupt Enable"]
    #[inline(always)]
    #[must_use]
    pub fn rxfull(&mut self) -> RXFULL_W<3> {
        RXFULL_W::new(self)
    }
    #[doc = "Bit 4 - RXOF Interrupt Enable"]
    #[inline(always)]
    #[must_use]
    pub fn rxof(&mut self) -> RXOF_W<4> {
        RXOF_W::new(self)
    }
    #[doc = "Bit 5 - RXUF Interrupt Enable"]
    #[inline(always)]
    #[must_use]
    pub fn rxuf(&mut self) -> RXUF_W<5> {
        RXUF_W::new(self)
    }
    #[doc = "Bit 6 - TXOF Interrupt Enable"]
    #[inline(always)]
    #[must_use]
    pub fn txof(&mut self) -> TXOF_W<6> {
        TXOF_W::new(self)
    }
    #[doc = "Bit 7 - TXUF Interrupt Enable"]
    #[inline(always)]
    #[must_use]
    pub fn txuf(&mut self) -> TXUF_W<7> {
        TXUF_W::new(self)
    }
    #[doc = "Bit 8 - PERR Interrupt Enable"]
    #[inline(always)]
    #[must_use]
    pub fn perr(&mut self) -> PERR_W<8> {
        PERR_W::new(self)
    }
    #[doc = "Bit 9 - FERR Interrupt Enable"]
    #[inline(always)]
    #[must_use]
    pub fn ferr(&mut self) -> FERR_W<9> {
        FERR_W::new(self)
    }
    #[doc = "Bit 10 - MPAF Interrupt Enable"]
    #[inline(always)]
    #[must_use]
    pub fn mpaf(&mut self) -> MPAF_W<10> {
        MPAF_W::new(self)
    }
    #[doc = "Bit 11 - SSM Interrupt Enable"]
    #[inline(always)]
    #[must_use]
    pub fn ssm(&mut self) -> SSM_W<11> {
        SSM_W::new(self)
    }
    #[doc = "Bit 12 - CCF Interrupt Enable"]
    #[inline(always)]
    #[must_use]
    pub fn ccf(&mut self) -> CCF_W<12> {
        CCF_W::new(self)
    }
    #[doc = "Bit 13 - TXIDLE Interrupt Enable"]
    #[inline(always)]
    #[must_use]
    pub fn txidle(&mut self) -> TXIDLE_W<13> {
        TXIDLE_W::new(self)
    }
    #[doc = "Bit 14 - TCMP0 Interrupt Enable"]
    #[inline(always)]
    #[must_use]
    pub fn tcmp0(&mut self) -> TCMP0_W<14> {
        TCMP0_W::new(self)
    }
    #[doc = "Bit 15 - TCMP1 Interrupt Enable"]
    #[inline(always)]
    #[must_use]
    pub fn tcmp1(&mut self) -> TCMP1_W<15> {
        TCMP1_W::new(self)
    }
    #[doc = "Bit 16 - TCMP2 Interrupt Enable"]
    #[inline(always)]
    #[must_use]
    pub fn tcmp2(&mut self) -> TCMP2_W<16> {
        TCMP2_W::new(self)
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
    const ZERO_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
}
#[doc = "`reset()` method sets IEN to value 0"]
impl crate::Resettable for IEN_SPEC {
    const RESET_VALUE: Self::Ux = 0;
}
