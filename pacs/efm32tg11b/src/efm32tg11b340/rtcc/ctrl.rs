#[doc = "Register `CTRL` reader"]
pub struct R(crate::R<CTRL_SPEC>);
impl core::ops::Deref for R {
    type Target = crate::R<CTRL_SPEC>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
impl From<crate::R<CTRL_SPEC>> for R {
    #[inline(always)]
    fn from(reader: crate::R<CTRL_SPEC>) -> Self {
        R(reader)
    }
}
#[doc = "Register `CTRL` writer"]
pub struct W(crate::W<CTRL_SPEC>);
impl core::ops::Deref for W {
    type Target = crate::W<CTRL_SPEC>;
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
impl From<crate::W<CTRL_SPEC>> for W {
    #[inline(always)]
    fn from(writer: crate::W<CTRL_SPEC>) -> Self {
        W(writer)
    }
}
#[doc = "Field `ENABLE` reader - RTCC Enable"]
pub type ENABLE_R = crate::BitReader<bool>;
#[doc = "Field `ENABLE` writer - RTCC Enable"]
pub type ENABLE_W<'a, const O: u8> = crate::BitWriter<'a, u32, CTRL_SPEC, bool, O>;
#[doc = "Field `DEBUGRUN` reader - Debug Mode Run Enable"]
pub type DEBUGRUN_R = crate::BitReader<bool>;
#[doc = "Field `DEBUGRUN` writer - Debug Mode Run Enable"]
pub type DEBUGRUN_W<'a, const O: u8> = crate::BitWriter<'a, u32, CTRL_SPEC, bool, O>;
#[doc = "Field `PRECCV0TOP` reader - Pre-counter CCV0 Top Value Enable"]
pub type PRECCV0TOP_R = crate::BitReader<bool>;
#[doc = "Field `PRECCV0TOP` writer - Pre-counter CCV0 Top Value Enable"]
pub type PRECCV0TOP_W<'a, const O: u8> = crate::BitWriter<'a, u32, CTRL_SPEC, bool, O>;
#[doc = "Field `CCV1TOP` reader - CCV1 Top Value Enable"]
pub type CCV1TOP_R = crate::BitReader<bool>;
#[doc = "Field `CCV1TOP` writer - CCV1 Top Value Enable"]
pub type CCV1TOP_W<'a, const O: u8> = crate::BitWriter<'a, u32, CTRL_SPEC, bool, O>;
#[doc = "Field `CNTPRESC` reader - Counter Prescaler Value"]
pub type CNTPRESC_R = crate::FieldReader<u8, CNTPRESC_A>;
#[doc = "Counter Prescaler Value\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
#[repr(u8)]
pub enum CNTPRESC_A {
    #[doc = "0: CLKCNT = LFECLKRTCC/1"]
    DIV1 = 0,
    #[doc = "1: CLKCNT = LFECLKRTCC/2"]
    DIV2 = 1,
    #[doc = "2: CLKCNT = LFECLKRTCC/4"]
    DIV4 = 2,
    #[doc = "3: CLKCNT = LFECLKRTCC/8"]
    DIV8 = 3,
    #[doc = "4: CLKCNT = LFECLKRTCC/16"]
    DIV16 = 4,
    #[doc = "5: CLKCNT = LFECLKRTCC/32"]
    DIV32 = 5,
    #[doc = "6: CLKCNT = LFECLKRTCC/64"]
    DIV64 = 6,
    #[doc = "7: CLKCNT = LFECLKRTCC/128"]
    DIV128 = 7,
    #[doc = "8: CLKCNT = LFECLKRTCC/256"]
    DIV256 = 8,
    #[doc = "9: CLKCNT = LFECLKRTCC/512"]
    DIV512 = 9,
    #[doc = "10: CLKCNT = LFECLKRTCC/1024"]
    DIV1024 = 10,
    #[doc = "11: CLKCNT = LFECLKRTCC/2048"]
    DIV2048 = 11,
    #[doc = "12: CLKCNT = LFECLKRTCC/4096"]
    DIV4096 = 12,
    #[doc = "13: CLKCNT = LFECLKRTCC/8192"]
    DIV8192 = 13,
    #[doc = "14: CLKCNT = LFECLKRTCC/16384"]
    DIV16384 = 14,
    #[doc = "15: CLKCNT = LFECLKRTCC/32768"]
    DIV32768 = 15,
}
impl From<CNTPRESC_A> for u8 {
    #[inline(always)]
    fn from(variant: CNTPRESC_A) -> Self {
        variant as _
    }
}
impl CNTPRESC_R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> CNTPRESC_A {
        match self.bits {
            0 => CNTPRESC_A::DIV1,
            1 => CNTPRESC_A::DIV2,
            2 => CNTPRESC_A::DIV4,
            3 => CNTPRESC_A::DIV8,
            4 => CNTPRESC_A::DIV16,
            5 => CNTPRESC_A::DIV32,
            6 => CNTPRESC_A::DIV64,
            7 => CNTPRESC_A::DIV128,
            8 => CNTPRESC_A::DIV256,
            9 => CNTPRESC_A::DIV512,
            10 => CNTPRESC_A::DIV1024,
            11 => CNTPRESC_A::DIV2048,
            12 => CNTPRESC_A::DIV4096,
            13 => CNTPRESC_A::DIV8192,
            14 => CNTPRESC_A::DIV16384,
            15 => CNTPRESC_A::DIV32768,
            _ => unreachable!(),
        }
    }
    #[doc = "Checks if the value of the field is `DIV1`"]
    #[inline(always)]
    pub fn is_div1(&self) -> bool {
        *self == CNTPRESC_A::DIV1
    }
    #[doc = "Checks if the value of the field is `DIV2`"]
    #[inline(always)]
    pub fn is_div2(&self) -> bool {
        *self == CNTPRESC_A::DIV2
    }
    #[doc = "Checks if the value of the field is `DIV4`"]
    #[inline(always)]
    pub fn is_div4(&self) -> bool {
        *self == CNTPRESC_A::DIV4
    }
    #[doc = "Checks if the value of the field is `DIV8`"]
    #[inline(always)]
    pub fn is_div8(&self) -> bool {
        *self == CNTPRESC_A::DIV8
    }
    #[doc = "Checks if the value of the field is `DIV16`"]
    #[inline(always)]
    pub fn is_div16(&self) -> bool {
        *self == CNTPRESC_A::DIV16
    }
    #[doc = "Checks if the value of the field is `DIV32`"]
    #[inline(always)]
    pub fn is_div32(&self) -> bool {
        *self == CNTPRESC_A::DIV32
    }
    #[doc = "Checks if the value of the field is `DIV64`"]
    #[inline(always)]
    pub fn is_div64(&self) -> bool {
        *self == CNTPRESC_A::DIV64
    }
    #[doc = "Checks if the value of the field is `DIV128`"]
    #[inline(always)]
    pub fn is_div128(&self) -> bool {
        *self == CNTPRESC_A::DIV128
    }
    #[doc = "Checks if the value of the field is `DIV256`"]
    #[inline(always)]
    pub fn is_div256(&self) -> bool {
        *self == CNTPRESC_A::DIV256
    }
    #[doc = "Checks if the value of the field is `DIV512`"]
    #[inline(always)]
    pub fn is_div512(&self) -> bool {
        *self == CNTPRESC_A::DIV512
    }
    #[doc = "Checks if the value of the field is `DIV1024`"]
    #[inline(always)]
    pub fn is_div1024(&self) -> bool {
        *self == CNTPRESC_A::DIV1024
    }
    #[doc = "Checks if the value of the field is `DIV2048`"]
    #[inline(always)]
    pub fn is_div2048(&self) -> bool {
        *self == CNTPRESC_A::DIV2048
    }
    #[doc = "Checks if the value of the field is `DIV4096`"]
    #[inline(always)]
    pub fn is_div4096(&self) -> bool {
        *self == CNTPRESC_A::DIV4096
    }
    #[doc = "Checks if the value of the field is `DIV8192`"]
    #[inline(always)]
    pub fn is_div8192(&self) -> bool {
        *self == CNTPRESC_A::DIV8192
    }
    #[doc = "Checks if the value of the field is `DIV16384`"]
    #[inline(always)]
    pub fn is_div16384(&self) -> bool {
        *self == CNTPRESC_A::DIV16384
    }
    #[doc = "Checks if the value of the field is `DIV32768`"]
    #[inline(always)]
    pub fn is_div32768(&self) -> bool {
        *self == CNTPRESC_A::DIV32768
    }
}
#[doc = "Field `CNTPRESC` writer - Counter Prescaler Value"]
pub type CNTPRESC_W<'a, const O: u8> =
    crate::FieldWriterSafe<'a, u32, CTRL_SPEC, u8, CNTPRESC_A, 4, O>;
impl<'a, const O: u8> CNTPRESC_W<'a, O> {
    #[doc = "CLKCNT = LFECLKRTCC/1"]
    #[inline(always)]
    pub fn div1(self) -> &'a mut W {
        self.variant(CNTPRESC_A::DIV1)
    }
    #[doc = "CLKCNT = LFECLKRTCC/2"]
    #[inline(always)]
    pub fn div2(self) -> &'a mut W {
        self.variant(CNTPRESC_A::DIV2)
    }
    #[doc = "CLKCNT = LFECLKRTCC/4"]
    #[inline(always)]
    pub fn div4(self) -> &'a mut W {
        self.variant(CNTPRESC_A::DIV4)
    }
    #[doc = "CLKCNT = LFECLKRTCC/8"]
    #[inline(always)]
    pub fn div8(self) -> &'a mut W {
        self.variant(CNTPRESC_A::DIV8)
    }
    #[doc = "CLKCNT = LFECLKRTCC/16"]
    #[inline(always)]
    pub fn div16(self) -> &'a mut W {
        self.variant(CNTPRESC_A::DIV16)
    }
    #[doc = "CLKCNT = LFECLKRTCC/32"]
    #[inline(always)]
    pub fn div32(self) -> &'a mut W {
        self.variant(CNTPRESC_A::DIV32)
    }
    #[doc = "CLKCNT = LFECLKRTCC/64"]
    #[inline(always)]
    pub fn div64(self) -> &'a mut W {
        self.variant(CNTPRESC_A::DIV64)
    }
    #[doc = "CLKCNT = LFECLKRTCC/128"]
    #[inline(always)]
    pub fn div128(self) -> &'a mut W {
        self.variant(CNTPRESC_A::DIV128)
    }
    #[doc = "CLKCNT = LFECLKRTCC/256"]
    #[inline(always)]
    pub fn div256(self) -> &'a mut W {
        self.variant(CNTPRESC_A::DIV256)
    }
    #[doc = "CLKCNT = LFECLKRTCC/512"]
    #[inline(always)]
    pub fn div512(self) -> &'a mut W {
        self.variant(CNTPRESC_A::DIV512)
    }
    #[doc = "CLKCNT = LFECLKRTCC/1024"]
    #[inline(always)]
    pub fn div1024(self) -> &'a mut W {
        self.variant(CNTPRESC_A::DIV1024)
    }
    #[doc = "CLKCNT = LFECLKRTCC/2048"]
    #[inline(always)]
    pub fn div2048(self) -> &'a mut W {
        self.variant(CNTPRESC_A::DIV2048)
    }
    #[doc = "CLKCNT = LFECLKRTCC/4096"]
    #[inline(always)]
    pub fn div4096(self) -> &'a mut W {
        self.variant(CNTPRESC_A::DIV4096)
    }
    #[doc = "CLKCNT = LFECLKRTCC/8192"]
    #[inline(always)]
    pub fn div8192(self) -> &'a mut W {
        self.variant(CNTPRESC_A::DIV8192)
    }
    #[doc = "CLKCNT = LFECLKRTCC/16384"]
    #[inline(always)]
    pub fn div16384(self) -> &'a mut W {
        self.variant(CNTPRESC_A::DIV16384)
    }
    #[doc = "CLKCNT = LFECLKRTCC/32768"]
    #[inline(always)]
    pub fn div32768(self) -> &'a mut W {
        self.variant(CNTPRESC_A::DIV32768)
    }
}
#[doc = "Field `CNTTICK` reader - Counter Prescaler Mode"]
pub type CNTTICK_R = crate::BitReader<bool>;
#[doc = "Field `CNTTICK` writer - Counter Prescaler Mode"]
pub type CNTTICK_W<'a, const O: u8> = crate::BitWriter<'a, u32, CTRL_SPEC, bool, O>;
#[doc = "Field `BUMODETSEN` reader - Backup Mode Timestamp Enable"]
pub type BUMODETSEN_R = crate::BitReader<bool>;
#[doc = "Field `BUMODETSEN` writer - Backup Mode Timestamp Enable"]
pub type BUMODETSEN_W<'a, const O: u8> = crate::BitWriter<'a, u32, CTRL_SPEC, bool, O>;
#[doc = "Field `OSCFDETEN` reader - Oscillator Failure Detection Enable"]
pub type OSCFDETEN_R = crate::BitReader<bool>;
#[doc = "Field `OSCFDETEN` writer - Oscillator Failure Detection Enable"]
pub type OSCFDETEN_W<'a, const O: u8> = crate::BitWriter<'a, u32, CTRL_SPEC, bool, O>;
#[doc = "Field `CNTMODE` reader - Main Counter Mode"]
pub type CNTMODE_R = crate::BitReader<bool>;
#[doc = "Field `CNTMODE` writer - Main Counter Mode"]
pub type CNTMODE_W<'a, const O: u8> = crate::BitWriter<'a, u32, CTRL_SPEC, bool, O>;
#[doc = "Field `LYEARCORRDIS` reader - Leap Year Correction Disabled"]
pub type LYEARCORRDIS_R = crate::BitReader<bool>;
#[doc = "Field `LYEARCORRDIS` writer - Leap Year Correction Disabled"]
pub type LYEARCORRDIS_W<'a, const O: u8> = crate::BitWriter<'a, u32, CTRL_SPEC, bool, O>;
impl R {
    #[doc = "Bit 0 - RTCC Enable"]
    #[inline(always)]
    pub fn enable(&self) -> ENABLE_R {
        ENABLE_R::new((self.bits & 1) != 0)
    }
    #[doc = "Bit 2 - Debug Mode Run Enable"]
    #[inline(always)]
    pub fn debugrun(&self) -> DEBUGRUN_R {
        DEBUGRUN_R::new(((self.bits >> 2) & 1) != 0)
    }
    #[doc = "Bit 4 - Pre-counter CCV0 Top Value Enable"]
    #[inline(always)]
    pub fn preccv0top(&self) -> PRECCV0TOP_R {
        PRECCV0TOP_R::new(((self.bits >> 4) & 1) != 0)
    }
    #[doc = "Bit 5 - CCV1 Top Value Enable"]
    #[inline(always)]
    pub fn ccv1top(&self) -> CCV1TOP_R {
        CCV1TOP_R::new(((self.bits >> 5) & 1) != 0)
    }
    #[doc = "Bits 8:11 - Counter Prescaler Value"]
    #[inline(always)]
    pub fn cntpresc(&self) -> CNTPRESC_R {
        CNTPRESC_R::new(((self.bits >> 8) & 0x0f) as u8)
    }
    #[doc = "Bit 12 - Counter Prescaler Mode"]
    #[inline(always)]
    pub fn cnttick(&self) -> CNTTICK_R {
        CNTTICK_R::new(((self.bits >> 12) & 1) != 0)
    }
    #[doc = "Bit 14 - Backup Mode Timestamp Enable"]
    #[inline(always)]
    pub fn bumodetsen(&self) -> BUMODETSEN_R {
        BUMODETSEN_R::new(((self.bits >> 14) & 1) != 0)
    }
    #[doc = "Bit 15 - Oscillator Failure Detection Enable"]
    #[inline(always)]
    pub fn oscfdeten(&self) -> OSCFDETEN_R {
        OSCFDETEN_R::new(((self.bits >> 15) & 1) != 0)
    }
    #[doc = "Bit 16 - Main Counter Mode"]
    #[inline(always)]
    pub fn cntmode(&self) -> CNTMODE_R {
        CNTMODE_R::new(((self.bits >> 16) & 1) != 0)
    }
    #[doc = "Bit 17 - Leap Year Correction Disabled"]
    #[inline(always)]
    pub fn lyearcorrdis(&self) -> LYEARCORRDIS_R {
        LYEARCORRDIS_R::new(((self.bits >> 17) & 1) != 0)
    }
}
impl W {
    #[doc = "Bit 0 - RTCC Enable"]
    #[inline(always)]
    #[must_use]
    pub fn enable(&mut self) -> ENABLE_W<0> {
        ENABLE_W::new(self)
    }
    #[doc = "Bit 2 - Debug Mode Run Enable"]
    #[inline(always)]
    #[must_use]
    pub fn debugrun(&mut self) -> DEBUGRUN_W<2> {
        DEBUGRUN_W::new(self)
    }
    #[doc = "Bit 4 - Pre-counter CCV0 Top Value Enable"]
    #[inline(always)]
    #[must_use]
    pub fn preccv0top(&mut self) -> PRECCV0TOP_W<4> {
        PRECCV0TOP_W::new(self)
    }
    #[doc = "Bit 5 - CCV1 Top Value Enable"]
    #[inline(always)]
    #[must_use]
    pub fn ccv1top(&mut self) -> CCV1TOP_W<5> {
        CCV1TOP_W::new(self)
    }
    #[doc = "Bits 8:11 - Counter Prescaler Value"]
    #[inline(always)]
    #[must_use]
    pub fn cntpresc(&mut self) -> CNTPRESC_W<8> {
        CNTPRESC_W::new(self)
    }
    #[doc = "Bit 12 - Counter Prescaler Mode"]
    #[inline(always)]
    #[must_use]
    pub fn cnttick(&mut self) -> CNTTICK_W<12> {
        CNTTICK_W::new(self)
    }
    #[doc = "Bit 14 - Backup Mode Timestamp Enable"]
    #[inline(always)]
    #[must_use]
    pub fn bumodetsen(&mut self) -> BUMODETSEN_W<14> {
        BUMODETSEN_W::new(self)
    }
    #[doc = "Bit 15 - Oscillator Failure Detection Enable"]
    #[inline(always)]
    #[must_use]
    pub fn oscfdeten(&mut self) -> OSCFDETEN_W<15> {
        OSCFDETEN_W::new(self)
    }
    #[doc = "Bit 16 - Main Counter Mode"]
    #[inline(always)]
    #[must_use]
    pub fn cntmode(&mut self) -> CNTMODE_W<16> {
        CNTMODE_W::new(self)
    }
    #[doc = "Bit 17 - Leap Year Correction Disabled"]
    #[inline(always)]
    #[must_use]
    pub fn lyearcorrdis(&mut self) -> LYEARCORRDIS_W<17> {
        LYEARCORRDIS_W::new(self)
    }
    #[doc = "Writes raw bits to the register."]
    #[inline(always)]
    pub unsafe fn bits(&mut self, bits: u32) -> &mut Self {
        self.0.bits(bits);
        self
    }
}
#[doc = "Control Register\n\nThis register you can [`read`](crate::generic::Reg::read), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [ctrl](index.html) module"]
pub struct CTRL_SPEC;
impl crate::RegisterSpec for CTRL_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [ctrl::R](R) reader structure"]
impl crate::Readable for CTRL_SPEC {
    type Reader = R;
}
#[doc = "`write(|w| ..)` method takes [ctrl::W](W) writer structure"]
impl crate::Writable for CTRL_SPEC {
    type Writer = W;
    const ZERO_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
}
#[doc = "`reset()` method sets CTRL to value 0"]
impl crate::Resettable for CTRL_SPEC {
    const RESET_VALUE: Self::Ux = 0;
}
