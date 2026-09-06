//! Automatically rewritten from C Header to Rust Module
//! Source: drivers/net/ethernet/stmicro/stmmac/dwmac4_descs.h
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


// SPDX-License-Identifier: GPL-2.0-only
//
// Header File to describe the DMA descriptors and related definitions specific
// for DesignWare databook 4.xx.
//
// Copyright (C) 2015  STMicroelectronics Ltd
//
// Author: Alexandre Torgue <alexandre.torgue@st.com>
//

// Normal transmit descriptor defines (without split feature)
// TDES2 (read format)

// TDES3 (read format)

// TDES3 (write back format)

pub const TDES3_TIMESTAMP_STATUS_SHIFT: c_int = 17;
// TDES3 context

// TDES3 Common

pub const TDES3_RS1V_SHIFT: c_int = 26;

pub const TDES3_LAST_DESCRIPTOR_SHIFT: c_int = 28;

pub const TDES3_CONTEXT_TYPE_SHIFT: c_int = 30;
// TDES4

// TDES5

// TDS3 use for both format (read and write back)

pub const TDES3_OWN_SHIFT: c_int = 31;
// Normal receive descriptor defines (without split feature)
// RDES0 (write back format)

// RDES1 (write back format)

pub const RDES1_TIMESTAMP_AVAILABLE_SHIFT: c_int = 14;

// RDES2 (write back format)

pub const RDES2_L3_L4_FILT_NB_MATCH_SHIFT: c_int = 26;

// RDES3 (write back format)

pub const RDES3_CONTEXT_DESCRIPTOR_SHIFT: c_int = 30;
// RDES3 (read format)

// TDS3 use for both format (read and write back)

