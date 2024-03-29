#[doc = "Register `CHUSEBURSTS` reader"]
pub struct R(crate::R<CHUSEBURSTS_SPEC>);
impl core::ops::Deref for R {
    type Target = crate::R<CHUSEBURSTS_SPEC>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
impl From<crate::R<CHUSEBURSTS_SPEC>> for R {
    #[inline(always)]
    fn from(reader: crate::R<CHUSEBURSTS_SPEC>) -> Self {
        R(reader)
    }
}
#[doc = "Register `CHUSEBURSTS` writer"]
pub struct W(crate::W<CHUSEBURSTS_SPEC>);
impl core::ops::Deref for W {
    type Target = crate::W<CHUSEBURSTS_SPEC>;
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
impl From<crate::W<CHUSEBURSTS_SPEC>> for W {
    #[inline(always)]
    fn from(writer: crate::W<CHUSEBURSTS_SPEC>) -> Self {
        W(writer)
    }
}
#[doc = "Field `CH0USEBURSTS` reader - Channel 0 Useburst Set"]
pub type CH0USEBURSTS_R = crate::BitReader<bool>;
#[doc = "Field `CH0USEBURSTS` writer - Channel 0 Useburst Set"]
pub type CH0USEBURSTS_W<'a, const O: u8> = crate::BitWriter<'a, u32, CHUSEBURSTS_SPEC, bool, O>;
#[doc = "Field `CH1USEBURSTS` reader - Channel 1 Useburst Set"]
pub type CH1USEBURSTS_R = crate::BitReader<bool>;
#[doc = "Field `CH1USEBURSTS` writer - Channel 1 Useburst Set"]
pub type CH1USEBURSTS_W<'a, const O: u8> = crate::BitWriter<'a, u32, CHUSEBURSTS_SPEC, bool, O>;
#[doc = "Field `CH2USEBURSTS` reader - Channel 2 Useburst Set"]
pub type CH2USEBURSTS_R = crate::BitReader<bool>;
#[doc = "Field `CH2USEBURSTS` writer - Channel 2 Useburst Set"]
pub type CH2USEBURSTS_W<'a, const O: u8> = crate::BitWriter<'a, u32, CHUSEBURSTS_SPEC, bool, O>;
#[doc = "Field `CH3USEBURSTS` reader - Channel 3 Useburst Set"]
pub type CH3USEBURSTS_R = crate::BitReader<bool>;
#[doc = "Field `CH3USEBURSTS` writer - Channel 3 Useburst Set"]
pub type CH3USEBURSTS_W<'a, const O: u8> = crate::BitWriter<'a, u32, CHUSEBURSTS_SPEC, bool, O>;
#[doc = "Field `CH4USEBURSTS` reader - Channel 4 Useburst Set"]
pub type CH4USEBURSTS_R = crate::BitReader<bool>;
#[doc = "Field `CH4USEBURSTS` writer - Channel 4 Useburst Set"]
pub type CH4USEBURSTS_W<'a, const O: u8> = crate::BitWriter<'a, u32, CHUSEBURSTS_SPEC, bool, O>;
#[doc = "Field `CH5USEBURSTS` reader - Channel 5 Useburst Set"]
pub type CH5USEBURSTS_R = crate::BitReader<bool>;
#[doc = "Field `CH5USEBURSTS` writer - Channel 5 Useburst Set"]
pub type CH5USEBURSTS_W<'a, const O: u8> = crate::BitWriter<'a, u32, CHUSEBURSTS_SPEC, bool, O>;
#[doc = "Field `CH6USEBURSTS` reader - Channel 6 Useburst Set"]
pub type CH6USEBURSTS_R = crate::BitReader<bool>;
#[doc = "Field `CH6USEBURSTS` writer - Channel 6 Useburst Set"]
pub type CH6USEBURSTS_W<'a, const O: u8> = crate::BitWriter<'a, u32, CHUSEBURSTS_SPEC, bool, O>;
#[doc = "Field `CH7USEBURSTS` reader - Channel 7 Useburst Set"]
pub type CH7USEBURSTS_R = crate::BitReader<bool>;
#[doc = "Field `CH7USEBURSTS` writer - Channel 7 Useburst Set"]
pub type CH7USEBURSTS_W<'a, const O: u8> = crate::BitWriter<'a, u32, CHUSEBURSTS_SPEC, bool, O>;
impl R {
    #[doc = "Bit 0 - Channel 0 Useburst Set"]
    #[inline(always)]
    pub fn ch0usebursts(&self) -> CH0USEBURSTS_R {
        CH0USEBURSTS_R::new((self.bits & 1) != 0)
    }
    #[doc = "Bit 1 - Channel 1 Useburst Set"]
    #[inline(always)]
    pub fn ch1usebursts(&self) -> CH1USEBURSTS_R {
        CH1USEBURSTS_R::new(((self.bits >> 1) & 1) != 0)
    }
    #[doc = "Bit 2 - Channel 2 Useburst Set"]
    #[inline(always)]
    pub fn ch2usebursts(&self) -> CH2USEBURSTS_R {
        CH2USEBURSTS_R::new(((self.bits >> 2) & 1) != 0)
    }
    #[doc = "Bit 3 - Channel 3 Useburst Set"]
    #[inline(always)]
    pub fn ch3usebursts(&self) -> CH3USEBURSTS_R {
        CH3USEBURSTS_R::new(((self.bits >> 3) & 1) != 0)
    }
    #[doc = "Bit 4 - Channel 4 Useburst Set"]
    #[inline(always)]
    pub fn ch4usebursts(&self) -> CH4USEBURSTS_R {
        CH4USEBURSTS_R::new(((self.bits >> 4) & 1) != 0)
    }
    #[doc = "Bit 5 - Channel 5 Useburst Set"]
    #[inline(always)]
    pub fn ch5usebursts(&self) -> CH5USEBURSTS_R {
        CH5USEBURSTS_R::new(((self.bits >> 5) & 1) != 0)
    }
    #[doc = "Bit 6 - Channel 6 Useburst Set"]
    #[inline(always)]
    pub fn ch6usebursts(&self) -> CH6USEBURSTS_R {
        CH6USEBURSTS_R::new(((self.bits >> 6) & 1) != 0)
    }
    #[doc = "Bit 7 - Channel 7 Useburst Set"]
    #[inline(always)]
    pub fn ch7usebursts(&self) -> CH7USEBURSTS_R {
        CH7USEBURSTS_R::new(((self.bits >> 7) & 1) != 0)
    }
}
impl W {
    #[doc = "Bit 0 - Channel 0 Useburst Set"]
    #[inline(always)]
    #[must_use]
    pub fn ch0usebursts(&mut self) -> CH0USEBURSTS_W<0> {
        CH0USEBURSTS_W::new(self)
    }
    #[doc = "Bit 1 - Channel 1 Useburst Set"]
    #[inline(always)]
    #[must_use]
    pub fn ch1usebursts(&mut self) -> CH1USEBURSTS_W<1> {
        CH1USEBURSTS_W::new(self)
    }
    #[doc = "Bit 2 - Channel 2 Useburst Set"]
    #[inline(always)]
    #[must_use]
    pub fn ch2usebursts(&mut self) -> CH2USEBURSTS_W<2> {
        CH2USEBURSTS_W::new(self)
    }
    #[doc = "Bit 3 - Channel 3 Useburst Set"]
    #[inline(always)]
    #[must_use]
    pub fn ch3usebursts(&mut self) -> CH3USEBURSTS_W<3> {
        CH3USEBURSTS_W::new(self)
    }
    #[doc = "Bit 4 - Channel 4 Useburst Set"]
    #[inline(always)]
    #[must_use]
    pub fn ch4usebursts(&mut self) -> CH4USEBURSTS_W<4> {
        CH4USEBURSTS_W::new(self)
    }
    #[doc = "Bit 5 - Channel 5 Useburst Set"]
    #[inline(always)]
    #[must_use]
    pub fn ch5usebursts(&mut self) -> CH5USEBURSTS_W<5> {
        CH5USEBURSTS_W::new(self)
    }
    #[doc = "Bit 6 - Channel 6 Useburst Set"]
    #[inline(always)]
    #[must_use]
    pub fn ch6usebursts(&mut self) -> CH6USEBURSTS_W<6> {
        CH6USEBURSTS_W::new(self)
    }
    #[doc = "Bit 7 - Channel 7 Useburst Set"]
    #[inline(always)]
    #[must_use]
    pub fn ch7usebursts(&mut self) -> CH7USEBURSTS_W<7> {
        CH7USEBURSTS_W::new(self)
    }
    #[doc = "Writes raw bits to the register."]
    #[inline(always)]
    pub unsafe fn bits(&mut self, bits: u32) -> &mut Self {
        self.0.bits(bits);
        self
    }
}
#[doc = "Channel Useburst Set Register\n\nThis register you can [`read`](crate::generic::Reg::read), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [chusebursts](index.html) module"]
pub struct CHUSEBURSTS_SPEC;
impl crate::RegisterSpec for CHUSEBURSTS_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [chusebursts::R](R) reader structure"]
impl crate::Readable for CHUSEBURSTS_SPEC {
    type Reader = R;
}
#[doc = "`write(|w| ..)` method takes [chusebursts::W](W) writer structure"]
impl crate::Writable for CHUSEBURSTS_SPEC {
    type Writer = W;
    const ZERO_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
}
#[doc = "`reset()` method sets CHUSEBURSTS to value 0"]
impl crate::Resettable for CHUSEBURSTS_SPEC {
    const RESET_VALUE: Self::Ux = 0;
}
