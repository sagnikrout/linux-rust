//! Automatically rewritten from C Header to Rust Module
//! Source: include/dt-bindings/usb/pd.h
#![no_std]
#![allow(non_camel_case_types)]
#![allow(non_snake_case)]
#![allow(non_upper_case_globals)]
#![allow(dead_code)]
#![allow(unused_variables)]
#![allow(unused_mut)]

use core::ffi::*;

// --- Linux Kernel Primitives Prelude ---
pub type uid_t = u32;
pub type gid_t = u32;
pub type uid16_t = u16;
pub type gid16_t = u16;
pub type pid_t = i32;
pub type mode_t = u32;
pub type umode_t = u16;
pub type nlink_t = u32;
pub type off_t = i64;
pub type loff_t = i64;
pub type dev_t = u32;
pub type ino_t = u64;
pub type size_t = usize;
pub type ssize_t = isize;
pub type uintptr_t = usize;
pub type intptr_t = isize;
pub type ptrdiff_t = isize;
pub type clockid_t = i32;
pub type timer_t = i32;
pub type time64_t = i64;
pub type atomic_t = core::sync::atomic::AtomicI32;
pub type atomic64_t = core::sync::atomic::AtomicI64;
// ---------------------------------------


// SPDX-License-Identifier: GPL-2.0
// Power delivery Power Data Object definitions
pub const PDO_TYPE_FIXED: c_int = 0;
pub const PDO_TYPE_BATT: c_int = 1;
pub const PDO_TYPE_VAR: c_int = 2;
pub const PDO_TYPE_APDO: c_int = 3;
pub const PDO_TYPE_SHIFT: c_int = 30;
pub const PDO_TYPE_MASK: c_uint = 0x3;

pub const PDO_VOLT_MASK: c_uint = 0x3ff;
pub const PDO_CURR_MASK: c_uint = 0x3ff;
pub const PDO_PWR_MASK: c_uint = 0x3ff;

pub const APDO_TYPE_PPS: c_int = 0;
pub const APDO_TYPE_SPR_AVS: c_int = 2;

pub const PDO_APDO_TYPE_MASK: c_uint = 0x3;

pub const PDO_PPS_APDO_VOLT_MASK: c_uint = 0xff;
pub const PDO_PPS_APDO_CURR_MASK: c_uint = 0x7f;

pub const PDO_SPR_AVS_APDO_MAX_CURR_MASK: c_uint = 0x3ff;

//
// Based on "Table 6-14 Fixed Supply PDO - Sink" of "USB Power Delivery Specification Revision 3.0,
// Version 1.2"
// Initial current capability of the new source when vSafe5V is applied.
//
pub const FRS_DEFAULT_POWER: c_int = 1;
pub const FRS_5V_1P5A: c_int = 2;
pub const FRS_5V_3A: c_int = 3;
//
// SVDM Identity Header
// --------------------
// <31>     :: data capable as a USB host
// <30>     :: data capable as a USB device
// <29:27>  :: product type (UFP / Cable / VPD)
// <26>     :: modal operation supported (1b == yes)
// <25:23>  :: product type (DFP) (SVDM version 2.0+ only; set to zero in version 1.0)
// <22:21>  :: connector type (SVDM version 2.0+ only; set to zero in version 1.0)
// <20:16>  :: Reserved, Shall be set to zero
// <15:0>   :: USB-IF assigned VID for this cable vendor
//
// PD Rev2.0 definition
pub const IDH_PTYPE_UNDEF: c_int = 0;
// SOP Product Type (UFP)
pub const IDH_PTYPE_NOT_UFP: c_int = 0;
pub const IDH_PTYPE_HUB: c_int = 1;
pub const IDH_PTYPE_PERIPH: c_int = 2;
pub const IDH_PTYPE_PSD: c_int = 3;
pub const IDH_PTYPE_AMA: c_int = 5;
// SOP' Product Type (Cable Plug / VPD)
pub const IDH_PTYPE_NOT_CABLE: c_int = 0;
pub const IDH_PTYPE_PCABLE: c_int = 3;
pub const IDH_PTYPE_ACABLE: c_int = 4;
pub const IDH_PTYPE_VPD: c_int = 6;
// SOP Product Type (DFP)
pub const IDH_PTYPE_NOT_DFP: c_int = 0;
pub const IDH_PTYPE_DFP_HUB: c_int = 1;
pub const IDH_PTYPE_DFP_HOST: c_int = 2;
pub const IDH_PTYPE_DFP_PB: c_int = 3;

//
// Cert Stat VDO
// -------------
// <31:0>  : USB-IF assigned XID for this cable
//

