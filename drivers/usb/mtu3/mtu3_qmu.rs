//! Automatically rewritten from C Header to Rust Module
//! Source: drivers/usb/mtu3/mtu3_qmu.h
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
// mtu3_qmu.h - Queue Management Unit driver header
//
// Copyright (C) 2016 MediaTek Inc.
//
// Author: Chunfeng Yun <chunfeng.yun@mediatek.com>
//
pub const MAX_GPD_NUM: c_int = 64;

pub const GPD_BUF_SIZE: c_int = 65532;
pub const GPD_BUF_SIZE_EL: c_int = 1048572;
extern "C" {
    pub fn mtu3_qmu_stop(mep: *mut mtu3_ep);
}
extern "C" {
    pub fn mtu3_qmu_start(mep: *mut mtu3_ep) -> c_int;
}
extern "C" {
    pub fn mtu3_qmu_resume(mep: *mut mtu3_ep);
}
extern "C" {
    pub fn mtu3_qmu_flush(mep: *mut mtu3_ep);
}
extern "C" {
    pub fn mtu3_insert_gpd(mep: *mut mtu3_ep, mreq: *mut mtu3_request);
}
extern "C" {
    pub fn mtu3_prepare_transfer(mep: *mut mtu3_ep) -> c_int;
}
extern "C" {
    pub fn mtu3_gpd_ring_alloc(mep: *mut mtu3_ep) -> c_int;
}
extern "C" {
    pub fn mtu3_gpd_ring_free(mep: *mut mtu3_ep);
}
extern "C" {
    pub fn mtu3_qmu_isr(mtu: *mut mtu3) -> irqreturn_t;
}
extern "C" {
    pub fn mtu3_qmu_init(mtu: *mut mtu3) -> c_int;
}
extern "C" {
    pub fn mtu3_qmu_exit(mtu: *mut mtu3);
}
