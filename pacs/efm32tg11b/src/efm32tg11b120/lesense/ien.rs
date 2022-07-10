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
#[doc = "Field `CH0` reader - CH0 Interrupt Enable"]
pub type CH0_R = crate::BitReader<bool>;
#[doc = "Field `CH0` writer - CH0 Interrupt Enable"]
pub type CH0_W<'a> = crate::BitWriter<'a, u32, IEN_SPEC, bool, 0>;
#[doc = "Field `CH1` reader - CH1 Interrupt Enable"]
pub type CH1_R = crate::BitReader<bool>;
#[doc = "Field `CH1` writer - CH1 Interrupt Enable"]
pub type CH1_W<'a> = crate::BitWriter<'a, u32, IEN_SPEC, bool, 1>;
#[doc = "Field `CH2` reader - CH2 Interrupt Enable"]
pub type CH2_R = crate::BitReader<bool>;
#[doc = "Field `CH2` writer - CH2 Interrupt Enable"]
pub type CH2_W<'a> = crate::BitWriter<'a, u32, IEN_SPEC, bool, 2>;
#[doc = "Field `CH3` reader - CH3 Interrupt Enable"]
pub type CH3_R = crate::BitReader<bool>;
#[doc = "Field `CH3` writer - CH3 Interrupt Enable"]
pub type CH3_W<'a> = crate::BitWriter<'a, u32, IEN_SPEC, bool, 3>;
#[doc = "Field `CH4` reader - CH4 Interrupt Enable"]
pub type CH4_R = crate::BitReader<bool>;
#[doc = "Field `CH4` writer - CH4 Interrupt Enable"]
pub type CH4_W<'a> = crate::BitWriter<'a, u32, IEN_SPEC, bool, 4>;
#[doc = "Field `CH5` reader - CH5 Interrupt Enable"]
pub type CH5_R = crate::BitReader<bool>;
#[doc = "Field `CH5` writer - CH5 Interrupt Enable"]
pub type CH5_W<'a> = crate::BitWriter<'a, u32, IEN_SPEC, bool, 5>;
#[doc = "Field `CH6` reader - CH6 Interrupt Enable"]
pub type CH6_R = crate::BitReader<bool>;
#[doc = "Field `CH6` writer - CH6 Interrupt Enable"]
pub type CH6_W<'a> = crate::BitWriter<'a, u32, IEN_SPEC, bool, 6>;
#[doc = "Field `CH7` reader - CH7 Interrupt Enable"]
pub type CH7_R = crate::BitReader<bool>;
#[doc = "Field `CH7` writer - CH7 Interrupt Enable"]
pub type CH7_W<'a> = crate::BitWriter<'a, u32, IEN_SPEC, bool, 7>;
#[doc = "Field `CH8` reader - CH8 Interrupt Enable"]
pub type CH8_R = crate::BitReader<bool>;
#[doc = "Field `CH8` writer - CH8 Interrupt Enable"]
pub type CH8_W<'a> = crate::BitWriter<'a, u32, IEN_SPEC, bool, 8>;
#[doc = "Field `CH9` reader - CH9 Interrupt Enable"]
pub type CH9_R = crate::BitReader<bool>;
#[doc = "Field `CH9` writer - CH9 Interrupt Enable"]
pub type CH9_W<'a> = crate::BitWriter<'a, u32, IEN_SPEC, bool, 9>;
#[doc = "Field `CH10` reader - CH10 Interrupt Enable"]
pub type CH10_R = crate::BitReader<bool>;
#[doc = "Field `CH10` writer - CH10 Interrupt Enable"]
pub type CH10_W<'a> = crate::BitWriter<'a, u32, IEN_SPEC, bool, 10>;
#[doc = "Field `CH11` reader - CH11 Interrupt Enable"]
pub type CH11_R = crate::BitReader<bool>;
#[doc = "Field `CH11` writer - CH11 Interrupt Enable"]
pub type CH11_W<'a> = crate::BitWriter<'a, u32, IEN_SPEC, bool, 11>;
#[doc = "Field `CH12` reader - CH12 Interrupt Enable"]
pub type CH12_R = crate::BitReader<bool>;
#[doc = "Field `CH12` writer - CH12 Interrupt Enable"]
pub type CH12_W<'a> = crate::BitWriter<'a, u32, IEN_SPEC, bool, 12>;
#[doc = "Field `CH13` reader - CH13 Interrupt Enable"]
pub type CH13_R = crate::BitReader<bool>;
#[doc = "Field `CH13` writer - CH13 Interrupt Enable"]
pub type CH13_W<'a> = crate::BitWriter<'a, u32, IEN_SPEC, bool, 13>;
#[doc = "Field `CH14` reader - CH14 Interrupt Enable"]
pub type CH14_R = crate::BitReader<bool>;
#[doc = "Field `CH14` writer - CH14 Interrupt Enable"]
pub type CH14_W<'a> = crate::BitWriter<'a, u32, IEN_SPEC, bool, 14>;
#[doc = "Field `CH15` reader - CH15 Interrupt Enable"]
pub type CH15_R = crate::BitReader<bool>;
#[doc = "Field `CH15` writer - CH15 Interrupt Enable"]
pub type CH15_W<'a> = crate::BitWriter<'a, u32, IEN_SPEC, bool, 15>;
#[doc = "Field `SCANCOMPLETE` reader - SCANCOMPLETE Interrupt Enable"]
pub type SCANCOMPLETE_R = crate::BitReader<bool>;
#[doc = "Field `SCANCOMPLETE` writer - SCANCOMPLETE Interrupt Enable"]
pub type SCANCOMPLETE_W<'a> = crate::BitWriter<'a, u32, IEN_SPEC, bool, 16>;
#[doc = "Field `DEC` reader - DEC Interrupt Enable"]
pub type DEC_R = crate::BitReader<bool>;
#[doc = "Field `DEC` writer - DEC Interrupt Enable"]
pub type DEC_W<'a> = crate::BitWriter<'a, u32, IEN_SPEC, bool, 17>;
#[doc = "Field `DECERR` reader - DECERR Interrupt Enable"]
pub type DECERR_R = crate::BitReader<bool>;
#[doc = "Field `DECERR` writer - DECERR Interrupt Enable"]
pub type DECERR_W<'a> = crate::BitWriter<'a, u32, IEN_SPEC, bool, 18>;
#[doc = "Field `BUFDATAV` reader - BUFDATAV Interrupt Enable"]
pub type BUFDATAV_R = crate::BitReader<bool>;
#[doc = "Field `BUFDATAV` writer - BUFDATAV Interrupt Enable"]
pub type BUFDATAV_W<'a> = crate::BitWriter<'a, u32, IEN_SPEC, bool, 19>;
#[doc = "Field `BUFLEVEL` reader - BUFLEVEL Interrupt Enable"]
pub type BUFLEVEL_R = crate::BitReader<bool>;
#[doc = "Field `BUFLEVEL` writer - BUFLEVEL Interrupt Enable"]
pub type BUFLEVEL_W<'a> = crate::BitWriter<'a, u32, IEN_SPEC, bool, 20>;
#[doc = "Field `BUFOF` reader - BUFOF Interrupt Enable"]
pub type BUFOF_R = crate::BitReader<bool>;
#[doc = "Field `BUFOF` writer - BUFOF Interrupt Enable"]
pub type BUFOF_W<'a> = crate::BitWriter<'a, u32, IEN_SPEC, bool, 21>;
#[doc = "Field `CNTOF` reader - CNTOF Interrupt Enable"]
pub type CNTOF_R = crate::BitReader<bool>;
#[doc = "Field `CNTOF` writer - CNTOF Interrupt Enable"]
pub type CNTOF_W<'a> = crate::BitWriter<'a, u32, IEN_SPEC, bool, 22>;
impl R {
    #[doc = "Bit 0 - CH0 Interrupt Enable"]
    #[inline(always)]
    pub fn ch0(&self) -> CH0_R {
        CH0_R::new((self.bits & 1) != 0)
    }
    #[doc = "Bit 1 - CH1 Interrupt Enable"]
    #[inline(always)]
    pub fn ch1(&self) -> CH1_R {
        CH1_R::new(((self.bits >> 1) & 1) != 0)
    }
    #[doc = "Bit 2 - CH2 Interrupt Enable"]
    #[inline(always)]
    pub fn ch2(&self) -> CH2_R {
        CH2_R::new(((self.bits >> 2) & 1) != 0)
    }
    #[doc = "Bit 3 - CH3 Interrupt Enable"]
    #[inline(always)]
    pub fn ch3(&self) -> CH3_R {
        CH3_R::new(((self.bits >> 3) & 1) != 0)
    }
    #[doc = "Bit 4 - CH4 Interrupt Enable"]
    #[inline(always)]
    pub fn ch4(&self) -> CH4_R {
        CH4_R::new(((self.bits >> 4) & 1) != 0)
    }
    #[doc = "Bit 5 - CH5 Interrupt Enable"]
    #[inline(always)]
    pub fn ch5(&self) -> CH5_R {
        CH5_R::new(((self.bits >> 5) & 1) != 0)
    }
    #[doc = "Bit 6 - CH6 Interrupt Enable"]
    #[inline(always)]
    pub fn ch6(&self) -> CH6_R {
        CH6_R::new(((self.bits >> 6) & 1) != 0)
    }
    #[doc = "Bit 7 - CH7 Interrupt Enable"]
    #[inline(always)]
    pub fn ch7(&self) -> CH7_R {
        CH7_R::new(((self.bits >> 7) & 1) != 0)
    }
    #[doc = "Bit 8 - CH8 Interrupt Enable"]
    #[inline(always)]
    pub fn ch8(&self) -> CH8_R {
        CH8_R::new(((self.bits >> 8) & 1) != 0)
    }
    #[doc = "Bit 9 - CH9 Interrupt Enable"]
    #[inline(always)]
    pub fn ch9(&self) -> CH9_R {
        CH9_R::new(((self.bits >> 9) & 1) != 0)
    }
    #[doc = "Bit 10 - CH10 Interrupt Enable"]
    #[inline(always)]
    pub fn ch10(&self) -> CH10_R {
        CH10_R::new(((self.bits >> 10) & 1) != 0)
    }
    #[doc = "Bit 11 - CH11 Interrupt Enable"]
    #[inline(always)]
    pub fn ch11(&self) -> CH11_R {
        CH11_R::new(((self.bits >> 11) & 1) != 0)
    }
    #[doc = "Bit 12 - CH12 Interrupt Enable"]
    #[inline(always)]
    pub fn ch12(&self) -> CH12_R {
        CH12_R::new(((self.bits >> 12) & 1) != 0)
    }
    #[doc = "Bit 13 - CH13 Interrupt Enable"]
    #[inline(always)]
    pub fn ch13(&self) -> CH13_R {
        CH13_R::new(((self.bits >> 13) & 1) != 0)
    }
    #[doc = "Bit 14 - CH14 Interrupt Enable"]
    #[inline(always)]
    pub fn ch14(&self) -> CH14_R {
        CH14_R::new(((self.bits >> 14) & 1) != 0)
    }
    #[doc = "Bit 15 - CH15 Interrupt Enable"]
    #[inline(always)]
    pub fn ch15(&self) -> CH15_R {
        CH15_R::new(((self.bits >> 15) & 1) != 0)
    }
    #[doc = "Bit 16 - SCANCOMPLETE Interrupt Enable"]
    #[inline(always)]
    pub fn scancomplete(&self) -> SCANCOMPLETE_R {
        SCANCOMPLETE_R::new(((self.bits >> 16) & 1) != 0)
    }
    #[doc = "Bit 17 - DEC Interrupt Enable"]
    #[inline(always)]
    pub fn dec(&self) -> DEC_R {
        DEC_R::new(((self.bits >> 17) & 1) != 0)
    }
    #[doc = "Bit 18 - DECERR Interrupt Enable"]
    #[inline(always)]
    pub fn decerr(&self) -> DECERR_R {
        DECERR_R::new(((self.bits >> 18) & 1) != 0)
    }
    #[doc = "Bit 19 - BUFDATAV Interrupt Enable"]
    #[inline(always)]
    pub fn bufdatav(&self) -> BUFDATAV_R {
        BUFDATAV_R::new(((self.bits >> 19) & 1) != 0)
    }
    #[doc = "Bit 20 - BUFLEVEL Interrupt Enable"]
    #[inline(always)]
    pub fn buflevel(&self) -> BUFLEVEL_R {
        BUFLEVEL_R::new(((self.bits >> 20) & 1) != 0)
    }
    #[doc = "Bit 21 - BUFOF Interrupt Enable"]
    #[inline(always)]
    pub fn bufof(&self) -> BUFOF_R {
        BUFOF_R::new(((self.bits >> 21) & 1) != 0)
    }
    #[doc = "Bit 22 - CNTOF Interrupt Enable"]
    #[inline(always)]
    pub fn cntof(&self) -> CNTOF_R {
        CNTOF_R::new(((self.bits >> 22) & 1) != 0)
    }
}
impl W {
    #[doc = "Bit 0 - CH0 Interrupt Enable"]
    #[inline(always)]
    pub fn ch0(&mut self) -> CH0_W {
        CH0_W::new(self)
    }
    #[doc = "Bit 1 - CH1 Interrupt Enable"]
    #[inline(always)]
    pub fn ch1(&mut self) -> CH1_W {
        CH1_W::new(self)
    }
    #[doc = "Bit 2 - CH2 Interrupt Enable"]
    #[inline(always)]
    pub fn ch2(&mut self) -> CH2_W {
        CH2_W::new(self)
    }
    #[doc = "Bit 3 - CH3 Interrupt Enable"]
    #[inline(always)]
    pub fn ch3(&mut self) -> CH3_W {
        CH3_W::new(self)
    }
    #[doc = "Bit 4 - CH4 Interrupt Enable"]
    #[inline(always)]
    pub fn ch4(&mut self) -> CH4_W {
        CH4_W::new(self)
    }
    #[doc = "Bit 5 - CH5 Interrupt Enable"]
    #[inline(always)]
    pub fn ch5(&mut self) -> CH5_W {
        CH5_W::new(self)
    }
    #[doc = "Bit 6 - CH6 Interrupt Enable"]
    #[inline(always)]
    pub fn ch6(&mut self) -> CH6_W {
        CH6_W::new(self)
    }
    #[doc = "Bit 7 - CH7 Interrupt Enable"]
    #[inline(always)]
    pub fn ch7(&mut self) -> CH7_W {
        CH7_W::new(self)
    }
    #[doc = "Bit 8 - CH8 Interrupt Enable"]
    #[inline(always)]
    pub fn ch8(&mut self) -> CH8_W {
        CH8_W::new(self)
    }
    #[doc = "Bit 9 - CH9 Interrupt Enable"]
    #[inline(always)]
    pub fn ch9(&mut self) -> CH9_W {
        CH9_W::new(self)
    }
    #[doc = "Bit 10 - CH10 Interrupt Enable"]
    #[inline(always)]
    pub fn ch10(&mut self) -> CH10_W {
        CH10_W::new(self)
    }
    #[doc = "Bit 11 - CH11 Interrupt Enable"]
    #[inline(always)]
    pub fn ch11(&mut self) -> CH11_W {
        CH11_W::new(self)
    }
    #[doc = "Bit 12 - CH12 Interrupt Enable"]
    #[inline(always)]
    pub fn ch12(&mut self) -> CH12_W {
        CH12_W::new(self)
    }
    #[doc = "Bit 13 - CH13 Interrupt Enable"]
    #[inline(always)]
    pub fn ch13(&mut self) -> CH13_W {
        CH13_W::new(self)
    }
    #[doc = "Bit 14 - CH14 Interrupt Enable"]
    #[inline(always)]
    pub fn ch14(&mut self) -> CH14_W {
        CH14_W::new(self)
    }
    #[doc = "Bit 15 - CH15 Interrupt Enable"]
    #[inline(always)]
    pub fn ch15(&mut self) -> CH15_W {
        CH15_W::new(self)
    }
    #[doc = "Bit 16 - SCANCOMPLETE Interrupt Enable"]
    #[inline(always)]
    pub fn scancomplete(&mut self) -> SCANCOMPLETE_W {
        SCANCOMPLETE_W::new(self)
    }
    #[doc = "Bit 17 - DEC Interrupt Enable"]
    #[inline(always)]
    pub fn dec(&mut self) -> DEC_W {
        DEC_W::new(self)
    }
    #[doc = "Bit 18 - DECERR Interrupt Enable"]
    #[inline(always)]
    pub fn decerr(&mut self) -> DECERR_W {
        DECERR_W::new(self)
    }
    #[doc = "Bit 19 - BUFDATAV Interrupt Enable"]
    #[inline(always)]
    pub fn bufdatav(&mut self) -> BUFDATAV_W {
        BUFDATAV_W::new(self)
    }
    #[doc = "Bit 20 - BUFLEVEL Interrupt Enable"]
    #[inline(always)]
    pub fn buflevel(&mut self) -> BUFLEVEL_W {
        BUFLEVEL_W::new(self)
    }
    #[doc = "Bit 21 - BUFOF Interrupt Enable"]
    #[inline(always)]
    pub fn bufof(&mut self) -> BUFOF_W {
        BUFOF_W::new(self)
    }
    #[doc = "Bit 22 - CNTOF Interrupt Enable"]
    #[inline(always)]
    pub fn cntof(&mut self) -> CNTOF_W {
        CNTOF_W::new(self)
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
}
#[doc = "`reset()` method sets IEN to value 0"]
impl crate::Resettable for IEN_SPEC {
    #[inline(always)]
    fn reset_value() -> Self::Ux {
        0
    }
}
