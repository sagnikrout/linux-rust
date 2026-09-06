//! Automatically rewritten from C Header to Rust Module
//! Source: include/dt-bindings/interconnect/qcom,qcs404.h
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
// Qualcomm interconnect IDs
//
// Copyright (c) 2019, Linaro Ltd.
// Author: Georgi Djakov <georgi.djakov@linaro.org>
//
pub const MASTER_AMPSS_M0: c_int = 0;
pub const MASTER_OXILI: c_int = 1;
pub const MASTER_MDP_PORT0: c_int = 2;
pub const MASTER_SNOC_BIMC_1: c_int = 3;
pub const MASTER_TCU_0: c_int = 4;
pub const SLAVE_EBI_CH0: c_int = 5;
pub const SLAVE_BIMC_SNOC: c_int = 6;
pub const MASTER_SPDM: c_int = 0;
pub const MASTER_BLSP_1: c_int = 1;
pub const MASTER_BLSP_2: c_int = 2;
pub const MASTER_XI_USB_HS1: c_int = 3;
pub const MASTER_CRYPT0: c_int = 4;
pub const MASTER_SDCC_1: c_int = 5;
pub const MASTER_SDCC_2: c_int = 6;
pub const MASTER_SNOC_PCNOC: c_int = 7;
pub const MASTER_QPIC: c_int = 8;
pub const PCNOC_INT_0: c_int = 9;
pub const PCNOC_INT_2: c_int = 10;
pub const PCNOC_INT_3: c_int = 11;
pub const PCNOC_S_0: c_int = 12;
pub const PCNOC_S_1: c_int = 13;
pub const PCNOC_S_2: c_int = 14;
pub const PCNOC_S_3: c_int = 15;
pub const PCNOC_S_4: c_int = 16;
pub const PCNOC_S_6: c_int = 17;
pub const PCNOC_S_7: c_int = 18;
pub const PCNOC_S_8: c_int = 19;
pub const PCNOC_S_9: c_int = 20;
pub const PCNOC_S_10: c_int = 21;
pub const PCNOC_S_11: c_int = 22;
pub const SLAVE_SPDM: c_int = 23;
pub const SLAVE_PDM: c_int = 24;
pub const SLAVE_PRNG: c_int = 25;
pub const SLAVE_TCSR: c_int = 26;
pub const SLAVE_SNOC_CFG: c_int = 27;
pub const SLAVE_MESSAGE_RAM: c_int = 28;
pub const SLAVE_DISP_SS_CFG: c_int = 29;
pub const SLAVE_GPU_CFG: c_int = 30;
pub const SLAVE_BLSP_1: c_int = 31;
pub const SLAVE_BLSP_2: c_int = 32;
pub const SLAVE_TLMM_NORTH: c_int = 33;
pub const SLAVE_PCIE: c_int = 34;
pub const SLAVE_ETHERNET: c_int = 35;
pub const SLAVE_TLMM_EAST: c_int = 36;
pub const SLAVE_TCU: c_int = 37;
pub const SLAVE_PMIC_ARB: c_int = 38;
pub const SLAVE_SDCC_1: c_int = 39;
pub const SLAVE_SDCC_2: c_int = 40;
pub const SLAVE_TLMM_SOUTH: c_int = 41;
pub const SLAVE_USB_HS: c_int = 42;
pub const SLAVE_USB3: c_int = 43;
pub const SLAVE_CRYPTO_0_CFG: c_int = 44;
pub const SLAVE_PCNOC_SNOC: c_int = 45;
pub const MASTER_QDSS_BAM: c_int = 0;
pub const MASTER_BIMC_SNOC: c_int = 1;
pub const MASTER_PCNOC_SNOC: c_int = 2;
pub const MASTER_QDSS_ETR: c_int = 3;
pub const MASTER_EMAC: c_int = 4;
pub const MASTER_PCIE: c_int = 5;
pub const MASTER_USB3: c_int = 6;
pub const QDSS_INT: c_int = 7;
pub const SNOC_INT_0: c_int = 8;
pub const SNOC_INT_1: c_int = 9;
pub const SNOC_INT_2: c_int = 10;
pub const SLAVE_KPSS_AHB: c_int = 11;
pub const SLAVE_WCSS: c_int = 12;
pub const SLAVE_SNOC_BIMC_1: c_int = 13;
pub const SLAVE_IMEM: c_int = 14;
pub const SLAVE_SNOC_PCNOC: c_int = 15;
pub const SLAVE_QDSS_STM: c_int = 16;
pub const SLAVE_CATS_0: c_int = 17;
pub const SLAVE_CATS_1: c_int = 18;
pub const SLAVE_LPASS: c_int = 19;
