//! Automatically rewritten from C Header to Rust Module
//! Source: include/dt-bindings/interconnect/qcom,msm8937.h
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
// Qualcomm MSM8937 interconnect IDs
//
// BIMC fabric
pub const MAS_APPS_PROC: c_int = 0;
pub const MAS_OXILI: c_int = 1;
pub const MAS_SNOC_BIMC_0: c_int = 2;
pub const MAS_SNOC_BIMC_2: c_int = 3;
pub const MAS_SNOC_BIMC_1: c_int = 4;
pub const MAS_TCU_0: c_int = 5;
pub const SLV_EBI: c_int = 6;
pub const SLV_BIMC_SNOC: c_int = 7;
// PCNOC fabric
pub const MAS_SPDM: c_int = 0;
pub const MAS_BLSP_1: c_int = 1;
pub const MAS_BLSP_2: c_int = 2;
pub const MAS_USB_HS1: c_int = 3;
pub const MAS_XI_USB_HS1: c_int = 4;
pub const MAS_CRYPTO: c_int = 5;
pub const MAS_SDCC_1: c_int = 6;
pub const MAS_SDCC_2: c_int = 7;
pub const MAS_SNOC_PCNOC: c_int = 8;
pub const PCNOC_M_0: c_int = 9;
pub const PCNOC_M_1: c_int = 10;
pub const PCNOC_INT_0: c_int = 11;
pub const PCNOC_INT_1: c_int = 12;
pub const PCNOC_INT_2: c_int = 13;
pub const PCNOC_INT_3: c_int = 14;
pub const PCNOC_S_0: c_int = 15;
pub const PCNOC_S_1: c_int = 16;
pub const PCNOC_S_2: c_int = 17;
pub const PCNOC_S_3: c_int = 18;
pub const PCNOC_S_4: c_int = 19;
pub const PCNOC_S_6: c_int = 20;
pub const PCNOC_S_7: c_int = 21;
pub const PCNOC_S_8: c_int = 22;
pub const SLV_SDCC_2: c_int = 23;
pub const SLV_SPDM: c_int = 24;
pub const SLV_PDM: c_int = 25;
pub const SLV_PRNG: c_int = 26;
pub const SLV_TCSR: c_int = 27;
pub const SLV_SNOC_CFG: c_int = 28;
pub const SLV_MESSAGE_RAM: c_int = 29;
pub const SLV_CAMERA_SS_CFG: c_int = 30;
pub const SLV_DISP_SS_CFG: c_int = 31;
pub const SLV_VENUS_CFG: c_int = 32;
pub const SLV_GPU_CFG: c_int = 33;
pub const SLV_TLMM: c_int = 34;
pub const SLV_BLSP_1: c_int = 35;
pub const SLV_BLSP_2: c_int = 36;
pub const SLV_PMIC_ARB: c_int = 37;
pub const SLV_SDCC_1: c_int = 38;
pub const SLV_CRYPTO_0_CFG: c_int = 39;
pub const SLV_USB_HS: c_int = 40;
pub const SLV_TCU: c_int = 41;
pub const SLV_PCNOC_SNOC: c_int = 42;
// SNOC fabric
pub const MAS_QDSS_BAM: c_int = 0;
pub const MAS_BIMC_SNOC: c_int = 1;
pub const MAS_PCNOC_SNOC: c_int = 2;
pub const MAS_QDSS_ETR: c_int = 3;
pub const QDSS_INT: c_int = 4;
pub const SNOC_INT_0: c_int = 5;
pub const SNOC_INT_1: c_int = 6;
pub const SNOC_INT_2: c_int = 7;
pub const SLV_KPSS_AHB: c_int = 8;
pub const SLV_WCSS: c_int = 9;
pub const SLV_SNOC_BIMC_1: c_int = 10;
pub const SLV_IMEM: c_int = 11;
pub const SLV_SNOC_PCNOC: c_int = 12;
pub const SLV_QDSS_STM: c_int = 13;
pub const SLV_CATS_1: c_int = 14;
pub const SLV_LPASS: c_int = 15;
// SNOC-MM fabric
pub const MAS_JPEG: c_int = 0;
pub const MAS_MDP: c_int = 1;
pub const MAS_VENUS: c_int = 2;
pub const MAS_VFE0: c_int = 3;
pub const MAS_VFE1: c_int = 4;
pub const MAS_CPP: c_int = 5;
pub const SLV_SNOC_BIMC_0: c_int = 6;
pub const SLV_SNOC_BIMC_2: c_int = 7;
pub const SLV_CATS_0: c_int = 8;
