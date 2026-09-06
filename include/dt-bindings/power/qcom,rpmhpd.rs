//! Automatically rewritten from C Header to Rust Module
//! Source: include/dt-bindings/power/qcom,rpmhpd.h
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


// SPDX-License-Identifier: (GPL-2.0-only OR BSD-2-Clause)
//
// Copyright (c) 2023, Qualcomm Innovation Center, Inc. All rights reserved.
//
// Generic RPMH Power Domain Indexes
pub const RPMHPD_CX: c_int = 0;
pub const RPMHPD_CX_AO: c_int = 1;
pub const RPMHPD_EBI: c_int = 2;
pub const RPMHPD_GFX: c_int = 3;
pub const RPMHPD_LCX: c_int = 4;
pub const RPMHPD_LMX: c_int = 5;
pub const RPMHPD_MMCX: c_int = 6;
pub const RPMHPD_MMCX_AO: c_int = 7;
pub const RPMHPD_MX: c_int = 8;
pub const RPMHPD_MX_AO: c_int = 9;
pub const RPMHPD_MXC: c_int = 10;
pub const RPMHPD_MXC_AO: c_int = 11;
pub const RPMHPD_MSS: c_int = 12;
pub const RPMHPD_NSP: c_int = 13;
pub const RPMHPD_NSP0: c_int = 14;
pub const RPMHPD_NSP1: c_int = 15;
pub const RPMHPD_QPHY: c_int = 16;
pub const RPMHPD_DDR: c_int = 17;
pub const RPMHPD_XO: c_int = 18;
pub const RPMHPD_NSP2: c_int = 19;
pub const RPMHPD_GMXC: c_int = 20;
pub const RPMHPD_DCX: c_int = 21;
pub const RPMHPD_GBX: c_int = 22;
pub const RPMHPD_NSP3: c_int = 23;
pub const RPMHPD_GFX1: c_int = 24;
// RPMh Power Domain performance levels
pub const RPMH_REGULATOR_LEVEL_RETENTION: c_int = 16;
pub const RPMH_REGULATOR_LEVEL_MIN_SVS: c_int = 48;
pub const RPMH_REGULATOR_LEVEL_LOW_SVS_D3_0: c_int = 49;
pub const RPMH_REGULATOR_LEVEL_LOW_SVS_D3: c_int = 50;
pub const RPMH_REGULATOR_LEVEL_LOW_SVS_D2_1: c_int = 51;
pub const RPMH_REGULATOR_LEVEL_LOW_SVS_D2: c_int = 52;
pub const RPMH_REGULATOR_LEVEL_LOW_SVS_D1_1: c_int = 54;
pub const RPMH_REGULATOR_LEVEL_LOW_SVS_D1_0: c_int = 55;
pub const RPMH_REGULATOR_LEVEL_LOW_SVS_D1: c_int = 56;
pub const RPMH_REGULATOR_LEVEL_LOW_SVS_D0_0: c_int = 59;
pub const RPMH_REGULATOR_LEVEL_LOW_SVS_D0: c_int = 60;
pub const RPMH_REGULATOR_LEVEL_LOW_SVS: c_int = 64;
pub const RPMH_REGULATOR_LEVEL_LOW_SVS_P1: c_int = 72;
pub const RPMH_REGULATOR_LEVEL_LOW_SVS_L0: c_int = 76;
pub const RPMH_REGULATOR_LEVEL_LOW_SVS_L1: c_int = 80;
pub const RPMH_REGULATOR_LEVEL_LOW_SVS_L2: c_int = 96;
pub const RPMH_REGULATOR_LEVEL_SVS: c_int = 128;
pub const RPMH_REGULATOR_LEVEL_SVS_L0: c_int = 144;
pub const RPMH_REGULATOR_LEVEL_SVS_L1: c_int = 192;
pub const RPMH_REGULATOR_LEVEL_SVS_L2: c_int = 224;
pub const RPMH_REGULATOR_LEVEL_SVS_L2_0: c_int = 225;
pub const RPMH_REGULATOR_LEVEL_NOM: c_int = 256;
pub const RPMH_REGULATOR_LEVEL_NOM_L0: c_int = 288;
pub const RPMH_REGULATOR_LEVEL_NOM_L1: c_int = 320;
pub const RPMH_REGULATOR_LEVEL_NOM_L2: c_int = 336;
pub const RPMH_REGULATOR_LEVEL_TURBO: c_int = 384;
pub const RPMH_REGULATOR_LEVEL_TURBO_L0: c_int = 400;
pub const RPMH_REGULATOR_LEVEL_TURBO_L1: c_int = 416;
pub const RPMH_REGULATOR_LEVEL_TURBO_L1_0: c_int = 417;
pub const RPMH_REGULATOR_LEVEL_TURBO_L1_1: c_int = 418;
pub const RPMH_REGULATOR_LEVEL_TURBO_L1_2: c_int = 419;
pub const RPMH_REGULATOR_LEVEL_TURBO_L2: c_int = 432;
pub const RPMH_REGULATOR_LEVEL_TURBO_L3: c_int = 448;
pub const RPMH_REGULATOR_LEVEL_TURBO_L3_0: c_int = 449;
pub const RPMH_REGULATOR_LEVEL_TURBO_L3_1: c_int = 450;
pub const RPMH_REGULATOR_LEVEL_TURBO_L3_2: c_int = 451;
pub const RPMH_REGULATOR_LEVEL_TURBO_L4: c_int = 452;
pub const RPMH_REGULATOR_LEVEL_TURBO_L5: c_int = 456;
pub const RPMH_REGULATOR_LEVEL_SUPER_TURBO: c_int = 464;
pub const RPMH_REGULATOR_LEVEL_SUPER_TURBO_NO_CPR: c_int = 480;
//
// Platform-specific power domain bindings. Don't add new entries here, use
// RPMHPD_* above.
//
// SA8775P Power Domain Indexes
pub const SA8775P_CX: c_int = 0;
pub const SA8775P_CX_AO: c_int = 1;
pub const SA8775P_DDR: c_int = 2;
pub const SA8775P_EBI: c_int = 3;
pub const SA8775P_GFX: c_int = 4;
pub const SA8775P_LCX: c_int = 5;
pub const SA8775P_LMX: c_int = 6;
pub const SA8775P_MMCX: c_int = 7;
pub const SA8775P_MMCX_AO: c_int = 8;
pub const SA8775P_MSS: c_int = 9;
pub const SA8775P_MX: c_int = 10;
pub const SA8775P_MX_AO: c_int = 11;
pub const SA8775P_MXC: c_int = 12;
pub const SA8775P_MXC_AO: c_int = 13;
pub const SA8775P_NSP0: c_int = 14;
pub const SA8775P_NSP1: c_int = 15;
pub const SA8775P_XO: c_int = 16;
// SDM670 Power Domain Indexes
pub const SDM670_MX: c_int = 0;
pub const SDM670_MX_AO: c_int = 1;
pub const SDM670_CX: c_int = 2;
pub const SDM670_CX_AO: c_int = 3;
pub const SDM670_LMX: c_int = 4;
pub const SDM670_LCX: c_int = 5;
pub const SDM670_GFX: c_int = 6;
pub const SDM670_MSS: c_int = 7;
// SDM845 Power Domain Indexes
pub const SDM845_EBI: c_int = 0;
pub const SDM845_MX: c_int = 1;
pub const SDM845_MX_AO: c_int = 2;
pub const SDM845_CX: c_int = 3;
pub const SDM845_CX_AO: c_int = 4;
pub const SDM845_LMX: c_int = 5;
pub const SDM845_LCX: c_int = 6;
pub const SDM845_GFX: c_int = 7;
pub const SDM845_MSS: c_int = 8;
// SDX55 Power Domain Indexes
pub const SDX55_MSS: c_int = 0;
pub const SDX55_MX: c_int = 1;
pub const SDX55_CX: c_int = 2;
// SDX65 Power Domain Indexes
pub const SDX65_MSS: c_int = 0;
pub const SDX65_MX: c_int = 1;
pub const SDX65_MX_AO: c_int = 2;
pub const SDX65_CX: c_int = 3;
pub const SDX65_CX_AO: c_int = 4;
pub const SDX65_MXC: c_int = 5;
// SM6350 Power Domain Indexes
pub const SM6350_CX: c_int = 0;
pub const SM6350_GFX: c_int = 1;
pub const SM6350_LCX: c_int = 2;
pub const SM6350_LMX: c_int = 3;
pub const SM6350_MSS: c_int = 4;
pub const SM6350_MX: c_int = 5;
// SM8150 Power Domain Indexes
pub const SM8150_MSS: c_int = 0;
pub const SM8150_EBI: c_int = 1;
pub const SM8150_LMX: c_int = 2;
pub const SM8150_LCX: c_int = 3;
pub const SM8150_GFX: c_int = 4;
pub const SM8150_MX: c_int = 5;
pub const SM8150_MX_AO: c_int = 6;
pub const SM8150_CX: c_int = 7;
pub const SM8150_CX_AO: c_int = 8;
pub const SM8150_MMCX: c_int = 9;
pub const SM8150_MMCX_AO: c_int = 10;
// SA8155P is a special case, kept for backwards compatibility

