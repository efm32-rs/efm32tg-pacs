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
#[doc = "Field `VMONAVDDFALL` reader - VMONAVDDFALL Interrupt Enable"]
pub type VMONAVDDFALL_R = crate::BitReader<bool>;
#[doc = "Field `VMONAVDDFALL` writer - VMONAVDDFALL Interrupt Enable"]
pub type VMONAVDDFALL_W<'a, const O: u8> = crate::BitWriter<'a, u32, IEN_SPEC, bool, O>;
#[doc = "Field `VMONAVDDRISE` reader - VMONAVDDRISE Interrupt Enable"]
pub type VMONAVDDRISE_R = crate::BitReader<bool>;
#[doc = "Field `VMONAVDDRISE` writer - VMONAVDDRISE Interrupt Enable"]
pub type VMONAVDDRISE_W<'a, const O: u8> = crate::BitWriter<'a, u32, IEN_SPEC, bool, O>;
#[doc = "Field `VMONALTAVDDFALL` reader - VMONALTAVDDFALL Interrupt Enable"]
pub type VMONALTAVDDFALL_R = crate::BitReader<bool>;
#[doc = "Field `VMONALTAVDDFALL` writer - VMONALTAVDDFALL Interrupt Enable"]
pub type VMONALTAVDDFALL_W<'a, const O: u8> = crate::BitWriter<'a, u32, IEN_SPEC, bool, O>;
#[doc = "Field `VMONALTAVDDRISE` reader - VMONALTAVDDRISE Interrupt Enable"]
pub type VMONALTAVDDRISE_R = crate::BitReader<bool>;
#[doc = "Field `VMONALTAVDDRISE` writer - VMONALTAVDDRISE Interrupt Enable"]
pub type VMONALTAVDDRISE_W<'a, const O: u8> = crate::BitWriter<'a, u32, IEN_SPEC, bool, O>;
#[doc = "Field `VMONDVDDFALL` reader - VMONDVDDFALL Interrupt Enable"]
pub type VMONDVDDFALL_R = crate::BitReader<bool>;
#[doc = "Field `VMONDVDDFALL` writer - VMONDVDDFALL Interrupt Enable"]
pub type VMONDVDDFALL_W<'a, const O: u8> = crate::BitWriter<'a, u32, IEN_SPEC, bool, O>;
#[doc = "Field `VMONDVDDRISE` reader - VMONDVDDRISE Interrupt Enable"]
pub type VMONDVDDRISE_R = crate::BitReader<bool>;
#[doc = "Field `VMONDVDDRISE` writer - VMONDVDDRISE Interrupt Enable"]
pub type VMONDVDDRISE_W<'a, const O: u8> = crate::BitWriter<'a, u32, IEN_SPEC, bool, O>;
#[doc = "Field `VMONIO0FALL` reader - VMONIO0FALL Interrupt Enable"]
pub type VMONIO0FALL_R = crate::BitReader<bool>;
#[doc = "Field `VMONIO0FALL` writer - VMONIO0FALL Interrupt Enable"]
pub type VMONIO0FALL_W<'a, const O: u8> = crate::BitWriter<'a, u32, IEN_SPEC, bool, O>;
#[doc = "Field `VMONIO0RISE` reader - VMONIO0RISE Interrupt Enable"]
pub type VMONIO0RISE_R = crate::BitReader<bool>;
#[doc = "Field `VMONIO0RISE` writer - VMONIO0RISE Interrupt Enable"]
pub type VMONIO0RISE_W<'a, const O: u8> = crate::BitWriter<'a, u32, IEN_SPEC, bool, O>;
#[doc = "Field `VMONBUVDDFALL` reader - VMONBUVDDFALL Interrupt Enable"]
pub type VMONBUVDDFALL_R = crate::BitReader<bool>;
#[doc = "Field `VMONBUVDDFALL` writer - VMONBUVDDFALL Interrupt Enable"]
pub type VMONBUVDDFALL_W<'a, const O: u8> = crate::BitWriter<'a, u32, IEN_SPEC, bool, O>;
#[doc = "Field `VMONBUVDDRISE` reader - VMONBUVDDRISE Interrupt Enable"]
pub type VMONBUVDDRISE_R = crate::BitReader<bool>;
#[doc = "Field `VMONBUVDDRISE` writer - VMONBUVDDRISE Interrupt Enable"]
pub type VMONBUVDDRISE_W<'a, const O: u8> = crate::BitWriter<'a, u32, IEN_SPEC, bool, O>;
#[doc = "Field `VMONFVDDFALL` reader - VMONFVDDFALL Interrupt Enable"]
pub type VMONFVDDFALL_R = crate::BitReader<bool>;
#[doc = "Field `VMONFVDDFALL` writer - VMONFVDDFALL Interrupt Enable"]
pub type VMONFVDDFALL_W<'a, const O: u8> = crate::BitWriter<'a, u32, IEN_SPEC, bool, O>;
#[doc = "Field `VMONFVDDRISE` reader - VMONFVDDRISE Interrupt Enable"]
pub type VMONFVDDRISE_R = crate::BitReader<bool>;
#[doc = "Field `VMONFVDDRISE` writer - VMONFVDDRISE Interrupt Enable"]
pub type VMONFVDDRISE_W<'a, const O: u8> = crate::BitWriter<'a, u32, IEN_SPEC, bool, O>;
#[doc = "Field `PFETOVERCURRENTLIMIT` reader - PFETOVERCURRENTLIMIT Interrupt Enable"]
pub type PFETOVERCURRENTLIMIT_R = crate::BitReader<bool>;
#[doc = "Field `PFETOVERCURRENTLIMIT` writer - PFETOVERCURRENTLIMIT Interrupt Enable"]
pub type PFETOVERCURRENTLIMIT_W<'a, const O: u8> = crate::BitWriter<'a, u32, IEN_SPEC, bool, O>;
#[doc = "Field `NFETOVERCURRENTLIMIT` reader - NFETOVERCURRENTLIMIT Interrupt Enable"]
pub type NFETOVERCURRENTLIMIT_R = crate::BitReader<bool>;
#[doc = "Field `NFETOVERCURRENTLIMIT` writer - NFETOVERCURRENTLIMIT Interrupt Enable"]
pub type NFETOVERCURRENTLIMIT_W<'a, const O: u8> = crate::BitWriter<'a, u32, IEN_SPEC, bool, O>;
#[doc = "Field `DCDCLPRUNNING` reader - DCDCLPRUNNING Interrupt Enable"]
pub type DCDCLPRUNNING_R = crate::BitReader<bool>;
#[doc = "Field `DCDCLPRUNNING` writer - DCDCLPRUNNING Interrupt Enable"]
pub type DCDCLPRUNNING_W<'a, const O: u8> = crate::BitWriter<'a, u32, IEN_SPEC, bool, O>;
#[doc = "Field `DCDCLNRUNNING` reader - DCDCLNRUNNING Interrupt Enable"]
pub type DCDCLNRUNNING_R = crate::BitReader<bool>;
#[doc = "Field `DCDCLNRUNNING` writer - DCDCLNRUNNING Interrupt Enable"]
pub type DCDCLNRUNNING_W<'a, const O: u8> = crate::BitWriter<'a, u32, IEN_SPEC, bool, O>;
#[doc = "Field `DCDCINBYPASS` reader - DCDCINBYPASS Interrupt Enable"]
pub type DCDCINBYPASS_R = crate::BitReader<bool>;
#[doc = "Field `DCDCINBYPASS` writer - DCDCINBYPASS Interrupt Enable"]
pub type DCDCINBYPASS_W<'a, const O: u8> = crate::BitWriter<'a, u32, IEN_SPEC, bool, O>;
#[doc = "Field `BURDY` reader - BURDY Interrupt Enable"]
pub type BURDY_R = crate::BitReader<bool>;
#[doc = "Field `BURDY` writer - BURDY Interrupt Enable"]
pub type BURDY_W<'a, const O: u8> = crate::BitWriter<'a, u32, IEN_SPEC, bool, O>;
#[doc = "Field `EM23WAKEUP` reader - EM23WAKEUP Interrupt Enable"]
pub type EM23WAKEUP_R = crate::BitReader<bool>;
#[doc = "Field `EM23WAKEUP` writer - EM23WAKEUP Interrupt Enable"]
pub type EM23WAKEUP_W<'a, const O: u8> = crate::BitWriter<'a, u32, IEN_SPEC, bool, O>;
#[doc = "Field `VSCALEDONE` reader - VSCALEDONE Interrupt Enable"]
pub type VSCALEDONE_R = crate::BitReader<bool>;
#[doc = "Field `VSCALEDONE` writer - VSCALEDONE Interrupt Enable"]
pub type VSCALEDONE_W<'a, const O: u8> = crate::BitWriter<'a, u32, IEN_SPEC, bool, O>;
#[doc = "Field `TEMP` reader - TEMP Interrupt Enable"]
pub type TEMP_R = crate::BitReader<bool>;
#[doc = "Field `TEMP` writer - TEMP Interrupt Enable"]
pub type TEMP_W<'a, const O: u8> = crate::BitWriter<'a, u32, IEN_SPEC, bool, O>;
#[doc = "Field `TEMPLOW` reader - TEMPLOW Interrupt Enable"]
pub type TEMPLOW_R = crate::BitReader<bool>;
#[doc = "Field `TEMPLOW` writer - TEMPLOW Interrupt Enable"]
pub type TEMPLOW_W<'a, const O: u8> = crate::BitWriter<'a, u32, IEN_SPEC, bool, O>;
#[doc = "Field `TEMPHIGH` reader - TEMPHIGH Interrupt Enable"]
pub type TEMPHIGH_R = crate::BitReader<bool>;
#[doc = "Field `TEMPHIGH` writer - TEMPHIGH Interrupt Enable"]
pub type TEMPHIGH_W<'a, const O: u8> = crate::BitWriter<'a, u32, IEN_SPEC, bool, O>;
impl R {
    #[doc = "Bit 0 - VMONAVDDFALL Interrupt Enable"]
    #[inline(always)]
    pub fn vmonavddfall(&self) -> VMONAVDDFALL_R {
        VMONAVDDFALL_R::new((self.bits & 1) != 0)
    }
    #[doc = "Bit 1 - VMONAVDDRISE Interrupt Enable"]
    #[inline(always)]
    pub fn vmonavddrise(&self) -> VMONAVDDRISE_R {
        VMONAVDDRISE_R::new(((self.bits >> 1) & 1) != 0)
    }
    #[doc = "Bit 2 - VMONALTAVDDFALL Interrupt Enable"]
    #[inline(always)]
    pub fn vmonaltavddfall(&self) -> VMONALTAVDDFALL_R {
        VMONALTAVDDFALL_R::new(((self.bits >> 2) & 1) != 0)
    }
    #[doc = "Bit 3 - VMONALTAVDDRISE Interrupt Enable"]
    #[inline(always)]
    pub fn vmonaltavddrise(&self) -> VMONALTAVDDRISE_R {
        VMONALTAVDDRISE_R::new(((self.bits >> 3) & 1) != 0)
    }
    #[doc = "Bit 4 - VMONDVDDFALL Interrupt Enable"]
    #[inline(always)]
    pub fn vmondvddfall(&self) -> VMONDVDDFALL_R {
        VMONDVDDFALL_R::new(((self.bits >> 4) & 1) != 0)
    }
    #[doc = "Bit 5 - VMONDVDDRISE Interrupt Enable"]
    #[inline(always)]
    pub fn vmondvddrise(&self) -> VMONDVDDRISE_R {
        VMONDVDDRISE_R::new(((self.bits >> 5) & 1) != 0)
    }
    #[doc = "Bit 6 - VMONIO0FALL Interrupt Enable"]
    #[inline(always)]
    pub fn vmonio0fall(&self) -> VMONIO0FALL_R {
        VMONIO0FALL_R::new(((self.bits >> 6) & 1) != 0)
    }
    #[doc = "Bit 7 - VMONIO0RISE Interrupt Enable"]
    #[inline(always)]
    pub fn vmonio0rise(&self) -> VMONIO0RISE_R {
        VMONIO0RISE_R::new(((self.bits >> 7) & 1) != 0)
    }
    #[doc = "Bit 12 - VMONBUVDDFALL Interrupt Enable"]
    #[inline(always)]
    pub fn vmonbuvddfall(&self) -> VMONBUVDDFALL_R {
        VMONBUVDDFALL_R::new(((self.bits >> 12) & 1) != 0)
    }
    #[doc = "Bit 13 - VMONBUVDDRISE Interrupt Enable"]
    #[inline(always)]
    pub fn vmonbuvddrise(&self) -> VMONBUVDDRISE_R {
        VMONBUVDDRISE_R::new(((self.bits >> 13) & 1) != 0)
    }
    #[doc = "Bit 14 - VMONFVDDFALL Interrupt Enable"]
    #[inline(always)]
    pub fn vmonfvddfall(&self) -> VMONFVDDFALL_R {
        VMONFVDDFALL_R::new(((self.bits >> 14) & 1) != 0)
    }
    #[doc = "Bit 15 - VMONFVDDRISE Interrupt Enable"]
    #[inline(always)]
    pub fn vmonfvddrise(&self) -> VMONFVDDRISE_R {
        VMONFVDDRISE_R::new(((self.bits >> 15) & 1) != 0)
    }
    #[doc = "Bit 16 - PFETOVERCURRENTLIMIT Interrupt Enable"]
    #[inline(always)]
    pub fn pfetovercurrentlimit(&self) -> PFETOVERCURRENTLIMIT_R {
        PFETOVERCURRENTLIMIT_R::new(((self.bits >> 16) & 1) != 0)
    }
    #[doc = "Bit 17 - NFETOVERCURRENTLIMIT Interrupt Enable"]
    #[inline(always)]
    pub fn nfetovercurrentlimit(&self) -> NFETOVERCURRENTLIMIT_R {
        NFETOVERCURRENTLIMIT_R::new(((self.bits >> 17) & 1) != 0)
    }
    #[doc = "Bit 18 - DCDCLPRUNNING Interrupt Enable"]
    #[inline(always)]
    pub fn dcdclprunning(&self) -> DCDCLPRUNNING_R {
        DCDCLPRUNNING_R::new(((self.bits >> 18) & 1) != 0)
    }
    #[doc = "Bit 19 - DCDCLNRUNNING Interrupt Enable"]
    #[inline(always)]
    pub fn dcdclnrunning(&self) -> DCDCLNRUNNING_R {
        DCDCLNRUNNING_R::new(((self.bits >> 19) & 1) != 0)
    }
    #[doc = "Bit 20 - DCDCINBYPASS Interrupt Enable"]
    #[inline(always)]
    pub fn dcdcinbypass(&self) -> DCDCINBYPASS_R {
        DCDCINBYPASS_R::new(((self.bits >> 20) & 1) != 0)
    }
    #[doc = "Bit 22 - BURDY Interrupt Enable"]
    #[inline(always)]
    pub fn burdy(&self) -> BURDY_R {
        BURDY_R::new(((self.bits >> 22) & 1) != 0)
    }
    #[doc = "Bit 24 - EM23WAKEUP Interrupt Enable"]
    #[inline(always)]
    pub fn em23wakeup(&self) -> EM23WAKEUP_R {
        EM23WAKEUP_R::new(((self.bits >> 24) & 1) != 0)
    }
    #[doc = "Bit 25 - VSCALEDONE Interrupt Enable"]
    #[inline(always)]
    pub fn vscaledone(&self) -> VSCALEDONE_R {
        VSCALEDONE_R::new(((self.bits >> 25) & 1) != 0)
    }
    #[doc = "Bit 29 - TEMP Interrupt Enable"]
    #[inline(always)]
    pub fn temp(&self) -> TEMP_R {
        TEMP_R::new(((self.bits >> 29) & 1) != 0)
    }
    #[doc = "Bit 30 - TEMPLOW Interrupt Enable"]
    #[inline(always)]
    pub fn templow(&self) -> TEMPLOW_R {
        TEMPLOW_R::new(((self.bits >> 30) & 1) != 0)
    }
    #[doc = "Bit 31 - TEMPHIGH Interrupt Enable"]
    #[inline(always)]
    pub fn temphigh(&self) -> TEMPHIGH_R {
        TEMPHIGH_R::new(((self.bits >> 31) & 1) != 0)
    }
}
impl W {
    #[doc = "Bit 0 - VMONAVDDFALL Interrupt Enable"]
    #[inline(always)]
    #[must_use]
    pub fn vmonavddfall(&mut self) -> VMONAVDDFALL_W<0> {
        VMONAVDDFALL_W::new(self)
    }
    #[doc = "Bit 1 - VMONAVDDRISE Interrupt Enable"]
    #[inline(always)]
    #[must_use]
    pub fn vmonavddrise(&mut self) -> VMONAVDDRISE_W<1> {
        VMONAVDDRISE_W::new(self)
    }
    #[doc = "Bit 2 - VMONALTAVDDFALL Interrupt Enable"]
    #[inline(always)]
    #[must_use]
    pub fn vmonaltavddfall(&mut self) -> VMONALTAVDDFALL_W<2> {
        VMONALTAVDDFALL_W::new(self)
    }
    #[doc = "Bit 3 - VMONALTAVDDRISE Interrupt Enable"]
    #[inline(always)]
    #[must_use]
    pub fn vmonaltavddrise(&mut self) -> VMONALTAVDDRISE_W<3> {
        VMONALTAVDDRISE_W::new(self)
    }
    #[doc = "Bit 4 - VMONDVDDFALL Interrupt Enable"]
    #[inline(always)]
    #[must_use]
    pub fn vmondvddfall(&mut self) -> VMONDVDDFALL_W<4> {
        VMONDVDDFALL_W::new(self)
    }
    #[doc = "Bit 5 - VMONDVDDRISE Interrupt Enable"]
    #[inline(always)]
    #[must_use]
    pub fn vmondvddrise(&mut self) -> VMONDVDDRISE_W<5> {
        VMONDVDDRISE_W::new(self)
    }
    #[doc = "Bit 6 - VMONIO0FALL Interrupt Enable"]
    #[inline(always)]
    #[must_use]
    pub fn vmonio0fall(&mut self) -> VMONIO0FALL_W<6> {
        VMONIO0FALL_W::new(self)
    }
    #[doc = "Bit 7 - VMONIO0RISE Interrupt Enable"]
    #[inline(always)]
    #[must_use]
    pub fn vmonio0rise(&mut self) -> VMONIO0RISE_W<7> {
        VMONIO0RISE_W::new(self)
    }
    #[doc = "Bit 12 - VMONBUVDDFALL Interrupt Enable"]
    #[inline(always)]
    #[must_use]
    pub fn vmonbuvddfall(&mut self) -> VMONBUVDDFALL_W<12> {
        VMONBUVDDFALL_W::new(self)
    }
    #[doc = "Bit 13 - VMONBUVDDRISE Interrupt Enable"]
    #[inline(always)]
    #[must_use]
    pub fn vmonbuvddrise(&mut self) -> VMONBUVDDRISE_W<13> {
        VMONBUVDDRISE_W::new(self)
    }
    #[doc = "Bit 14 - VMONFVDDFALL Interrupt Enable"]
    #[inline(always)]
    #[must_use]
    pub fn vmonfvddfall(&mut self) -> VMONFVDDFALL_W<14> {
        VMONFVDDFALL_W::new(self)
    }
    #[doc = "Bit 15 - VMONFVDDRISE Interrupt Enable"]
    #[inline(always)]
    #[must_use]
    pub fn vmonfvddrise(&mut self) -> VMONFVDDRISE_W<15> {
        VMONFVDDRISE_W::new(self)
    }
    #[doc = "Bit 16 - PFETOVERCURRENTLIMIT Interrupt Enable"]
    #[inline(always)]
    #[must_use]
    pub fn pfetovercurrentlimit(&mut self) -> PFETOVERCURRENTLIMIT_W<16> {
        PFETOVERCURRENTLIMIT_W::new(self)
    }
    #[doc = "Bit 17 - NFETOVERCURRENTLIMIT Interrupt Enable"]
    #[inline(always)]
    #[must_use]
    pub fn nfetovercurrentlimit(&mut self) -> NFETOVERCURRENTLIMIT_W<17> {
        NFETOVERCURRENTLIMIT_W::new(self)
    }
    #[doc = "Bit 18 - DCDCLPRUNNING Interrupt Enable"]
    #[inline(always)]
    #[must_use]
    pub fn dcdclprunning(&mut self) -> DCDCLPRUNNING_W<18> {
        DCDCLPRUNNING_W::new(self)
    }
    #[doc = "Bit 19 - DCDCLNRUNNING Interrupt Enable"]
    #[inline(always)]
    #[must_use]
    pub fn dcdclnrunning(&mut self) -> DCDCLNRUNNING_W<19> {
        DCDCLNRUNNING_W::new(self)
    }
    #[doc = "Bit 20 - DCDCINBYPASS Interrupt Enable"]
    #[inline(always)]
    #[must_use]
    pub fn dcdcinbypass(&mut self) -> DCDCINBYPASS_W<20> {
        DCDCINBYPASS_W::new(self)
    }
    #[doc = "Bit 22 - BURDY Interrupt Enable"]
    #[inline(always)]
    #[must_use]
    pub fn burdy(&mut self) -> BURDY_W<22> {
        BURDY_W::new(self)
    }
    #[doc = "Bit 24 - EM23WAKEUP Interrupt Enable"]
    #[inline(always)]
    #[must_use]
    pub fn em23wakeup(&mut self) -> EM23WAKEUP_W<24> {
        EM23WAKEUP_W::new(self)
    }
    #[doc = "Bit 25 - VSCALEDONE Interrupt Enable"]
    #[inline(always)]
    #[must_use]
    pub fn vscaledone(&mut self) -> VSCALEDONE_W<25> {
        VSCALEDONE_W::new(self)
    }
    #[doc = "Bit 29 - TEMP Interrupt Enable"]
    #[inline(always)]
    #[must_use]
    pub fn temp(&mut self) -> TEMP_W<29> {
        TEMP_W::new(self)
    }
    #[doc = "Bit 30 - TEMPLOW Interrupt Enable"]
    #[inline(always)]
    #[must_use]
    pub fn templow(&mut self) -> TEMPLOW_W<30> {
        TEMPLOW_W::new(self)
    }
    #[doc = "Bit 31 - TEMPHIGH Interrupt Enable"]
    #[inline(always)]
    #[must_use]
    pub fn temphigh(&mut self) -> TEMPHIGH_W<31> {
        TEMPHIGH_W::new(self)
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
    const ZERO_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
}
#[doc = "`reset()` method sets IEN to value 0"]
impl crate::Resettable for IEN_SPEC {
    const RESET_VALUE: Self::Ux = 0;
}
