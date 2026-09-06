//! Automatically rewritten from C Header to Rust Module
//! Source: include/dt-bindings/mailbox/qcom-ipcc.h
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


// SPDX-License-Identifier: GPL-2.0 OR BSD-2-Clause
//
// Copyright (c) 2018-2020, The Linux Foundation. All rights reserved.
//
// Signal IDs for MPROC protocol
pub const IPCC_MPROC_SIGNAL_GLINK_QMP: c_int = 0;
pub const IPCC_MPROC_SIGNAL_TZ: c_int = 1;
pub const IPCC_MPROC_SIGNAL_SMP2P: c_int = 2;
pub const IPCC_MPROC_SIGNAL_PING: c_int = 3;
// Client IDs
pub const IPCC_CLIENT_AOP: c_int = 0;
pub const IPCC_CLIENT_TZ: c_int = 1;
pub const IPCC_CLIENT_MPSS: c_int = 2;
pub const IPCC_CLIENT_LPASS: c_int = 3;
pub const IPCC_CLIENT_SLPI: c_int = 4;
pub const IPCC_CLIENT_SDC: c_int = 5;
pub const IPCC_CLIENT_CDSP: c_int = 6;
pub const IPCC_CLIENT_NPU: c_int = 7;
pub const IPCC_CLIENT_APSS: c_int = 8;
pub const IPCC_CLIENT_GPU: c_int = 9;
pub const IPCC_CLIENT_CVP: c_int = 10;
pub const IPCC_CLIENT_CAM: c_int = 11;
pub const IPCC_CLIENT_VPU: c_int = 12;
pub const IPCC_CLIENT_PCIE0: c_int = 13;
pub const IPCC_CLIENT_PCIE1: c_int = 14;
pub const IPCC_CLIENT_PCIE2: c_int = 15;
pub const IPCC_CLIENT_SPSS: c_int = 16;
pub const IPCC_CLIENT_NSP1: c_int = 18;
pub const IPCC_CLIENT_TME: c_int = 23;
pub const IPCC_CLIENT_WPSS: c_int = 24;
pub const IPCC_CLIENT_GPDSP0: c_int = 31;
pub const IPCC_CLIENT_GPDSP1: c_int = 32;
