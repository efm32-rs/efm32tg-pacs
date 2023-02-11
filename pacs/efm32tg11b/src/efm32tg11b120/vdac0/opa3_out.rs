#[doc = "Register `OPA3_OUT` reader"]
pub struct R(crate::R<OPA3_OUT_SPEC>);
impl core::ops::Deref for R {
    type Target = crate::R<OPA3_OUT_SPEC>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
impl From<crate::R<OPA3_OUT_SPEC>> for R {
    #[inline(always)]
    fn from(reader: crate::R<OPA3_OUT_SPEC>) -> Self {
        R(reader)
    }
}
#[doc = "Register `OPA3_OUT` writer"]
pub struct W(crate::W<OPA3_OUT_SPEC>);
impl core::ops::Deref for W {
    type Target = crate::W<OPA3_OUT_SPEC>;
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
impl From<crate::W<OPA3_OUT_SPEC>> for W {
    #[inline(always)]
    fn from(writer: crate::W<OPA3_OUT_SPEC>) -> Self {
        W(writer)
    }
}
#[doc = "Field `MAINOUTEN` reader - OPAx Main Output Enable"]
pub type MAINOUTEN_R = crate::BitReader<bool>;
#[doc = "Field `MAINOUTEN` writer - OPAx Main Output Enable"]
pub type MAINOUTEN_W<'a, const O: u8> = crate::BitWriter<'a, u32, OPA3_OUT_SPEC, bool, O>;
#[doc = "Field `ALTOUTEN` reader - OPAx Alternative Output Enable"]
pub type ALTOUTEN_R = crate::BitReader<bool>;
#[doc = "Field `ALTOUTEN` writer - OPAx Alternative Output Enable"]
pub type ALTOUTEN_W<'a, const O: u8> = crate::BitWriter<'a, u32, OPA3_OUT_SPEC, bool, O>;
#[doc = "Field `APORTOUTEN` reader - OPAx Aport Output Enable"]
pub type APORTOUTEN_R = crate::BitReader<bool>;
#[doc = "Field `APORTOUTEN` writer - OPAx Aport Output Enable"]
pub type APORTOUTEN_W<'a, const O: u8> = crate::BitWriter<'a, u32, OPA3_OUT_SPEC, bool, O>;
#[doc = "Field `SHORT` reader - OPAx Main and Alternative Output Short"]
pub type SHORT_R = crate::BitReader<bool>;
#[doc = "Field `SHORT` writer - OPAx Main and Alternative Output Short"]
pub type SHORT_W<'a, const O: u8> = crate::BitWriter<'a, u32, OPA3_OUT_SPEC, bool, O>;
#[doc = "Field `ALTOUTPADEN` reader - OPAx Output Enable Value"]
pub type ALTOUTPADEN_R = crate::FieldReader<u8, ALTOUTPADEN_A>;
#[doc = "OPAx Output Enable Value\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
#[repr(u8)]
pub enum ALTOUTPADEN_A {
    #[doc = "1: Alternate Output 0"]
    OUT0 = 1,
    #[doc = "2: Alternate Output 1"]
    OUT1 = 2,
    #[doc = "4: Alternate Output 2"]
    OUT2 = 4,
    #[doc = "8: Alternate Output 3"]
    OUT3 = 8,
    #[doc = "16: Alternate Output 4"]
    OUT4 = 16,
}
impl From<ALTOUTPADEN_A> for u8 {
    #[inline(always)]
    fn from(variant: ALTOUTPADEN_A) -> Self {
        variant as _
    }
}
impl ALTOUTPADEN_R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> Option<ALTOUTPADEN_A> {
        match self.bits {
            1 => Some(ALTOUTPADEN_A::OUT0),
            2 => Some(ALTOUTPADEN_A::OUT1),
            4 => Some(ALTOUTPADEN_A::OUT2),
            8 => Some(ALTOUTPADEN_A::OUT3),
            16 => Some(ALTOUTPADEN_A::OUT4),
            _ => None,
        }
    }
    #[doc = "Checks if the value of the field is `OUT0`"]
    #[inline(always)]
    pub fn is_out0(&self) -> bool {
        *self == ALTOUTPADEN_A::OUT0
    }
    #[doc = "Checks if the value of the field is `OUT1`"]
    #[inline(always)]
    pub fn is_out1(&self) -> bool {
        *self == ALTOUTPADEN_A::OUT1
    }
    #[doc = "Checks if the value of the field is `OUT2`"]
    #[inline(always)]
    pub fn is_out2(&self) -> bool {
        *self == ALTOUTPADEN_A::OUT2
    }
    #[doc = "Checks if the value of the field is `OUT3`"]
    #[inline(always)]
    pub fn is_out3(&self) -> bool {
        *self == ALTOUTPADEN_A::OUT3
    }
    #[doc = "Checks if the value of the field is `OUT4`"]
    #[inline(always)]
    pub fn is_out4(&self) -> bool {
        *self == ALTOUTPADEN_A::OUT4
    }
}
#[doc = "Field `ALTOUTPADEN` writer - OPAx Output Enable Value"]
pub type ALTOUTPADEN_W<'a, const O: u8> =
    crate::FieldWriter<'a, u32, OPA3_OUT_SPEC, u8, ALTOUTPADEN_A, 5, O>;
impl<'a, const O: u8> ALTOUTPADEN_W<'a, O> {
    #[doc = "Alternate Output 0"]
    #[inline(always)]
    pub fn out0(self) -> &'a mut W {
        self.variant(ALTOUTPADEN_A::OUT0)
    }
    #[doc = "Alternate Output 1"]
    #[inline(always)]
    pub fn out1(self) -> &'a mut W {
        self.variant(ALTOUTPADEN_A::OUT1)
    }
    #[doc = "Alternate Output 2"]
    #[inline(always)]
    pub fn out2(self) -> &'a mut W {
        self.variant(ALTOUTPADEN_A::OUT2)
    }
    #[doc = "Alternate Output 3"]
    #[inline(always)]
    pub fn out3(self) -> &'a mut W {
        self.variant(ALTOUTPADEN_A::OUT3)
    }
    #[doc = "Alternate Output 4"]
    #[inline(always)]
    pub fn out4(self) -> &'a mut W {
        self.variant(ALTOUTPADEN_A::OUT4)
    }
}
#[doc = "Field `APORTOUTSEL` reader - OPAx APORT Output"]
pub type APORTOUTSEL_R = crate::FieldReader<u8, u8>;
#[doc = "Field `APORTOUTSEL` writer - OPAx APORT Output"]
pub type APORTOUTSEL_W<'a, const O: u8> = crate::FieldWriter<'a, u32, OPA3_OUT_SPEC, u8, u8, 8, O>;
impl R {
    #[doc = "Bit 0 - OPAx Main Output Enable"]
    #[inline(always)]
    pub fn mainouten(&self) -> MAINOUTEN_R {
        MAINOUTEN_R::new((self.bits & 1) != 0)
    }
    #[doc = "Bit 1 - OPAx Alternative Output Enable"]
    #[inline(always)]
    pub fn altouten(&self) -> ALTOUTEN_R {
        ALTOUTEN_R::new(((self.bits >> 1) & 1) != 0)
    }
    #[doc = "Bit 2 - OPAx Aport Output Enable"]
    #[inline(always)]
    pub fn aportouten(&self) -> APORTOUTEN_R {
        APORTOUTEN_R::new(((self.bits >> 2) & 1) != 0)
    }
    #[doc = "Bit 3 - OPAx Main and Alternative Output Short"]
    #[inline(always)]
    pub fn short(&self) -> SHORT_R {
        SHORT_R::new(((self.bits >> 3) & 1) != 0)
    }
    #[doc = "Bits 4:8 - OPAx Output Enable Value"]
    #[inline(always)]
    pub fn altoutpaden(&self) -> ALTOUTPADEN_R {
        ALTOUTPADEN_R::new(((self.bits >> 4) & 0x1f) as u8)
    }
    #[doc = "Bits 16:23 - OPAx APORT Output"]
    #[inline(always)]
    pub fn aportoutsel(&self) -> APORTOUTSEL_R {
        APORTOUTSEL_R::new(((self.bits >> 16) & 0xff) as u8)
    }
}
impl W {
    #[doc = "Bit 0 - OPAx Main Output Enable"]
    #[inline(always)]
    #[must_use]
    pub fn mainouten(&mut self) -> MAINOUTEN_W<0> {
        MAINOUTEN_W::new(self)
    }
    #[doc = "Bit 1 - OPAx Alternative Output Enable"]
    #[inline(always)]
    #[must_use]
    pub fn altouten(&mut self) -> ALTOUTEN_W<1> {
        ALTOUTEN_W::new(self)
    }
    #[doc = "Bit 2 - OPAx Aport Output Enable"]
    #[inline(always)]
    #[must_use]
    pub fn aportouten(&mut self) -> APORTOUTEN_W<2> {
        APORTOUTEN_W::new(self)
    }
    #[doc = "Bit 3 - OPAx Main and Alternative Output Short"]
    #[inline(always)]
    #[must_use]
    pub fn short(&mut self) -> SHORT_W<3> {
        SHORT_W::new(self)
    }
    #[doc = "Bits 4:8 - OPAx Output Enable Value"]
    #[inline(always)]
    #[must_use]
    pub fn altoutpaden(&mut self) -> ALTOUTPADEN_W<4> {
        ALTOUTPADEN_W::new(self)
    }
    #[doc = "Bits 16:23 - OPAx APORT Output"]
    #[inline(always)]
    #[must_use]
    pub fn aportoutsel(&mut self) -> APORTOUTSEL_W<16> {
        APORTOUTSEL_W::new(self)
    }
    #[doc = "Writes raw bits to the register."]
    #[inline(always)]
    pub unsafe fn bits(&mut self, bits: u32) -> &mut Self {
        self.0.bits(bits);
        self
    }
}
#[doc = "Operational Amplifier Output Configuration Register\n\nThis register you can [`read`](crate::generic::Reg::read), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [opa3_out](index.html) module"]
pub struct OPA3_OUT_SPEC;
impl crate::RegisterSpec for OPA3_OUT_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [opa3_out::R](R) reader structure"]
impl crate::Readable for OPA3_OUT_SPEC {
    type Reader = R;
}
#[doc = "`write(|w| ..)` method takes [opa3_out::W](W) writer structure"]
impl crate::Writable for OPA3_OUT_SPEC {
    type Writer = W;
    const ZERO_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
}
#[doc = "`reset()` method sets OPA3_OUT to value 0x01"]
impl crate::Resettable for OPA3_OUT_SPEC {
    const RESET_VALUE: Self::Ux = 0x01;
}
