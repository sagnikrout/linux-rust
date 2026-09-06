//! Automatically rewritten from C Header to Rust Module
//! Source: drivers/net/ethernet/wangxun/libwx/wx_vf_lib.h
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
// Copyright (c) 2015 - 2025 Beijing WangXun Technology Co., Ltd.
extern "C" {
    pub fn wx_write_eitr_vf(q_vector: *mut wx_q_vector);
}
extern "C" {
    pub fn wx_configure_msix_vf(wx: *mut wx);
}
extern "C" {
    pub fn wx_write_uc_addr_list_vf(netdev: *mut net_device) -> c_int;
}
extern "C" {
    pub fn wx_setup_psrtype_vf(wx: *mut wx);
}
extern "C" {
    pub fn wx_setup_vfmrqc_vf(wx: *mut wx);
}
extern "C" {
    pub fn wx_configure_tx_vf(wx: *mut wx);
}
extern "C" {
    pub fn wx_configure_rx_ring_vf(wx: *mut wx, ring: *mut wx_ring);
}
