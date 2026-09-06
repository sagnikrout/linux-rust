//! Automatically rewritten from C Header to Rust Module
//! Source: include/ufs/unipro.h
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


// SPDX-License-Identifier: GPL-2.0-or-later
//
// Copyright (C) 2013 Samsung Electronics Co., Ltd.
//
// M-TX Configuration Attributes
//
pub const TX_HIBERN8TIME_CAPABILITY: c_uint = 0x000F;
pub const TX_HS_DEEMPHASIS_SETTING_CAP: c_uint = 0x0012;
pub const TX_HS_PRESHOOT_SETTING_CAP: c_uint = 0x0015;
pub const TX_MODE: c_uint = 0x0021;
pub const TX_HSRATE_SERIES: c_uint = 0x0022;
pub const TX_HSGEAR: c_uint = 0x0023;
pub const TX_PWMGEAR: c_uint = 0x0024;
pub const TX_AMPLITUDE: c_uint = 0x0025;
pub const TX_HS_SLEWRATE: c_uint = 0x0026;
pub const TX_SYNC_SOURCE: c_uint = 0x0027;
pub const TX_HS_SYNC_LENGTH: c_uint = 0x0028;
pub const TX_HS_PREPARE_LENGTH: c_uint = 0x0029;
pub const TX_LS_PREPARE_LENGTH: c_uint = 0x002A;
pub const TX_HIBERN8_CONTROL: c_uint = 0x002B;
pub const TX_LCC_ENABLE: c_uint = 0x002C;
pub const TX_PWM_BURST_CLOSURE_EXTENSION: c_uint = 0x002D;
pub const TX_BYPASS_8B10B_ENABLE: c_uint = 0x002E;
pub const TX_DRIVER_POLARITY: c_uint = 0x002F;
pub const TX_HS_UNTERMINATED_LINE_DRIVE_ENABLE: c_uint = 0x0030;
pub const TX_LS_TERMINATED_LINE_DRIVE_ENABLE: c_uint = 0x0031;
pub const TX_LCC_SEQUENCER: c_uint = 0x0032;
pub const TX_MIN_ACTIVATETIME: c_uint = 0x0033;
pub const TX_PWM_G6_G7_SYNC_LENGTH: c_uint = 0x0034;
pub const TX_HS_DEEMPHASIS_SETTING: c_uint = 0x0037;
pub const TX_HS_PRESHOOT_SETTING: c_uint = 0x003B;
pub const TX_REFCLKFREQ: c_uint = 0x00EB;
pub const TX_CFGCLKFREQVAL: c_uint = 0x00EC;
pub const CFGEXTRATTR: c_uint = 0x00F0;
pub const DITHERCTRL2: c_uint = 0x00F1;
//
// M-RX Configuration Attributes
//
pub const RX_HS_G5_ADAPT_INITIAL_CAP: c_uint = 0x0074;
pub const RX_HS_G6_ADAPT_INITIAL_CAP: c_uint = 0x007B;
pub const RX_HS_G6_ADAPT_INITIAL_L0L1L2L3_CAP: c_uint = 0x007D;
pub const RX_HS_G1_SYNC_LENGTH_CAP: c_uint = 0x008B;
pub const RX_HS_G1_PREP_LENGTH_CAP: c_uint = 0x008C;
pub const RX_MIN_ACTIVATETIME_CAPABILITY: c_uint = 0x008F;
pub const RX_HIBERN8TIME_CAPABILITY: c_uint = 0x0092;
pub const RX_HS_G2_SYNC_LENGTH_CAP: c_uint = 0x0094;
pub const RX_HS_G3_SYNC_LENGTH_CAP: c_uint = 0x0095;
pub const RX_HS_G2_PREP_LENGTH_CAP: c_uint = 0x0096;
pub const RX_HS_G3_PREP_LENGTH_CAP: c_uint = 0x0097;
pub const RX_ADV_GRANULARITY_CAP: c_uint = 0x0098;
pub const RX_HIBERN8TIME_CAP: c_uint = 0x0092;
pub const RX_ADV_HIBERN8TIME_CAP: c_uint = 0x0099;
pub const RX_ADV_MIN_ACTIVATETIME_CAP: c_uint = 0x009A;
pub const RX_HS_G4_ADAPT_INITIAL_CAP: c_uint = 0x009F;
pub const RX_MODE: c_uint = 0x00A1;
pub const RX_HSRATE_SERIES: c_uint = 0x00A2;
pub const RX_HSGEAR: c_uint = 0x00A3;
pub const RX_PWMGEAR: c_uint = 0x00A4;
pub const RX_LS_TERMINATED_ENABLE: c_uint = 0x00A5;
pub const RX_HS_UNTERMINATED_ENABLE: c_uint = 0x00A6;
pub const RX_ENTER_HIBERN8: c_uint = 0x00A7;
pub const RX_BYPASS_8B10B_ENABLE: c_uint = 0x00A8;
pub const RX_TERMINATION_FORCE_ENABLE: c_uint = 0x00A9;
pub const RXCALCTRL: c_uint = 0x00B4;
pub const RXSQCTRL: c_uint = 0x00B5;
pub const CFGRXCDR8: c_uint = 0x00BA;
pub const CFGRXOVR8: c_uint = 0x00BD;
pub const CFGRXOVR6: c_uint = 0x00BF;
pub const RX_FOM: c_uint = 0x00C2;
pub const RXDIRECTCTRL2: c_uint = 0x00C7;
pub const CFGRXOVR4: c_uint = 0x00E9;
pub const RX_REFCLKFREQ: c_uint = 0x00EB;
pub const RX_CFGCLKFREQVAL: c_uint = 0x00EC;
pub const CFGWIDEINLN: c_uint = 0x00F0;
pub const RX_EYEMON_CAP: c_uint = 0x00F1;
pub const RX_EYEMON_TIMING_MAX_STEPS_CAP: c_uint = 0x00F2;
pub const RX_EYEMON_TIMING_MAX_OFFSET_CAP: c_uint = 0x00F3;
pub const RX_EYEMON_VOLTAGE_MAX_STEPS_CAP: c_uint = 0x00F4;
pub const RX_EYEMON_VOLTAGE_MAX_OFFSET_CAP: c_uint = 0x00F5;
pub const RX_EYEMON_ENABLE: c_uint = 0x00F6;
pub const RX_EYEMON_TIMING_STEPS: c_uint = 0x00F7;
pub const RX_EYEMON_VOLTAGE_STEPS: c_uint = 0x00F8;
pub const RX_EYEMON_TARGET_TEST_COUNT: c_uint = 0x00F9;
pub const RX_EYEMON_TESTED_COUNT: c_uint = 0x00FA;
pub const RX_EYEMON_ERROR_COUNT: c_uint = 0x00FB;
pub const RX_EYEMON_START: c_uint = 0x00FC;
pub const RX_EYEMON_EXTENDED_ERROR_COUNT: c_uint = 0x00FD;
pub const ENARXDIRECTCFG4: c_uint = 0x00F2;
pub const ENARXDIRECTCFG3: c_uint = 0x00F3;
pub const ENARXDIRECTCFG2: c_uint = 0x00F4;

