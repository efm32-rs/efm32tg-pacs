#[doc = "Register `DTCTRL` reader"]
pub struct R(crate::R<DTCTRL_SPEC>);
impl core::ops::Deref for R {
    type Target = crate::R<DTCTRL_SPEC>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
impl From<crate::R<DTCTRL_SPEC>> for R {
    #[inline(always)]
    fn from(reader: crate::R<DTCTRL_SPEC>) -> Self {
        R(reader)
    }
}
#[doc = "Register `DTCTRL` writer"]
pub struct W(crate::W<DTCTRL_SPEC>);
impl core::ops::Deref for W {
    type Target = crate::W<DTCTRL_SPEC>;
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
impl From<crate::W<DTCTRL_SPEC>> for W {
    #[inline(always)]
    fn from(writer: crate::W<DTCTRL_SPEC>) -> Self {
        W(writer)
    }
}
#[doc = "Field `DTEN` reader - DTI Enable"]
pub type DTEN_R = crate::BitReader<bool>;
#[doc = "Field `DTEN` writer - DTI Enable"]
pub type DTEN_W<'a> = crate::BitWriter<'a, u32, DTCTRL_SPEC, bool, 0>;
#[doc = "Field `DTDAS` reader - DTI Automatic Start-up Functionality"]
pub type DTDAS_R = crate::BitReader<bool>;
#[doc = "Field `DTDAS` writer - DTI Automatic Start-up Functionality"]
pub type DTDAS_W<'a> = crate::BitWriter<'a, u32, DTCTRL_SPEC, bool, 1>;
#[doc = "Field `DTIPOL` reader - DTI Inactive Polarity"]
pub type DTIPOL_R = crate::BitReader<bool>;
#[doc = "Field `DTIPOL` writer - DTI Inactive Polarity"]
pub type DTIPOL_W<'a> = crate::BitWriter<'a, u32, DTCTRL_SPEC, bool, 2>;
#[doc = "Field `DTCINV` reader - DTI Complementary Output Invert"]
pub type DTCINV_R = crate::BitReader<bool>;
#[doc = "Field `DTCINV` writer - DTI Complementary Output Invert"]
pub type DTCINV_W<'a> = crate::BitWriter<'a, u32, DTCTRL_SPEC, bool, 3>;
#[doc = "DTI PRS Source Channel Select\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq)]
#[repr(u8)]
pub enum DTPRSSEL_A {
    #[doc = "0: PRS Channel 0 selected as input"]
    PRSCH0 = 0,
    #[doc = "1: PRS Channel 1 selected as input"]
    PRSCH1 = 1,
    #[doc = "2: PRS Channel 2 selected as input"]
    PRSCH2 = 2,
    #[doc = "3: PRS Channel 3 selected as input"]
    PRSCH3 = 3,
    #[doc = "4: PRS Channel 4 selected as input"]
    PRSCH4 = 4,
    #[doc = "5: PRS Channel 5 selected as input"]
    PRSCH5 = 5,
    #[doc = "6: PRS Channel 6 selected as input"]
    PRSCH6 = 6,
    #[doc = "7: PRS Channel 7 selected as input"]
    PRSCH7 = 7,
}
impl From<DTPRSSEL_A> for u8 {
    #[inline(always)]
    fn from(variant: DTPRSSEL_A) -> Self {
        variant as _
    }
}
#[doc = "Field `DTPRSSEL` reader - DTI PRS Source Channel Select"]
pub type DTPRSSEL_R = crate::FieldReader<u8, DTPRSSEL_A>;
impl DTPRSSEL_R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> DTPRSSEL_A {
        match self.bits {
            0 => DTPRSSEL_A::PRSCH0,
            1 => DTPRSSEL_A::PRSCH1,
            2 => DTPRSSEL_A::PRSCH2,
            3 => DTPRSSEL_A::PRSCH3,
            4 => DTPRSSEL_A::PRSCH4,
            5 => DTPRSSEL_A::PRSCH5,
            6 => DTPRSSEL_A::PRSCH6,
            7 => DTPRSSEL_A::PRSCH7,
            _ => unreachable!(),
        }
    }
    #[doc = "Checks if the value of the field is `PRSCH0`"]
    #[inline(always)]
    pub fn is_prsch0(&self) -> bool {
        *self == DTPRSSEL_A::PRSCH0
    }
    #[doc = "Checks if the value of the field is `PRSCH1`"]
    #[inline(always)]
    pub fn is_prsch1(&self) -> bool {
        *self == DTPRSSEL_A::PRSCH1
    }
    #[doc = "Checks if the value of the field is `PRSCH2`"]
    #[inline(always)]
    pub fn is_prsch2(&self) -> bool {
        *self == DTPRSSEL_A::PRSCH2
    }
    #[doc = "Checks if the value of the field is `PRSCH3`"]
    #[inline(always)]
    pub fn is_prsch3(&self) -> bool {
        *self == DTPRSSEL_A::PRSCH3
    }
    #[doc = "Checks if the value of the field is `PRSCH4`"]
    #[inline(always)]
    pub fn is_prsch4(&self) -> bool {
        *self == DTPRSSEL_A::PRSCH4
    }
    #[doc = "Checks if the value of the field is `PRSCH5`"]
    #[inline(always)]
    pub fn is_prsch5(&self) -> bool {
        *self == DTPRSSEL_A::PRSCH5
    }
    #[doc = "Checks if the value of the field is `PRSCH6`"]
    #[inline(always)]
    pub fn is_prsch6(&self) -> bool {
        *self == DTPRSSEL_A::PRSCH6
    }
    #[doc = "Checks if the value of the field is `PRSCH7`"]
    #[inline(always)]
    pub fn is_prsch7(&self) -> bool {
        *self == DTPRSSEL_A::PRSCH7
    }
}
#[doc = "Field `DTPRSSEL` writer - DTI PRS Source Channel Select"]
pub type DTPRSSEL_W<'a> = crate::FieldWriterSafe<'a, u32, DTCTRL_SPEC, u8, DTPRSSEL_A, 3, 4>;
impl<'a> DTPRSSEL_W<'a> {
    #[doc = "PRS Channel 0 selected as input"]
    #[inline(always)]
    pub fn prsch0(self) -> &'a mut W {
        self.variant(DTPRSSEL_A::PRSCH0)
    }
    #[doc = "PRS Channel 1 selected as input"]
    #[inline(always)]
    pub fn prsch1(self) -> &'a mut W {
        self.variant(DTPRSSEL_A::PRSCH1)
    }
    #[doc = "PRS Channel 2 selected as input"]
    #[inline(always)]
    pub fn prsch2(self) -> &'a mut W {
        self.variant(DTPRSSEL_A::PRSCH2)
    }
    #[doc = "PRS Channel 3 selected as input"]
    #[inline(always)]
    pub fn prsch3(self) -> &'a mut W {
        self.variant(DTPRSSEL_A::PRSCH3)
    }
    #[doc = "PRS Channel 4 selected as input"]
    #[inline(always)]
    pub fn prsch4(self) -> &'a mut W {
        self.variant(DTPRSSEL_A::PRSCH4)
    }
    #[doc = "PRS Channel 5 selected as input"]
    #[inline(always)]
    pub fn prsch5(self) -> &'a mut W {
        self.variant(DTPRSSEL_A::PRSCH5)
    }
    #[doc = "PRS Channel 6 selected as input"]
    #[inline(always)]
    pub fn prsch6(self) -> &'a mut W {
        self.variant(DTPRSSEL_A::PRSCH6)
    }
    #[doc = "PRS Channel 7 selected as input"]
    #[inline(always)]
    pub fn prsch7(self) -> &'a mut W {
        self.variant(DTPRSSEL_A::PRSCH7)
    }
}
#[doc = "Field `DTAR` reader - DTI Always Run"]
pub type DTAR_R = crate::BitReader<bool>;
#[doc = "Field `DTAR` writer - DTI Always Run"]
pub type DTAR_W<'a> = crate::BitWriter<'a, u32, DTCTRL_SPEC, bool, 9>;
#[doc = "Field `DTFATS` reader - DTI Fault Action on Timer Stop"]
pub type DTFATS_R = crate::BitReader<bool>;
#[doc = "Field `DTFATS` writer - DTI Fault Action on Timer Stop"]
pub type DTFATS_W<'a> = crate::BitWriter<'a, u32, DTCTRL_SPEC, bool, 10>;
#[doc = "Field `DTPRSEN` reader - DTI PRS Source Enable"]
pub type DTPRSEN_R = crate::BitReader<bool>;
#[doc = "Field `DTPRSEN` writer - DTI PRS Source Enable"]
pub type DTPRSEN_W<'a> = crate::BitWriter<'a, u32, DTCTRL_SPEC, bool, 24>;
impl R {
    #[doc = "Bit 0 - DTI Enable"]
    #[inline(always)]
    pub fn dten(&self) -> DTEN_R {
        DTEN_R::new((self.bits & 1) != 0)
    }
    #[doc = "Bit 1 - DTI Automatic Start-up Functionality"]
    #[inline(always)]
    pub fn dtdas(&self) -> DTDAS_R {
        DTDAS_R::new(((self.bits >> 1) & 1) != 0)
    }
    #[doc = "Bit 2 - DTI Inactive Polarity"]
    #[inline(always)]
    pub fn dtipol(&self) -> DTIPOL_R {
        DTIPOL_R::new(((self.bits >> 2) & 1) != 0)
    }
    #[doc = "Bit 3 - DTI Complementary Output Invert"]
    #[inline(always)]
    pub fn dtcinv(&self) -> DTCINV_R {
        DTCINV_R::new(((self.bits >> 3) & 1) != 0)
    }
    #[doc = "Bits 4:6 - DTI PRS Source Channel Select"]
    #[inline(always)]
    pub fn dtprssel(&self) -> DTPRSSEL_R {
        DTPRSSEL_R::new(((self.bits >> 4) & 7) as u8)
    }
    #[doc = "Bit 9 - DTI Always Run"]
    #[inline(always)]
    pub fn dtar(&self) -> DTAR_R {
        DTAR_R::new(((self.bits >> 9) & 1) != 0)
    }
    #[doc = "Bit 10 - DTI Fault Action on Timer Stop"]
    #[inline(always)]
    pub fn dtfats(&self) -> DTFATS_R {
        DTFATS_R::new(((self.bits >> 10) & 1) != 0)
    }
    #[doc = "Bit 24 - DTI PRS Source Enable"]
    #[inline(always)]
    pub fn dtprsen(&self) -> DTPRSEN_R {
        DTPRSEN_R::new(((self.bits >> 24) & 1) != 0)
    }
}
impl W {
    #[doc = "Bit 0 - DTI Enable"]
    #[inline(always)]
    pub fn dten(&mut self) -> DTEN_W {
        DTEN_W::new(self)
    }
    #[doc = "Bit 1 - DTI Automatic Start-up Functionality"]
    #[inline(always)]
    pub fn dtdas(&mut self) -> DTDAS_W {
        DTDAS_W::new(self)
    }
    #[doc = "Bit 2 - DTI Inactive Polarity"]
    #[inline(always)]
    pub fn dtipol(&mut self) -> DTIPOL_W {
        DTIPOL_W::new(self)
    }
    #[doc = "Bit 3 - DTI Complementary Output Invert"]
    #[inline(always)]
    pub fn dtcinv(&mut self) -> DTCINV_W {
        DTCINV_W::new(self)
    }
    #[doc = "Bits 4:6 - DTI PRS Source Channel Select"]
    #[inline(always)]
    pub fn dtprssel(&mut self) -> DTPRSSEL_W {
        DTPRSSEL_W::new(self)
    }
    #[doc = "Bit 9 - DTI Always Run"]
    #[inline(always)]
    pub fn dtar(&mut self) -> DTAR_W {
        DTAR_W::new(self)
    }
    #[doc = "Bit 10 - DTI Fault Action on Timer Stop"]
    #[inline(always)]
    pub fn dtfats(&mut self) -> DTFATS_W {
        DTFATS_W::new(self)
    }
    #[doc = "Bit 24 - DTI PRS Source Enable"]
    #[inline(always)]
    pub fn dtprsen(&mut self) -> DTPRSEN_W {
        DTPRSEN_W::new(self)
    }
    #[doc = "Writes raw bits to the register."]
    #[inline(always)]
    pub unsafe fn bits(&mut self, bits: u32) -> &mut Self {
        self.0.bits(bits);
        self
    }
}
#[doc = "DTI Control Register\n\nThis register you can [`read`](crate::generic::Reg::read), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [dtctrl](index.html) module"]
pub struct DTCTRL_SPEC;
impl crate::RegisterSpec for DTCTRL_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [dtctrl::R](R) reader structure"]
impl crate::Readable for DTCTRL_SPEC {
    type Reader = R;
}
#[doc = "`write(|w| ..)` method takes [dtctrl::W](W) writer structure"]
impl crate::Writable for DTCTRL_SPEC {
    type Writer = W;
}
#[doc = "`reset()` method sets DTCTRL to value 0"]
impl crate::Resettable for DTCTRL_SPEC {
    #[inline(always)]
    fn reset_value() -> Self::Ux {
        0
    }
}
