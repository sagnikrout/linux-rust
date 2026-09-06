//! Automatically rewritten from C Header to Rust Module
//! Source: drivers/memory/tegra/tegra210-mc.h
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
// Copyright (c) 2015-2020, NVIDIA CORPORATION.  All rights reserved.
//

// register definitions
pub const MC_LATENCY_ALLOWANCE_AVPC_0: c_uint = 0x2e4;
pub const MC_LATENCY_ALLOWANCE_HC_0: c_uint = 0x310;
pub const MC_LATENCY_ALLOWANCE_HC_1: c_uint = 0x314;
pub const MC_LATENCY_ALLOWANCE_MPCORE_0: c_uint = 0x320;
pub const MC_LATENCY_ALLOWANCE_NVENC_0: c_uint = 0x328;
pub const MC_LATENCY_ALLOWANCE_PPCS_0: c_uint = 0x344;
pub const MC_LATENCY_ALLOWANCE_PPCS_1: c_uint = 0x348;
pub const MC_LATENCY_ALLOWANCE_ISP2_0: c_uint = 0x370;
pub const MC_LATENCY_ALLOWANCE_ISP2_1: c_uint = 0x374;
pub const MC_LATENCY_ALLOWANCE_XUSB_0: c_uint = 0x37c;
pub const MC_LATENCY_ALLOWANCE_XUSB_1: c_uint = 0x380;
pub const MC_LATENCY_ALLOWANCE_TSEC_0: c_uint = 0x390;
pub const MC_LATENCY_ALLOWANCE_VIC_0: c_uint = 0x394;
pub const MC_LATENCY_ALLOWANCE_VI2_0: c_uint = 0x398;
pub const MC_LATENCY_ALLOWANCE_GPU_0: c_uint = 0x3ac;
pub const MC_LATENCY_ALLOWANCE_SDMMCA_0: c_uint = 0x3b8;
pub const MC_LATENCY_ALLOWANCE_SDMMCAA_0: c_uint = 0x3bc;
pub const MC_LATENCY_ALLOWANCE_SDMMC_0: c_uint = 0x3c0;
pub const MC_LATENCY_ALLOWANCE_SDMMCAB_0: c_uint = 0x3c4;
pub const MC_LATENCY_ALLOWANCE_GPU2_0: c_uint = 0x3e8;
pub const MC_LATENCY_ALLOWANCE_NVDEC_0: c_uint = 0x3d8;
pub const MC_MLL_MPCORER_PTSA_RATE: c_uint = 0x44c;
pub const MC_FTOP_PTSA_RATE: c_uint = 0x50c;
pub const MC_EMEM_ARB_TIMING_RFCPB: c_uint = 0x6c0;
pub const MC_EMEM_ARB_TIMING_CCDMW: c_uint = 0x6c4;
pub const MC_EMEM_ARB_REFPB_HP_CTRL: c_uint = 0x6f0;
pub const MC_EMEM_ARB_REFPB_BANK_CTRL: c_uint = 0x6f4;
pub const MC_PTSA_GRANT_DECREMENT: c_uint = 0x960;
pub const MC_EMEM_ARB_DHYST_CTRL: c_uint = 0xbcc;
pub const MC_EMEM_ARB_DHYST_TIMEOUT_UTIL_0: c_uint = 0xbd0;
pub const MC_EMEM_ARB_DHYST_TIMEOUT_UTIL_1: c_uint = 0xbd4;
pub const MC_EMEM_ARB_DHYST_TIMEOUT_UTIL_2: c_uint = 0xbd8;
pub const MC_EMEM_ARB_DHYST_TIMEOUT_UTIL_3: c_uint = 0xbdc;
pub const MC_EMEM_ARB_DHYST_TIMEOUT_UTIL_4: c_uint = 0xbe0;
pub const MC_EMEM_ARB_DHYST_TIMEOUT_UTIL_5: c_uint = 0xbe4;
pub const MC_EMEM_ARB_DHYST_TIMEOUT_UTIL_6: c_uint = 0xbe8;
pub const MC_EMEM_ARB_DHYST_TIMEOUT_UTIL_7: c_uint = 0xbec;
