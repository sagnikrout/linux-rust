//! Automatically rewritten from C Header to Rust Module
//! Source: drivers/soc/apple/mailbox.h
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


// SPDX-License-Identifier: GPL-2.0-only OR MIT
//
// Apple mailbox message format
//
// Copyright The Asahi Linux Contributors
//

// encodes a single 96bit message sent over the single channel
#[repr(C)]
#[derive(Copy, Clone)]
pub struct apple_mbox_msg {
    pub msg0: u64,
    pub msg1: u32,
}

#[repr(C)]
#[derive(Copy, Clone)]
pub struct apple_mbox {
    pub dev: *mut device,
    pub regs: *mut void __iomem,
    pub hw: *const apple_mbox_hw,
    pub active: bool,
    pub irq_recv_not_empty: c_int,
    pub irq_send_empty: c_int,
    pub rx_lock: spinlock_t,
    pub tx_lock: spinlock_t,
    pub tx_empty: completion,
// Receive callback for incoming messages
    pub cookie): *mut *mut *mut void (rx)(struct apple_mbox mbox, struct apple_mbox_msg msg, void,
    pub cookie: *mut c_void,
}

extern "C" {
    pub fn apple_mbox_start(mbox: *mut apple_mbox) -> c_int;
}
extern "C" {
    pub fn apple_mbox_stop(mbox: *mut apple_mbox);
}
extern "C" {
    pub fn apple_mbox_poll(mbox: *mut apple_mbox) -> c_int;
}
