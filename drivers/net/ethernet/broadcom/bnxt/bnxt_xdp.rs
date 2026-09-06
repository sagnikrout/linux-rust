//! Automatically rewritten from C Header to Rust Module
//! Source: drivers/net/ethernet/broadcom/bnxt/bnxt_xdp.h
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


// Broadcom NetXtreme-C/E network driver.
//
// Copyright (c) 2016-2017 Broadcom Limited
//
// This program is free software; you can redistribute it and/or modify
// it under the terms of the GNU General Public License as published by
// the Free Software Foundation.
//
#[repr(C)]
#[derive(Copy, Clone)]
pub struct bnxt_xdp_buff {
    pub xdp: xdp_buff,
    pub rxcmp: *mut rx_cmp,
    pub rxcmp1: *mut rx_cmp_ext,
    pub cmp_type: u8,
}

extern "C" {
    pub fn bnxt_tx_int_xdp(bp: *mut bnxt, bnapi: *mut bnxt_napi, budget: c_int);
}
extern "C" {
    pub fn bnxt_xdp(dev: *mut net_device, xdp: *mut netdev_bpf) -> c_int;
}
extern "C" {
    pub fn bnxt_xdp_attached(bp: *mut bnxt, rxr: *mut bnxt_rx_ring_info) -> bool;
}
