//! Automatically rewritten from C Header to Rust Module
//! Source: include/dt-bindings/interconnect/qcom,ipq5332.h
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
pub const MASTER_SNOC_PCIE3_1_M: c_int = 0;
pub const SLAVE_SNOC_PCIE3_1_M: c_int = 1;
pub const MASTER_ANOC_PCIE3_1_S: c_int = 2;
pub const SLAVE_ANOC_PCIE3_1_S: c_int = 3;
pub const MASTER_SNOC_PCIE3_2_M: c_int = 4;
pub const SLAVE_SNOC_PCIE3_2_M: c_int = 5;
pub const MASTER_ANOC_PCIE3_2_S: c_int = 6;
pub const SLAVE_ANOC_PCIE3_2_S: c_int = 7;
pub const MASTER_SNOC_USB: c_int = 8;
pub const SLAVE_SNOC_USB: c_int = 9;
pub const MASTER_NSSNOC_NSSCC: c_int = 10;
pub const SLAVE_NSSNOC_NSSCC: c_int = 11;
pub const MASTER_NSSNOC_SNOC_0: c_int = 12;
pub const SLAVE_NSSNOC_SNOC_0: c_int = 13;
pub const MASTER_NSSNOC_SNOC_1: c_int = 14;
pub const SLAVE_NSSNOC_SNOC_1: c_int = 15;
pub const MASTER_NSSNOC_ATB: c_int = 16;
pub const SLAVE_NSSNOC_ATB: c_int = 17;
pub const MASTER_NSSNOC_PCNOC_1: c_int = 18;
pub const SLAVE_NSSNOC_PCNOC_1: c_int = 19;
pub const MASTER_NSSNOC_QOSGEN_REF: c_int = 20;
pub const SLAVE_NSSNOC_QOSGEN_REF: c_int = 21;
pub const MASTER_NSSNOC_TIMEOUT_REF: c_int = 22;
pub const SLAVE_NSSNOC_TIMEOUT_REF: c_int = 23;
pub const MASTER_NSSNOC_XO_DCD: c_int = 24;
pub const SLAVE_NSSNOC_XO_DCD: c_int = 25;
pub const MASTER_NSSNOC_PPE: c_int = 0;
pub const SLAVE_NSSNOC_PPE: c_int = 1;
pub const MASTER_NSSNOC_PPE_CFG: c_int = 2;
pub const SLAVE_NSSNOC_PPE_CFG: c_int = 3;
pub const MASTER_NSSNOC_NSS_CSR: c_int = 4;
pub const SLAVE_NSSNOC_NSS_CSR: c_int = 5;
pub const MASTER_NSSNOC_CE_APB: c_int = 6;
pub const SLAVE_NSSNOC_CE_APB: c_int = 7;
pub const MASTER_NSSNOC_CE_AXI: c_int = 8;
pub const SLAVE_NSSNOC_CE_AXI: c_int = 9;
pub const MASTER_CNOC_AHB: c_int = 0;
pub const SLAVE_CNOC_AHB: c_int = 1;