pub const RX_MIN_ACTIVATETIME_UNIT_US: c_int = 100;
pub const HIBERN8TIME_UNIT_US: c_int = 100;
//
// Common Block Attributes
//

pub const ADAPT_LENGTH_MASK: c_uint = 0x7F;

// Adapt definitions
pub const ADAPT_LENGTH_MAX: c_uint = 0x91;
pub const ADAPT_L0L3_LENGTH_MAX: c_uint = 0x90;
pub const ADAPT_L0L1L2L3_LENGTH_MAX: c_uint = 0x8C;
pub const TADAPT_FACTOR: c_int = 650;

//
// PHY Adapter attributes
//
pub const PA_PHY_TYPE: c_uint = 0x1500;
pub const PA_AVAILTXDATALANES: c_uint = 0x1520;
pub const PA_MAXTXSPEEDFAST: c_uint = 0x1521;
pub const PA_MAXTXSPEEDSLOW: c_uint = 0x1522;
pub const PA_MAXRXSPEEDFAST: c_uint = 0x1541;
pub const PA_MAXRXSPEEDSLOW: c_uint = 0x1542;
pub const PA_TXLINKSTARTUPHS: c_uint = 0x1544;
pub const PA_AVAILRXDATALANES: c_uint = 0x1540;
pub const PA_MINRXTRAILINGCLOCKS: c_uint = 0x1543;
pub const PA_TXHSG1SYNCLENGTH: c_uint = 0x1552;
pub const PA_TXHSG2SYNCLENGTH: c_uint = 0x1554;
pub const PA_TXHSG3SYNCLENGTH: c_uint = 0x1556;
pub const PA_LOCAL_TX_LCC_ENABLE: c_uint = 0x155E;
pub const PA_ACTIVETXDATALANES: c_uint = 0x1560;
pub const PA_CONNECTEDTXDATALANES: c_uint = 0x1561;
pub const PA_TXFORCECLOCK: c_uint = 0x1562;
pub const PA_TXPWRMODE: c_uint = 0x1563;
pub const PA_TXTRAILINGCLOCKS: c_uint = 0x1564;
pub const PA_TXSPEEDFAST: c_uint = 0x1565;
pub const PA_TXSPEEDSLOW: c_uint = 0x1566;
pub const PA_TXPWRSTATUS: c_uint = 0x1567;
pub const PA_TXGEAR: c_uint = 0x1568;
pub const PA_TXTERMINATION: c_uint = 0x1569;
pub const PA_HSSERIES: c_uint = 0x156A;
pub const PA_LEGACYDPHYESCDL: c_uint = 0x1570;
pub const PA_PWRMODE: c_uint = 0x1571;
pub const PA_ACTIVERXDATALANES: c_uint = 0x1580;
pub const PA_CONNECTEDRXDATALANES: c_uint = 0x1581;
pub const PA_RXPWRSTATUS: c_uint = 0x1582;
pub const PA_RXGEAR: c_uint = 0x1583;
pub const PA_RXTERMINATION: c_uint = 0x1584;
pub const PA_MAXRXPWMGEAR: c_uint = 0x1586;
pub const PA_MAXRXHSGEAR: c_uint = 0x1587;
pub const PA_PACPREQTIMEOUT: c_uint = 0x1590;
pub const PA_PACPREQEOBTIMEOUT: c_uint = 0x1591;
pub const PA_REMOTEVERINFO: c_uint = 0x15A0;
pub const PA_LOGICALLANEMAP: c_uint = 0x15A1;
pub const PA_SLEEPNOCONFIGTIME: c_uint = 0x15A2;
pub const PA_STALLNOCONFIGTIME: c_uint = 0x15A3;
pub const PA_SAVECONFIGTIME: c_uint = 0x15A4;
pub const PA_RXHSUNTERMCAP: c_uint = 0x15A5;
pub const PA_RXLSTERMCAP: c_uint = 0x15A6;
pub const PA_HIBERN8TIME: c_uint = 0x15A7;
pub const PA_LOCALVERINFO: c_uint = 0x15A9;
pub const PA_GRANULARITY: c_uint = 0x15AA;
pub const PA_TACTIVATE: c_uint = 0x15A8;
pub const PA_PWRMODEUSERDATA0: c_uint = 0x15B0;
pub const PA_PWRMODEUSERDATA1: c_uint = 0x15B1;
pub const PA_PWRMODEUSERDATA2: c_uint = 0x15B2;
pub const PA_PWRMODEUSERDATA3: c_uint = 0x15B3;
pub const PA_PWRMODEUSERDATA4: c_uint = 0x15B4;
pub const PA_PWRMODEUSERDATA5: c_uint = 0x15B5;
pub const PA_PWRMODEUSERDATA6: c_uint = 0x15B6;
pub const PA_PWRMODEUSERDATA7: c_uint = 0x15B7;
pub const PA_PWRMODEUSERDATA8: c_uint = 0x15B8;
pub const PA_PWRMODEUSERDATA9: c_uint = 0x15B9;
pub const PA_PWRMODEUSERDATA10: c_uint = 0x15BA;
pub const PA_PWRMODEUSERDATA11: c_uint = 0x15BB;
pub const PA_PACPFRAMECOUNT: c_uint = 0x15C0;
pub const PA_PACPERRORCOUNT: c_uint = 0x15C1;
pub const PA_PHYTESTCONTROL: c_uint = 0x15C2;
pub const PA_TXHSG4SYNCLENGTH: c_uint = 0x15D0;
pub const PA_PEERRXHSG4ADAPTINITIAL: c_uint = 0x15D3;
pub const PA_TXHSADAPTTYPE: c_uint = 0x15D4;
pub const PA_TXHSG5SYNCLENGTH: c_uint = 0x15D6;
pub const PA_PEERRXHSG5ADAPTINITIAL: c_uint = 0x15D9;
pub const PA_PEERRXHSG6ADAPTREFRESHL0L1L2L3: c_uint = 0x15DE;
pub const PA_PEERRXHSG6ADAPTINITIALL0L3: c_uint = 0x15DF;
pub const PA_PEERRXHSG6ADAPTINITIALL0L1L2L3: c_uint = 0x15E0;
pub const PA_TXEQG1SETTING: c_uint = 0x15E1;
pub const PA_TXEQG2SETTING: c_uint = 0x15E2;
pub const PA_TXEQG3SETTING: c_uint = 0x15E3;
pub const PA_TXEQG4SETTING: c_uint = 0x15E4;
pub const PA_TXEQG5SETTING: c_uint = 0x15E5;
pub const PA_TXEQG6SETTING: c_uint = 0x15E6;
pub const PA_TXEQTRSETTING: c_uint = 0x15E7;
pub const PA_PEERTXEQTRSETTING: c_uint = 0x15E8;
pub const PA_PRECODEEN: c_uint = 0x15E9;
pub const PA_EQTR_GEAR: c_uint = 0x15EA;
pub const PA_TXADAPTLENGTH_EQTR: c_uint = 0x15EB;
// Adapt type for PA_TXHSADAPTTYPE attribute
pub const PA_REFRESH_ADAPT: c_uint = 0x00;
pub const PA_INITIAL_ADAPT: c_uint = 0x01;
pub const PA_NO_ADAPT: c_uint = 0x03;
pub const PA_TACTIVATE_TIME_UNIT_US: c_int = 10;
pub const PA_HIBERN8_TIME_UNIT_US: c_int = 100;
// Other attributes
pub const VS_POWERSTATE: c_uint = 0xD083;
pub const VS_MPHYCFGUPDT: c_uint = 0xD085;
pub const VS_DEBUGOMC: c_uint = 0xD09E;
pub const VS_MPHYDISABLE: c_uint = 0xD0C1;
pub const PA_GRANULARITY_MIN_VAL: c_int = 1;
pub const PA_GRANULARITY_MAX_VAL: c_int = 6;
// PHY Adapter Protocol Constants
pub const PA_MAXDATALANES: c_int = 4;
//
// TX EQTR's minimum TAdapt should not be less than 10us.
// This value is rounded up into the nearest Unit Intervals (UI)
//
pub const TX_EQTR_HS_G4_MIN_T_ADAPT: c_int = 166400;
pub const TX_EQTR_HS_G5_MIN_T_ADAPT: c_int = 332800;
pub const TX_EQTR_HS_G6_MIN_T_ADAPT: c_int = 262144;
pub const TX_EQTR_HS_G4_ADAPT_DEFAULT: c_uint = 0x88;
pub const TX_EQTR_HS_G5_ADAPT_DEFAULT: c_uint = 0x89;
pub const TX_EQTR_HS_G6_ADAPT_DEFAULT: c_uint = 0x89;
pub const TX_EQTR_CAP_MASK: c_uint = 0x7F;
pub const TX_EQTR_ADAPT_LENGTH_L0L1L2L3_SHIFT: c_int = 8;
pub const TX_EQTR_ADAPT_RESERVED: c_uint = 0xFF;
pub const TX_HS_NUM_PRESHOOT: c_int = 8;
pub const TX_HS_NUM_DEEMPHASIS: c_int = 8;
pub const TX_HS_PRESHOOT_SHIFT: c_int = 4;
pub const TX_HS_DEEMPHASIS_SHIFT: c_int = 4;
pub const TX_HS_PRESHOOT_OFFSET: c_int = 0;
pub const TX_HS_DEEMPHASIS_OFFSET: c_int = 16;

