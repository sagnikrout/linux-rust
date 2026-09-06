//! Automatically rewritten from C Header to Rust Module
//! Source: sound/pci/ctxfi/ctamixer.h
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
// Copyright (C) 2008, Creative Technology Ltd. All Rights Reserved.
//
// @File	ctamixer.h
//
// @Brief
// This file contains the definition of the Audio Mixer
// resource management object.
//
// @Author	Liu Chun
// @Date 	May 21 2008
//

// Define the descriptor of a summation node resource
#[repr(C)]
#[derive(Copy, Clone)]
pub struct sum {
    pub /: *mut *mut rsc rsc; / Basic resource info,
    pub idx: [c_uchar; 8],
}

// Define sum resource request description info
#[repr(C)]
#[derive(Copy, Clone)]
pub struct sum_desc {
    pub msr: c_uint,
}

#[repr(C)]
#[derive(Copy, Clone)]
pub struct sum_mgr {
    pub /: *mut *mut rsc_mgr mgr; / Basic resource manager info,
    pub /: *mut *mut *mut snd_card card; / pointer to this card,
    pub mgr_lock: spinlock_t,
// request one sum resource
    pub rsum): *const *const sum_desc desc, sum,
// return one sum resource
    pub sum): *mut *mut *mut int (put_sum)(struct sum_mgr mgr, struct sum,
}

// Constructor and destructor of daio resource manager
extern "C" {
    pub fn sum_mgr_create(hw: *mut hw, ptr: *mut c_void) -> c_int;
}
extern "C" {
    pub fn sum_mgr_destroy(ptr: *mut c_void) -> c_int;
}
// Define the descriptor of a amixer resource
#[repr(C)]
#[derive(Copy, Clone)]
pub struct amixer {
    pub /: *mut *mut rsc rsc; / Basic resource info,
    pub idx: [c_uchar; 8],
    pub /: *mut *mut *mut rsc input; / pointer to a resource acting as source,
    pub /: *mut *mut *mut sum sum; / Put amixer output to this summation node,
    pub /: *const *const *const amixer_rsc_ops ops; / AMixer specific operations,
}

#[repr(C)]
#[derive(Copy, Clone)]
pub struct amixer_rsc_ops {
    pub rsc): *mut *mut *mut int (set_input)(struct amixer amixer, struct rsc,
    pub scale): *mut *mut *mut int (set_scale)(struct amixer amixer, unsigned int,
    pub iv): *mut *mut *mut int (set_invalid_squash)(struct amixer amixer, unsigned int,
    pub sum): *mut *mut *mut int (set_sum)(struct amixer amixer, struct sum,
    pub amixer): *mut *mut int (commit_write)(struct amixer,
// Only for interleaved recording
    pub amixer): *mut *mut int (commit_raw_write)(struct amixer,
    pub sum): *mut unsigned int scale, struct sum,
    pub amixer): *mut *mut int (get_scale)(struct amixer,
}

// Define amixer resource request description info
#[repr(C)]
#[derive(Copy, Clone)]
pub struct amixer_desc {
    pub msr: c_uint,
}

#[repr(C)]
#[derive(Copy, Clone)]
pub struct amixer_mgr {
    pub /: *mut *mut rsc_mgr mgr; / Basic resource manager info,
    pub /: *mut *mut *mut snd_card card; / pointer to this card,
    pub mgr_lock: spinlock_t,
// request one amixer resource
    pub ramixer): *mut amixer,
// return one amixer resource
    pub amixer): *mut *mut *mut int (put_amixer)(struct amixer_mgr mgr, struct amixer,
}

// Constructor and destructor of amixer resource manager
extern "C" {
    pub fn amixer_mgr_create(hw: *mut hw, ramixer_mgr: *mut c_void) -> c_int;
}
extern "C" {
    pub fn amixer_mgr_destroy(amixer_mgr: *mut c_void) -> c_int;
}