// SM8250 Power Domain Indexes
pub const SM8250_CX: c_int = 0;
pub const SM8250_CX_AO: c_int = 1;
pub const SM8250_EBI: c_int = 2;
pub const SM8250_GFX: c_int = 3;
pub const SM8250_LCX: c_int = 4;
pub const SM8250_LMX: c_int = 5;
pub const SM8250_MMCX: c_int = 6;
pub const SM8250_MMCX_AO: c_int = 7;
pub const SM8250_MX: c_int = 8;
pub const SM8250_MX_AO: c_int = 9;
// SM8350 Power Domain Indexes
pub const SM8350_CX: c_int = 0;
pub const SM8350_CX_AO: c_int = 1;
pub const SM8350_EBI: c_int = 2;
pub const SM8350_GFX: c_int = 3;
pub const SM8350_LCX: c_int = 4;
pub const SM8350_LMX: c_int = 5;
pub const SM8350_MMCX: c_int = 6;
pub const SM8350_MMCX_AO: c_int = 7;
pub const SM8350_MX: c_int = 8;
pub const SM8350_MX_AO: c_int = 9;
pub const SM8350_MXC: c_int = 10;
pub const SM8350_MXC_AO: c_int = 11;
pub const SM8350_MSS: c_int = 12;
// SM8450 Power Domain Indexes
pub const SM8450_CX: c_int = 0;
pub const SM8450_CX_AO: c_int = 1;
pub const SM8450_EBI: c_int = 2;
pub const SM8450_GFX: c_int = 3;
pub const SM8450_LCX: c_int = 4;
pub const SM8450_LMX: c_int = 5;
pub const SM8450_MMCX: c_int = 6;
pub const SM8450_MMCX_AO: c_int = 7;
pub const SM8450_MX: c_int = 8;
pub const SM8450_MX_AO: c_int = 9;
pub const SM8450_MXC: c_int = 10;
pub const SM8450_MXC_AO: c_int = 11;
pub const SM8450_MSS: c_int = 12;
// SM8550 Power Domain Indexes
pub const SM8550_CX: c_int = 0;
pub const SM8550_CX_AO: c_int = 1;
pub const SM8550_EBI: c_int = 2;
pub const SM8550_GFX: c_int = 3;
pub const SM8550_LCX: c_int = 4;
pub const SM8550_LMX: c_int = 5;
pub const SM8550_MMCX: c_int = 6;
pub const SM8550_MMCX_AO: c_int = 7;
pub const SM8550_MX: c_int = 8;
pub const SM8550_MX_AO: c_int = 9;
pub const SM8550_MXC: c_int = 10;
pub const SM8550_MXC_AO: c_int = 11;
pub const SM8550_MSS: c_int = 12;
pub const SM8550_NSP: c_int = 13;
// QDU1000/QRU1000 Power Domain Indexes
pub const QDU1000_EBI: c_int = 0;
pub const QDU1000_MSS: c_int = 1;
pub const QDU1000_CX: c_int = 2;
pub const QDU1000_MX: c_int = 3;
// SC7180 Power Domain Indexes
pub const SC7180_CX: c_int = 0;
pub const SC7180_CX_AO: c_int = 1;
pub const SC7180_GFX: c_int = 2;
pub const SC7180_MX: c_int = 3;
pub const SC7180_MX_AO: c_int = 4;
pub const SC7180_LMX: c_int = 5;
pub const SC7180_LCX: c_int = 6;
pub const SC7180_MSS: c_int = 7;
// SC7280 Power Domain Indexes
pub const SC7280_CX: c_int = 0;
pub const SC7280_CX_AO: c_int = 1;
pub const SC7280_EBI: c_int = 2;
pub const SC7280_GFX: c_int = 3;
pub const SC7280_MX: c_int = 4;
pub const SC7280_MX_AO: c_int = 5;
pub const SC7280_LMX: c_int = 6;
pub const SC7280_LCX: c_int = 7;
pub const SC7280_MSS: c_int = 8;
// SC8180X Power Domain Indexes
pub const SC8180X_CX: c_int = 0;
pub const SC8180X_CX_AO: c_int = 1;
pub const SC8180X_EBI: c_int = 2;
pub const SC8180X_GFX: c_int = 3;
pub const SC8180X_LCX: c_int = 4;
pub const SC8180X_LMX: c_int = 5;
pub const SC8180X_MMCX: c_int = 6;
pub const SC8180X_MMCX_AO: c_int = 7;
pub const SC8180X_MSS: c_int = 8;
pub const SC8180X_MX: c_int = 9;
pub const SC8180X_MX_AO: c_int = 10;
// SC8280XP Power Domain Indexes
pub const SC8280XP_CX: c_int = 0;
pub const SC8280XP_CX_AO: c_int = 1;
pub const SC8280XP_DDR: c_int = 2;
pub const SC8280XP_EBI: c_int = 3;
pub const SC8280XP_GFX: c_int = 4;
pub const SC8280XP_LCX: c_int = 5;
pub const SC8280XP_LMX: c_int = 6;
pub const SC8280XP_MMCX: c_int = 7;
pub const SC8280XP_MMCX_AO: c_int = 8;
pub const SC8280XP_MSS: c_int = 9;
pub const SC8280XP_MX: c_int = 10;
pub const SC8280XP_MXC: c_int = 12;
pub const SC8280XP_MX_AO: c_int = 11;
pub const SC8280XP_NSP: c_int = 13;
pub const SC8280XP_QPHY: c_int = 14;
pub const SC8280XP_XO: c_int = 15;
pub const SC8280XP_MXC_AO: c_int = 16;
