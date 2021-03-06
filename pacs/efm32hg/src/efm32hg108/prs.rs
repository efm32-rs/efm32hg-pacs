#[doc = r"Register block"]
#[repr(C)]
pub struct RegisterBlock {
    #[doc = "0x00 - Software Pulse Register"]
    pub swpulse: crate::Reg<swpulse::SWPULSE_SPEC>,
    #[doc = "0x04 - Software Level Register"]
    pub swlevel: crate::Reg<swlevel::SWLEVEL_SPEC>,
    #[doc = "0x08 - I/O Routing Register"]
    pub route: crate::Reg<route::ROUTE_SPEC>,
    _reserved3: [u8; 0x04],
    #[doc = "0x10 - Channel Control Register"]
    pub ch0_ctrl: crate::Reg<ch0_ctrl::CH0_CTRL_SPEC>,
    #[doc = "0x14 - Channel Control Register"]
    pub ch1_ctrl: crate::Reg<ch1_ctrl::CH1_CTRL_SPEC>,
    #[doc = "0x18 - Channel Control Register"]
    pub ch2_ctrl: crate::Reg<ch2_ctrl::CH2_CTRL_SPEC>,
    #[doc = "0x1c - Channel Control Register"]
    pub ch3_ctrl: crate::Reg<ch3_ctrl::CH3_CTRL_SPEC>,
    #[doc = "0x20 - Channel Control Register"]
    pub ch4_ctrl: crate::Reg<ch4_ctrl::CH4_CTRL_SPEC>,
    #[doc = "0x24 - Channel Control Register"]
    pub ch5_ctrl: crate::Reg<ch5_ctrl::CH5_CTRL_SPEC>,
    _reserved9: [u8; 0x18],
    #[doc = "0x40 - MTB Trace Control Register"]
    pub tracectrl: crate::Reg<tracectrl::TRACECTRL_SPEC>,
}
#[doc = "SWPULSE register accessor: an alias for `Reg<SWPULSE_SPEC>`"]
pub type SWPULSE = crate::Reg<swpulse::SWPULSE_SPEC>;
#[doc = "Software Pulse Register"]
pub mod swpulse;
#[doc = "SWLEVEL register accessor: an alias for `Reg<SWLEVEL_SPEC>`"]
pub type SWLEVEL = crate::Reg<swlevel::SWLEVEL_SPEC>;
#[doc = "Software Level Register"]
pub mod swlevel;
#[doc = "ROUTE register accessor: an alias for `Reg<ROUTE_SPEC>`"]
pub type ROUTE = crate::Reg<route::ROUTE_SPEC>;
#[doc = "I/O Routing Register"]
pub mod route;
#[doc = "CH0_CTRL register accessor: an alias for `Reg<CH0_CTRL_SPEC>`"]
pub type CH0_CTRL = crate::Reg<ch0_ctrl::CH0_CTRL_SPEC>;
#[doc = "Channel Control Register"]
pub mod ch0_ctrl;
#[doc = "CH1_CTRL register accessor: an alias for `Reg<CH1_CTRL_SPEC>`"]
pub type CH1_CTRL = crate::Reg<ch1_ctrl::CH1_CTRL_SPEC>;
#[doc = "Channel Control Register"]
pub mod ch1_ctrl;
#[doc = "CH2_CTRL register accessor: an alias for `Reg<CH2_CTRL_SPEC>`"]
pub type CH2_CTRL = crate::Reg<ch2_ctrl::CH2_CTRL_SPEC>;
#[doc = "Channel Control Register"]
pub mod ch2_ctrl;
#[doc = "CH3_CTRL register accessor: an alias for `Reg<CH3_CTRL_SPEC>`"]
pub type CH3_CTRL = crate::Reg<ch3_ctrl::CH3_CTRL_SPEC>;
#[doc = "Channel Control Register"]
pub mod ch3_ctrl;
#[doc = "CH4_CTRL register accessor: an alias for `Reg<CH4_CTRL_SPEC>`"]
pub type CH4_CTRL = crate::Reg<ch4_ctrl::CH4_CTRL_SPEC>;
#[doc = "Channel Control Register"]
pub mod ch4_ctrl;
#[doc = "CH5_CTRL register accessor: an alias for `Reg<CH5_CTRL_SPEC>`"]
pub type CH5_CTRL = crate::Reg<ch5_ctrl::CH5_CTRL_SPEC>;
#[doc = "Channel Control Register"]
pub mod ch5_ctrl;
#[doc = "TRACECTRL register accessor: an alias for `Reg<TRACECTRL_SPEC>`"]
pub type TRACECTRL = crate::Reg<tracectrl::TRACECTRL_SPEC>;
#[doc = "MTB Trace Control Register"]
pub mod tracectrl;
