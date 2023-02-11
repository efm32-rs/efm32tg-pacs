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
#[doc = "Field `CH0` writer - Set CH0 Interrupt Flag"]
pub type CH0_W<'a, const O: u8> = crate::BitWriter<'a, u32, IFS_SPEC, bool, O>;
#[doc = "Field `CH1` writer - Set CH1 Interrupt Flag"]
pub type CH1_W<'a, const O: u8> = crate::BitWriter<'a, u32, IFS_SPEC, bool, O>;
#[doc = "Field `CH2` writer - Set CH2 Interrupt Flag"]
pub type CH2_W<'a, const O: u8> = crate::BitWriter<'a, u32, IFS_SPEC, bool, O>;
#[doc = "Field `CH3` writer - Set CH3 Interrupt Flag"]
pub type CH3_W<'a, const O: u8> = crate::BitWriter<'a, u32, IFS_SPEC, bool, O>;
#[doc = "Field `CH4` writer - Set CH4 Interrupt Flag"]
pub type CH4_W<'a, const O: u8> = crate::BitWriter<'a, u32, IFS_SPEC, bool, O>;
#[doc = "Field `CH5` writer - Set CH5 Interrupt Flag"]
pub type CH5_W<'a, const O: u8> = crate::BitWriter<'a, u32, IFS_SPEC, bool, O>;
#[doc = "Field `CH6` writer - Set CH6 Interrupt Flag"]
pub type CH6_W<'a, const O: u8> = crate::BitWriter<'a, u32, IFS_SPEC, bool, O>;
#[doc = "Field `CH7` writer - Set CH7 Interrupt Flag"]
pub type CH7_W<'a, const O: u8> = crate::BitWriter<'a, u32, IFS_SPEC, bool, O>;
#[doc = "Field `CH8` writer - Set CH8 Interrupt Flag"]
pub type CH8_W<'a, const O: u8> = crate::BitWriter<'a, u32, IFS_SPEC, bool, O>;
#[doc = "Field `CH9` writer - Set CH9 Interrupt Flag"]
pub type CH9_W<'a, const O: u8> = crate::BitWriter<'a, u32, IFS_SPEC, bool, O>;
#[doc = "Field `CH10` writer - Set CH10 Interrupt Flag"]
pub type CH10_W<'a, const O: u8> = crate::BitWriter<'a, u32, IFS_SPEC, bool, O>;
#[doc = "Field `CH11` writer - Set CH11 Interrupt Flag"]
pub type CH11_W<'a, const O: u8> = crate::BitWriter<'a, u32, IFS_SPEC, bool, O>;
#[doc = "Field `CH12` writer - Set CH12 Interrupt Flag"]
pub type CH12_W<'a, const O: u8> = crate::BitWriter<'a, u32, IFS_SPEC, bool, O>;
#[doc = "Field `CH13` writer - Set CH13 Interrupt Flag"]
pub type CH13_W<'a, const O: u8> = crate::BitWriter<'a, u32, IFS_SPEC, bool, O>;
#[doc = "Field `CH14` writer - Set CH14 Interrupt Flag"]
pub type CH14_W<'a, const O: u8> = crate::BitWriter<'a, u32, IFS_SPEC, bool, O>;
#[doc = "Field `CH15` writer - Set CH15 Interrupt Flag"]
pub type CH15_W<'a, const O: u8> = crate::BitWriter<'a, u32, IFS_SPEC, bool, O>;
#[doc = "Field `SCANCOMPLETE` writer - Set SCANCOMPLETE Interrupt Flag"]
pub type SCANCOMPLETE_W<'a, const O: u8> = crate::BitWriter<'a, u32, IFS_SPEC, bool, O>;
#[doc = "Field `DEC` writer - Set DEC Interrupt Flag"]
pub type DEC_W<'a, const O: u8> = crate::BitWriter<'a, u32, IFS_SPEC, bool, O>;
#[doc = "Field `DECERR` writer - Set DECERR Interrupt Flag"]
pub type DECERR_W<'a, const O: u8> = crate::BitWriter<'a, u32, IFS_SPEC, bool, O>;
#[doc = "Field `BUFDATAV` writer - Set BUFDATAV Interrupt Flag"]
pub type BUFDATAV_W<'a, const O: u8> = crate::BitWriter<'a, u32, IFS_SPEC, bool, O>;
#[doc = "Field `BUFLEVEL` writer - Set BUFLEVEL Interrupt Flag"]
pub type BUFLEVEL_W<'a, const O: u8> = crate::BitWriter<'a, u32, IFS_SPEC, bool, O>;
#[doc = "Field `BUFOF` writer - Set BUFOF Interrupt Flag"]
pub type BUFOF_W<'a, const O: u8> = crate::BitWriter<'a, u32, IFS_SPEC, bool, O>;
#[doc = "Field `CNTOF` writer - Set CNTOF Interrupt Flag"]
pub type CNTOF_W<'a, const O: u8> = crate::BitWriter<'a, u32, IFS_SPEC, bool, O>;
impl W {
    #[doc = "Bit 0 - Set CH0 Interrupt Flag"]
    #[inline(always)]
    #[must_use]
    pub fn ch0(&mut self) -> CH0_W<0> {
        CH0_W::new(self)
    }
    #[doc = "Bit 1 - Set CH1 Interrupt Flag"]
    #[inline(always)]
    #[must_use]
    pub fn ch1(&mut self) -> CH1_W<1> {
        CH1_W::new(self)
    }
    #[doc = "Bit 2 - Set CH2 Interrupt Flag"]
    #[inline(always)]
    #[must_use]
    pub fn ch2(&mut self) -> CH2_W<2> {
        CH2_W::new(self)
    }
    #[doc = "Bit 3 - Set CH3 Interrupt Flag"]
    #[inline(always)]
    #[must_use]
    pub fn ch3(&mut self) -> CH3_W<3> {
        CH3_W::new(self)
    }
    #[doc = "Bit 4 - Set CH4 Interrupt Flag"]
    #[inline(always)]
    #[must_use]
    pub fn ch4(&mut self) -> CH4_W<4> {
        CH4_W::new(self)
    }
    #[doc = "Bit 5 - Set CH5 Interrupt Flag"]
    #[inline(always)]
    #[must_use]
    pub fn ch5(&mut self) -> CH5_W<5> {
        CH5_W::new(self)
    }
    #[doc = "Bit 6 - Set CH6 Interrupt Flag"]
    #[inline(always)]
    #[must_use]
    pub fn ch6(&mut self) -> CH6_W<6> {
        CH6_W::new(self)
    }
    #[doc = "Bit 7 - Set CH7 Interrupt Flag"]
    #[inline(always)]
    #[must_use]
    pub fn ch7(&mut self) -> CH7_W<7> {
        CH7_W::new(self)
    }
    #[doc = "Bit 8 - Set CH8 Interrupt Flag"]
    #[inline(always)]
    #[must_use]
    pub fn ch8(&mut self) -> CH8_W<8> {
        CH8_W::new(self)
    }
    #[doc = "Bit 9 - Set CH9 Interrupt Flag"]
    #[inline(always)]
    #[must_use]
    pub fn ch9(&mut self) -> CH9_W<9> {
        CH9_W::new(self)
    }
    #[doc = "Bit 10 - Set CH10 Interrupt Flag"]
    #[inline(always)]
    #[must_use]
    pub fn ch10(&mut self) -> CH10_W<10> {
        CH10_W::new(self)
    }
    #[doc = "Bit 11 - Set CH11 Interrupt Flag"]
    #[inline(always)]
    #[must_use]
    pub fn ch11(&mut self) -> CH11_W<11> {
        CH11_W::new(self)
    }
    #[doc = "Bit 12 - Set CH12 Interrupt Flag"]
    #[inline(always)]
    #[must_use]
    pub fn ch12(&mut self) -> CH12_W<12> {
        CH12_W::new(self)
    }
    #[doc = "Bit 13 - Set CH13 Interrupt Flag"]
    #[inline(always)]
    #[must_use]
    pub fn ch13(&mut self) -> CH13_W<13> {
        CH13_W::new(self)
    }
    #[doc = "Bit 14 - Set CH14 Interrupt Flag"]
    #[inline(always)]
    #[must_use]
    pub fn ch14(&mut self) -> CH14_W<14> {
        CH14_W::new(self)
    }
    #[doc = "Bit 15 - Set CH15 Interrupt Flag"]
    #[inline(always)]
    #[must_use]
    pub fn ch15(&mut self) -> CH15_W<15> {
        CH15_W::new(self)
    }
    #[doc = "Bit 16 - Set SCANCOMPLETE Interrupt Flag"]
    #[inline(always)]
    #[must_use]
    pub fn scancomplete(&mut self) -> SCANCOMPLETE_W<16> {
        SCANCOMPLETE_W::new(self)
    }
    #[doc = "Bit 17 - Set DEC Interrupt Flag"]
    #[inline(always)]
    #[must_use]
    pub fn dec(&mut self) -> DEC_W<17> {
        DEC_W::new(self)
    }
    #[doc = "Bit 18 - Set DECERR Interrupt Flag"]
    #[inline(always)]
    #[must_use]
    pub fn decerr(&mut self) -> DECERR_W<18> {
        DECERR_W::new(self)
    }
    #[doc = "Bit 19 - Set BUFDATAV Interrupt Flag"]
    #[inline(always)]
    #[must_use]
    pub fn bufdatav(&mut self) -> BUFDATAV_W<19> {
        BUFDATAV_W::new(self)
    }
    #[doc = "Bit 20 - Set BUFLEVEL Interrupt Flag"]
    #[inline(always)]
    #[must_use]
    pub fn buflevel(&mut self) -> BUFLEVEL_W<20> {
        BUFLEVEL_W::new(self)
    }
    #[doc = "Bit 21 - Set BUFOF Interrupt Flag"]
    #[inline(always)]
    #[must_use]
    pub fn bufof(&mut self) -> BUFOF_W<21> {
        BUFOF_W::new(self)
    }
    #[doc = "Bit 22 - Set CNTOF Interrupt Flag"]
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
#[doc = "Interrupt Flag Set Register\n\nThis register you can [`write_with_zero`](crate::generic::Reg::write_with_zero), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [ifs](index.html) module"]
pub struct IFS_SPEC;
impl crate::RegisterSpec for IFS_SPEC {
    type Ux = u32;
}
#[doc = "`write(|w| ..)` method takes [ifs::W](W) writer structure"]
impl crate::Writable for IFS_SPEC {
    type Writer = W;
    const ZERO_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
}
#[doc = "`reset()` method sets IFS to value 0"]
impl crate::Resettable for IFS_SPEC {
    const RESET_VALUE: Self::Ux = 0;
}
