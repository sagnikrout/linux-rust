//! Automatically rewritten from C Header to Rust Module
//! Source: drivers/net/ethernet/wangxun/libwx/wx_err.h
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


// SPDX-License-Identifier: GPL-2.0
//
// WangXun Gigabit PCI Express Linux driver
// Copyright (c) 2015 - 2026 Beijing WangXun Technology Co., Ltd.
//
extern "C" {
    pub fn wx_check_err_subtask(wx: *mut wx);
}
extern "C" {
    pub fn wx_init_err_task(wx: *mut wx) -> c_int;
}
extern "C" {
    pub fn wx_check_hang_subtask(wx: *mut wx);
}
extern "C" {
    pub fn wx_tx_timeout(netdev: *mut net_device, txqueue: c_uint);
}
extern "C" {
    pub fn wx_handle_tx_hang(tx_ring: *mut wx_ring, next: c_uint);
}
