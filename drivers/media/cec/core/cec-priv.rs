//! Automatically rewritten from C Header to Rust Module
//! Source: drivers/media/cec/core/cec-priv.h
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
// cec-priv.h - HDMI Consumer Electronics Control internal header
//
// Copyright 2016 Cisco Systems, Inc. and/or its affiliates. All rights reserved.
//

// devnode to cec_adapter

// cec-core.c
// cec-adap.c
extern "C" {
    pub fn cec_monitor_all_cnt_inc(adap: *mut cec_adapter) -> c_int;
}
extern "C" {
    pub fn cec_monitor_all_cnt_dec(adap: *mut cec_adapter);
}
extern "C" {
    pub fn cec_monitor_pin_cnt_inc(adap: *mut cec_adapter) -> c_int;
}
extern "C" {
    pub fn cec_monitor_pin_cnt_dec(adap: *mut cec_adapter);
}
extern "C" {
    pub fn cec_adap_status(file: *mut seq_file, priv: *mut c_void) -> c_int;
}
extern "C" {
    pub fn cec_thread_func(_adap: *mut c_void) -> c_int;
}
extern "C" {
    pub fn cec_adap_enable(adap: *mut cec_adapter) -> c_int;
}
extern "C" {
    pub fn __cec_s_phys_addr(adap: *mut cec_adapter, phys_addr: u16, block: bool);
}
// cec-api.c
