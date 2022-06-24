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
pub type VMONAVDDFALL_W<'a> = crate::BitWriter<'a, u32, IEN_SPEC, bool, 0>;
#[doc = "Field `VMONAVDDRISE` reader - VMONAVDDRISE Interrupt Enable"]
pub type VMONAVDDRISE_R = crate::BitReader<bool>;
#[doc = "Field `VMONAVDDRISE` writer - VMONAVDDRISE Interrupt Enable"]
pub type VMONAVDDRISE_W<'a> = crate::BitWriter<'a, u32, IEN_SPEC, bool, 1>;
#[doc = "Field `VMONALTAVDDFALL` reader - VMONALTAVDDFALL Interrupt Enable"]
pub type VMONALTAVDDFALL_R = crate::BitReader<bool>;
#[doc = "Field `VMONALTAVDDFALL` writer - VMONALTAVDDFALL Interrupt Enable"]
pub type VMONALTAVDDFALL_W<'a> = crate::BitWriter<'a, u32, IEN_SPEC, bool, 2>;
#[doc = "Field `VMONALTAVDDRISE` reader - VMONALTAVDDRISE Interrupt Enable"]
pub type VMONALTAVDDRISE_R = crate::BitReader<bool>;
#[doc = "Field `VMONALTAVDDRISE` writer - VMONALTAVDDRISE Interrupt Enable"]
pub type VMONALTAVDDRISE_W<'a> = crate::BitWriter<'a, u32, IEN_SPEC, bool, 3>;
#[doc = "Field `VMONDVDDFALL` reader - VMONDVDDFALL Interrupt Enable"]
pub type VMONDVDDFALL_R = crate::BitReader<bool>;
#[doc = "Field `VMONDVDDFALL` writer - VMONDVDDFALL Interrupt Enable"]
pub type VMONDVDDFALL_W<'a> = crate::BitWriter<'a, u32, IEN_SPEC, bool, 4>;
#[doc = "Field `VMONDVDDRISE` reader - VMONDVDDRISE Interrupt Enable"]
pub type VMONDVDDRISE_R = crate::BitReader<bool>;
#[doc = "Field `VMONDVDDRISE` writer - VMONDVDDRISE Interrupt Enable"]
pub type VMONDVDDRISE_W<'a> = crate::BitWriter<'a, u32, IEN_SPEC, bool, 5>;
#[doc = "Field `VMONIO0FALL` reader - VMONIO0FALL Interrupt Enable"]
pub type VMONIO0FALL_R = crate::BitReader<bool>;
#[doc = "Field `VMONIO0FALL` writer - VMONIO0FALL Interrupt Enable"]
pub type VMONIO0FALL_W<'a> = crate::BitWriter<'a, u32, IEN_SPEC, bool, 6>;
#[doc = "Field `VMONIO0RISE` reader - VMONIO0RISE Interrupt Enable"]
pub type VMONIO0RISE_R = crate::BitReader<bool>;
#[doc = "Field `VMONIO0RISE` writer - VMONIO0RISE Interrupt Enable"]
pub type VMONIO0RISE_W<'a> = crate::BitWriter<'a, u32, IEN_SPEC, bool, 7>;
#[doc = "Field `VMONBUVDDFALL` reader - VMONBUVDDFALL Interrupt Enable"]
pub type VMONBUVDDFALL_R = crate::BitReader<bool>;
#[doc = "Field `VMONBUVDDFALL` writer - VMONBUVDDFALL Interrupt Enable"]
pub type VMONBUVDDFALL_W<'a> = crate::BitWriter<'a, u32, IEN_SPEC, bool, 12>;
#[doc = "Field `VMONBUVDDRISE` reader - VMONBUVDDRISE Interrupt Enable"]
pub type VMONBUVDDRISE_R = crate::BitReader<bool>;
#[doc = "Field `VMONBUVDDRISE` writer - VMONBUVDDRISE Interrupt Enable"]
pub type VMONBUVDDRISE_W<'a> = crate::BitWriter<'a, u32, IEN_SPEC, bool, 13>;
#[doc = "Field `VMONFVDDFALL` reader - VMONFVDDFALL Interrupt Enable"]
pub type VMONFVDDFALL_R = crate::BitReader<bool>;
#[doc = "Field `VMONFVDDFALL` writer - VMONFVDDFALL Interrupt Enable"]
pub type VMONFVDDFALL_W<'a> = crate::BitWriter<'a, u32, IEN_SPEC, bool, 14>;
#[doc = "Field `VMONFVDDRISE` reader - VMONFVDDRISE Interrupt Enable"]
pub type VMONFVDDRISE_R = crate::BitReader<bool>;
#[doc = "Field `VMONFVDDRISE` writer - VMONFVDDRISE Interrupt Enable"]
pub type VMONFVDDRISE_W<'a> = crate::BitWriter<'a, u32, IEN_SPEC, bool, 15>;
#[doc = "Field `PFETOVERCURRENTLIMIT` reader - PFETOVERCURRENTLIMIT Interrupt Enable"]
pub type PFETOVERCURRENTLIMIT_R = crate::BitReader<bool>;
#[doc = "Field `PFETOVERCURRENTLIMIT` writer - PFETOVERCURRENTLIMIT Interrupt Enable"]
pub type PFETOVERCURRENTLIMIT_W<'a> = crate::BitWriter<'a, u32, IEN_SPEC, bool, 16>;
#[doc = "Field `NFETOVERCURRENTLIMIT` reader - NFETOVERCURRENTLIMIT Interrupt Enable"]
pub type NFETOVERCURRENTLIMIT_R = crate::BitReader<bool>;
#[doc = "Field `NFETOVERCURRENTLIMIT` writer - NFETOVERCURRENTLIMIT Interrupt Enable"]
pub type NFETOVERCURRENTLIMIT_W<'a> = crate::BitWriter<'a, u32, IEN_SPEC, bool, 17>;
#[doc = "Field `DCDCLPRUNNING` reader - DCDCLPRUNNING Interrupt Enable"]
pub type DCDCLPRUNNING_R = crate::BitReader<bool>;
#[doc = "Field `DCDCLPRUNNING` writer - DCDCLPRUNNING Interrupt Enable"]
pub type DCDCLPRUNNING_W<'a> = crate::BitWriter<'a, u32, IEN_SPEC, bool, 18>;
#[doc = "Field `DCDCLNRUNNING` reader - DCDCLNRUNNING Interrupt Enable"]
pub type DCDCLNRUNNING_R = crate::BitReader<bool>;
#[doc = "Field `DCDCLNRUNNING` writer - DCDCLNRUNNING Interrupt Enable"]
pub type DCDCLNRUNNING_W<'a> = crate::BitWriter<'a, u32, IEN_SPEC, bool, 19>;
#[doc = "Field `DCDCINBYPASS` reader - DCDCINBYPASS Interrupt Enable"]
pub type DCDCINBYPASS_R = crate::BitReader<bool>;
#[doc = "Field `DCDCINBYPASS` writer - DCDCINBYPASS Interrupt Enable"]
pub type DCDCINBYPASS_W<'a> = crate::BitWriter<'a, u32, IEN_SPEC, bool, 20>;
#[doc = "Field `BURDY` reader - BURDY Interrupt Enable"]
pub type BURDY_R = crate::BitReader<bool>;
#[doc = "Field `BURDY` writer - BURDY Interrupt Enable"]
pub type BURDY_W<'a> = crate::BitWriter<'a, u32, IEN_SPEC, bool, 22>;
#[doc = "Field `EM23WAKEUP` reader - EM23WAKEUP Interrupt Enable"]
pub type EM23WAKEUP_R = crate::BitReader<bool>;
#[doc = "Field `EM23WAKEUP` writer - EM23WAKEUP Interrupt Enable"]
pub type EM23WAKEUP_W<'a> = crate::BitWriter<'a, u32, IEN_SPEC, bool, 24>;
#[doc = "Field `VSCALEDONE` reader - VSCALEDONE Interrupt Enable"]
pub type VSCALEDONE_R = crate::BitReader<bool>;
#[doc = "Field `VSCALEDONE` writer - VSCALEDONE Interrupt Enable"]
pub type VSCALEDONE_W<'a> = crate::BitWriter<'a, u32, IEN_SPEC, bool, 25>;
#[doc = "Field `TEMP` reader - TEMP Interrupt Enable"]
pub type TEMP_R = crate::BitReader<bool>;
#[doc = "Field `TEMP` writer - TEMP Interrupt Enable"]
pub type TEMP_W<'a> = crate::BitWriter<'a, u32, IEN_SPEC, bool, 29>;
#[doc = "Field `TEMPLOW` reader - TEMPLOW Interrupt Enable"]
pub type TEMPLOW_R = crate::BitReader<bool>;
#[doc = "Field `TEMPLOW` writer - TEMPLOW Interrupt Enable"]
pub type TEMPLOW_W<'a> = crate::BitWriter<'a, u32, IEN_SPEC, bool, 30>;
#[doc = "Field `TEMPHIGH` reader - TEMPHIGH Interrupt Enable"]
pub type TEMPHIGH_R = crate::BitReader<bool>;
#[doc = "Field `TEMPHIGH` writer - TEMPHIGH Interrupt Enable"]
pub type TEMPHIGH_W<'a> = crate::BitWriter<'a, u32, IEN_SPEC, bool, 31>;
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
    pub fn vmonavddfall(&mut self) -> VMONAVDDFALL_W {
        VMONAVDDFALL_W::new(self)
    }
    #[doc = "Bit 1 - VMONAVDDRISE Interrupt Enable"]
    #[inline(always)]
    pub fn vmonavddrise(&mut self) -> VMONAVDDRISE_W {
        VMONAVDDRISE_W::new(self)
    }
    #[doc = "Bit 2 - VMONALTAVDDFALL Interrupt Enable"]
    #[inline(always)]
    pub fn vmonaltavddfall(&mut self) -> VMONALTAVDDFALL_W {
        VMONALTAVDDFALL_W::new(self)
    }
    #[doc = "Bit 3 - VMONALTAVDDRISE Interrupt Enable"]
    #[inline(always)]
    pub fn vmonaltavddrise(&mut self) -> VMONALTAVDDRISE_W {
        VMONALTAVDDRISE_W::new(self)
    }
    #[doc = "Bit 4 - VMONDVDDFALL Interrupt Enable"]
    #[inline(always)]
    pub fn vmondvddfall(&mut self) -> VMONDVDDFALL_W {
        VMONDVDDFALL_W::new(self)
    }
    #[doc = "Bit 5 - VMONDVDDRISE Interrupt Enable"]
    #[inline(always)]
    pub fn vmondvddrise(&mut self) -> VMONDVDDRISE_W {
        VMONDVDDRISE_W::new(self)
    }
    #[doc = "Bit 6 - VMONIO0FALL Interrupt Enable"]
    #[inline(always)]
    pub fn vmonio0fall(&mut self) -> VMONIO0FALL_W {
        VMONIO0FALL_W::new(self)
    }
    #[doc = "Bit 7 - VMONIO0RISE Interrupt Enable"]
    #[inline(always)]
    pub fn vmonio0rise(&mut self) -> VMONIO0RISE_W {
        VMONIO0RISE_W::new(self)
    }
    #[doc = "Bit 12 - VMONBUVDDFALL Interrupt Enable"]
    #[inline(always)]
    pub fn vmonbuvddfall(&mut self) -> VMONBUVDDFALL_W {
        VMONBUVDDFALL_W::new(self)
    }
    #[doc = "Bit 13 - VMONBUVDDRISE Interrupt Enable"]
    #[inline(always)]
    pub fn vmonbuvddrise(&mut self) -> VMONBUVDDRISE_W {
        VMONBUVDDRISE_W::new(self)
    }
    #[doc = "Bit 14 - VMONFVDDFALL Interrupt Enable"]
    #[inline(always)]
    pub fn vmonfvddfall(&mut self) -> VMONFVDDFALL_W {
        VMONFVDDFALL_W::new(self)
    }
    #[doc = "Bit 15 - VMONFVDDRISE Interrupt Enable"]
    #[inline(always)]
    pub fn vmonfvddrise(&mut self) -> VMONFVDDRISE_W {
        VMONFVDDRISE_W::new(self)
    }
    #[doc = "Bit 16 - PFETOVERCURRENTLIMIT Interrupt Enable"]
    #[inline(always)]
    pub fn pfetovercurrentlimit(&mut self) -> PFETOVERCURRENTLIMIT_W {
        PFETOVERCURRENTLIMIT_W::new(self)
    }
    #[doc = "Bit 17 - NFETOVERCURRENTLIMIT Interrupt Enable"]
    #[inline(always)]
    pub fn nfetovercurrentlimit(&mut self) -> NFETOVERCURRENTLIMIT_W {
        NFETOVERCURRENTLIMIT_W::new(self)
    }
    #[doc = "Bit 18 - DCDCLPRUNNING Interrupt Enable"]
    #[inline(always)]
    pub fn dcdclprunning(&mut self) -> DCDCLPRUNNING_W {
        DCDCLPRUNNING_W::new(self)
    }
    #[doc = "Bit 19 - DCDCLNRUNNING Interrupt Enable"]
    #[inline(always)]
    pub fn dcdclnrunning(&mut self) -> DCDCLNRUNNING_W {
        DCDCLNRUNNING_W::new(self)
    }
    #[doc = "Bit 20 - DCDCINBYPASS Interrupt Enable"]
    #[inline(always)]
    pub fn dcdcinbypass(&mut self) -> DCDCINBYPASS_W {
        DCDCINBYPASS_W::new(self)
    }
    #[doc = "Bit 22 - BURDY Interrupt Enable"]
    #[inline(always)]
    pub fn burdy(&mut self) -> BURDY_W {
        BURDY_W::new(self)
    }
    #[doc = "Bit 24 - EM23WAKEUP Interrupt Enable"]
    #[inline(always)]
    pub fn em23wakeup(&mut self) -> EM23WAKEUP_W {
        EM23WAKEUP_W::new(self)
    }
    #[doc = "Bit 25 - VSCALEDONE Interrupt Enable"]
    #[inline(always)]
    pub fn vscaledone(&mut self) -> VSCALEDONE_W {
        VSCALEDONE_W::new(self)
    }
    #[doc = "Bit 29 - TEMP Interrupt Enable"]
    #[inline(always)]
    pub fn temp(&mut self) -> TEMP_W {
        TEMP_W::new(self)
    }
    #[doc = "Bit 30 - TEMPLOW Interrupt Enable"]
    #[inline(always)]
    pub fn templow(&mut self) -> TEMPLOW_W {
        TEMPLOW_W::new(self)
    }
    #[doc = "Bit 31 - TEMPHIGH Interrupt Enable"]
    #[inline(always)]
    pub fn temphigh(&mut self) -> TEMPHIGH_W {
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
}
#[doc = "`reset()` method sets IEN to value 0"]
impl crate::Resettable for IEN_SPEC {
    #[inline(always)]
    fn reset_value() -> Self::Ux {
        0
    }
}
