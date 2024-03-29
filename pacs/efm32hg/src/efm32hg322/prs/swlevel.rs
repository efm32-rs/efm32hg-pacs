#[doc = "Register `SWLEVEL` reader"]
pub struct R(crate::R<SWLEVEL_SPEC>);
impl core::ops::Deref for R {
    type Target = crate::R<SWLEVEL_SPEC>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
impl From<crate::R<SWLEVEL_SPEC>> for R {
    #[inline(always)]
    fn from(reader: crate::R<SWLEVEL_SPEC>) -> Self {
        R(reader)
    }
}
#[doc = "Register `SWLEVEL` writer"]
pub struct W(crate::W<SWLEVEL_SPEC>);
impl core::ops::Deref for W {
    type Target = crate::W<SWLEVEL_SPEC>;
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
impl From<crate::W<SWLEVEL_SPEC>> for W {
    #[inline(always)]
    fn from(writer: crate::W<SWLEVEL_SPEC>) -> Self {
        W(writer)
    }
}
#[doc = "Field `CH0LEVEL` reader - Channel 0 Software Level"]
pub type CH0LEVEL_R = crate::BitReader<bool>;
#[doc = "Field `CH0LEVEL` writer - Channel 0 Software Level"]
pub type CH0LEVEL_W<'a, const O: u8> = crate::BitWriter<'a, u32, SWLEVEL_SPEC, bool, O>;
#[doc = "Field `CH1LEVEL` reader - Channel 1 Software Level"]
pub type CH1LEVEL_R = crate::BitReader<bool>;
#[doc = "Field `CH1LEVEL` writer - Channel 1 Software Level"]
pub type CH1LEVEL_W<'a, const O: u8> = crate::BitWriter<'a, u32, SWLEVEL_SPEC, bool, O>;
#[doc = "Field `CH2LEVEL` reader - Channel 2 Software Level"]
pub type CH2LEVEL_R = crate::BitReader<bool>;
#[doc = "Field `CH2LEVEL` writer - Channel 2 Software Level"]
pub type CH2LEVEL_W<'a, const O: u8> = crate::BitWriter<'a, u32, SWLEVEL_SPEC, bool, O>;
#[doc = "Field `CH3LEVEL` reader - Channel 3 Software Level"]
pub type CH3LEVEL_R = crate::BitReader<bool>;
#[doc = "Field `CH3LEVEL` writer - Channel 3 Software Level"]
pub type CH3LEVEL_W<'a, const O: u8> = crate::BitWriter<'a, u32, SWLEVEL_SPEC, bool, O>;
#[doc = "Field `CH4LEVEL` reader - Channel 4 Software Level"]
pub type CH4LEVEL_R = crate::BitReader<bool>;
#[doc = "Field `CH4LEVEL` writer - Channel 4 Software Level"]
pub type CH4LEVEL_W<'a, const O: u8> = crate::BitWriter<'a, u32, SWLEVEL_SPEC, bool, O>;
#[doc = "Field `CH5LEVEL` reader - Channel 5 Software Level"]
pub type CH5LEVEL_R = crate::BitReader<bool>;
#[doc = "Field `CH5LEVEL` writer - Channel 5 Software Level"]
pub type CH5LEVEL_W<'a, const O: u8> = crate::BitWriter<'a, u32, SWLEVEL_SPEC, bool, O>;
impl R {
    #[doc = "Bit 0 - Channel 0 Software Level"]
    #[inline(always)]
    pub fn ch0level(&self) -> CH0LEVEL_R {
        CH0LEVEL_R::new((self.bits & 1) != 0)
    }
    #[doc = "Bit 1 - Channel 1 Software Level"]
    #[inline(always)]
    pub fn ch1level(&self) -> CH1LEVEL_R {
        CH1LEVEL_R::new(((self.bits >> 1) & 1) != 0)
    }
    #[doc = "Bit 2 - Channel 2 Software Level"]
    #[inline(always)]
    pub fn ch2level(&self) -> CH2LEVEL_R {
        CH2LEVEL_R::new(((self.bits >> 2) & 1) != 0)
    }
    #[doc = "Bit 3 - Channel 3 Software Level"]
    #[inline(always)]
    pub fn ch3level(&self) -> CH3LEVEL_R {
        CH3LEVEL_R::new(((self.bits >> 3) & 1) != 0)
    }
    #[doc = "Bit 4 - Channel 4 Software Level"]
    #[inline(always)]
    pub fn ch4level(&self) -> CH4LEVEL_R {
        CH4LEVEL_R::new(((self.bits >> 4) & 1) != 0)
    }
    #[doc = "Bit 5 - Channel 5 Software Level"]
    #[inline(always)]
    pub fn ch5level(&self) -> CH5LEVEL_R {
        CH5LEVEL_R::new(((self.bits >> 5) & 1) != 0)
    }
}
impl W {
    #[doc = "Bit 0 - Channel 0 Software Level"]
    #[inline(always)]
    #[must_use]
    pub fn ch0level(&mut self) -> CH0LEVEL_W<0> {
        CH0LEVEL_W::new(self)
    }
    #[doc = "Bit 1 - Channel 1 Software Level"]
    #[inline(always)]
    #[must_use]
    pub fn ch1level(&mut self) -> CH1LEVEL_W<1> {
        CH1LEVEL_W::new(self)
    }
    #[doc = "Bit 2 - Channel 2 Software Level"]
    #[inline(always)]
    #[must_use]
    pub fn ch2level(&mut self) -> CH2LEVEL_W<2> {
        CH2LEVEL_W::new(self)
    }
    #[doc = "Bit 3 - Channel 3 Software Level"]
    #[inline(always)]
    #[must_use]
    pub fn ch3level(&mut self) -> CH3LEVEL_W<3> {
        CH3LEVEL_W::new(self)
    }
    #[doc = "Bit 4 - Channel 4 Software Level"]
    #[inline(always)]
    #[must_use]
    pub fn ch4level(&mut self) -> CH4LEVEL_W<4> {
        CH4LEVEL_W::new(self)
    }
    #[doc = "Bit 5 - Channel 5 Software Level"]
    #[inline(always)]
    #[must_use]
    pub fn ch5level(&mut self) -> CH5LEVEL_W<5> {
        CH5LEVEL_W::new(self)
    }
    #[doc = "Writes raw bits to the register."]
    #[inline(always)]
    pub unsafe fn bits(&mut self, bits: u32) -> &mut Self {
        self.0.bits(bits);
        self
    }
}
#[doc = "Software Level Register\n\nThis register you can [`read`](crate::generic::Reg::read), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [swlevel](index.html) module"]
pub struct SWLEVEL_SPEC;
impl crate::RegisterSpec for SWLEVEL_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [swlevel::R](R) reader structure"]
impl crate::Readable for SWLEVEL_SPEC {
    type Reader = R;
}
#[doc = "`write(|w| ..)` method takes [swlevel::W](W) writer structure"]
impl crate::Writable for SWLEVEL_SPEC {
    type Writer = W;
    const ZERO_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
}
#[doc = "`reset()` method sets SWLEVEL to value 0"]
impl crate::Resettable for SWLEVEL_SPEC {
    const RESET_VALUE: Self::Ux = 0;
}
