//! Automatically rewritten from C Header to Rust Module
//! Source: drivers/infiniband/hw/irdma/uda_d.h
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


// SPDX-License-Identifier: GPL-2.0 OR Linux-OpenIB
// Copyright (c) 2016 - 2021 Intel Corporation
// L4 packet type
pub const IRDMA_E_UDA_SQ_L4T_UNKNOWN: c_int = 0;
pub const IRDMA_E_UDA_SQ_L4T_TCP: c_int = 1;
pub const IRDMA_E_UDA_SQ_L4T_SCTP: c_int = 2;
pub const IRDMA_E_UDA_SQ_L4T_UDP: c_int = 3;
// Inner IP header type
pub const IRDMA_E_UDA_SQ_IIPT_UNKNOWN: c_int = 0;
pub const IRDMA_E_UDA_SQ_IIPT_IPV6: c_int = 1;
pub const IRDMA_E_UDA_SQ_IIPT_IPV4_NO_CSUM: c_int = 2;
pub const IRDMA_E_UDA_SQ_IIPT_IPV4_CSUM: c_int = 3;

pub const IRDMA_UDA_QPSQ_MACLEN_LINE: c_int = 2;

pub const IRDMA_UDA_QPSQ_IPLEN_LINE: c_int = 2;

pub const IRDMA_UDA_QPSQ_L4T_LINE: c_int = 2;

pub const IRDMA_UDA_QPSQ_IIPT_LINE: c_int = 2;
pub const IRDMA_UDA_QPSQ_DO_LPB_LINE: c_int = 3;

pub const IRDMA_UDA_QPSQ_FWD_PROG_CONFIRM_LINE: c_int = 3;

// Byte Offset 0

