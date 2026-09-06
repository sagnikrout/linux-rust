//! Automatically rewritten from C Header to Rust Module
//! Source: drivers/net/wireless/silabs/wfx/bh.h
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
// Interrupt bottom half (BH).
//
// Copyright (c) 2017-2020, Silicon Laboratories, Inc.
// Copyright (c) 2010, ST-Ericsson
//

#[repr(C)]
#[derive(Copy, Clone)]
pub struct wfx_hif {
    pub bh: work_struct,
    pub ctrl_ready: completion,
    pub tx_buffers_empty: wait_queue_head_t,
    pub ctrl_reg: core::sync::atomic::AtomicI32,
    pub rx_seqnum: c_int,
    pub tx_seqnum: c_int,
    pub tx_buffers_used: c_int,
}

extern "C" {
    pub fn wfx_bh_register(wdev: *mut wfx_dev);
}
extern "C" {
    pub fn wfx_bh_unregister(wdev: *mut wfx_dev);
}
extern "C" {
    pub fn wfx_bh_request_rx(wdev: *mut wfx_dev);
}
extern "C" {
    pub fn wfx_bh_request_tx(wdev: *mut wfx_dev);
}
extern "C" {
    pub fn wfx_bh_poll_irq(wdev: *mut wfx_dev);
}
