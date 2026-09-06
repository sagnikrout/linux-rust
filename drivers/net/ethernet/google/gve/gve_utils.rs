//! Automatically rewritten from C Header to Rust Module
//! Source: drivers/net/ethernet/google/gve/gve_utils.h
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


// SPDX-License-Identifier: (GPL-2.0 OR MIT)
// Google virtual Ethernet (gve) driver
//
// Copyright (C) 2015-2021 Google, Inc.
//

extern "C" {
    pub fn gve_tx_was_added_to_block(priv: *mut gve_priv, queue_idx: c_int) -> bool;
}
extern "C" {
    pub fn gve_tx_remove_from_block(priv: *mut gve_priv, queue_idx: c_int);
}
extern "C" {
    pub fn gve_tx_add_to_block(priv: *mut gve_priv, queue_idx: c_int);
}
extern "C" {
    pub fn gve_rx_was_added_to_block(priv: *mut gve_priv, queue_idx: c_int) -> bool;
}
extern "C" {
    pub fn gve_rx_remove_from_block(priv: *mut gve_priv, queue_idx: c_int);
}
extern "C" {
    pub fn gve_rx_add_to_block(priv: *mut gve_priv, queue_idx: c_int);
}
// Decrement pagecnt_bias. Set it back to INT_MAX if it reached zero.
extern "C" {
    pub fn gve_dec_pagecnt_bias(page_info: *mut gve_rx_slot_page_info);
}
extern "C" {
    pub fn gve_remove_napi(priv: *mut gve_priv, ntfy_idx: c_int);
}
