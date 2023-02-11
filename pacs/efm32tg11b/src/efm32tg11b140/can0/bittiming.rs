#[doc = "Register `BITTIMING` reader"]
pub struct R(crate::R<BITTIMING_SPEC>);
impl core::ops::Deref for R {
    type Target = crate::R<BITTIMING_SPEC>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
impl From<crate::R<BITTIMING_SPEC>> for R {
    #[inline(always)]
    fn from(reader: crate::R<BITTIMING_SPEC>) -> Self {
        R(reader)
    }
}
#[doc = "Register `BITTIMING` writer"]
pub struct W(crate::W<BITTIMING_SPEC>);
impl core::ops::Deref for W {
    type Target = crate::W<BITTIMING_SPEC>;
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
impl From<crate::W<BITTIMING_SPEC>> for W {
    #[inline(always)]
    fn from(writer: crate::W<BITTIMING_SPEC>) -> Self {
        W(writer)
    }
}
#[doc = "Field `BRP` reader - Baud Rate Prescaler"]
pub type BRP_R = crate::FieldReader<u8, u8>;
#[doc = "Field `BRP` writer - Baud Rate Prescaler"]
pub type BRP_W<'a, const O: u8> = crate::FieldWriter<'a, u32, BITTIMING_SPEC, u8, u8, 6, O>;
#[doc = "Field `SJW` reader - Synchronization Jump Width"]
pub type SJW_R = crate::FieldReader<u8, u8>;
#[doc = "Field `SJW` writer - Synchronization Jump Width"]
pub type SJW_W<'a, const O: u8> = crate::FieldWriter<'a, u32, BITTIMING_SPEC, u8, u8, 2, O>;
#[doc = "Field `TSEG1` reader - Time Segment Before the Sample Point"]
pub type TSEG1_R = crate::FieldReader<u8, u8>;
#[doc = "Field `TSEG1` writer - Time Segment Before the Sample Point"]
pub type TSEG1_W<'a, const O: u8> = crate::FieldWriter<'a, u32, BITTIMING_SPEC, u8, u8, 4, O>;
#[doc = "Field `TSEG2` reader - Time Segment After the Sample Point"]
pub type TSEG2_R = crate::FieldReader<u8, u8>;
#[doc = "Field `TSEG2` writer - Time Segment After the Sample Point"]
pub type TSEG2_W<'a, const O: u8> = crate::FieldWriter<'a, u32, BITTIMING_SPEC, u8, u8, 3, O>;
impl R {
    #[doc = "Bits 0:5 - Baud Rate Prescaler"]
    #[inline(always)]
    pub fn brp(&self) -> BRP_R {
        BRP_R::new((self.bits & 0x3f) as u8)
    }
    #[doc = "Bits 6:7 - Synchronization Jump Width"]
    #[inline(always)]
    pub fn sjw(&self) -> SJW_R {
        SJW_R::new(((self.bits >> 6) & 3) as u8)
    }
    #[doc = "Bits 8:11 - Time Segment Before the Sample Point"]
    #[inline(always)]
    pub fn tseg1(&self) -> TSEG1_R {
        TSEG1_R::new(((self.bits >> 8) & 0x0f) as u8)
    }
    #[doc = "Bits 12:14 - Time Segment After the Sample Point"]
    #[inline(always)]
    pub fn tseg2(&self) -> TSEG2_R {
        TSEG2_R::new(((self.bits >> 12) & 7) as u8)
    }
}
impl W {
    #[doc = "Bits 0:5 - Baud Rate Prescaler"]
    #[inline(always)]
    #[must_use]
    pub fn brp(&mut self) -> BRP_W<0> {
        BRP_W::new(self)
    }
    #[doc = "Bits 6:7 - Synchronization Jump Width"]
    #[inline(always)]
    #[must_use]
    pub fn sjw(&mut self) -> SJW_W<6> {
        SJW_W::new(self)
    }
    #[doc = "Bits 8:11 - Time Segment Before the Sample Point"]
    #[inline(always)]
    #[must_use]
    pub fn tseg1(&mut self) -> TSEG1_W<8> {
        TSEG1_W::new(self)
    }
    #[doc = "Bits 12:14 - Time Segment After the Sample Point"]
    #[inline(always)]
    #[must_use]
    pub fn tseg2(&mut self) -> TSEG2_W<12> {
        TSEG2_W::new(self)
    }
    #[doc = "Writes raw bits to the register."]
    #[inline(always)]
    pub unsafe fn bits(&mut self, bits: u32) -> &mut Self {
        self.0.bits(bits);
        self
    }
}
#[doc = "Bit Timing Register\n\nThis register you can [`read`](crate::generic::Reg::read), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [bittiming](index.html) module"]
pub struct BITTIMING_SPEC;
impl crate::RegisterSpec for BITTIMING_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [bittiming::R](R) reader structure"]
impl crate::Readable for BITTIMING_SPEC {
    type Reader = R;
}
#[doc = "`write(|w| ..)` method takes [bittiming::W](W) writer structure"]
impl crate::Writable for BITTIMING_SPEC {
    type Writer = W;
    const ZERO_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
}
#[doc = "`reset()` method sets BITTIMING to value 0x2301"]
impl crate::Resettable for BITTIMING_SPEC {
    const RESET_VALUE: Self::Ux = 0x2301;
}
