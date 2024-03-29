#[doc = "Register `CHREQMASKS` writer"]
pub struct W(crate::W<CHREQMASKS_SPEC>);
impl core::ops::Deref for W {
    type Target = crate::W<CHREQMASKS_SPEC>;
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
impl From<crate::W<CHREQMASKS_SPEC>> for W {
    #[inline(always)]
    fn from(writer: crate::W<CHREQMASKS_SPEC>) -> Self {
        W(writer)
    }
}
#[doc = "Field `CH0REQMASKS` writer - Channel 0 Request Mask Set"]
pub type CH0REQMASKS_W<'a, const O: u8> = crate::BitWriter<'a, u32, CHREQMASKS_SPEC, bool, O>;
#[doc = "Field `CH1REQMASKS` writer - Channel 1 Request Mask Set"]
pub type CH1REQMASKS_W<'a, const O: u8> = crate::BitWriter<'a, u32, CHREQMASKS_SPEC, bool, O>;
#[doc = "Field `CH2REQMASKS` writer - Channel 2 Request Mask Set"]
pub type CH2REQMASKS_W<'a, const O: u8> = crate::BitWriter<'a, u32, CHREQMASKS_SPEC, bool, O>;
#[doc = "Field `CH3REQMASKS` writer - Channel 3 Request Mask Set"]
pub type CH3REQMASKS_W<'a, const O: u8> = crate::BitWriter<'a, u32, CHREQMASKS_SPEC, bool, O>;
#[doc = "Field `CH4REQMASKS` writer - Channel 4 Request Mask Set"]
pub type CH4REQMASKS_W<'a, const O: u8> = crate::BitWriter<'a, u32, CHREQMASKS_SPEC, bool, O>;
#[doc = "Field `CH5REQMASKS` writer - Channel 5 Request Mask Set"]
pub type CH5REQMASKS_W<'a, const O: u8> = crate::BitWriter<'a, u32, CHREQMASKS_SPEC, bool, O>;
impl W {
    #[doc = "Bit 0 - Channel 0 Request Mask Set"]
    #[inline(always)]
    #[must_use]
    pub fn ch0reqmasks(&mut self) -> CH0REQMASKS_W<0> {
        CH0REQMASKS_W::new(self)
    }
    #[doc = "Bit 1 - Channel 1 Request Mask Set"]
    #[inline(always)]
    #[must_use]
    pub fn ch1reqmasks(&mut self) -> CH1REQMASKS_W<1> {
        CH1REQMASKS_W::new(self)
    }
    #[doc = "Bit 2 - Channel 2 Request Mask Set"]
    #[inline(always)]
    #[must_use]
    pub fn ch2reqmasks(&mut self) -> CH2REQMASKS_W<2> {
        CH2REQMASKS_W::new(self)
    }
    #[doc = "Bit 3 - Channel 3 Request Mask Set"]
    #[inline(always)]
    #[must_use]
    pub fn ch3reqmasks(&mut self) -> CH3REQMASKS_W<3> {
        CH3REQMASKS_W::new(self)
    }
    #[doc = "Bit 4 - Channel 4 Request Mask Set"]
    #[inline(always)]
    #[must_use]
    pub fn ch4reqmasks(&mut self) -> CH4REQMASKS_W<4> {
        CH4REQMASKS_W::new(self)
    }
    #[doc = "Bit 5 - Channel 5 Request Mask Set"]
    #[inline(always)]
    #[must_use]
    pub fn ch5reqmasks(&mut self) -> CH5REQMASKS_W<5> {
        CH5REQMASKS_W::new(self)
    }
    #[doc = "Writes raw bits to the register."]
    #[inline(always)]
    pub unsafe fn bits(&mut self, bits: u32) -> &mut Self {
        self.0.bits(bits);
        self
    }
}
#[doc = "Channel Request Mask Set Register\n\nThis register you can [`write_with_zero`](crate::generic::Reg::write_with_zero), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [chreqmasks](index.html) module"]
pub struct CHREQMASKS_SPEC;
impl crate::RegisterSpec for CHREQMASKS_SPEC {
    type Ux = u32;
}
#[doc = "`write(|w| ..)` method takes [chreqmasks::W](W) writer structure"]
impl crate::Writable for CHREQMASKS_SPEC {
    type Writer = W;
    const ZERO_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
}
#[doc = "`reset()` method sets CHREQMASKS to value 0"]
impl crate::Resettable for CHREQMASKS_SPEC {
    const RESET_VALUE: Self::Ux = 0;
}
