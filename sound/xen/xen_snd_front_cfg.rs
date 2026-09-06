//! Automatically rewritten from C Header to Rust Module
//! Source: sound/xen/xen_snd_front_cfg.h
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
pub struct xen_front_cfg_stream {
    pub index: c_int,
    pub xenstore_path: *mut c_char,
    pub pcm_hw: snd_pcm_hardware,
}

#[repr(C)]
#[derive(Copy, Clone)]
pub struct xen_front_cfg_pcm_instance {
    pub name: [c_char; 80],
    pub device_id: c_int,
    pub pcm_hw: snd_pcm_hardware,
    pub num_streams_pb: c_int,
    pub streams_pb: *mut xen_front_cfg_stream,
    pub num_streams_cap: c_int,
    pub streams_cap: *mut xen_front_cfg_stream,
}

#[repr(C)]
#[derive(Copy, Clone)]
pub struct xen_front_cfg_card {
    pub name_short: [c_char; 32],
    pub name_long: [c_char; 80],
    pub pcm_hw: snd_pcm_hardware,
    pub num_pcm_instances: c_int,
    pub pcm_instances: *mut xen_front_cfg_pcm_instance,
}