pub const RX_FOM_VALUE_MASK: c_uint = 0x7F;

pub const PRECODEEN_TX_OFFSET: c_int = 0;
pub const PRECODEEN_RX_OFFSET: c_int = 4;

#[repr(C)]
#[derive(Copy, Clone, PartialEq, Eq)]
pub enum ufs_tx_eq_preset {
    UFS_TX_EQ_PRESET_P0,
    UFS_TX_EQ_PRESET_P1,
    UFS_TX_EQ_PRESET_P2,
    UFS_TX_EQ_PRESET_P3,
    UFS_TX_EQ_PRESET_P4,
    UFS_TX_EQ_PRESET_P5,
    UFS_TX_EQ_PRESET_P6,
    UFS_TX_EQ_PRESET_P7,
    UFS_TX_EQ_PRESET_MAX,
}

#[repr(C)]
#[derive(Copy, Clone, PartialEq, Eq)]
pub enum ufs_tx_hs_preshoot {
    UFS_TX_HS_PRESHOOT_DB_0P0,
    UFS_TX_HS_PRESHOOT_DB_0P4,
    UFS_TX_HS_PRESHOOT_DB_0P8,
    UFS_TX_HS_PRESHOOT_DB_1P2,
    UFS_TX_HS_PRESHOOT_DB_1P6,
    UFS_TX_HS_PRESHOOT_DB_2P5,
    UFS_TX_HS_PRESHOOT_DB_3P5,
    UFS_TX_HS_PRESHOOT_DB_4P7,
}

