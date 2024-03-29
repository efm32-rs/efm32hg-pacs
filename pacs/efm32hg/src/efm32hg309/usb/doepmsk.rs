#[doc = "Register `DOEPMSK` reader"]
pub struct R(crate::R<DOEPMSK_SPEC>);
impl core::ops::Deref for R {
    type Target = crate::R<DOEPMSK_SPEC>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
impl From<crate::R<DOEPMSK_SPEC>> for R {
    #[inline(always)]
    fn from(reader: crate::R<DOEPMSK_SPEC>) -> Self {
        R(reader)
    }
}
#[doc = "Register `DOEPMSK` writer"]
pub struct W(crate::W<DOEPMSK_SPEC>);
impl core::ops::Deref for W {
    type Target = crate::W<DOEPMSK_SPEC>;
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
impl From<crate::W<DOEPMSK_SPEC>> for W {
    #[inline(always)]
    fn from(writer: crate::W<DOEPMSK_SPEC>) -> Self {
        W(writer)
    }
}
#[doc = "Field `XFERCOMPLMSK` reader - Transfer Completed Interrupt Mask"]
pub type XFERCOMPLMSK_R = crate::BitReader<bool>;
#[doc = "Field `XFERCOMPLMSK` writer - Transfer Completed Interrupt Mask"]
pub type XFERCOMPLMSK_W<'a, const O: u8> = crate::BitWriter<'a, u32, DOEPMSK_SPEC, bool, O>;
#[doc = "Field `EPDISBLDMSK` reader - Endpoint Disabled Interrupt Mask"]
pub type EPDISBLDMSK_R = crate::BitReader<bool>;
#[doc = "Field `EPDISBLDMSK` writer - Endpoint Disabled Interrupt Mask"]
pub type EPDISBLDMSK_W<'a, const O: u8> = crate::BitWriter<'a, u32, DOEPMSK_SPEC, bool, O>;
#[doc = "Field `AHBERRMSK` reader - AHB Error"]
pub type AHBERRMSK_R = crate::BitReader<bool>;
#[doc = "Field `AHBERRMSK` writer - AHB Error"]
pub type AHBERRMSK_W<'a, const O: u8> = crate::BitWriter<'a, u32, DOEPMSK_SPEC, bool, O>;
#[doc = "Field `SETUPMSK` reader - SETUP Phase Done Mask"]
pub type SETUPMSK_R = crate::BitReader<bool>;
#[doc = "Field `SETUPMSK` writer - SETUP Phase Done Mask"]
pub type SETUPMSK_W<'a, const O: u8> = crate::BitWriter<'a, u32, DOEPMSK_SPEC, bool, O>;
#[doc = "Field `OUTTKNEPDISMSK` reader - OUT Token Received when Endpoint Disabled Mask"]
pub type OUTTKNEPDISMSK_R = crate::BitReader<bool>;
#[doc = "Field `OUTTKNEPDISMSK` writer - OUT Token Received when Endpoint Disabled Mask"]
pub type OUTTKNEPDISMSK_W<'a, const O: u8> = crate::BitWriter<'a, u32, DOEPMSK_SPEC, bool, O>;
#[doc = "Field `STSPHSERCVDMSK` reader - Status Phase Received Mask"]
pub type STSPHSERCVDMSK_R = crate::BitReader<bool>;
#[doc = "Field `STSPHSERCVDMSK` writer - Status Phase Received Mask"]
pub type STSPHSERCVDMSK_W<'a, const O: u8> = crate::BitWriter<'a, u32, DOEPMSK_SPEC, bool, O>;
#[doc = "Field `BACK2BACKSETUP` reader - Back-to-Back SETUP Packets Received Mask"]
pub type BACK2BACKSETUP_R = crate::BitReader<bool>;
#[doc = "Field `BACK2BACKSETUP` writer - Back-to-Back SETUP Packets Received Mask"]
pub type BACK2BACKSETUP_W<'a, const O: u8> = crate::BitWriter<'a, u32, DOEPMSK_SPEC, bool, O>;
#[doc = "Field `OUTPKTERRMSK` reader - OUT Packet Error Mask"]
pub type OUTPKTERRMSK_R = crate::BitReader<bool>;
#[doc = "Field `OUTPKTERRMSK` writer - OUT Packet Error Mask"]
pub type OUTPKTERRMSK_W<'a, const O: u8> = crate::BitWriter<'a, u32, DOEPMSK_SPEC, bool, O>;
#[doc = "Field `BBLEERRMSK` reader - Babble Error interrupt Mask"]
pub type BBLEERRMSK_R = crate::BitReader<bool>;
#[doc = "Field `BBLEERRMSK` writer - Babble Error interrupt Mask"]
pub type BBLEERRMSK_W<'a, const O: u8> = crate::BitWriter<'a, u32, DOEPMSK_SPEC, bool, O>;
#[doc = "Field `NAKMSK` reader - NAK interrupt Mask"]
pub type NAKMSK_R = crate::BitReader<bool>;
#[doc = "Field `NAKMSK` writer - NAK interrupt Mask"]
pub type NAKMSK_W<'a, const O: u8> = crate::BitWriter<'a, u32, DOEPMSK_SPEC, bool, O>;
impl R {
    #[doc = "Bit 0 - Transfer Completed Interrupt Mask"]
    #[inline(always)]
    pub fn xfercomplmsk(&self) -> XFERCOMPLMSK_R {
        XFERCOMPLMSK_R::new((self.bits & 1) != 0)
    }
    #[doc = "Bit 1 - Endpoint Disabled Interrupt Mask"]
    #[inline(always)]
    pub fn epdisbldmsk(&self) -> EPDISBLDMSK_R {
        EPDISBLDMSK_R::new(((self.bits >> 1) & 1) != 0)
    }
    #[doc = "Bit 2 - AHB Error"]
    #[inline(always)]
    pub fn ahberrmsk(&self) -> AHBERRMSK_R {
        AHBERRMSK_R::new(((self.bits >> 2) & 1) != 0)
    }
    #[doc = "Bit 3 - SETUP Phase Done Mask"]
    #[inline(always)]
    pub fn setupmsk(&self) -> SETUPMSK_R {
        SETUPMSK_R::new(((self.bits >> 3) & 1) != 0)
    }
    #[doc = "Bit 4 - OUT Token Received when Endpoint Disabled Mask"]
    #[inline(always)]
    pub fn outtknepdismsk(&self) -> OUTTKNEPDISMSK_R {
        OUTTKNEPDISMSK_R::new(((self.bits >> 4) & 1) != 0)
    }
    #[doc = "Bit 5 - Status Phase Received Mask"]
    #[inline(always)]
    pub fn stsphsercvdmsk(&self) -> STSPHSERCVDMSK_R {
        STSPHSERCVDMSK_R::new(((self.bits >> 5) & 1) != 0)
    }
    #[doc = "Bit 6 - Back-to-Back SETUP Packets Received Mask"]
    #[inline(always)]
    pub fn back2backsetup(&self) -> BACK2BACKSETUP_R {
        BACK2BACKSETUP_R::new(((self.bits >> 6) & 1) != 0)
    }
    #[doc = "Bit 8 - OUT Packet Error Mask"]
    #[inline(always)]
    pub fn outpkterrmsk(&self) -> OUTPKTERRMSK_R {
        OUTPKTERRMSK_R::new(((self.bits >> 8) & 1) != 0)
    }
    #[doc = "Bit 12 - Babble Error interrupt Mask"]
    #[inline(always)]
    pub fn bbleerrmsk(&self) -> BBLEERRMSK_R {
        BBLEERRMSK_R::new(((self.bits >> 12) & 1) != 0)
    }
    #[doc = "Bit 13 - NAK interrupt Mask"]
    #[inline(always)]
    pub fn nakmsk(&self) -> NAKMSK_R {
        NAKMSK_R::new(((self.bits >> 13) & 1) != 0)
    }
}
impl W {
    #[doc = "Bit 0 - Transfer Completed Interrupt Mask"]
    #[inline(always)]
    #[must_use]
    pub fn xfercomplmsk(&mut self) -> XFERCOMPLMSK_W<0> {
        XFERCOMPLMSK_W::new(self)
    }
    #[doc = "Bit 1 - Endpoint Disabled Interrupt Mask"]
    #[inline(always)]
    #[must_use]
    pub fn epdisbldmsk(&mut self) -> EPDISBLDMSK_W<1> {
        EPDISBLDMSK_W::new(self)
    }
    #[doc = "Bit 2 - AHB Error"]
    #[inline(always)]
    #[must_use]
    pub fn ahberrmsk(&mut self) -> AHBERRMSK_W<2> {
        AHBERRMSK_W::new(self)
    }
    #[doc = "Bit 3 - SETUP Phase Done Mask"]
    #[inline(always)]
    #[must_use]
    pub fn setupmsk(&mut self) -> SETUPMSK_W<3> {
        SETUPMSK_W::new(self)
    }
    #[doc = "Bit 4 - OUT Token Received when Endpoint Disabled Mask"]
    #[inline(always)]
    #[must_use]
    pub fn outtknepdismsk(&mut self) -> OUTTKNEPDISMSK_W<4> {
        OUTTKNEPDISMSK_W::new(self)
    }
    #[doc = "Bit 5 - Status Phase Received Mask"]
    #[inline(always)]
    #[must_use]
    pub fn stsphsercvdmsk(&mut self) -> STSPHSERCVDMSK_W<5> {
        STSPHSERCVDMSK_W::new(self)
    }
    #[doc = "Bit 6 - Back-to-Back SETUP Packets Received Mask"]
    #[inline(always)]
    #[must_use]
    pub fn back2backsetup(&mut self) -> BACK2BACKSETUP_W<6> {
        BACK2BACKSETUP_W::new(self)
    }
    #[doc = "Bit 8 - OUT Packet Error Mask"]
    #[inline(always)]
    #[must_use]
    pub fn outpkterrmsk(&mut self) -> OUTPKTERRMSK_W<8> {
        OUTPKTERRMSK_W::new(self)
    }
    #[doc = "Bit 12 - Babble Error interrupt Mask"]
    #[inline(always)]
    #[must_use]
    pub fn bbleerrmsk(&mut self) -> BBLEERRMSK_W<12> {
        BBLEERRMSK_W::new(self)
    }
    #[doc = "Bit 13 - NAK interrupt Mask"]
    #[inline(always)]
    #[must_use]
    pub fn nakmsk(&mut self) -> NAKMSK_W<13> {
        NAKMSK_W::new(self)
    }
    #[doc = "Writes raw bits to the register."]
    #[inline(always)]
    pub unsafe fn bits(&mut self, bits: u32) -> &mut Self {
        self.0.bits(bits);
        self
    }
}
#[doc = "Device OUT Endpoint Common Interrupt Mask Register\n\nThis register you can [`read`](crate::generic::Reg::read), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [doepmsk](index.html) module"]
pub struct DOEPMSK_SPEC;
impl crate::RegisterSpec for DOEPMSK_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [doepmsk::R](R) reader structure"]
impl crate::Readable for DOEPMSK_SPEC {
    type Reader = R;
}
#[doc = "`write(|w| ..)` method takes [doepmsk::W](W) writer structure"]
impl crate::Writable for DOEPMSK_SPEC {
    type Writer = W;
    const ZERO_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
}
#[doc = "`reset()` method sets DOEPMSK to value 0"]
impl crate::Resettable for DOEPMSK_SPEC {
    const RESET_VALUE: Self::Ux = 0;
}
