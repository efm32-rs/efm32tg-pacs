#[doc = "Register `MIR0_DATAL` reader"]
pub struct R(crate::R<MIR0_DATAL_SPEC>);
impl core::ops::Deref for R {
    type Target = crate::R<MIR0_DATAL_SPEC>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
impl From<crate::R<MIR0_DATAL_SPEC>> for R {
    #[inline(always)]
    fn from(reader: crate::R<MIR0_DATAL_SPEC>) -> Self {
        R(reader)
    }
}
#[doc = "Register `MIR0_DATAL` writer"]
pub struct W(crate::W<MIR0_DATAL_SPEC>);
impl core::ops::Deref for W {
    type Target = crate::W<MIR0_DATAL_SPEC>;
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
impl From<crate::W<MIR0_DATAL_SPEC>> for W {
    #[inline(always)]
    fn from(writer: crate::W<MIR0_DATAL_SPEC>) -> Self {
        W(writer)
    }
}
#[doc = "Field `DATA0` reader - First Byte of CAN Data Frame"]
pub type DATA0_R = crate::FieldReader<u8, u8>;
#[doc = "Field `DATA0` writer - First Byte of CAN Data Frame"]
pub type DATA0_W<'a, const O: u8> = crate::FieldWriter<'a, u32, MIR0_DATAL_SPEC, u8, u8, 8, O>;
#[doc = "Field `DATA1` reader - Second Byte of CAN Data Frame"]
pub type DATA1_R = crate::FieldReader<u8, u8>;
#[doc = "Field `DATA1` writer - Second Byte of CAN Data Frame"]
pub type DATA1_W<'a, const O: u8> = crate::FieldWriter<'a, u32, MIR0_DATAL_SPEC, u8, u8, 8, O>;
#[doc = "Field `DATA2` reader - Third Byte of CAN Data Frame"]
pub type DATA2_R = crate::FieldReader<u8, u8>;
#[doc = "Field `DATA2` writer - Third Byte of CAN Data Frame"]
pub type DATA2_W<'a, const O: u8> = crate::FieldWriter<'a, u32, MIR0_DATAL_SPEC, u8, u8, 8, O>;
#[doc = "Field `DATA3` reader - Fourth Byte of CAN Data Frame"]
pub type DATA3_R = crate::FieldReader<u8, u8>;
#[doc = "Field `DATA3` writer - Fourth Byte of CAN Data Frame"]
pub type DATA3_W<'a, const O: u8> = crate::FieldWriter<'a, u32, MIR0_DATAL_SPEC, u8, u8, 8, O>;
impl R {
    #[doc = "Bits 0:7 - First Byte of CAN Data Frame"]
    #[inline(always)]
    pub fn data0(&self) -> DATA0_R {
        DATA0_R::new((self.bits & 0xff) as u8)
    }
    #[doc = "Bits 8:15 - Second Byte of CAN Data Frame"]
    #[inline(always)]
    pub fn data1(&self) -> DATA1_R {
        DATA1_R::new(((self.bits >> 8) & 0xff) as u8)
    }
    #[doc = "Bits 16:23 - Third Byte of CAN Data Frame"]
    #[inline(always)]
    pub fn data2(&self) -> DATA2_R {
        DATA2_R::new(((self.bits >> 16) & 0xff) as u8)
    }
    #[doc = "Bits 24:31 - Fourth Byte of CAN Data Frame"]
    #[inline(always)]
    pub fn data3(&self) -> DATA3_R {
        DATA3_R::new(((self.bits >> 24) & 0xff) as u8)
    }
}
impl W {
    #[doc = "Bits 0:7 - First Byte of CAN Data Frame"]
    #[inline(always)]
    #[must_use]
    pub fn data0(&mut self) -> DATA0_W<0> {
        DATA0_W::new(self)
    }
    #[doc = "Bits 8:15 - Second Byte of CAN Data Frame"]
    #[inline(always)]
    #[must_use]
    pub fn data1(&mut self) -> DATA1_W<8> {
        DATA1_W::new(self)
    }
    #[doc = "Bits 16:23 - Third Byte of CAN Data Frame"]
    #[inline(always)]
    #[must_use]
    pub fn data2(&mut self) -> DATA2_W<16> {
        DATA2_W::new(self)
    }
    #[doc = "Bits 24:31 - Fourth Byte of CAN Data Frame"]
    #[inline(always)]
    #[must_use]
    pub fn data3(&mut self) -> DATA3_W<24> {
        DATA3_W::new(self)
    }
    #[doc = "Writes raw bits to the register."]
    #[inline(always)]
    pub unsafe fn bits(&mut self, bits: u32) -> &mut Self {
        self.0.bits(bits);
        self
    }
}
#[doc = "Interface Data a Register\n\nThis register you can [`read`](crate::generic::Reg::read), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [mir0_datal](index.html) module"]
pub struct MIR0_DATAL_SPEC;
impl crate::RegisterSpec for MIR0_DATAL_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [mir0_datal::R](R) reader structure"]
impl crate::Readable for MIR0_DATAL_SPEC {
    type Reader = R;
}
#[doc = "`write(|w| ..)` method takes [mir0_datal::W](W) writer structure"]
impl crate::Writable for MIR0_DATAL_SPEC {
    type Writer = W;
    const ZERO_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
}
#[doc = "`reset()` method sets MIR0_DATAL to value 0"]
impl crate::Resettable for MIR0_DATAL_SPEC {
    const RESET_VALUE: Self::Ux = 0;
}