#[repr(C)]
#[derive(Copy, Clone, PartialEq, Eq)]
pub enum ufs_tx_hs_deemphasis {
    UFS_TX_HS_DEEMPHASIS_DB_0P0,
    UFS_TX_HS_DEEMPHASIS_DB_0P8,
    UFS_TX_HS_DEEMPHASIS_DB_1P6,
    UFS_TX_HS_DEEMPHASIS_DB_2P5,
    UFS_TX_HS_DEEMPHASIS_DB_3P5,
    UFS_TX_HS_DEEMPHASIS_DB_4P7,
    UFS_TX_HS_DEEMPHASIS_DB_6P0,
    UFS_TX_HS_DEEMPHASIS_DB_7P6,
}

#[repr(C)]
#[derive(Copy, Clone, PartialEq, Eq)]
pub enum ufs_eom_eye_mask {
    UFS_EOM_EYE_MASK_M,
    UFS_EOM_EYE_MASK_L,
    UFS_EOM_EYE_MASK_U,
}

pub const DL_FC0ProtectionTimeOutVal_Default: c_int = 8191;
pub const DL_TC0ReplayTimeOutVal_Default: c_int = 65535;
pub const DL_AFC0ReqTimeOutVal_Default: c_int = 32767;
pub const DL_FC1ProtectionTimeOutVal_Default: c_int = 8191;
pub const DL_TC1ReplayTimeOutVal_Default: c_int = 65535;
pub const DL_AFC1ReqTimeOutVal_Default: c_int = 32767;
pub const DME_LocalFC0ProtectionTimeOutVal: c_uint = 0xD041;
pub const DME_LocalTC0ReplayTimeOutVal: c_uint = 0xD042;
pub const DME_LocalAFC0ReqTimeOutVal: c_uint = 0xD043;
#[repr(C)]
#[derive(Copy, Clone, PartialEq, Eq)]
pub enum ufs_op_mode {
    LS_MODE = 1,
    HS_MODE = 2,
}