//
// Product VDO
// -----------
// <31:16> : USB Product ID
// <15:0>  : USB bcdDevice
//

//
// UFP VDO (PD Revision 3.0+ only)
// --------
// <31:29> :: UFP VDO version
// <28>    :: Reserved
// <27:24> :: Device capability
// <23:22> :: Connector type (10b == receptacle, 11b == captive plug)
// <21:11> :: Reserved
// <10:8>  :: Vconn power (AMA only)
// <7>     :: Vconn required (AMA only, 0b == no, 1b == yes)
// <6>     :: Vbus required (AMA only, 0b == yes, 1b == no)
// <5:3>   :: Alternate modes
// <2:0>   :: USB highest speed
//
// UFP VDO Version
pub const UFP_VDO_VER1_2: c_int = 2;
// Device Capability

// Connector Type
pub const UFP_RECEPTACLE: c_int = 2;
pub const UFP_CAPTIVE: c_int = 3;
// Vconn Power (AMA only, set to AMA_VCONN_NOT_REQ if Vconn is not required)
pub const AMA_VCONN_PWR_1W: c_int = 0;
pub const AMA_VCONN_PWR_1W5: c_int = 1;
pub const AMA_VCONN_PWR_2W: c_int = 2;
pub const AMA_VCONN_PWR_3W: c_int = 3;
pub const AMA_VCONN_PWR_4W: c_int = 4;
pub const AMA_VCONN_PWR_5W: c_int = 5;
pub const AMA_VCONN_PWR_6W: c_int = 6;
// Vconn Required (AMA only)
pub const AMA_VCONN_NOT_REQ: c_int = 0;
pub const AMA_VCONN_REQ: c_int = 1;
// Vbus Required (AMA only)
pub const AMA_VBUS_REQ: c_int = 0;
pub const AMA_VBUS_NOT_REQ: c_int = 1;
// Alternate Modes
pub const UFP_ALTMODE_NOT_SUPP: c_int = 0;

// USB Highest Speed
pub const UFP_USB2_ONLY: c_int = 0;
pub const UFP_USB32_GEN1: c_int = 1;
pub const UFP_USB32_4_GEN2: c_int = 2;
pub const UFP_USB4_GEN3: c_int = 3;

//
// DFP VDO (PD Revision 3.0+ only)
// --------
// <31:29> :: DFP VDO version
// <28:27> :: Reserved
// <26:24> :: Host capability
// <23:22> :: Connector type (10b == receptacle, 11b == captive plug)
// <21:5>  :: Reserved
// <4:0>   :: Port number
//
pub const DFP_VDO_VER1_1: c_int = 1;

pub const DFP_RECEPTACLE: c_int = 2;
pub const DFP_CAPTIVE: c_int = 3;

