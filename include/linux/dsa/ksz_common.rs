//! Automatically rewritten from C Header to Rust Module
//! Source: include/linux/dsa/ksz_common.h
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
// Microchip switch tag common header
//
// Copyright (C) 2022 Microchip Technology Inc.
//

// All time stamps from the KSZ consist of 2 bits for seconds and 30 bits for
// nanoseconds. This is NOT the same as 32 bits for nanoseconds.
//

extern "C" {
    pub fn ns_to_ktime(_arg: ns) -> return;
}
#[repr(C)]
#[derive(Copy, Clone)]
pub struct ksz_deferred_xmit_work {
    pub dp: *mut dsa_port,
    pub skb: *mut sk_buff,
    pub work: kthread_work,
}

#[repr(C)]
#[derive(Copy, Clone)]
pub struct ksz_tagger_data {
    pub work): *mut *mut void (xmit_work_fn)(struct kthread_work,
    pub on): *mut *mut *mut void (hwtstamp_set_state)(struct dsa_switch ds, bool,
}

#[repr(C)]
#[derive(Copy, Clone)]
pub struct ksz_skb_cb {
    pub clone: *mut sk_buff,
    pub ptp_type: c_uint,
    pub update_correction: bool,
    pub tstamp: u32,
}

