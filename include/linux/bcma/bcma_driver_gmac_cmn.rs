//! Automatically rewritten from C Header to Rust Module
//! Source: include/linux/bcma/bcma_driver_gmac_cmn.h
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

pub const BCMA_GMAC_CMN_STAG0: c_uint = 0x000;
pub const BCMA_GMAC_CMN_STAG1: c_uint = 0x004;
pub const BCMA_GMAC_CMN_STAG2: c_uint = 0x008;
pub const BCMA_GMAC_CMN_STAG3: c_uint = 0x00C;
pub const BCMA_GMAC_CMN_PARSER_CTL: c_uint = 0x020;
pub const BCMA_GMAC_CMN_MIB_MAX_LEN: c_uint = 0x024;
pub const BCMA_GMAC_CMN_PHY_ACCESS: c_uint = 0x100;
pub const BCMA_GMAC_CMN_PA_DATA_MASK: c_uint = 0x0000ffff;
pub const BCMA_GMAC_CMN_PA_ADDR_MASK: c_uint = 0x001f0000;
pub const BCMA_GMAC_CMN_PA_ADDR_SHIFT: c_int = 16;
pub const BCMA_GMAC_CMN_PA_REG_MASK: c_uint = 0x1f000000;
pub const BCMA_GMAC_CMN_PA_REG_SHIFT: c_int = 24;
pub const BCMA_GMAC_CMN_PA_WRITE: c_uint = 0x20000000;
pub const BCMA_GMAC_CMN_PA_START: c_uint = 0x40000000;
pub const BCMA_GMAC_CMN_PHY_CTL: c_uint = 0x104;
pub const BCMA_GMAC_CMN_PC_EPA_MASK: c_uint = 0x0000001f;
pub const BCMA_GMAC_CMN_PC_MCT_MASK: c_uint = 0x007f0000;
pub const BCMA_GMAC_CMN_PC_MCT_SHIFT: c_int = 16;
pub const BCMA_GMAC_CMN_PC_MTE: c_uint = 0x00800000;
pub const BCMA_GMAC_CMN_GMAC0_RGMII_CTL: c_uint = 0x110;
pub const BCMA_GMAC_CMN_CFP_ACCESS: c_uint = 0x200;
pub const BCMA_GMAC_CMN_CFP_TCAM_DATA0: c_uint = 0x210;
pub const BCMA_GMAC_CMN_CFP_TCAM_DATA1: c_uint = 0x214;
pub const BCMA_GMAC_CMN_CFP_TCAM_DATA2: c_uint = 0x218;
pub const BCMA_GMAC_CMN_CFP_TCAM_DATA3: c_uint = 0x21C;
pub const BCMA_GMAC_CMN_CFP_TCAM_DATA4: c_uint = 0x220;
pub const BCMA_GMAC_CMN_CFP_TCAM_DATA5: c_uint = 0x224;
pub const BCMA_GMAC_CMN_CFP_TCAM_DATA6: c_uint = 0x228;
pub const BCMA_GMAC_CMN_CFP_TCAM_DATA7: c_uint = 0x22C;
pub const BCMA_GMAC_CMN_CFP_TCAM_MASK0: c_uint = 0x230;
pub const BCMA_GMAC_CMN_CFP_TCAM_MASK1: c_uint = 0x234;
pub const BCMA_GMAC_CMN_CFP_TCAM_MASK2: c_uint = 0x238;
pub const BCMA_GMAC_CMN_CFP_TCAM_MASK3: c_uint = 0x23C;
pub const BCMA_GMAC_CMN_CFP_TCAM_MASK4: c_uint = 0x240;
pub const BCMA_GMAC_CMN_CFP_TCAM_MASK5: c_uint = 0x244;
pub const BCMA_GMAC_CMN_CFP_TCAM_MASK6: c_uint = 0x248;
pub const BCMA_GMAC_CMN_CFP_TCAM_MASK7: c_uint = 0x24C;
pub const BCMA_GMAC_CMN_CFP_ACTION_DATA: c_uint = 0x250;
pub const BCMA_GMAC_CMN_TCAM_BIST_CTL: c_uint = 0x2A0;
pub const BCMA_GMAC_CMN_TCAM_BIST_STATUS: c_uint = 0x2A4;
pub const BCMA_GMAC_CMN_TCAM_CMP_STATUS: c_uint = 0x2A8;
pub const BCMA_GMAC_CMN_TCAM_DISABLE: c_uint = 0x2AC;
pub const BCMA_GMAC_CMN_TCAM_TEST_CTL: c_uint = 0x2F0;
pub const BCMA_GMAC_CMN_UDF_0_A3_A0: c_uint = 0x300;
pub const BCMA_GMAC_CMN_UDF_0_A7_A4: c_uint = 0x304;
pub const BCMA_GMAC_CMN_UDF_0_A8: c_uint = 0x308;
pub const BCMA_GMAC_CMN_UDF_1_A3_A0: c_uint = 0x310;
pub const BCMA_GMAC_CMN_UDF_1_A7_A4: c_uint = 0x314;
pub const BCMA_GMAC_CMN_UDF_1_A8: c_uint = 0x318;
pub const BCMA_GMAC_CMN_UDF_2_A3_A0: c_uint = 0x320;
pub const BCMA_GMAC_CMN_UDF_2_A7_A4: c_uint = 0x324;
pub const BCMA_GMAC_CMN_UDF_2_A8: c_uint = 0x328;
pub const BCMA_GMAC_CMN_UDF_0_B3_B0: c_uint = 0x330;
pub const BCMA_GMAC_CMN_UDF_0_B7_B4: c_uint = 0x334;
pub const BCMA_GMAC_CMN_UDF_0_B8: c_uint = 0x338;
pub const BCMA_GMAC_CMN_UDF_1_B3_B0: c_uint = 0x340;
pub const BCMA_GMAC_CMN_UDF_1_B7_B4: c_uint = 0x344;
pub const BCMA_GMAC_CMN_UDF_1_B8: c_uint = 0x348;
pub const BCMA_GMAC_CMN_UDF_2_B3_B0: c_uint = 0x350;
pub const BCMA_GMAC_CMN_UDF_2_B7_B4: c_uint = 0x354;
pub const BCMA_GMAC_CMN_UDF_2_B8: c_uint = 0x358;
pub const BCMA_GMAC_CMN_UDF_0_C3_C0: c_uint = 0x360;
pub const BCMA_GMAC_CMN_UDF_0_C7_C4: c_uint = 0x364;
pub const BCMA_GMAC_CMN_UDF_0_C8: c_uint = 0x368;
pub const BCMA_GMAC_CMN_UDF_1_C3_C0: c_uint = 0x370;
pub const BCMA_GMAC_CMN_UDF_1_C7_C4: c_uint = 0x374;
pub const BCMA_GMAC_CMN_UDF_1_C8: c_uint = 0x378;
pub const BCMA_GMAC_CMN_UDF_2_C3_C0: c_uint = 0x380;
pub const BCMA_GMAC_CMN_UDF_2_C7_C4: c_uint = 0x384;
pub const BCMA_GMAC_CMN_UDF_2_C8: c_uint = 0x388;
pub const BCMA_GMAC_CMN_UDF_0_D3_D0: c_uint = 0x390;
pub const BCMA_GMAC_CMN_UDF_0_D7_D4: c_uint = 0x394;
pub const BCMA_GMAC_CMN_UDF_0_D11_D8: c_uint = 0x394;
#[repr(C)]
#[derive(Copy, Clone)]
pub struct bcma_drv_gmac_cmn {
    pub core: *mut bcma_device,
// Drivers accessing BCMA_GMAC_CMN_PHY_ACCESS and
// BCMA_GMAC_CMN_PHY_CTL need to take that mutex first.
    pub phy_mutex: mutex,
}

// Register access