//
// Cable VDO (for both Passive and Active Cable VDO in PD Rev2.0)
// ---------
// <31:28> :: Cable HW version
// <27:24> :: Cable FW version
// <23:20> :: Reserved, Shall be set to zero
// <19:18> :: type-C to Type-A/B/C/Captive (00b == A, 01 == B, 10 == C, 11 == Captive)
// <17>    :: Reserved, Shall be set to zero
// <16:13> :: cable latency (0001 == <10ns(~1m length))
// <12:11> :: cable termination type (11b == both ends active VCONN req)
// <10>    :: SSTX1 Directionality support (0b == fixed, 1b == cfgable)
// <9>     :: SSTX2 Directionality support
// <8>     :: SSRX1 Directionality support
// <7>     :: SSRX2 Directionality support
// <6:5>   :: Vbus current handling capability (01b == 3A, 10b == 5A)
// <4>     :: Vbus through cable (0b == no, 1b == yes)
// <3>     :: SOP" controller present? (0b == no, 1b == yes)
// <2:0>   :: USB SS Signaling support
//
// Passive Cable VDO (PD Rev3.0+)
// ---------
// <31:28> :: Cable HW version
// <27:24> :: Cable FW version
// <23:21> :: VDO version
// <20>    :: Reserved, Shall be set to zero
// <19:18> :: Type-C to Type-C/Captive (10b == C, 11b == Captive)
// <17>    :: Reserved, Shall be set to zero
// <16:13> :: cable latency (0001 == <10ns(~1m length))
// <12:11> :: cable termination type (10b == Vconn not req, 01b == Vconn req)
// <10:9>  :: Maximum Vbus voltage (00b == 20V, 01b == 30V, 10b == 40V, 11b == 50V)
// <8:7>   :: Reserved, Shall be set to zero
// <6:5>   :: Vbus current handling capability (01b == 3A, 10b == 5A)
// <4:3>   :: Reserved, Shall be set to zero
// <2:0>   :: USB highest speed
//
// Active Cable VDO 1 (PD Rev3.0+)
// ---------
// <31:28> :: Cable HW version
// <27:24> :: Cable FW version
// <23:21> :: VDO version
// <20>    :: Reserved, Shall be set to zero
// <19:18> :: Connector type (10b == C, 11b == Captive)
// <17>    :: Reserved, Shall be set to zero
// <16:13> :: cable latency (0001 == <10ns(~1m length))
// <12:11> :: cable termination type (10b == one end active, 11b == both ends active VCONN req)
// <10:9>  :: Maximum Vbus voltage (00b == 20V, 01b == 30V, 10b == 40V, 11b == 50V)
// <8>     :: SBU supported (0b == supported, 1b == not supported)
// <7>     :: SBU type (0b == passive, 1b == active)
// <6:5>   :: Vbus current handling capability (01b == 3A, 10b == 5A)
// <4>     :: Vbus through cable (0b == no, 1b == yes)
// <3>     :: SOP" controller present? (0b == no, 1b == yes)
// <2:0>   :: USB highest speed
//
// Cable VDO Version
pub const CABLE_VDO_VER1_0: c_int = 0;
pub const CABLE_VDO_VER1_3: c_int = 3;
// Connector Type (_ATYPE and _BTYPE are for PD Rev2.0 only)
pub const CABLE_ATYPE: c_int = 0;
pub const CABLE_BTYPE: c_int = 1;
pub const CABLE_CTYPE: c_int = 2;
pub const CABLE_CAPTIVE: c_int = 3;
// Cable Latency
pub const CABLE_LATENCY_1M: c_int = 1;
pub const CABLE_LATENCY_2M: c_int = 2;
pub const CABLE_LATENCY_3M: c_int = 3;
pub const CABLE_LATENCY_4M: c_int = 4;
pub const CABLE_LATENCY_5M: c_int = 5;
pub const CABLE_LATENCY_6M: c_int = 6;
pub const CABLE_LATENCY_7M: c_int = 7;
pub const CABLE_LATENCY_7M_PLUS: c_int = 8;
// Cable Termination Type
pub const PCABLE_VCONN_NOT_REQ: c_int = 0;
pub const PCABLE_VCONN_REQ: c_int = 1;
pub const ACABLE_ONE_END: c_int = 2;
pub const ACABLE_BOTH_END: c_int = 3;
// Maximum Vbus Voltage
pub const CABLE_MAX_VBUS_20V: c_int = 0;
pub const CABLE_MAX_VBUS_30V: c_int = 1;
pub const CABLE_MAX_VBUS_40V: c_int = 2;
pub const CABLE_MAX_VBUS_50V: c_int = 3;
// Active Cable SBU Supported/Type
pub const ACABLE_SBU_SUPP: c_int = 0;
pub const ACABLE_SBU_NOT_SUPP: c_int = 1;
pub const ACABLE_SBU_PASSIVE: c_int = 0;
pub const ACABLE_SBU_ACTIVE: c_int = 1;
// Vbus Current Handling Capability
pub const CABLE_CURR_DEF: c_int = 0;
pub const CABLE_CURR_3A: c_int = 1;
pub const CABLE_CURR_5A: c_int = 2;
// USB SuperSpeed Signaling Support (PD Rev2.0)
pub const CABLE_USBSS_U2_ONLY: c_int = 0;
pub const CABLE_USBSS_U31_GEN1: c_int = 1;
pub const CABLE_USBSS_U31_GEN2: c_int = 2;
// USB Highest Speed
pub const CABLE_USB2_ONLY: c_int = 0;
pub const CABLE_USB32_GEN1: c_int = 1;
pub const CABLE_USB32_4_GEN2: c_int = 2;
pub const CABLE_USB4_GEN3: c_int = 3;