// PA power modes
#[repr(C)]
#[derive(Copy, Clone, PartialEq, Eq)]
pub enum ufs_pa_pwr_mode {
    FAST_MODE	= 1,
    SLOW_MODE	= 2,
    FASTAUTO_MODE	= 4,
    SLOWAUTO_MODE	= 5,
    UNCHANGED	= 7,
}

pub const PWRMODE_MASK: c_uint = 0xF;
pub const PWRMODE_RX_OFFSET: c_int = 4;
// PA TX/RX Frequency Series
#[repr(C)]
#[derive(Copy, Clone, PartialEq, Eq)]
pub enum ufs_hs_gear_rate {
    PA_HS_MODE_A	= 1,
    PA_HS_MODE_B	= 2,
}

#[repr(C)]
#[derive(Copy, Clone, PartialEq, Eq)]
pub enum ufs_pwm_gear_tag {
    UFS_PWM_DONT_CHANGE,	/* Don't change Gear */
    UFS_PWM_G1,		/* PWM Gear 1 (default for reset) */
    UFS_PWM_G2,		/* PWM Gear 2 */
    UFS_PWM_G3,		/* PWM Gear 3 */
    UFS_PWM_G4,		/* PWM Gear 4 */
    UFS_PWM_G5,		/* PWM Gear 5 */
    UFS_PWM_G6,		/* PWM Gear 6 */
    UFS_PWM_G7,		/* PWM Gear 7 */
}

