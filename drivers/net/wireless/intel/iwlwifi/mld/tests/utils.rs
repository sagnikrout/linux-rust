//! Automatically rewritten from C Header to Rust Module
//! Source: drivers/net/wireless/intel/iwlwifi/mld/tests/utils.h
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


// SPDX-License-Identifier: GPL-2.0 OR BSD-3-Clause
//
// Copyright (C) 2024-2026 Intel Corporation
//

// Macro flag: #define __iwl_mld_kunit_utils_h__

extern "C" {
    pub fn iwlmld_kunit_test_init(test: *mut kunit) -> c_int;
}
#[repr(C)]
#[derive(Copy, Clone)]
pub struct iwl_mld_kunit_link {
    pub chandef: *const cfg80211_chan_def,
    pub id: u8,
}

// Feel free to add more

// Feel free to add more

// Allocate a sta, initialize it and move it to the wanted state

//
// iwlmld_kunit_get_phy_of_link - Get the phy of a link
//
// @vif: The vif to get the phy from.
// @link_id: The id of the link to get the phy for.
//
// given a vif and link id, return the phy pointer of that link.
// This assumes that the link exists, and that it had a chanctx
// assigned.
// If this is not the case, the test will fail.
//
// Return: phy pointer.
//
