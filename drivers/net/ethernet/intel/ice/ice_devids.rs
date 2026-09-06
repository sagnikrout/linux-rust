//! Automatically rewritten from C Header to Rust Module
//! Source: drivers/net/ethernet/intel/ice/ice_devids.h
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
// Copyright (c) 2018-2023, Intel Corporation.
// Device IDs
pub const ICE_DEV_ID_E822_SI_DFLT: c_uint = 0x1888;
// Intel(R) Ethernet Controller E835-CC for backplane
pub const ICE_DEV_ID_E835CC_BACKPLANE: c_uint = 0x1248;
// Intel(R) Ethernet Controller E835-CC for QSFP
pub const ICE_DEV_ID_E835CC_QSFP56: c_uint = 0x1249;
// Intel(R) Ethernet Controller E835-CC for SFP
pub const ICE_DEV_ID_E835CC_SFP: c_uint = 0x124A;
// Intel(R) Ethernet Controller E835-C for backplane
pub const ICE_DEV_ID_E835C_BACKPLANE: c_uint = 0x1261;
// Intel(R) Ethernet Controller E835-C for QSFP
pub const ICE_DEV_ID_E835C_QSFP: c_uint = 0x1262;
// Intel(R) Ethernet Controller E835-C for SFP
pub const ICE_DEV_ID_E835C_SFP: c_uint = 0x1263;
// Intel(R) Ethernet Controller E835-L for backplane
pub const ICE_DEV_ID_E835_L_BACKPLANE: c_uint = 0x1265;
// Intel(R) Ethernet Controller E835-L for QSFP
pub const ICE_DEV_ID_E835_L_QSFP: c_uint = 0x1266;
// Intel(R) Ethernet Controller E835-L for SFP
pub const ICE_DEV_ID_E835_L_SFP: c_uint = 0x1267;
// Intel(R) Ethernet Connection E823-L for backplane
pub const ICE_DEV_ID_E823L_BACKPLANE: c_uint = 0x124C;
// Intel(R) Ethernet Connection E823-L for SFP
pub const ICE_DEV_ID_E823L_SFP: c_uint = 0x124D;
// Intel(R) Ethernet Connection E823-L/X557-AT 10GBASE-T
pub const ICE_DEV_ID_E823L_10G_BASE_T: c_uint = 0x124E;
// Intel(R) Ethernet Connection E823-L 1GbE
pub const ICE_DEV_ID_E823L_1GBE: c_uint = 0x124F;
// Intel(R) Ethernet Connection E823-L for QSFP
pub const ICE_DEV_ID_E823L_QSFP: c_uint = 0x151D;
// Intel(R) Ethernet Controller E830-CC for backplane
pub const ICE_DEV_ID_E830CC_BACKPLANE: c_uint = 0x12D1;
// Intel(R) Ethernet Controller E830-CC for QSFP
pub const ICE_DEV_ID_E830CC_QSFP56: c_uint = 0x12D2;
// Intel(R) Ethernet Controller E830-CC for SFP
pub const ICE_DEV_ID_E830CC_SFP: c_uint = 0x12D3;
// Intel(R) Ethernet Controller E830-CC for SFP-DD
pub const ICE_DEV_ID_E830CC_SFP_DD: c_uint = 0x12D4;
// Intel(R) Ethernet Controller E830-C for backplane
pub const ICE_DEV_ID_E830C_BACKPLANE: c_uint = 0x12D5;
// Intel(R) Ethernet Controller E830-C for QSFP
pub const ICE_DEV_ID_E830C_QSFP: c_uint = 0x12D8;
// Intel(R) Ethernet Controller E830-C for SFP
pub const ICE_DEV_ID_E830C_SFP: c_uint = 0x12DA;
// Intel(R) Ethernet Controller E830-XXV for backplane
pub const ICE_DEV_ID_E830_XXV_BACKPLANE: c_uint = 0x12DC;
// Intel(R) Ethernet Controller E830-XXV for QSFP
pub const ICE_DEV_ID_E830_XXV_QSFP: c_uint = 0x12DD;
// Intel(R) Ethernet Controller E830-XXV for SFP
pub const ICE_DEV_ID_E830_XXV_SFP: c_uint = 0x12DE;
// Intel(R) Ethernet Controller E810-C for backplane
pub const ICE_DEV_ID_E810C_BACKPLANE: c_uint = 0x1591;
// Intel(R) Ethernet Controller E810-C for QSFP
pub const ICE_DEV_ID_E810C_QSFP: c_uint = 0x1592;
// Intel(R) Ethernet Controller E810-C for SFP
pub const ICE_DEV_ID_E810C_SFP: c_uint = 0x1593;
pub const ICE_SUBDEV_ID_E810T: c_uint = 0x000E;
pub const ICE_SUBDEV_ID_E810T2: c_uint = 0x000F;
pub const ICE_SUBDEV_ID_E810T3: c_uint = 0x0010;
pub const ICE_SUBDEV_ID_E810T4: c_uint = 0x0011;
pub const ICE_SUBDEV_ID_E810T5: c_uint = 0x0012;
pub const ICE_SUBDEV_ID_E810T6: c_uint = 0x02E9;
pub const ICE_SUBDEV_ID_E810T7: c_uint = 0x02EA;
// Intel(R) Ethernet Controller E810-XXV for backplane
pub const ICE_DEV_ID_E810_XXV_BACKPLANE: c_uint = 0x1599;
// Intel(R) Ethernet Controller E810-XXV for QSFP
pub const ICE_DEV_ID_E810_XXV_QSFP: c_uint = 0x159A;
// Intel(R) Ethernet Controller E810-XXV for SFP
pub const ICE_DEV_ID_E810_XXV_SFP: c_uint = 0x159B;
// Intel(R) Ethernet Connection E823-C for backplane
pub const ICE_DEV_ID_E823C_BACKPLANE: c_uint = 0x188A;
// Intel(R) Ethernet Connection E823-C for QSFP
pub const ICE_DEV_ID_E823C_QSFP: c_uint = 0x188B;
// Intel(R) Ethernet Connection E823-C for SFP
pub const ICE_DEV_ID_E823C_SFP: c_uint = 0x188C;
// Intel(R) Ethernet Connection E823-C/X557-AT 10GBASE-T
pub const ICE_DEV_ID_E823C_10G_BASE_T: c_uint = 0x188D;
// Intel(R) Ethernet Connection E823-C 1GbE
pub const ICE_DEV_ID_E823C_SGMII: c_uint = 0x188E;
// Intel(R) Ethernet Connection E822-C for backplane
pub const ICE_DEV_ID_E822C_BACKPLANE: c_uint = 0x1890;
// Intel(R) Ethernet Connection E822-C for QSFP
pub const ICE_DEV_ID_E822C_QSFP: c_uint = 0x1891;
// Intel(R) Ethernet Connection E822-C for SFP
pub const ICE_DEV_ID_E822C_SFP: c_uint = 0x1892;
// Intel(R) Ethernet Connection E822-C/X557-AT 10GBASE-T
pub const ICE_DEV_ID_E822C_10G_BASE_T: c_uint = 0x1893;
// Intel(R) Ethernet Connection E822-C 1GbE
pub const ICE_DEV_ID_E822C_SGMII: c_uint = 0x1894;
// Intel(R) Ethernet Connection E822-L for backplane
pub const ICE_DEV_ID_E822L_BACKPLANE: c_uint = 0x1897;
// Intel(R) Ethernet Connection E822-L for SFP
pub const ICE_DEV_ID_E822L_SFP: c_uint = 0x1898;
// Intel(R) Ethernet Connection E822-L/X557-AT 10GBASE-T
pub const ICE_DEV_ID_E822L_10G_BASE_T: c_uint = 0x1899;
// Intel(R) Ethernet Connection E822-L 1GbE
pub const ICE_DEV_ID_E822L_SGMII: c_uint = 0x189A;
// Intel(R) Ethernet Connection E825-C for backplane
pub const ICE_DEV_ID_E825C_BACKPLANE: c_uint = 0x579c;
// Intel(R) Ethernet Connection E825-C for QSFP
pub const ICE_DEV_ID_E825C_QSFP: c_uint = 0x579d;
// Intel(R) Ethernet Connection E825-C for SFP
pub const ICE_DEV_ID_E825C_SFP: c_uint = 0x579e;
// Intel(R) Ethernet Connection E825-C 1GbE
pub const ICE_DEV_ID_E825C_SGMII: c_uint = 0x579f;
