#[doc = "Register `PEEK` reader"]
pub struct R(crate::R<PEEK_SPEC>);
impl core::ops::Deref for R {
    type Target = crate::R<PEEK_SPEC>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
impl From<crate::R<PEEK_SPEC>> for R {
    #[inline(always)]
    fn from(reader: crate::R<PEEK_SPEC>) -> Self {
        R(reader)
    }
}
#[doc = "Field `CH0VAL` reader - Channel 0 Current Value"]
pub type CH0VAL_R = crate::BitReader<bool>;
#[doc = "Field `CH1VAL` reader - Channel 1 Current Value"]
pub type CH1VAL_R = crate::BitReader<bool>;
#[doc = "Field `CH2VAL` reader - Channel 2 Current Value"]
pub type CH2VAL_R = crate::BitReader<bool>;
#[doc = "Field `CH3VAL` reader - Channel 3 Current Value"]
pub type CH3VAL_R = crate::BitReader<bool>;
#[doc = "Field `CH4VAL` reader - Channel 4 Current Value"]
pub type CH4VAL_R = crate::BitReader<bool>;
#[doc = "Field `CH5VAL` reader - Channel 5 Current Value"]
pub type CH5VAL_R = crate::BitReader<bool>;
#[doc = "Field `CH6VAL` reader - Channel 6 Current Value"]
pub type CH6VAL_R = crate::BitReader<bool>;
#[doc = "Field `CH7VAL` reader - Channel 7 Current Value"]
pub type CH7VAL_R = crate::BitReader<bool>;
impl R {
    #[doc = "Bit 0 - Channel 0 Current Value"]
    #[inline(always)]
    pub fn ch0val(&self) -> CH0VAL_R {
        CH0VAL_R::new((self.bits & 1) != 0)
    }
    #[doc = "Bit 1 - Channel 1 Current Value"]
    #[inline(always)]
    pub fn ch1val(&self) -> CH1VAL_R {
        CH1VAL_R::new(((self.bits >> 1) & 1) != 0)
    }
    #[doc = "Bit 2 - Channel 2 Current Value"]
    #[inline(always)]
    pub fn ch2val(&self) -> CH2VAL_R {
        CH2VAL_R::new(((self.bits >> 2) & 1) != 0)
    }
    #[doc = "Bit 3 - Channel 3 Current Value"]
    #[inline(always)]
    pub fn ch3val(&self) -> CH3VAL_R {
        CH3VAL_R::new(((self.bits >> 3) & 1) != 0)
    }
    #[doc = "Bit 4 - Channel 4 Current Value"]
    #[inline(always)]
    pub fn ch4val(&self) -> CH4VAL_R {
        CH4VAL_R::new(((self.bits >> 4) & 1) != 0)
    }
    #[doc = "Bit 5 - Channel 5 Current Value"]
    #[inline(always)]
    pub fn ch5val(&self) -> CH5VAL_R {
        CH5VAL_R::new(((self.bits >> 5) & 1) != 0)
    }
    #[doc = "Bit 6 - Channel 6 Current Value"]
    #[inline(always)]
    pub fn ch6val(&self) -> CH6VAL_R {
        CH6VAL_R::new(((self.bits >> 6) & 1) != 0)
    }
    #[doc = "Bit 7 - Channel 7 Current Value"]
    #[inline(always)]
    pub fn ch7val(&self) -> CH7VAL_R {
        CH7VAL_R::new(((self.bits >> 7) & 1) != 0)
    }
}
#[doc = "PRS Channel Values\n\nThis register you can [`read`](crate::generic::Reg::read). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [peek](index.html) module"]
pub struct PEEK_SPEC;
impl crate::RegisterSpec for PEEK_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [peek::R](R) reader structure"]
impl crate::Readable for PEEK_SPEC {
    type Reader = R;
}
#[doc = "`reset()` method sets PEEK to value 0"]
impl crate::Resettable for PEEK_SPEC {
    #[inline(always)]
    fn reset_value() -> Self::Ux {
        0
    }
}
