//! Automatically rewritten from C Header to Rust Module
//! Source: sound/soc/codecs/sigmadsp.h
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


// SPDX-License-Identifier: GPL-2.0-or-later
//
// Load firmware files from Analog Devices SigmaStudio
//
// Copyright 2009-2011 Analog Devices Inc.
//

#[repr(C)]
#[derive(Copy, Clone)]
pub struct sigmadsp_ops {
    pub len): *const *const uint8_t data, size_t,
}

#[repr(C)]
#[derive(Copy, Clone)]
pub struct sigmadsp {
    pub ops: *const sigmadsp_ops,
    pub ctrl_list: list_head,
    pub data_list: list_head,
    pub rate_constraints: snd_pcm_hw_constraint_list,
    pub current_samplerate: c_uint,
    pub component: *mut snd_soc_component,
    pub dev: *mut device,
    pub lock: mutex,
    pub control_data: *mut c_void,
    pub size_t): *const *const *const *const int (write)(void , unsigned int, uint8_t ,,
    pub size_t): *mut *mut *mut *mut int (read)(void , unsigned int, uint8_t ,,
}

extern "C" {
    pub fn sigmadsp_setup(sigmadsp: *mut sigmadsp, samplerate: c_uint) -> c_int;
}
extern "C" {
    pub fn sigmadsp_reset(sigmadsp: *mut sigmadsp);
}