#[repr(C)]
#[derive(Copy, Clone, PartialEq, Eq)]
pub enum ufs_hs_gear_tag {
    UFS_HS_DONT_CHANGE,	/* Don't change Gear */
    UFS_HS_G1,		/* HS Gear 1 (default for reset) */
    UFS_HS_G2,		/* HS Gear 2 */
    UFS_HS_G3,		/* HS Gear 3 */
    UFS_HS_G4,		/* HS Gear 4 */
    UFS_HS_G5,		/* HS Gear 5 */
    UFS_HS_G6,		/* HS Gear 6 */
    UFS_HS_GEAR_MAX = UFS_HS_G6,
}

#[repr(C)]
#[derive(Copy, Clone, PartialEq, Eq)]
pub enum ufs_lanes {
    UFS_LANE_DONT_CHANGE,	/* Don't change Lane */
    UFS_LANE_1,		/* Lane 1 (default for reset) */
    UFS_LANE_2,		/* Lane 2 */
}

#[repr(C)]
#[derive(Copy, Clone, PartialEq, Eq)]
pub enum ufs_unipro_ver {
    UFS_UNIPRO_VER_RESERVED = 0,
    UFS_UNIPRO_VER_1_40 = 1, /* UniPro version 1.40 */
    UFS_UNIPRO_VER_1_41 = 2, /* UniPro version 1.41 */
    UFS_UNIPRO_VER_1_6  = 3, /* UniPro version 1.6 */
    UFS_UNIPRO_VER_1_61 = 4, /* UniPro version 1.61 */
    UFS_UNIPRO_VER_1_8  = 5, /* UniPro version 1.8 */
    UFS_UNIPRO_VER_MAX  = 6, /* UniPro unsupported version */
// UniPro version field mask in PA_LOCALVERINFO
    UFS_UNIPRO_VER_MASK = 0xF,
}

