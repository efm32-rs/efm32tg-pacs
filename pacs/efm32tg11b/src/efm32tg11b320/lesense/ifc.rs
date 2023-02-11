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
#[doc = "Field `CH0` writer - Clear CH0 Interrupt Flag"]
pub type CH0_W<'a, const O: u8> = crate::BitWriter<'a, u32, IFC_SPEC, bool, O>;
#[doc = "Field `CH1` writer - Clear CH1 Interrupt Flag"]
pub type CH1_W<'a, const O: u8> = crate::BitWriter<'a, u32, IFC_SPEC, bool, O>;
#[doc = "Field `CH2` writer - Clear CH2 Interrupt Flag"]
pub type CH2_W<'a, const O: u8> = crate::BitWriter<'a, u32, IFC_SPEC, bool, O>;
#[doc = "Field `CH3` writer - Clear CH3 Interrupt Flag"]
pub type CH3_W<'a, const O: u8> = crate::BitWriter<'a, u32, IFC_SPEC, bool, O>;
#[doc = "Field `CH4` writer - Clear CH4 Interrupt Flag"]
pub type CH4_W<'a, const O: u8> = crate::BitWriter<'a, u32, IFC_SPEC, bool, O>;
#[doc = "Field `CH5` writer - Clear CH5 Interrupt Flag"]
pub type CH5_W<'a, const O: u8> = crate::BitWriter<'a, u32, IFC_SPEC, bool, O>;
#[doc = "Field `CH6` writer - Clear CH6 Interrupt Flag"]
pub type CH6_W<'a, const O: u8> = crate::BitWriter<'a, u32, IFC_SPEC, bool, O>;
#[doc = "Field `CH7` writer - Clear CH7 Interrupt Flag"]
pub type CH7_W<'a, const O: u8> = crate::BitWriter<'a, u32, IFC_SPEC, bool, O>;
#[doc = "Field `CH8` writer - Clear CH8 Interrupt Flag"]
pub type CH8_W<'a, const O: u8> = crate::BitWriter<'a, u32, IFC_SPEC, bool, O>;
#[doc = "Field `CH9` writer - Clear CH9 Interrupt Flag"]
pub type CH9_W<'a, const O: u8> = crate::BitWriter<'a, u32, IFC_SPEC, bool, O>;
#[doc = "Field `CH10` writer - Clear CH10 Interrupt Flag"]
pub type CH10_W<'a, const O: u8> = crate::BitWriter<'a, u32, IFC_SPEC, bool, O>;
#[doc = "Field `CH11` writer - Clear CH11 Interrupt Flag"]
pub type CH11_W<'a, const O: u8> = crate::BitWriter<'a, u32, IFC_SPEC, bool, O>;
#[doc = "Field `CH12` writer - Clear CH12 Interrupt Flag"]
pub type CH12_W<'a, const O: u8> = crate::BitWriter<'a, u32, IFC_SPEC, bool, O>;
#[doc = "Field `CH13` writer - Clear CH13 Interrupt Flag"]
pub type CH13_W<'a, const O: u8> = crate::BitWriter<'a, u32, IFC_SPEC, bool, O>;
#[doc = "Field `CH14` writer - Clear CH14 Interrupt Flag"]
pub type CH14_W<'a, const O: u8> = crate::BitWriter<'a, u32, IFC_SPEC, bool, O>;
#[doc = "Field `CH15` writer - Clear CH15 Interrupt Flag"]
pub type CH15_W<'a, const O: u8> = crate::BitWriter<'a, u32, IFC_SPEC, bool, O>;
#[doc = "Field `SCANCOMPLETE` writer - Clear SCANCOMPLETE Interrupt Flag"]
pub type SCANCOMPLETE_W<'a, const O: u8> = crate::BitWriter<'a, u32, IFC_SPEC, bool, O>;
#[doc = "Field `DEC` writer - Clear DEC Interrupt Flag"]
pub type DEC_W<'a, const O: u8> = crate::BitWriter<'a, u32, IFC_SPEC, bool, O>;
#[doc = "Field `DECERR` writer - Clear DECERR Interrupt Flag"]
pub type DECERR_W<'a, const O: u8> = crate::BitWriter<'a, u32, IFC_SPEC, bool, O>;
#[doc = "Field `BUFDATAV` writer - Clear BUFDATAV Interrupt Flag"]
pub type BUFDATAV_W<'a, const O: u8> = crate::BitWriter<'a, u32, IFC_SPEC, bool, O>;
#[doc = "Field `BUFLEVEL` writer - Clear BUFLEVEL Interrupt Flag"]
pub type BUFLEVEL_W<'a, const O: u8> = crate::BitWriter<'a, u32, IFC_SPEC, bool, O>;
#[doc = "Field `BUFOF` writer - Clear BUFOF Interrupt Flag"]
pub type BUFOF_W<'a, const O: u8> = crate::BitWriter<'a, u32, IFC_SPEC, bool, O>;
#[doc = "Field `CNTOF` writer - Clear CNTOF Interrupt Flag"]
pub type CNTOF_W<'a, const O: u8> = crate::BitWriter<'a, u32, IFC_SPEC, bool, O>;
impl W {
    #[doc = "Bit 0 - Clear CH0 Interrupt Flag"]
    #[inline(always)]
    #[must_use]
    pub fn ch0(&mut self) -> CH0_W<0> {
        CH0_W::new(self)
    }
    #[doc = "Bit 1 - Clear CH1 Interrupt Flag"]
    #[inline(always)]
    #[must_use]
    pub fn ch1(&mut self) -> CH1_W<1> {
        CH1_W::new(self)
    }
    #[doc = "Bit 2 - Clear CH2 Interrupt Flag"]
    #[inline(always)]
    #[must_use]
    pub fn ch2(&mut self) -> CH2_W<2> {
        CH2_W::new(self)
    }
    #[doc = "Bit 3 - Clear CH3 Interrupt Flag"]
    #[inline(always)]
    #[must_use]
    pub fn ch3(&mut self) -> CH3_W<3> {
        CH3_W::new(self)
    }
    #[doc = "Bit 4 - Clear CH4 Interrupt Flag"]
    #[inline(always)]
    #[must_use]
    pub fn ch4(&mut self) -> CH4_W<4> {
        CH4_W::new(self)
    }
    #[doc = "Bit 5 - Clear CH5 Interrupt Flag"]
    #[inline(always)]
    #[must_use]
    pub fn ch5(&mut self) -> CH5_W<5> {
        CH5_W::new(self)
    }
    #[doc = "Bit 6 - Clear CH6 Interrupt Flag"]
    #[inline(always)]
    #[must_use]
    pub fn ch6(&mut self) -> CH6_W<6> {
        CH6_W::new(self)
    }
    #[doc = "Bit 7 - Clear CH7 Interrupt Flag"]
    #[inline(always)]
    #[must_use]
    pub fn ch7(&mut self) -> CH7_W<7> {
        CH7_W::new(self)
    }
    #[doc = "Bit 8 - Clear CH8 Interrupt Flag"]
    #[inline(always)]
    #[must_use]
    pub fn ch8(&mut self) -> CH8_W<8> {
        CH8_W::new(self)
    }
    #[doc = "Bit 9 - Clear CH9 Interrupt Flag"]
    #[inline(always)]
    #[must_use]
    pub fn ch9(&mut self) -> CH9_W<9> {
        CH9_W::new(self)
    }
    #[doc = "Bit 10 - Clear CH10 Interrupt Flag"]
    #[inline(always)]
    #[must_use]
    pub fn ch10(&mut self) -> CH10_W<10> {
        CH10_W::new(self)
    }
    #[doc = "Bit 11 - Clear CH11 Interrupt Flag"]
    #[inline(always)]
    #[must_use]
    pub fn ch11(&mut self) -> CH11_W<11> {
        CH11_W::new(self)
    }
    #[doc = "Bit 12 - Clear CH12 Interrupt Flag"]
    #[inline(always)]
    #[must_use]
    pub fn ch12(&mut self) -> CH12_W<12> {
        CH12_W::new(self)
    }
    #[doc = "Bit 13 - Clear CH13 Interrupt Flag"]
    #[inline(always)]
    #[must_use]
    pub fn ch13(&mut self) -> CH13_W<13> {
        CH13_W::new(self)
    }
    #[doc = "Bit 14 - Clear CH14 Interrupt Flag"]
    #[inline(always)]
    #[must_use]
    pub fn ch14(&mut self) -> CH14_W<14> {
        CH14_W::new(self)
    }
    #[doc = "Bit 15 - Clear CH15 Interrupt Flag"]
    #[inline(always)]
    #[must_use]
    pub fn ch15(&mut self) -> CH15_W<15> {
        CH15_W::new(self)
    }
    #[doc = "Bit 16 - Clear SCANCOMPLETE Interrupt Flag"]
    #[inline(always)]
    #[must_use]
    pub fn scancomplete(&mut self) -> SCANCOMPLETE_W<16> {
        SCANCOMPLETE_W::new(self)
    }
    #[doc = "Bit 17 - Clear DEC Interrupt Flag"]
    #[inline(always)]
    #[must_use]
    pub fn dec(&mut self) -> DEC_W<17> {
        DEC_W::new(self)
    }
    #[doc = "Bit 18 - Clear DECERR Interrupt Flag"]
    #[inline(always)]
    #[must_use]
    pub fn decerr(&mut self) -> DECERR_W<18> {
        DECERR_W::new(self)
    }
    #[doc = "Bit 19 - Clear BUFDATAV Interrupt Flag"]
    #[inline(always)]
    #[must_use]
    pub fn bufdatav(&mut self) -> BUFDATAV_W<19> {
        BUFDATAV_W::new(self)
    }
    #[doc = "Bit 20 - Clear BUFLEVEL Interrupt Flag"]
    #[inline(always)]
    #[must_use]
    pub fn buflevel(&mut self) -> BUFLEVEL_W<20> {
        BUFLEVEL_W::new(self)
    }
    #[doc = "Bit 21 - Clear BUFOF Interrupt Flag"]
    #[inline(always)]
    #[must_use]
    pub fn bufof(&mut self) -> BUFOF_W<21> {
        BUFOF_W::new(self)
    }
    #[doc = "Bit 22 - Clear CNTOF Interrupt Flag"]
    #[inline(always)]
    #[must_use]
    pub fn cntof(&mut self) -> CNTOF_W<22> {
        CNTOF_W::new(self)
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
