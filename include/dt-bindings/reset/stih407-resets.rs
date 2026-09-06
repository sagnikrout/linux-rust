//! Automatically rewritten from C Header to Rust Module
//! Source: include/dt-bindings/reset/stih407-resets.h
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
//
// This header provides constants for the reset controller
// based peripheral powerdown requests on the STMicroelectronics
// STiH407 SoC.
//
// Powerdown requests control 0
pub const STIH407_EMISS_POWERDOWN: c_int = 0;
pub const STIH407_NAND_POWERDOWN: c_int = 1;
// Synp GMAC PowerDown
pub const STIH407_ETH1_POWERDOWN: c_int = 2;
// Powerdown requests control 1
pub const STIH407_USB3_POWERDOWN: c_int = 3;
pub const STIH407_USB2_PORT1_POWERDOWN: c_int = 4;
pub const STIH407_USB2_PORT0_POWERDOWN: c_int = 5;
pub const STIH407_PCIE1_POWERDOWN: c_int = 6;
pub const STIH407_PCIE0_POWERDOWN: c_int = 7;
pub const STIH407_SATA1_POWERDOWN: c_int = 8;
pub const STIH407_SATA0_POWERDOWN: c_int = 9;
// Reset defines
pub const STIH407_ETH1_SOFTRESET: c_int = 0;
pub const STIH407_MMC1_SOFTRESET: c_int = 1;
pub const STIH407_PICOPHY_SOFTRESET: c_int = 2;
pub const STIH407_IRB_SOFTRESET: c_int = 3;
pub const STIH407_PCIE0_SOFTRESET: c_int = 4;
pub const STIH407_PCIE1_SOFTRESET: c_int = 5;
pub const STIH407_SATA0_SOFTRESET: c_int = 6;
pub const STIH407_SATA1_SOFTRESET: c_int = 7;
pub const STIH407_MIPHY0_SOFTRESET: c_int = 8;
pub const STIH407_MIPHY1_SOFTRESET: c_int = 9;
pub const STIH407_MIPHY2_SOFTRESET: c_int = 10;
pub const STIH407_SATA0_PWR_SOFTRESET: c_int = 11;
pub const STIH407_SATA1_PWR_SOFTRESET: c_int = 12;
pub const STIH407_DELTA_SOFTRESET: c_int = 13;
pub const STIH407_BLITTER_SOFTRESET: c_int = 14;
pub const STIH407_HDTVOUT_SOFTRESET: c_int = 15;
pub const STIH407_HDQVDP_SOFTRESET: c_int = 16;
pub const STIH407_VDP_AUX_SOFTRESET: c_int = 17;
pub const STIH407_COMPO_SOFTRESET: c_int = 18;
pub const STIH407_HDMI_TX_PHY_SOFTRESET: c_int = 19;
pub const STIH407_JPEG_DEC_SOFTRESET: c_int = 20;
pub const STIH407_VP8_DEC_SOFTRESET: c_int = 21;
pub const STIH407_GPU_SOFTRESET: c_int = 22;
pub const STIH407_HVA_SOFTRESET: c_int = 23;
pub const STIH407_ERAM_HVA_SOFTRESET: c_int = 24;
pub const STIH407_LPM_SOFTRESET: c_int = 25;
pub const STIH407_KEYSCAN_SOFTRESET: c_int = 26;
pub const STIH407_USB2_PORT0_SOFTRESET: c_int = 27;
pub const STIH407_USB2_PORT1_SOFTRESET: c_int = 28;
pub const STIH407_ST231_AUD_SOFTRESET: c_int = 29;
pub const STIH407_ST231_DMU_SOFTRESET: c_int = 30;
pub const STIH407_ST231_GP0_SOFTRESET: c_int = 31;
pub const STIH407_ST231_GP1_SOFTRESET: c_int = 32;
// Picophy reset defines
pub const STIH407_PICOPHY0_RESET: c_int = 0;
pub const STIH407_PICOPHY1_RESET: c_int = 1;
pub const STIH407_PICOPHY2_RESET: c_int = 2;
