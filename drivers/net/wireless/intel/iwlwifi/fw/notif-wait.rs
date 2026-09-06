//! Automatically rewritten from C Header to Rust Module
//! Source: drivers/net/wireless/intel/iwlwifi/fw/notif-wait.h
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
// Copyright (C) 2005-2014, 2023 Intel Corporation
// Copyright (C) 2015-2017 Intel Deutschland GmbH
//

// Macro flag: #define __iwl_notif_wait_h__

#[repr(C)]
#[derive(Copy, Clone)]
pub struct iwl_notif_wait_data {
    pub notif_waits: list_head,
    pub notif_wait_lock: spinlock_t,
    pub notif_waitq: wait_queue_head_t,
}

pub const MAX_NOTIF_CMDS: c_int = 5;
//
// struct iwl_notification_wait - notification wait entry
// @list: list head for global list
// @fn: Function called with the notification. If the function
// returns true, the wait is over, if it returns false then
// the waiter stays blocked. If no function is given, any
// of the listed commands will unblock the waiter.
// @fn_data: pointer to pass to the @fn's data argument
// @cmds: command IDs
// @n_cmds: number of command IDs
// @triggered: waiter should be woken up
// @aborted: wait was aborted
//
// This structure is not used directly, to wait for a
// notification declare it on the stack, and call
// iwl_init_notification_wait() with appropriate
// parameters. Then do whatever will cause the ucode
// to notify the driver, and to wait for that then
// call iwl_wait_notification().
//
// Each notification is one-shot. If at some point we
// need to support multi-shot notifications (which
// can't be allocated on the stack) we need to modify
// the code for them.
//
#[repr(C)]
#[derive(Copy, Clone)]
pub struct iwl_notification_wait {
    pub list: list_head,
    pub data): *mut *mut iwl_rx_packet pkt, void,
    pub fn_data: *mut c_void,
    pub cmds: [u16; MAX_NOTIF_CMDS],
    pub n_cmds: u8,
    pub aborted: bool triggered,,
}

// caller functions
extern "C" {
    pub fn iwl_notification_wait_init(notif_data: *mut iwl_notif_wait_data);
}
extern "C" {
    pub fn iwl_abort_notification_waits(notif_data: *mut iwl_notif_wait_data);
}
// user functions