//
// Data Link Layer Attributes
//
pub const DL_TXPREEMPTIONCAP: c_uint = 0x2000;
pub const DL_TC0TXMAXSDUSIZE: c_uint = 0x2001;
pub const DL_TC0RXINITCREDITVAL: c_uint = 0x2002;
pub const DL_TC1TXMAXSDUSIZE: c_uint = 0x2003;
pub const DL_TC1RXINITCREDITVAL: c_uint = 0x2004;
pub const DL_TC0TXBUFFERSIZE: c_uint = 0x2005;
pub const DL_TC1TXBUFFERSIZE: c_uint = 0x2006;
pub const DL_TC0TXFCTHRESHOLD: c_uint = 0x2040;
pub const DL_FC0PROTTIMEOUTVAL: c_uint = 0x2041;
pub const DL_TC0REPLAYTIMEOUTVAL: c_uint = 0x2042;
pub const DL_AFC0REQTIMEOUTVAL: c_uint = 0x2043;
pub const DL_AFC0CREDITTHRESHOLD: c_uint = 0x2044;
pub const DL_TC0OUTACKTHRESHOLD: c_uint = 0x2045;
pub const DL_PEERTC0PRESENT: c_uint = 0x2046;
pub const DL_PEERTC0RXINITCREVAL: c_uint = 0x2047;
pub const DL_TC1TXFCTHRESHOLD: c_uint = 0x2060;
pub const DL_FC1PROTTIMEOUTVAL: c_uint = 0x2061;
pub const DL_TC1REPLAYTIMEOUTVAL: c_uint = 0x2062;
pub const DL_AFC1REQTIMEOUTVAL: c_uint = 0x2063;
pub const DL_AFC1CREDITTHRESHOLD: c_uint = 0x2064;
pub const DL_TC1OUTACKTHRESHOLD: c_uint = 0x2065;
pub const DL_PEERTC1PRESENT: c_uint = 0x2066;
pub const DL_PEERTC1RXINITCREVAL: c_uint = 0x2067;
//
// Network Layer Attributes
//
pub const N_DEVICEID: c_uint = 0x3000;
pub const N_DEVICEID_VALID: c_uint = 0x3001;
pub const N_TC0TXMAXSDUSIZE: c_uint = 0x3020;
pub const N_TC1TXMAXSDUSIZE: c_uint = 0x3021;
//
// Transport Layer Attributes
//
pub const T_NUMCPORTS: c_uint = 0x4000;
pub const T_NUMTESTFEATURES: c_uint = 0x4001;
pub const T_CONNECTIONSTATE: c_uint = 0x4020;
pub const T_PEERDEVICEID: c_uint = 0x4021;
pub const T_PEERCPORTID: c_uint = 0x4022;
pub const T_TRAFFICCLASS: c_uint = 0x4023;
pub const T_PROTOCOLID: c_uint = 0x4024;
pub const T_CPORTFLAGS: c_uint = 0x4025;
pub const T_TXTOKENVALUE: c_uint = 0x4026;
pub const T_RXTOKENVALUE: c_uint = 0x4027;
pub const T_LOCALBUFFERSPACE: c_uint = 0x4028;
pub const T_PEERBUFFERSPACE: c_uint = 0x4029;
pub const T_CREDITSTOSEND: c_uint = 0x402A;
pub const T_CPORTMODE: c_uint = 0x402B;
pub const T_TC0TXMAXSDUSIZE: c_uint = 0x4060;
pub const T_TC1TXMAXSDUSIZE: c_uint = 0x4061;
// CPort setting

// CPort connection state
