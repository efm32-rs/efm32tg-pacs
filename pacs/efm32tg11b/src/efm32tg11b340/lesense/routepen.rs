#[doc = "Register `ROUTEPEN` reader"]
pub struct R(crate::R<ROUTEPEN_SPEC>);
impl core::ops::Deref for R {
    type Target = crate::R<ROUTEPEN_SPEC>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
impl From<crate::R<ROUTEPEN_SPEC>> for R {
    #[inline(always)]
    fn from(reader: crate::R<ROUTEPEN_SPEC>) -> Self {
        R(reader)
    }
}
#[doc = "Register `ROUTEPEN` writer"]
pub struct W(crate::W<ROUTEPEN_SPEC>);
impl core::ops::Deref for W {
    type Target = crate::W<ROUTEPEN_SPEC>;
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
impl From<crate::W<ROUTEPEN_SPEC>> for W {
    #[inline(always)]
    fn from(writer: crate::W<ROUTEPEN_SPEC>) -> Self {
        W(writer)
    }
}
#[doc = "Field `CH0PEN` reader - CH0 Pin Enable"]
pub type CH0PEN_R = crate::BitReader<bool>;
#[doc = "Field `CH0PEN` writer - CH0 Pin Enable"]
pub type CH0PEN_W<'a, const O: u8> = crate::BitWriter<'a, u32, ROUTEPEN_SPEC, bool, O>;
#[doc = "Field `CH1PEN` reader - CH1 Pin Enable"]
pub type CH1PEN_R = crate::BitReader<bool>;
#[doc = "Field `CH1PEN` writer - CH1 Pin Enable"]
pub type CH1PEN_W<'a, const O: u8> = crate::BitWriter<'a, u32, ROUTEPEN_SPEC, bool, O>;
#[doc = "Field `CH2PEN` reader - CH2 Pin Enable"]
pub type CH2PEN_R = crate::BitReader<bool>;
#[doc = "Field `CH2PEN` writer - CH2 Pin Enable"]
pub type CH2PEN_W<'a, const O: u8> = crate::BitWriter<'a, u32, ROUTEPEN_SPEC, bool, O>;
#[doc = "Field `CH3PEN` reader - CH3 Pin Enable"]
pub type CH3PEN_R = crate::BitReader<bool>;
#[doc = "Field `CH3PEN` writer - CH3 Pin Enable"]
pub type CH3PEN_W<'a, const O: u8> = crate::BitWriter<'a, u32, ROUTEPEN_SPEC, bool, O>;
#[doc = "Field `CH4PEN` reader - CH4 Pin Enable"]
pub type CH4PEN_R = crate::BitReader<bool>;
#[doc = "Field `CH4PEN` writer - CH4 Pin Enable"]
pub type CH4PEN_W<'a, const O: u8> = crate::BitWriter<'a, u32, ROUTEPEN_SPEC, bool, O>;
#[doc = "Field `CH5PEN` reader - CH5 Pin Enable"]
pub type CH5PEN_R = crate::BitReader<bool>;
#[doc = "Field `CH5PEN` writer - CH5 Pin Enable"]
pub type CH5PEN_W<'a, const O: u8> = crate::BitWriter<'a, u32, ROUTEPEN_SPEC, bool, O>;
#[doc = "Field `CH6PEN` reader - CH6 Pin Enable"]
pub type CH6PEN_R = crate::BitReader<bool>;
#[doc = "Field `CH6PEN` writer - CH6 Pin Enable"]
pub type CH6PEN_W<'a, const O: u8> = crate::BitWriter<'a, u32, ROUTEPEN_SPEC, bool, O>;
#[doc = "Field `CH7PEN` reader - CH7 Pin Enable"]
pub type CH7PEN_R = crate::BitReader<bool>;
#[doc = "Field `CH7PEN` writer - CH7 Pin Enable"]
pub type CH7PEN_W<'a, const O: u8> = crate::BitWriter<'a, u32, ROUTEPEN_SPEC, bool, O>;
#[doc = "Field `CH8PEN` reader - CH8 Pin Enable"]
pub type CH8PEN_R = crate::BitReader<bool>;
#[doc = "Field `CH8PEN` writer - CH8 Pin Enable"]
pub type CH8PEN_W<'a, const O: u8> = crate::BitWriter<'a, u32, ROUTEPEN_SPEC, bool, O>;
#[doc = "Field `CH9PEN` reader - CH9 Pin Enable"]
pub type CH9PEN_R = crate::BitReader<bool>;
#[doc = "Field `CH9PEN` writer - CH9 Pin Enable"]
pub type CH9PEN_W<'a, const O: u8> = crate::BitWriter<'a, u32, ROUTEPEN_SPEC, bool, O>;
#[doc = "Field `CH10PEN` reader - CH10 Pin Enable"]
pub type CH10PEN_R = crate::BitReader<bool>;
#[doc = "Field `CH10PEN` writer - CH10 Pin Enable"]
pub type CH10PEN_W<'a, const O: u8> = crate::BitWriter<'a, u32, ROUTEPEN_SPEC, bool, O>;
#[doc = "Field `CH11PEN` reader - CH11 Pin Enable"]
pub type CH11PEN_R = crate::BitReader<bool>;
#[doc = "Field `CH11PEN` writer - CH11 Pin Enable"]
pub type CH11PEN_W<'a, const O: u8> = crate::BitWriter<'a, u32, ROUTEPEN_SPEC, bool, O>;
#[doc = "Field `CH12PEN` reader - CH12 Pin Enable"]
pub type CH12PEN_R = crate::BitReader<bool>;
#[doc = "Field `CH12PEN` writer - CH12 Pin Enable"]
pub type CH12PEN_W<'a, const O: u8> = crate::BitWriter<'a, u32, ROUTEPEN_SPEC, bool, O>;
#[doc = "Field `CH13PEN` reader - CH13 Pin Enable"]
pub type CH13PEN_R = crate::BitReader<bool>;
#[doc = "Field `CH13PEN` writer - CH13 Pin Enable"]
pub type CH13PEN_W<'a, const O: u8> = crate::BitWriter<'a, u32, ROUTEPEN_SPEC, bool, O>;
#[doc = "Field `CH14PEN` reader - CH14 Pin Enable"]
pub type CH14PEN_R = crate::BitReader<bool>;
#[doc = "Field `CH14PEN` writer - CH14 Pin Enable"]
pub type CH14PEN_W<'a, const O: u8> = crate::BitWriter<'a, u32, ROUTEPEN_SPEC, bool, O>;
#[doc = "Field `CH15PEN` reader - CH15 Pin Enable"]
pub type CH15PEN_R = crate::BitReader<bool>;
#[doc = "Field `CH15PEN` writer - CH15 Pin Enable"]
pub type CH15PEN_W<'a, const O: u8> = crate::BitWriter<'a, u32, ROUTEPEN_SPEC, bool, O>;
#[doc = "Field `ALTEX0PEN` reader - ALTEX0 Pin Enable"]
pub type ALTEX0PEN_R = crate::BitReader<bool>;
#[doc = "Field `ALTEX0PEN` writer - ALTEX0 Pin Enable"]
pub type ALTEX0PEN_W<'a, const O: u8> = crate::BitWriter<'a, u32, ROUTEPEN_SPEC, bool, O>;
#[doc = "Field `ALTEX1PEN` reader - ALTEX1 Pin Enable"]
pub type ALTEX1PEN_R = crate::BitReader<bool>;
#[doc = "Field `ALTEX1PEN` writer - ALTEX1 Pin Enable"]
pub type ALTEX1PEN_W<'a, const O: u8> = crate::BitWriter<'a, u32, ROUTEPEN_SPEC, bool, O>;
#[doc = "Field `ALTEX2PEN` reader - ALTEX2 Pin Enable"]
pub type ALTEX2PEN_R = crate::BitReader<bool>;
#[doc = "Field `ALTEX2PEN` writer - ALTEX2 Pin Enable"]
pub type ALTEX2PEN_W<'a, const O: u8> = crate::BitWriter<'a, u32, ROUTEPEN_SPEC, bool, O>;
#[doc = "Field `ALTEX3PEN` reader - ALTEX3 Pin Enable"]
pub type ALTEX3PEN_R = crate::BitReader<bool>;
#[doc = "Field `ALTEX3PEN` writer - ALTEX3 Pin Enable"]
pub type ALTEX3PEN_W<'a, const O: u8> = crate::BitWriter<'a, u32, ROUTEPEN_SPEC, bool, O>;
#[doc = "Field `ALTEX4PEN` reader - ALTEX4 Pin Enable"]
pub type ALTEX4PEN_R = crate::BitReader<bool>;
#[doc = "Field `ALTEX4PEN` writer - ALTEX4 Pin Enable"]
pub type ALTEX4PEN_W<'a, const O: u8> = crate::BitWriter<'a, u32, ROUTEPEN_SPEC, bool, O>;
#[doc = "Field `ALTEX5PEN` reader - ALTEX5 Pin Enable"]
pub type ALTEX5PEN_R = crate::BitReader<bool>;
#[doc = "Field `ALTEX5PEN` writer - ALTEX5 Pin Enable"]
pub type ALTEX5PEN_W<'a, const O: u8> = crate::BitWriter<'a, u32, ROUTEPEN_SPEC, bool, O>;
#[doc = "Field `ALTEX6PEN` reader - ALTEX6 Pin Enable"]
pub type ALTEX6PEN_R = crate::BitReader<bool>;
#[doc = "Field `ALTEX6PEN` writer - ALTEX6 Pin Enable"]
pub type ALTEX6PEN_W<'a, const O: u8> = crate::BitWriter<'a, u32, ROUTEPEN_SPEC, bool, O>;
#[doc = "Field `ALTEX7PEN` reader - ALTEX7 Pin Enable"]
pub type ALTEX7PEN_R = crate::BitReader<bool>;
#[doc = "Field `ALTEX7PEN` writer - ALTEX7 Pin Enable"]
pub type ALTEX7PEN_W<'a, const O: u8> = crate::BitWriter<'a, u32, ROUTEPEN_SPEC, bool, O>;
impl R {
    #[doc = "Bit 0 - CH0 Pin Enable"]
    #[inline(always)]
    pub fn ch0pen(&self) -> CH0PEN_R {
        CH0PEN_R::new((self.bits & 1) != 0)
    }
    #[doc = "Bit 1 - CH1 Pin Enable"]
    #[inline(always)]
    pub fn ch1pen(&self) -> CH1PEN_R {
        CH1PEN_R::new(((self.bits >> 1) & 1) != 0)
    }
    #[doc = "Bit 2 - CH2 Pin Enable"]
    #[inline(always)]
    pub fn ch2pen(&self) -> CH2PEN_R {
        CH2PEN_R::new(((self.bits >> 2) & 1) != 0)
    }
    #[doc = "Bit 3 - CH3 Pin Enable"]
    #[inline(always)]
    pub fn ch3pen(&self) -> CH3PEN_R {
        CH3PEN_R::new(((self.bits >> 3) & 1) != 0)
    }
    #[doc = "Bit 4 - CH4 Pin Enable"]
    #[inline(always)]
    pub fn ch4pen(&self) -> CH4PEN_R {
        CH4PEN_R::new(((self.bits >> 4) & 1) != 0)
    }
    #[doc = "Bit 5 - CH5 Pin Enable"]
    #[inline(always)]
    pub fn ch5pen(&self) -> CH5PEN_R {
        CH5PEN_R::new(((self.bits >> 5) & 1) != 0)
    }
    #[doc = "Bit 6 - CH6 Pin Enable"]
    #[inline(always)]
    pub fn ch6pen(&self) -> CH6PEN_R {
        CH6PEN_R::new(((self.bits >> 6) & 1) != 0)
    }
    #[doc = "Bit 7 - CH7 Pin Enable"]
    #[inline(always)]
    pub fn ch7pen(&self) -> CH7PEN_R {
        CH7PEN_R::new(((self.bits >> 7) & 1) != 0)
    }
    #[doc = "Bit 8 - CH8 Pin Enable"]
    #[inline(always)]
    pub fn ch8pen(&self) -> CH8PEN_R {
        CH8PEN_R::new(((self.bits >> 8) & 1) != 0)
    }
    #[doc = "Bit 9 - CH9 Pin Enable"]
    #[inline(always)]
    pub fn ch9pen(&self) -> CH9PEN_R {
        CH9PEN_R::new(((self.bits >> 9) & 1) != 0)
    }
    #[doc = "Bit 10 - CH10 Pin Enable"]
    #[inline(always)]
    pub fn ch10pen(&self) -> CH10PEN_R {
        CH10PEN_R::new(((self.bits >> 10) & 1) != 0)
    }
    #[doc = "Bit 11 - CH11 Pin Enable"]
    #[inline(always)]
    pub fn ch11pen(&self) -> CH11PEN_R {
        CH11PEN_R::new(((self.bits >> 11) & 1) != 0)
    }
    #[doc = "Bit 12 - CH12 Pin Enable"]
    #[inline(always)]
    pub fn ch12pen(&self) -> CH12PEN_R {
        CH12PEN_R::new(((self.bits >> 12) & 1) != 0)
    }
    #[doc = "Bit 13 - CH13 Pin Enable"]
    #[inline(always)]
    pub fn ch13pen(&self) -> CH13PEN_R {
        CH13PEN_R::new(((self.bits >> 13) & 1) != 0)
    }
    #[doc = "Bit 14 - CH14 Pin Enable"]
    #[inline(always)]
    pub fn ch14pen(&self) -> CH14PEN_R {
        CH14PEN_R::new(((self.bits >> 14) & 1) != 0)
    }
    #[doc = "Bit 15 - CH15 Pin Enable"]
    #[inline(always)]
    pub fn ch15pen(&self) -> CH15PEN_R {
        CH15PEN_R::new(((self.bits >> 15) & 1) != 0)
    }
    #[doc = "Bit 16 - ALTEX0 Pin Enable"]
    #[inline(always)]
    pub fn altex0pen(&self) -> ALTEX0PEN_R {
        ALTEX0PEN_R::new(((self.bits >> 16) & 1) != 0)
    }
    #[doc = "Bit 17 - ALTEX1 Pin Enable"]
    #[inline(always)]
    pub fn altex1pen(&self) -> ALTEX1PEN_R {
        ALTEX1PEN_R::new(((self.bits >> 17) & 1) != 0)
    }
    #[doc = "Bit 18 - ALTEX2 Pin Enable"]
    #[inline(always)]
    pub fn altex2pen(&self) -> ALTEX2PEN_R {
        ALTEX2PEN_R::new(((self.bits >> 18) & 1) != 0)
    }
    #[doc = "Bit 19 - ALTEX3 Pin Enable"]
    #[inline(always)]
    pub fn altex3pen(&self) -> ALTEX3PEN_R {
        ALTEX3PEN_R::new(((self.bits >> 19) & 1) != 0)
    }
    #[doc = "Bit 20 - ALTEX4 Pin Enable"]
    #[inline(always)]
    pub fn altex4pen(&self) -> ALTEX4PEN_R {
        ALTEX4PEN_R::new(((self.bits >> 20) & 1) != 0)
    }
    #[doc = "Bit 21 - ALTEX5 Pin Enable"]
    #[inline(always)]
    pub fn altex5pen(&self) -> ALTEX5PEN_R {
        ALTEX5PEN_R::new(((self.bits >> 21) & 1) != 0)
    }
    #[doc = "Bit 22 - ALTEX6 Pin Enable"]
    #[inline(always)]
    pub fn altex6pen(&self) -> ALTEX6PEN_R {
        ALTEX6PEN_R::new(((self.bits >> 22) & 1) != 0)
    }
    #[doc = "Bit 23 - ALTEX7 Pin Enable"]
    #[inline(always)]
    pub fn altex7pen(&self) -> ALTEX7PEN_R {
        ALTEX7PEN_R::new(((self.bits >> 23) & 1) != 0)
    }
}
impl W {
    #[doc = "Bit 0 - CH0 Pin Enable"]
    #[inline(always)]
    #[must_use]
    pub fn ch0pen(&mut self) -> CH0PEN_W<0> {
        CH0PEN_W::new(self)
    }
    #[doc = "Bit 1 - CH1 Pin Enable"]
    #[inline(always)]
    #[must_use]
    pub fn ch1pen(&mut self) -> CH1PEN_W<1> {
        CH1PEN_W::new(self)
    }
    #[doc = "Bit 2 - CH2 Pin Enable"]
    #[inline(always)]
    #[must_use]
    pub fn ch2pen(&mut self) -> CH2PEN_W<2> {
        CH2PEN_W::new(self)
    }
    #[doc = "Bit 3 - CH3 Pin Enable"]
    #[inline(always)]
    #[must_use]
    pub fn ch3pen(&mut self) -> CH3PEN_W<3> {
        CH3PEN_W::new(self)
    }
    #[doc = "Bit 4 - CH4 Pin Enable"]
    #[inline(always)]
    #[must_use]
    pub fn ch4pen(&mut self) -> CH4PEN_W<4> {
        CH4PEN_W::new(self)
    }
    #[doc = "Bit 5 - CH5 Pin Enable"]
    #[inline(always)]
    #[must_use]
    pub fn ch5pen(&mut self) -> CH5PEN_W<5> {
        CH5PEN_W::new(self)
    }
    #[doc = "Bit 6 - CH6 Pin Enable"]
    #[inline(always)]
    #[must_use]
    pub fn ch6pen(&mut self) -> CH6PEN_W<6> {
        CH6PEN_W::new(self)
    }
    #[doc = "Bit 7 - CH7 Pin Enable"]
    #[inline(always)]
    #[must_use]
    pub fn ch7pen(&mut self) -> CH7PEN_W<7> {
        CH7PEN_W::new(self)
    }
    #[doc = "Bit 8 - CH8 Pin Enable"]
    #[inline(always)]
    #[must_use]
    pub fn ch8pen(&mut self) -> CH8PEN_W<8> {
        CH8PEN_W::new(self)
    }
    #[doc = "Bit 9 - CH9 Pin Enable"]
    #[inline(always)]
    #[must_use]
    pub fn ch9pen(&mut self) -> CH9PEN_W<9> {
        CH9PEN_W::new(self)
    }
    #[doc = "Bit 10 - CH10 Pin Enable"]
    #[inline(always)]
    #[must_use]
    pub fn ch10pen(&mut self) -> CH10PEN_W<10> {
        CH10PEN_W::new(self)
    }
    #[doc = "Bit 11 - CH11 Pin Enable"]
    #[inline(always)]
    #[must_use]
    pub fn ch11pen(&mut self) -> CH11PEN_W<11> {
        CH11PEN_W::new(self)
    }
    #[doc = "Bit 12 - CH12 Pin Enable"]
    #[inline(always)]
    #[must_use]
    pub fn ch12pen(&mut self) -> CH12PEN_W<12> {
        CH12PEN_W::new(self)
    }
    #[doc = "Bit 13 - CH13 Pin Enable"]
    #[inline(always)]
    #[must_use]
    pub fn ch13pen(&mut self) -> CH13PEN_W<13> {
        CH13PEN_W::new(self)
    }
    #[doc = "Bit 14 - CH14 Pin Enable"]
    #[inline(always)]
    #[must_use]
    pub fn ch14pen(&mut self) -> CH14PEN_W<14> {
        CH14PEN_W::new(self)
    }
    #[doc = "Bit 15 - CH15 Pin Enable"]
    #[inline(always)]
    #[must_use]
    pub fn ch15pen(&mut self) -> CH15PEN_W<15> {
        CH15PEN_W::new(self)
    }
    #[doc = "Bit 16 - ALTEX0 Pin Enable"]
    #[inline(always)]
    #[must_use]
    pub fn altex0pen(&mut self) -> ALTEX0PEN_W<16> {
        ALTEX0PEN_W::new(self)
    }
    #[doc = "Bit 17 - ALTEX1 Pin Enable"]
    #[inline(always)]
    #[must_use]
    pub fn altex1pen(&mut self) -> ALTEX1PEN_W<17> {
        ALTEX1PEN_W::new(self)
    }
    #[doc = "Bit 18 - ALTEX2 Pin Enable"]
    #[inline(always)]
    #[must_use]
    pub fn altex2pen(&mut self) -> ALTEX2PEN_W<18> {
        ALTEX2PEN_W::new(self)
    }
    #[doc = "Bit 19 - ALTEX3 Pin Enable"]
    #[inline(always)]
    #[must_use]
    pub fn altex3pen(&mut self) -> ALTEX3PEN_W<19> {
        ALTEX3PEN_W::new(self)
    }
    #[doc = "Bit 20 - ALTEX4 Pin Enable"]
    #[inline(always)]
    #[must_use]
    pub fn altex4pen(&mut self) -> ALTEX4PEN_W<20> {
        ALTEX4PEN_W::new(self)
    }
    #[doc = "Bit 21 - ALTEX5 Pin Enable"]
    #[inline(always)]
    #[must_use]
    pub fn altex5pen(&mut self) -> ALTEX5PEN_W<21> {
        ALTEX5PEN_W::new(self)
    }
    #[doc = "Bit 22 - ALTEX6 Pin Enable"]
    #[inline(always)]
    #[must_use]
    pub fn altex6pen(&mut self) -> ALTEX6PEN_W<22> {
        ALTEX6PEN_W::new(self)
    }
    #[doc = "Bit 23 - ALTEX7 Pin Enable"]
    #[inline(always)]
    #[must_use]
    pub fn altex7pen(&mut self) -> ALTEX7PEN_W<23> {
        ALTEX7PEN_W::new(self)
    }
    #[doc = "Writes raw bits to the register."]
    #[inline(always)]
    pub unsafe fn bits(&mut self, bits: u32) -> &mut Self {
        self.0.bits(bits);
        self
    }
}
#[doc = "I/O Routing Register\n\nThis register you can [`read`](crate::generic::Reg::read), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [routepen](index.html) module"]
pub struct ROUTEPEN_SPEC;
impl crate::RegisterSpec for ROUTEPEN_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [routepen::R](R) reader structure"]
impl crate::Readable for ROUTEPEN_SPEC {
    type Reader = R;
}
#[doc = "`write(|w| ..)` method takes [routepen::W](W) writer structure"]
impl crate::Writable for ROUTEPEN_SPEC {
    type Writer = W;
    const ZERO_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
}
#[doc = "`reset()` method sets ROUTEPEN to value 0"]
impl crate::Resettable for ROUTEPEN_SPEC {
    const RESET_VALUE: Self::Ux = 0;
}
