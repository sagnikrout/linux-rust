//! Automatically rewritten from C Header to Rust Module
//! Source: drivers/net/wireless/intel/iwlwifi/iwl-devtrace.h
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
// Copyright(c) 2009 - 2014 Intel Corporation. All rights reserved.
// Copyright(C) 2016        Intel Deutschland GmbH
// Copyright(c) 2018, 2023, 2025 Intel Corporation
//

// If upper layers wanted TX status it's an important frame
// Try to determine if the frame is EAPOL. This might have false
// positives (if there's no RFC 1042 header and we compare to some
// payload instead) but since we're only doing tracing that's not
// a problem.
//
// don't account for crypto - these are unencrypted
// also account for the RFC 1042 header, of course
// (__be16 *)(skb->data + offs) != cpu_to_be16(ETH_P_PAE);
// out_hdr_offset = hdr_offset;
// maybe try to identify EAPOL frames?

// Macro flag: #define DECLARE_EVENT_CLASS(...)

extern "C" {
    pub fn __trace_iwlwifi_dev_rx(trans: *mut iwl_trans, pkt: *mut c_void, len: usize);
}

