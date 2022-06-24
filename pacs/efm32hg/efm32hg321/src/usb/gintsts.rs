#[doc = "Register `GINTSTS` reader"]
pub struct R(crate::R<GINTSTS_SPEC>);
impl core::ops::Deref for R {
    type Target = crate::R<GINTSTS_SPEC>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
impl From<crate::R<GINTSTS_SPEC>> for R {
    #[inline(always)]
    fn from(reader: crate::R<GINTSTS_SPEC>) -> Self {
        R(reader)
    }
}
#[doc = "Register `GINTSTS` writer"]
pub struct W(crate::W<GINTSTS_SPEC>);
impl core::ops::Deref for W {
    type Target = crate::W<GINTSTS_SPEC>;
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
impl From<crate::W<GINTSTS_SPEC>> for W {
    #[inline(always)]
    fn from(writer: crate::W<GINTSTS_SPEC>) -> Self {
        W(writer)
    }
}
#[doc = "Field `CURMOD` reader - Current Mode of Operation"]
pub type CURMOD_R = crate::BitReader<bool>;
#[doc = "Field `SOF` reader - Start of Frame"]
pub type SOF_R = crate::BitReader<bool>;
#[doc = "Field `SOF` writer - Start of Frame"]
pub type SOF_W<'a> = crate::BitWriter<'a, u32, GINTSTS_SPEC, bool, 3>;
#[doc = "Field `RXFLVL` reader - RxFIFO Non-Empty"]
pub type RXFLVL_R = crate::BitReader<bool>;
#[doc = "Field `GINNAKEFF` reader - Global IN Non-periodic NAK Effective"]
pub type GINNAKEFF_R = crate::BitReader<bool>;
#[doc = "Field `GOUTNAKEFF` reader - Global OUT NAK Effective"]
pub type GOUTNAKEFF_R = crate::BitReader<bool>;
#[doc = "Field `ERLYSUSP` reader - Early Suspend"]
pub type ERLYSUSP_R = crate::BitReader<bool>;
#[doc = "Field `ERLYSUSP` writer - Early Suspend"]
pub type ERLYSUSP_W<'a> = crate::BitWriter<'a, u32, GINTSTS_SPEC, bool, 10>;
#[doc = "Field `USBSUSP` reader - USB Suspend"]
pub type USBSUSP_R = crate::BitReader<bool>;
#[doc = "Field `USBSUSP` writer - USB Suspend"]
pub type USBSUSP_W<'a> = crate::BitWriter<'a, u32, GINTSTS_SPEC, bool, 11>;
#[doc = "Field `USBRST` reader - USB Reset"]
pub type USBRST_R = crate::BitReader<bool>;
#[doc = "Field `USBRST` writer - USB Reset"]
pub type USBRST_W<'a> = crate::BitWriter<'a, u32, GINTSTS_SPEC, bool, 12>;
#[doc = "Field `ENUMDONE` reader - Enumeration Done"]
pub type ENUMDONE_R = crate::BitReader<bool>;
#[doc = "Field `ENUMDONE` writer - Enumeration Done"]
pub type ENUMDONE_W<'a> = crate::BitWriter<'a, u32, GINTSTS_SPEC, bool, 13>;
#[doc = "Field `ISOOUTDROP` reader - Isochronous OUT Packet Dropped Interrupt"]
pub type ISOOUTDROP_R = crate::BitReader<bool>;
#[doc = "Field `ISOOUTDROP` writer - Isochronous OUT Packet Dropped Interrupt"]
pub type ISOOUTDROP_W<'a> = crate::BitWriter<'a, u32, GINTSTS_SPEC, bool, 14>;
#[doc = "Field `EOPF` reader - End of Periodic Frame Interrupt"]
pub type EOPF_R = crate::BitReader<bool>;
#[doc = "Field `EOPF` writer - End of Periodic Frame Interrupt"]
pub type EOPF_W<'a> = crate::BitWriter<'a, u32, GINTSTS_SPEC, bool, 15>;
#[doc = "Field `IEPINT` reader - IN Endpoints Interrupt"]
pub type IEPINT_R = crate::BitReader<bool>;
#[doc = "Field `OEPINT` reader - OUT Endpoints Interrupt"]
pub type OEPINT_R = crate::BitReader<bool>;
#[doc = "Field `INCOMPISOIN` reader - Incomplete Isochronous IN Transfer"]
pub type INCOMPISOIN_R = crate::BitReader<bool>;
#[doc = "Field `INCOMPISOIN` writer - Incomplete Isochronous IN Transfer"]
pub type INCOMPISOIN_W<'a> = crate::BitWriter<'a, u32, GINTSTS_SPEC, bool, 20>;
#[doc = "Field `INCOMPLP` reader - Incomplete Periodic Transfer"]
pub type INCOMPLP_R = crate::BitReader<bool>;
#[doc = "Field `INCOMPLP` writer - Incomplete Periodic Transfer"]
pub type INCOMPLP_W<'a> = crate::BitWriter<'a, u32, GINTSTS_SPEC, bool, 21>;
#[doc = "Field `FETSUSP` reader - Data Fetch Suspended"]
pub type FETSUSP_R = crate::BitReader<bool>;
#[doc = "Field `FETSUSP` writer - Data Fetch Suspended"]
pub type FETSUSP_W<'a> = crate::BitWriter<'a, u32, GINTSTS_SPEC, bool, 22>;
#[doc = "Field `RESETDET` reader - Reset detected Interrupt"]
pub type RESETDET_R = crate::BitReader<bool>;
#[doc = "Field `RESETDET` writer - Reset detected Interrupt"]
pub type RESETDET_W<'a> = crate::BitWriter<'a, u32, GINTSTS_SPEC, bool, 23>;
#[doc = "Field `WKUPINT` reader - Resume/Remote Wakeup Detected Interrupt"]
pub type WKUPINT_R = crate::BitReader<bool>;
#[doc = "Field `WKUPINT` writer - Resume/Remote Wakeup Detected Interrupt"]
pub type WKUPINT_W<'a> = crate::BitWriter<'a, u32, GINTSTS_SPEC, bool, 31>;
impl R {
    #[doc = "Bit 0 - Current Mode of Operation"]
    #[inline(always)]
    pub fn curmod(&self) -> CURMOD_R {
        CURMOD_R::new((self.bits & 1) != 0)
    }
    #[doc = "Bit 3 - Start of Frame"]
    #[inline(always)]
    pub fn sof(&self) -> SOF_R {
        SOF_R::new(((self.bits >> 3) & 1) != 0)
    }
    #[doc = "Bit 4 - RxFIFO Non-Empty"]
    #[inline(always)]
    pub fn rxflvl(&self) -> RXFLVL_R {
        RXFLVL_R::new(((self.bits >> 4) & 1) != 0)
    }
    #[doc = "Bit 6 - Global IN Non-periodic NAK Effective"]
    #[inline(always)]
    pub fn ginnakeff(&self) -> GINNAKEFF_R {
        GINNAKEFF_R::new(((self.bits >> 6) & 1) != 0)
    }
    #[doc = "Bit 7 - Global OUT NAK Effective"]
    #[inline(always)]
    pub fn goutnakeff(&self) -> GOUTNAKEFF_R {
        GOUTNAKEFF_R::new(((self.bits >> 7) & 1) != 0)
    }
    #[doc = "Bit 10 - Early Suspend"]
    #[inline(always)]
    pub fn erlysusp(&self) -> ERLYSUSP_R {
        ERLYSUSP_R::new(((self.bits >> 10) & 1) != 0)
    }
    #[doc = "Bit 11 - USB Suspend"]
    #[inline(always)]
    pub fn usbsusp(&self) -> USBSUSP_R {
        USBSUSP_R::new(((self.bits >> 11) & 1) != 0)
    }
    #[doc = "Bit 12 - USB Reset"]
    #[inline(always)]
    pub fn usbrst(&self) -> USBRST_R {
        USBRST_R::new(((self.bits >> 12) & 1) != 0)
    }
    #[doc = "Bit 13 - Enumeration Done"]
    #[inline(always)]
    pub fn enumdone(&self) -> ENUMDONE_R {
        ENUMDONE_R::new(((self.bits >> 13) & 1) != 0)
    }
    #[doc = "Bit 14 - Isochronous OUT Packet Dropped Interrupt"]
    #[inline(always)]
    pub fn isooutdrop(&self) -> ISOOUTDROP_R {
        ISOOUTDROP_R::new(((self.bits >> 14) & 1) != 0)
    }
    #[doc = "Bit 15 - End of Periodic Frame Interrupt"]
    #[inline(always)]
    pub fn eopf(&self) -> EOPF_R {
        EOPF_R::new(((self.bits >> 15) & 1) != 0)
    }
    #[doc = "Bit 18 - IN Endpoints Interrupt"]
    #[inline(always)]
    pub fn iepint(&self) -> IEPINT_R {
        IEPINT_R::new(((self.bits >> 18) & 1) != 0)
    }
    #[doc = "Bit 19 - OUT Endpoints Interrupt"]
    #[inline(always)]
    pub fn oepint(&self) -> OEPINT_R {
        OEPINT_R::new(((self.bits >> 19) & 1) != 0)
    }
    #[doc = "Bit 20 - Incomplete Isochronous IN Transfer"]
    #[inline(always)]
    pub fn incompisoin(&self) -> INCOMPISOIN_R {
        INCOMPISOIN_R::new(((self.bits >> 20) & 1) != 0)
    }
    #[doc = "Bit 21 - Incomplete Periodic Transfer"]
    #[inline(always)]
    pub fn incomplp(&self) -> INCOMPLP_R {
        INCOMPLP_R::new(((self.bits >> 21) & 1) != 0)
    }
    #[doc = "Bit 22 - Data Fetch Suspended"]
    #[inline(always)]
    pub fn fetsusp(&self) -> FETSUSP_R {
        FETSUSP_R::new(((self.bits >> 22) & 1) != 0)
    }
    #[doc = "Bit 23 - Reset detected Interrupt"]
    #[inline(always)]
    pub fn resetdet(&self) -> RESETDET_R {
        RESETDET_R::new(((self.bits >> 23) & 1) != 0)
    }
    #[doc = "Bit 31 - Resume/Remote Wakeup Detected Interrupt"]
    #[inline(always)]
    pub fn wkupint(&self) -> WKUPINT_R {
        WKUPINT_R::new(((self.bits >> 31) & 1) != 0)
    }
}
impl W {
    #[doc = "Bit 3 - Start of Frame"]
    #[inline(always)]
    pub fn sof(&mut self) -> SOF_W {
        SOF_W::new(self)
    }
    #[doc = "Bit 10 - Early Suspend"]
    #[inline(always)]
    pub fn erlysusp(&mut self) -> ERLYSUSP_W {
        ERLYSUSP_W::new(self)
    }
    #[doc = "Bit 11 - USB Suspend"]
    #[inline(always)]
    pub fn usbsusp(&mut self) -> USBSUSP_W {
        USBSUSP_W::new(self)
    }
    #[doc = "Bit 12 - USB Reset"]
    #[inline(always)]
    pub fn usbrst(&mut self) -> USBRST_W {
        USBRST_W::new(self)
    }
    #[doc = "Bit 13 - Enumeration Done"]
    #[inline(always)]
    pub fn enumdone(&mut self) -> ENUMDONE_W {
        ENUMDONE_W::new(self)
    }
    #[doc = "Bit 14 - Isochronous OUT Packet Dropped Interrupt"]
    #[inline(always)]
    pub fn isooutdrop(&mut self) -> ISOOUTDROP_W {
        ISOOUTDROP_W::new(self)
    }
    #[doc = "Bit 15 - End of Periodic Frame Interrupt"]
    #[inline(always)]
    pub fn eopf(&mut self) -> EOPF_W {
        EOPF_W::new(self)
    }
    #[doc = "Bit 20 - Incomplete Isochronous IN Transfer"]
    #[inline(always)]
    pub fn incompisoin(&mut self) -> INCOMPISOIN_W {
        INCOMPISOIN_W::new(self)
    }
    #[doc = "Bit 21 - Incomplete Periodic Transfer"]
    #[inline(always)]
    pub fn incomplp(&mut self) -> INCOMPLP_W {
        INCOMPLP_W::new(self)
    }
    #[doc = "Bit 22 - Data Fetch Suspended"]
    #[inline(always)]
    pub fn fetsusp(&mut self) -> FETSUSP_W {
        FETSUSP_W::new(self)
    }
    #[doc = "Bit 23 - Reset detected Interrupt"]
    #[inline(always)]
    pub fn resetdet(&mut self) -> RESETDET_W {
        RESETDET_W::new(self)
    }
    #[doc = "Bit 31 - Resume/Remote Wakeup Detected Interrupt"]
    #[inline(always)]
    pub fn wkupint(&mut self) -> WKUPINT_W {
        WKUPINT_W::new(self)
    }
    #[doc = "Writes raw bits to the register."]
    #[inline(always)]
    pub unsafe fn bits(&mut self, bits: u32) -> &mut Self {
        self.0.bits(bits);
        self
    }
}
#[doc = "Interrupt Register\n\nThis register you can [`read`](crate::generic::Reg::read), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [gintsts](index.html) module"]
pub struct GINTSTS_SPEC;
impl crate::RegisterSpec for GINTSTS_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [gintsts::R](R) reader structure"]
impl crate::Readable for GINTSTS_SPEC {
    type Reader = R;
}
#[doc = "`write(|w| ..)` method takes [gintsts::W](W) writer structure"]
impl crate::Writable for GINTSTS_SPEC {
    type Writer = W;
}
#[doc = "`reset()` method sets GINTSTS to value 0"]
impl crate::Resettable for GINTSTS_SPEC {
    #[inline(always)]
    fn reset_value() -> Self::Ux {
        0
    }
}
