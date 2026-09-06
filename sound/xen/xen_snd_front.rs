//! Automatically rewritten from C Header to Rust Module
//! Source: sound/xen/xen_snd_front.h
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


// SPDX-License-Identifier: GPL-2.0 OR MIT
//
// Xen para-virtual sound device
//
// Copyright (C) 2016-2018 EPAM Systems Inc.
//
// Author: Oleksandr Andrushchenko <oleksandr_andrushchenko@epam.com>
//

#[repr(C)]
#[derive(Copy, Clone)]
pub struct xen_snd_front_info {
    pub xb_dev: *mut xenbus_device,
    pub card_info: *mut xen_snd_front_card_info,
    pub num_evt_pairs: c_int,
    pub evt_pairs: *mut xen_snd_front_evtchnl_pair,
    pub cfg: xen_front_cfg_card,
}

extern "C" {
    pub fn xen_snd_front_stream_close(evtchnl: *mut xen_snd_front_evtchnl) -> c_int;
}
