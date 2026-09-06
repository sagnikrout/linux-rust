//! Automatically rewritten from C Header to Rust Module
//! Source: drivers/net/can/peak_canfd/peak_canfd_user.h
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
// CAN driver for PEAK System micro-CAN based adapters
//
// Copyright (C) 2003-2025 PEAK System-Technik GmbH
// Author: Stéphane Grosjean <s.grosjean@peak-system.fr>
//

// data structure private to each uCAN interface
#[repr(C)]
#[derive(Copy, Clone)]
pub struct peak_canfd_priv {
    pub /: *mut *mut can_priv can; / socket-can private data,
    pub /: *mut *mut *mut net_device ndev; / network device,
    pub /: *mut *mut int index; / channel index,
    pub /: *mut *mut can_berr_counter bec; / rx/tx err counters,
    pub /: *mut *mut int echo_idx; / echo skb free slot index,
    pub echo_lock: spinlock_t,
    pub cmd_len: c_int,
    pub cmd_buffer: *mut c_void,
    pub cmd_maxlen: c_int,
    pub priv): *mut *mut int (pre_cmd)(struct peak_canfd_priv,
    pub priv): *mut *mut int (write_cmd)(struct peak_canfd_priv,
    pub priv): *mut *mut int (post_cmd)(struct peak_canfd_priv,
    pub priv): *mut *mut int (enable_tx_path)(struct peak_canfd_priv,
    pub room_left): *mut c_int,
    pub msg): *mut pucan_tx_msg,
}
