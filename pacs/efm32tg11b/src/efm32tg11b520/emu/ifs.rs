#[doc = "Register `IFS` writer"]
pub struct W(crate::W<IFS_SPEC>);
impl core::ops::Deref for W {
    type Target = crate::W<IFS_SPEC>;
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
impl From<crate::W<IFS_SPEC>> for W {
    #[inline(always)]
    fn from(writer: crate::W<IFS_SPEC>) -> Self {
        W(writer)
    }
}
#[doc = "Field `VMONAVDDFALL` writer - Set VMONAVDDFALL Interrupt Flag"]
pub type VMONAVDDFALL_W<'a> = crate::BitWriter<'a, u32, IFS_SPEC, bool, 0>;
#[doc = "Field `VMONAVDDRISE` writer - Set VMONAVDDRISE Interrupt Flag"]
pub type VMONAVDDRISE_W<'a> = crate::BitWriter<'a, u32, IFS_SPEC, bool, 1>;
#[doc = "Field `VMONALTAVDDFALL` writer - Set VMONALTAVDDFALL Interrupt Flag"]
pub type VMONALTAVDDFALL_W<'a> = crate::BitWriter<'a, u32, IFS_SPEC, bool, 2>;
#[doc = "Field `VMONALTAVDDRISE` writer - Set VMONALTAVDDRISE Interrupt Flag"]
pub type VMONALTAVDDRISE_W<'a> = crate::BitWriter<'a, u32, IFS_SPEC, bool, 3>;
#[doc = "Field `VMONDVDDFALL` writer - Set VMONDVDDFALL Interrupt Flag"]
pub type VMONDVDDFALL_W<'a> = crate::BitWriter<'a, u32, IFS_SPEC, bool, 4>;
#[doc = "Field `VMONDVDDRISE` writer - Set VMONDVDDRISE Interrupt Flag"]
pub type VMONDVDDRISE_W<'a> = crate::BitWriter<'a, u32, IFS_SPEC, bool, 5>;
#[doc = "Field `VMONIO0FALL` writer - Set VMONIO0FALL Interrupt Flag"]
pub type VMONIO0FALL_W<'a> = crate::BitWriter<'a, u32, IFS_SPEC, bool, 6>;
#[doc = "Field `VMONIO0RISE` writer - Set VMONIO0RISE Interrupt Flag"]
pub type VMONIO0RISE_W<'a> = crate::BitWriter<'a, u32, IFS_SPEC, bool, 7>;
#[doc = "Field `VMONBUVDDFALL` writer - Set VMONBUVDDFALL Interrupt Flag"]
pub type VMONBUVDDFALL_W<'a> = crate::BitWriter<'a, u32, IFS_SPEC, bool, 12>;
#[doc = "Field `VMONBUVDDRISE` writer - Set VMONBUVDDRISE Interrupt Flag"]
pub type VMONBUVDDRISE_W<'a> = crate::BitWriter<'a, u32, IFS_SPEC, bool, 13>;
#[doc = "Field `VMONFVDDFALL` writer - Set VMONFVDDFALL Interrupt Flag"]
pub type VMONFVDDFALL_W<'a> = crate::BitWriter<'a, u32, IFS_SPEC, bool, 14>;
#[doc = "Field `VMONFVDDRISE` writer - Set VMONFVDDRISE Interrupt Flag"]
pub type VMONFVDDRISE_W<'a> = crate::BitWriter<'a, u32, IFS_SPEC, bool, 15>;
#[doc = "Field `PFETOVERCURRENTLIMIT` writer - Set PFETOVERCURRENTLIMIT Interrupt Flag"]
pub type PFETOVERCURRENTLIMIT_W<'a> = crate::BitWriter<'a, u32, IFS_SPEC, bool, 16>;
#[doc = "Field `NFETOVERCURRENTLIMIT` writer - Set NFETOVERCURRENTLIMIT Interrupt Flag"]
pub type NFETOVERCURRENTLIMIT_W<'a> = crate::BitWriter<'a, u32, IFS_SPEC, bool, 17>;
#[doc = "Field `DCDCLPRUNNING` writer - Set DCDCLPRUNNING Interrupt Flag"]
pub type DCDCLPRUNNING_W<'a> = crate::BitWriter<'a, u32, IFS_SPEC, bool, 18>;
#[doc = "Field `DCDCLNRUNNING` writer - Set DCDCLNRUNNING Interrupt Flag"]
pub type DCDCLNRUNNING_W<'a> = crate::BitWriter<'a, u32, IFS_SPEC, bool, 19>;
#[doc = "Field `DCDCINBYPASS` writer - Set DCDCINBYPASS Interrupt Flag"]
pub type DCDCINBYPASS_W<'a> = crate::BitWriter<'a, u32, IFS_SPEC, bool, 20>;
#[doc = "Field `BURDY` writer - Set BURDY Interrupt Flag"]
pub type BURDY_W<'a> = crate::BitWriter<'a, u32, IFS_SPEC, bool, 22>;
#[doc = "Field `EM23WAKEUP` writer - Set EM23WAKEUP Interrupt Flag"]
pub type EM23WAKEUP_W<'a> = crate::BitWriter<'a, u32, IFS_SPEC, bool, 24>;
#[doc = "Field `VSCALEDONE` writer - Set VSCALEDONE Interrupt Flag"]
pub type VSCALEDONE_W<'a> = crate::BitWriter<'a, u32, IFS_SPEC, bool, 25>;
#[doc = "Field `TEMP` writer - Set TEMP Interrupt Flag"]
pub type TEMP_W<'a> = crate::BitWriter<'a, u32, IFS_SPEC, bool, 29>;
#[doc = "Field `TEMPLOW` writer - Set TEMPLOW Interrupt Flag"]
pub type TEMPLOW_W<'a> = crate::BitWriter<'a, u32, IFS_SPEC, bool, 30>;
#[doc = "Field `TEMPHIGH` writer - Set TEMPHIGH Interrupt Flag"]
pub type TEMPHIGH_W<'a> = crate::BitWriter<'a, u32, IFS_SPEC, bool, 31>;
impl W {
    #[doc = "Bit 0 - Set VMONAVDDFALL Interrupt Flag"]
    #[inline(always)]
    pub fn vmonavddfall(&mut self) -> VMONAVDDFALL_W {
        VMONAVDDFALL_W::new(self)
    }
    #[doc = "Bit 1 - Set VMONAVDDRISE Interrupt Flag"]
    #[inline(always)]
    pub fn vmonavddrise(&mut self) -> VMONAVDDRISE_W {
        VMONAVDDRISE_W::new(self)
    }
    #[doc = "Bit 2 - Set VMONALTAVDDFALL Interrupt Flag"]
    #[inline(always)]
    pub fn vmonaltavddfall(&mut self) -> VMONALTAVDDFALL_W {
        VMONALTAVDDFALL_W::new(self)
    }
    #[doc = "Bit 3 - Set VMONALTAVDDRISE Interrupt Flag"]
    #[inline(always)]
    pub fn vmonaltavddrise(&mut self) -> VMONALTAVDDRISE_W {
        VMONALTAVDDRISE_W::new(self)
    }
    #[doc = "Bit 4 - Set VMONDVDDFALL Interrupt Flag"]
    #[inline(always)]
    pub fn vmondvddfall(&mut self) -> VMONDVDDFALL_W {
        VMONDVDDFALL_W::new(self)
    }
    #[doc = "Bit 5 - Set VMONDVDDRISE Interrupt Flag"]
    #[inline(always)]
    pub fn vmondvddrise(&mut self) -> VMONDVDDRISE_W {
        VMONDVDDRISE_W::new(self)
    }
    #[doc = "Bit 6 - Set VMONIO0FALL Interrupt Flag"]
    #[inline(always)]
    pub fn vmonio0fall(&mut self) -> VMONIO0FALL_W {
        VMONIO0FALL_W::new(self)
    }
    #[doc = "Bit 7 - Set VMONIO0RISE Interrupt Flag"]
    #[inline(always)]
    pub fn vmonio0rise(&mut self) -> VMONIO0RISE_W {
        VMONIO0RISE_W::new(self)
    }
    #[doc = "Bit 12 - Set VMONBUVDDFALL Interrupt Flag"]
    #[inline(always)]
    pub fn vmonbuvddfall(&mut self) -> VMONBUVDDFALL_W {
        VMONBUVDDFALL_W::new(self)
    }
    #[doc = "Bit 13 - Set VMONBUVDDRISE Interrupt Flag"]
    #[inline(always)]
    pub fn vmonbuvddrise(&mut self) -> VMONBUVDDRISE_W {
        VMONBUVDDRISE_W::new(self)
    }
    #[doc = "Bit 14 - Set VMONFVDDFALL Interrupt Flag"]
    #[inline(always)]
    pub fn vmonfvddfall(&mut self) -> VMONFVDDFALL_W {
        VMONFVDDFALL_W::new(self)
    }
    #[doc = "Bit 15 - Set VMONFVDDRISE Interrupt Flag"]
    #[inline(always)]
    pub fn vmonfvddrise(&mut self) -> VMONFVDDRISE_W {
        VMONFVDDRISE_W::new(self)
    }
    #[doc = "Bit 16 - Set PFETOVERCURRENTLIMIT Interrupt Flag"]
    #[inline(always)]
    pub fn pfetovercurrentlimit(&mut self) -> PFETOVERCURRENTLIMIT_W {
        PFETOVERCURRENTLIMIT_W::new(self)
    }
    #[doc = "Bit 17 - Set NFETOVERCURRENTLIMIT Interrupt Flag"]
    #[inline(always)]
    pub fn nfetovercurrentlimit(&mut self) -> NFETOVERCURRENTLIMIT_W {
        NFETOVERCURRENTLIMIT_W::new(self)
    }
    #[doc = "Bit 18 - Set DCDCLPRUNNING Interrupt Flag"]
    #[inline(always)]
    pub fn dcdclprunning(&mut self) -> DCDCLPRUNNING_W {
        DCDCLPRUNNING_W::new(self)
    }
    #[doc = "Bit 19 - Set DCDCLNRUNNING Interrupt Flag"]
    #[inline(always)]
    pub fn dcdclnrunning(&mut self) -> DCDCLNRUNNING_W {
        DCDCLNRUNNING_W::new(self)
    }
    #[doc = "Bit 20 - Set DCDCINBYPASS Interrupt Flag"]
    #[inline(always)]
    pub fn dcdcinbypass(&mut self) -> DCDCINBYPASS_W {
        DCDCINBYPASS_W::new(self)
    }
    #[doc = "Bit 22 - Set BURDY Interrupt Flag"]
    #[inline(always)]
    pub fn burdy(&mut self) -> BURDY_W {
        BURDY_W::new(self)
    }
    #[doc = "Bit 24 - Set EM23WAKEUP Interrupt Flag"]
    #[inline(always)]
    pub fn em23wakeup(&mut self) -> EM23WAKEUP_W {
        EM23WAKEUP_W::new(self)
    }
    #[doc = "Bit 25 - Set VSCALEDONE Interrupt Flag"]
    #[inline(always)]
    pub fn vscaledone(&mut self) -> VSCALEDONE_W {
        VSCALEDONE_W::new(self)
    }
    #[doc = "Bit 29 - Set TEMP Interrupt Flag"]
    #[inline(always)]
    pub fn temp(&mut self) -> TEMP_W {
        TEMP_W::new(self)
    }
    #[doc = "Bit 30 - Set TEMPLOW Interrupt Flag"]
    #[inline(always)]
    pub fn templow(&mut self) -> TEMPLOW_W {
        TEMPLOW_W::new(self)
    }
    #[doc = "Bit 31 - Set TEMPHIGH Interrupt Flag"]
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
#[doc = "Interrupt Flag Set Register\n\nThis register you can [`write_with_zero`](crate::generic::Reg::write_with_zero), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [ifs](index.html) module"]
pub struct IFS_SPEC;
impl crate::RegisterSpec for IFS_SPEC {
    type Ux = u32;
}
#[doc = "`write(|w| ..)` method takes [ifs::W](W) writer structure"]
impl crate::Writable for IFS_SPEC {
    type Writer = W;
}
#[doc = "`reset()` method sets IFS to value 0"]
impl crate::Resettable for IFS_SPEC {
    #[inline(always)]
    fn reset_value() -> Self::Ux {
        0
    }
}
