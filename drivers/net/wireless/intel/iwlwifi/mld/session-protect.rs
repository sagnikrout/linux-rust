//! Automatically rewritten from C Header to Rust Module
//! Source: drivers/net/wireless/intel/iwlwifi/mld/session-protect.h
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
// Copyright (C) 2024-2025 Intel Corporation
//

// Macro flag: #define __session_protect_h__

//
// DOC: session protection
//
// Session protection is an API from the firmware that allows the driver to
// request time on medium. This is needed before the association when we need
// to be on medium for the association frame exchange. Once we configure the
// firmware as 'associated', the firmware will allocate time on medium without
// needed a session protection.
//
// TDLS discover uses this API as well even after association to ensure that
// other activities internal to the firmware will not interrupt our presence
// on medium.
//
// struct iwl_mld_session_protect - session protection parameters
// @end_jiffies: expected end_jiffies of current session protection.
// 0 if not active
// @duration: the duration in tu of current session
// @session_requested: A session protection command was sent and wasn't yet
// answered
//
#[repr(C)]
#[derive(Copy, Clone)]
pub struct iwl_mld_session_protect {
    pub end_jiffies: c_ulong,
    pub duration: u32,
    pub session_requested: bool,
}

pub const IWL_MLD_SESSION_PROTECTION_ASSOC_TIME_MS: c_int = 900;
pub const IWL_MLD_SESSION_PROTECTION_MIN_TIME_MS: c_int = 400;
//
// iwl_mld_handle_session_prot_notif - handles %SESSION_PROTECTION_NOTIF
// @mld: the mld component
// @pkt: the RX packet containing the notification
//
// iwl_mld_schedule_session_protection - schedule a session protection
// @mld: the mld component
// @vif: the virtual interface for which the protection issued
// @duration: the requested duration of the protection
// @min_duration: the minimum duration of the protection
// @link_id: The link to schedule a session protection for
//
// iwl_mld_start_session_protection - start a session protection
// @mld: the mld component
// @vif: the virtual interface for which the protection issued
// @duration: the requested duration of the protection
// @min_duration: the minimum duration of the protection
// @link_id: The link to schedule a session protection for
// @timeout: timeout for waiting
//
// This schedules the session protection, and waits for it to start
// (with timeout)
//
// Returns: 0 if successful, error code otherwise
//
// iwl_mld_cancel_session_protection - cancel the session protection.
// @mld: the mld component
// @vif: the virtual interface for which the session is issued
// @link_id: cancel the session protection for given link
//
// This functions cancels the session protection which is an act of good
// citizenship. If it is not needed any more it should be canceled because
// the other mac contexts wait for the medium during that time.
//
// Returns: 0 if successful, error code otherwise
//
