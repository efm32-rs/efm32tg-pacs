#[doc = "Register `CH0_EVAL` reader"]
pub struct R(crate::R<CH0_EVAL_SPEC>);
impl core::ops::Deref for R {
    type Target = crate::R<CH0_EVAL_SPEC>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
impl From<crate::R<CH0_EVAL_SPEC>> for R {
    #[inline(always)]
    fn from(reader: crate::R<CH0_EVAL_SPEC>) -> Self {
        R(reader)
    }
}
#[doc = "Register `CH0_EVAL` writer"]
pub struct W(crate::W<CH0_EVAL_SPEC>);
impl core::ops::Deref for W {
    type Target = crate::W<CH0_EVAL_SPEC>;
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
impl From<crate::W<CH0_EVAL_SPEC>> for W {
    #[inline(always)]
    fn from(writer: crate::W<CH0_EVAL_SPEC>) -> Self {
        W(writer)
    }
}
#[doc = "Field `COMPTHRES` reader - Decision Threshold for Sensor Data"]
pub type COMPTHRES_R = crate::FieldReader<u16, u16>;
#[doc = "Field `COMPTHRES` writer - Decision Threshold for Sensor Data"]
pub type COMPTHRES_W<'a, const O: u8> = crate::FieldWriter<'a, u32, CH0_EVAL_SPEC, u16, u16, 16, O>;
#[doc = "Field `COMP` reader - Select Mode for Threshold Comparison"]
pub type COMP_R = crate::BitReader<bool>;
#[doc = "Field `COMP` writer - Select Mode for Threshold Comparison"]
pub type COMP_W<'a, const O: u8> = crate::BitWriter<'a, u32, CH0_EVAL_SPEC, bool, O>;
#[doc = "Field `DECODE` reader - Send Result to Decoder"]
pub type DECODE_R = crate::BitReader<bool>;
#[doc = "Field `DECODE` writer - Send Result to Decoder"]
pub type DECODE_W<'a, const O: u8> = crate::BitWriter<'a, u32, CH0_EVAL_SPEC, bool, O>;
#[doc = "Field `STRSAMPLE` reader - Enable Storing of Sensor Sample in Result Buffer"]
pub type STRSAMPLE_R = crate::FieldReader<u8, STRSAMPLE_A>;
#[doc = "Enable Storing of Sensor Sample in Result Buffer\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
#[repr(u8)]
pub enum STRSAMPLE_A {
    #[doc = "0: Nothing will be stored in the result buffer."]
    DISABLE = 0,
    #[doc = "1: The sensor sample data will be stored in the result buffer."]
    DATA = 1,
    #[doc = "2: The data source (i.e., the channel) will be stored alongside the sensor sample data."]
    DATASRC = 2,
}
impl From<STRSAMPLE_A> for u8 {
    #[inline(always)]
    fn from(variant: STRSAMPLE_A) -> Self {
        variant as _
    }
}
impl STRSAMPLE_R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> Option<STRSAMPLE_A> {
        match self.bits {
            0 => Some(STRSAMPLE_A::DISABLE),
            1 => Some(STRSAMPLE_A::DATA),
            2 => Some(STRSAMPLE_A::DATASRC),
            _ => None,
        }
    }
    #[doc = "Checks if the value of the field is `DISABLE`"]
    #[inline(always)]
    pub fn is_disable(&self) -> bool {
        *self == STRSAMPLE_A::DISABLE
    }
    #[doc = "Checks if the value of the field is `DATA`"]
    #[inline(always)]
    pub fn is_data(&self) -> bool {
        *self == STRSAMPLE_A::DATA
    }
    #[doc = "Checks if the value of the field is `DATASRC`"]
    #[inline(always)]
    pub fn is_datasrc(&self) -> bool {
        *self == STRSAMPLE_A::DATASRC
    }
}
#[doc = "Field `STRSAMPLE` writer - Enable Storing of Sensor Sample in Result Buffer"]
pub type STRSAMPLE_W<'a, const O: u8> =
    crate::FieldWriter<'a, u32, CH0_EVAL_SPEC, u8, STRSAMPLE_A, 2, O>;
impl<'a, const O: u8> STRSAMPLE_W<'a, O> {
    #[doc = "Nothing will be stored in the result buffer."]
    #[inline(always)]
    pub fn disable(self) -> &'a mut W {
        self.variant(STRSAMPLE_A::DISABLE)
    }
    #[doc = "The sensor sample data will be stored in the result buffer."]
    #[inline(always)]
    pub fn data(self) -> &'a mut W {
        self.variant(STRSAMPLE_A::DATA)
    }
    #[doc = "The data source (i.e., the channel) will be stored alongside the sensor sample data."]
    #[inline(always)]
    pub fn datasrc(self) -> &'a mut W {
        self.variant(STRSAMPLE_A::DATASRC)
    }
}
#[doc = "Field `SCANRESINV` reader - Enable Inversion of Result"]
pub type SCANRESINV_R = crate::BitReader<bool>;
#[doc = "Field `SCANRESINV` writer - Enable Inversion of Result"]
pub type SCANRESINV_W<'a, const O: u8> = crate::BitWriter<'a, u32, CH0_EVAL_SPEC, bool, O>;
#[doc = "Field `MODE` reader - Configure Evaluation Mode"]
pub type MODE_R = crate::FieldReader<u8, MODE_A>;
#[doc = "Configure Evaluation Mode\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
#[repr(u8)]
pub enum MODE_A {
    #[doc = "0: Threshold comparison is used to evaluate sensor result"]
    THRES = 0,
    #[doc = "1: Sliding window is used to evaluate sensor result"]
    SLIDINGWIN = 1,
    #[doc = "2: Step detection is used to evaluate sensor result"]
    STEPDET = 2,
}
impl From<MODE_A> for u8 {
    #[inline(always)]
    fn from(variant: MODE_A) -> Self {
        variant as _
    }
}
impl MODE_R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> Option<MODE_A> {
        match self.bits {
            0 => Some(MODE_A::THRES),
            1 => Some(MODE_A::SLIDINGWIN),
            2 => Some(MODE_A::STEPDET),
            _ => None,
        }
    }
    #[doc = "Checks if the value of the field is `THRES`"]
    #[inline(always)]
    pub fn is_thres(&self) -> bool {
        *self == MODE_A::THRES
    }
    #[doc = "Checks if the value of the field is `SLIDINGWIN`"]
    #[inline(always)]
    pub fn is_slidingwin(&self) -> bool {
        *self == MODE_A::SLIDINGWIN
    }
    #[doc = "Checks if the value of the field is `STEPDET`"]
    #[inline(always)]
    pub fn is_stepdet(&self) -> bool {
        *self == MODE_A::STEPDET
    }
}
#[doc = "Field `MODE` writer - Configure Evaluation Mode"]
pub type MODE_W<'a, const O: u8> = crate::FieldWriter<'a, u32, CH0_EVAL_SPEC, u8, MODE_A, 2, O>;
impl<'a, const O: u8> MODE_W<'a, O> {
    #[doc = "Threshold comparison is used to evaluate sensor result"]
    #[inline(always)]
    pub fn thres(self) -> &'a mut W {
        self.variant(MODE_A::THRES)
    }
    #[doc = "Sliding window is used to evaluate sensor result"]
    #[inline(always)]
    pub fn slidingwin(self) -> &'a mut W {
        self.variant(MODE_A::SLIDINGWIN)
    }
    #[doc = "Step detection is used to evaluate sensor result"]
    #[inline(always)]
    pub fn stepdet(self) -> &'a mut W {
        self.variant(MODE_A::STEPDET)
    }
}
impl R {
    #[doc = "Bits 0:15 - Decision Threshold for Sensor Data"]
    #[inline(always)]
    pub fn compthres(&self) -> COMPTHRES_R {
        COMPTHRES_R::new((self.bits & 0xffff) as u16)
    }
    #[doc = "Bit 16 - Select Mode for Threshold Comparison"]
    #[inline(always)]
    pub fn comp(&self) -> COMP_R {
        COMP_R::new(((self.bits >> 16) & 1) != 0)
    }
    #[doc = "Bit 17 - Send Result to Decoder"]
    #[inline(always)]
    pub fn decode(&self) -> DECODE_R {
        DECODE_R::new(((self.bits >> 17) & 1) != 0)
    }
    #[doc = "Bits 18:19 - Enable Storing of Sensor Sample in Result Buffer"]
    #[inline(always)]
    pub fn strsample(&self) -> STRSAMPLE_R {
        STRSAMPLE_R::new(((self.bits >> 18) & 3) as u8)
    }
    #[doc = "Bit 20 - Enable Inversion of Result"]
    #[inline(always)]
    pub fn scanresinv(&self) -> SCANRESINV_R {
        SCANRESINV_R::new(((self.bits >> 20) & 1) != 0)
    }
    #[doc = "Bits 21:22 - Configure Evaluation Mode"]
    #[inline(always)]
    pub fn mode(&self) -> MODE_R {
        MODE_R::new(((self.bits >> 21) & 3) as u8)
    }
}
impl W {
    #[doc = "Bits 0:15 - Decision Threshold for Sensor Data"]
    #[inline(always)]
    #[must_use]
    pub fn compthres(&mut self) -> COMPTHRES_W<0> {
        COMPTHRES_W::new(self)
    }
    #[doc = "Bit 16 - Select Mode for Threshold Comparison"]
    #[inline(always)]
    #[must_use]
    pub fn comp(&mut self) -> COMP_W<16> {
        COMP_W::new(self)
    }
    #[doc = "Bit 17 - Send Result to Decoder"]
    #[inline(always)]
    #[must_use]
    pub fn decode(&mut self) -> DECODE_W<17> {
        DECODE_W::new(self)
    }
    #[doc = "Bits 18:19 - Enable Storing of Sensor Sample in Result Buffer"]
    #[inline(always)]
    #[must_use]
    pub fn strsample(&mut self) -> STRSAMPLE_W<18> {
        STRSAMPLE_W::new(self)
    }
    #[doc = "Bit 20 - Enable Inversion of Result"]
    #[inline(always)]
    #[must_use]
    pub fn scanresinv(&mut self) -> SCANRESINV_W<20> {
        SCANRESINV_W::new(self)
    }
    #[doc = "Bits 21:22 - Configure Evaluation Mode"]
    #[inline(always)]
    #[must_use]
    pub fn mode(&mut self) -> MODE_W<21> {
        MODE_W::new(self)
    }
    #[doc = "Writes raw bits to the register."]
    #[inline(always)]
    pub unsafe fn bits(&mut self, bits: u32) -> &mut Self {
        self.0.bits(bits);
        self
    }
}
#[doc = "Scan Configuration\n\nThis register you can [`read`](crate::generic::Reg::read), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [ch0_eval](index.html) module"]
pub struct CH0_EVAL_SPEC;
impl crate::RegisterSpec for CH0_EVAL_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [ch0_eval::R](R) reader structure"]
impl crate::Readable for CH0_EVAL_SPEC {
    type Reader = R;
}
#[doc = "`write(|w| ..)` method takes [ch0_eval::W](W) writer structure"]
impl crate::Writable for CH0_EVAL_SPEC {
    type Writer = W;
    const ZERO_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
}
#[doc = "`reset()` method sets CH0_EVAL to value 0"]
impl crate::Resettable for CH0_EVAL_SPEC {
    const RESET_VALUE: Self::Ux = 0;
}