//
// Active Cable VDO 2
// ---------
// <31:24> :: Maximum operating temperature
// <23:16> :: Shutdown temperature
// <15>    :: Reserved, Shall be set to zero
// <14:12> :: U3/CLd power
// <11>    :: U3 to U0 transition mode (0b == direct, 1b == through U3S)
// <10>    :: Physical connection (0b == copper, 1b == optical)
// <9>     :: Active element (0b == redriver, 1b == retimer)
// <8>     :: USB4 supported (0b == yes, 1b == no)
// <7:6>   :: USB2 hub hops consumed
// <5>     :: USB2 supported (0b == yes, 1b == no)
// <4>     :: USB3.2 supported (0b == yes, 1b == no)
// <3>     :: USB lanes supported (0b == one lane, 1b == two lanes)
// <2>     :: Optically isolated active cable (0b == no, 1b == yes)
// <1>     :: Reserved, Shall be set to zero
// <0>     :: USB gen (0b == gen1, 1b == gen2+)
//
// U3/CLd Power
pub const ACAB2_U3_CLD_10MW_PLUS: c_int = 0;
pub const ACAB2_U3_CLD_10MW: c_int = 1;
pub const ACAB2_U3_CLD_5MW: c_int = 2;
pub const ACAB2_U3_CLD_1MW: c_int = 3;
pub const ACAB2_U3_CLD_500UW: c_int = 4;
pub const ACAB2_U3_CLD_200UW: c_int = 5;
pub const ACAB2_U3_CLD_50UW: c_int = 6;
// Other Active Cable VDO 2 Fields
pub const ACAB2_U3U0_DIRECT: c_int = 0;
pub const ACAB2_U3U0_U3S: c_int = 1;
pub const ACAB2_PHY_COPPER: c_int = 0;
pub const ACAB2_PHY_OPTICAL: c_int = 1;
pub const ACAB2_REDRIVER: c_int = 0;
pub const ACAB2_RETIMER: c_int = 1;
pub const ACAB2_USB4_SUPP: c_int = 0;
pub const ACAB2_USB4_NOT_SUPP: c_int = 1;
pub const ACAB2_USB2_SUPP: c_int = 0;
pub const ACAB2_USB2_NOT_SUPP: c_int = 1;
pub const ACAB2_USB32_SUPP: c_int = 0;
pub const ACAB2_USB32_NOT_SUPP: c_int = 1;
pub const ACAB2_LANES_ONE: c_int = 0;
pub const ACAB2_LANES_TWO: c_int = 1;
pub const ACAB2_OPT_ISO_NO: c_int = 0;
pub const ACAB2_OPT_ISO_YES: c_int = 1;
pub const ACAB2_GEN_1: c_int = 0;
pub const ACAB2_GEN_2_PLUS: c_int = 1;

//
// AMA VDO (PD Rev2.0)
// ---------
// <31:28> :: Cable HW version
// <27:24> :: Cable FW version
// <23:12> :: Reserved, Shall be set to zero
// <11>    :: SSTX1 Directionality support (0b == fixed, 1b == cfgable)
// <10>    :: SSTX2 Directionality support
// <9>     :: SSRX1 Directionality support
// <8>     :: SSRX2 Directionality support
// <7:5>   :: Vconn power
// <4>     :: Vconn power required
// <3>     :: Vbus power required
// <2:0>   :: USB SS Signaling support
//

pub const AMA_USBSS_U2_ONLY: c_int = 0;
pub const AMA_USBSS_U31_GEN1: c_int = 1;
pub const AMA_USBSS_U31_GEN2: c_int = 2;
pub const AMA_USBSS_BBONLY: c_int = 3;
//
// VPD VDO
// ---------
// <31:28> :: HW version
// <27:24> :: FW version
// <23:21> :: VDO version
// <20:17> :: Reserved, Shall be set to zero
// <16:15> :: Maximum Vbus voltage (00b == 20V, 01b == 30V, 10b == 40V, 11b == 50V)
// <14>    :: Charge through current support (0b == 3A, 1b == 5A)
// <13>    :: Reserved, Shall be set to zero
// <12:7>  :: Vbus impedance
// <6:1>   :: Ground impedance
// <0>     :: Charge through support (0b == no, 1b == yes)
//
pub const VPD_VDO_VER1_0: c_int = 0;
pub const VPD_MAX_VBUS_20V: c_int = 0;
pub const VPD_MAX_VBUS_30V: c_int = 1;
pub const VPD_MAX_VBUS_40V: c_int = 2;
pub const VPD_MAX_VBUS_50V: c_int = 3;
pub const VPDCT_CURR_3A: c_int = 0;
pub const VPDCT_CURR_5A: c_int = 1;
pub const VPDCT_NOT_SUPP: c_int = 0;
pub const VPDCT_SUPP: c_int = 1;

//
// Sink Load Characteristics
// -------------------------
// <15>    :: Can tolerate vbus voltage droop
// <11:14> :: Duty cycle in 5% increments when bits 4:0 are non-zero
// <10:5>  :: Overload period in 20ms when bits 4:0 are non-zero
// <4:0>   :: Percent overload in 10% increments. Values higher than 25 are
// clipped to 250%
//

// Compliance

