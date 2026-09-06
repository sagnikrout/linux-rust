//! Automatically rewritten from C Header to Rust Module
//! Source: include/dt-bindings/interconnect/qcom,sdx65.h
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


// SPDX-License-Identifier: (GPL-2.0 OR BSD-2-Clause)
//
// Copyright (c) 2022, Qualcomm Innovation Center, Inc. All rights reserved.
//
pub const MASTER_LLCC: c_int = 0;
pub const SLAVE_EBI1: c_int = 1;
pub const MASTER_TCU_0: c_int = 0;
pub const MASTER_SNOC_GC_MEM_NOC: c_int = 1;
pub const MASTER_APPSS_PROC: c_int = 2;
pub const SLAVE_LLCC: c_int = 3;
pub const SLAVE_MEM_NOC_SNOC: c_int = 4;
pub const SLAVE_MEM_NOC_PCIE_SNOC: c_int = 5;
pub const MASTER_AUDIO: c_int = 0;
pub const MASTER_BLSP_1: c_int = 1;
pub const MASTER_QDSS_BAM: c_int = 2;
pub const MASTER_QPIC: c_int = 3;
pub const MASTER_SNOC_CFG: c_int = 4;
pub const MASTER_SPMI_FETCHER: c_int = 5;
pub const MASTER_ANOC_SNOC: c_int = 6;
pub const MASTER_IPA: c_int = 7;
pub const MASTER_MEM_NOC_SNOC: c_int = 8;
pub const MASTER_MEM_NOC_PCIE_SNOC: c_int = 9;
pub const MASTER_CRYPTO: c_int = 10;
pub const MASTER_IPA_PCIE: c_int = 11;
pub const MASTER_PCIE_0: c_int = 12;
pub const MASTER_QDSS_ETR: c_int = 13;
pub const MASTER_SDCC_1: c_int = 14;
pub const MASTER_USB3: c_int = 15;
pub const SLAVE_AOSS: c_int = 16;
pub const SLAVE_APPSS: c_int = 17;
pub const SLAVE_AUDIO: c_int = 18;
pub const SLAVE_BLSP_1: c_int = 19;
pub const SLAVE_CLK_CTL: c_int = 20;
pub const SLAVE_CRYPTO_0_CFG: c_int = 21;
pub const SLAVE_CNOC_DDRSS: c_int = 22;
pub const SLAVE_ECC_CFG: c_int = 23;
pub const SLAVE_IMEM_CFG: c_int = 24;
pub const SLAVE_IPA_CFG: c_int = 25;
pub const SLAVE_CNOC_MSS: c_int = 26;
pub const SLAVE_PCIE_PARF: c_int = 27;
pub const SLAVE_PDM: c_int = 28;
pub const SLAVE_PRNG: c_int = 29;
pub const SLAVE_QDSS_CFG: c_int = 30;
pub const SLAVE_QPIC: c_int = 31;
pub const SLAVE_SDCC_1: c_int = 32;
pub const SLAVE_SNOC_CFG: c_int = 33;
pub const SLAVE_SPMI_FETCHER: c_int = 34;
pub const SLAVE_SPMI_VGI_COEX: c_int = 35;
pub const SLAVE_TCSR: c_int = 36;
pub const SLAVE_TLMM: c_int = 37;
pub const SLAVE_USB3: c_int = 38;
pub const SLAVE_USB3_PHY_CFG: c_int = 39;
pub const SLAVE_ANOC_SNOC: c_int = 40;
pub const SLAVE_SNOC_MEM_NOC_GC: c_int = 41;
pub const SLAVE_IMEM: c_int = 42;
pub const SLAVE_SERVICE_SNOC: c_int = 43;
pub const SLAVE_PCIE_0: c_int = 44;
pub const SLAVE_QDSS_STM: c_int = 45;
pub const SLAVE_TCU: c_int = 46;
