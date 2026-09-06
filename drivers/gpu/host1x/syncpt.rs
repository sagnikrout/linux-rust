//! Automatically rewritten from C Header to Rust Module
//! Source: drivers/gpu/host1x/syncpt.h
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
// Tegra host1x Syncpoints
//
// Copyright (c) 2010-2013, NVIDIA Corporation.
//

// Reserved for replacing an expired wait with a NOP
pub const HOST1X_SYNCPT_RESERVED: c_int = 0;
#[repr(C)]
#[derive(Copy, Clone)]
pub struct host1x_syncpt_base {
    pub id: c_uint,
    pub requested: bool,
}

#[repr(C)]
#[derive(Copy, Clone)]
pub struct host1x_syncpt {
    pub ref: kref,
    pub id: c_uint,
    pub min_val: core::sync::atomic::AtomicI32,
    pub max_val: core::sync::atomic::AtomicI32,
    pub base_val: u32,
    pub name: *const c_char,
    pub client_managed: bool,
    pub host: *mut host1x,
    pub base: *mut host1x_syncpt_base,
// interrupt data
    pub fences: host1x_fence_list,
//
// If a submission incrementing this syncpoint fails, lock it so that
// further submission cannot be made until application has handled the
// failure.
//
    pub locked: bool,
}

// Initialize sync point array
extern "C" {
    pub fn host1x_syncpt_init(host: *mut host1x) -> c_int;
}
// Free sync point array
extern "C" {
    pub fn host1x_syncpt_deinit(host: *mut host1x);
}
// Return number of sync point supported.
extern "C" {
    pub fn host1x_syncpt_nb_pts(host: *mut host1x) -> c_uint;
}
// Return number of wait bases supported.
extern "C" {
    pub fn host1x_syncpt_nb_bases(host: *mut host1x) -> c_uint;
}
// Return number of mlocks supported.
extern "C" {
    pub fn host1x_syncpt_nb_mlocks(host: *mut host1x) -> c_uint;
}
//
// Check sync point sanity. If max is larger than min, there have too many
// sync point increments.
//
// Client managed sync point are not tracked.
//
// Return true if sync point is client managed.
//
// Returns true if syncpoint min == max, which means that there are no
// outstanding operations.
//
// Load current value from hardware to the shadow register.
extern "C" {
    pub fn host1x_syncpt_load(sp: *mut host1x_syncpt) -> u32;
}
// Check if the given syncpoint value has already passed
extern "C" {
    pub fn host1x_syncpt_is_expired(sp: *mut host1x_syncpt, thresh: u32) -> bool;
}
// Save host1x sync point state into shadow registers.
extern "C" {
    pub fn host1x_syncpt_save(host: *mut host1x);
}
// Reset host1x sync point state from shadow registers.
extern "C" {
    pub fn host1x_syncpt_restore(host: *mut host1x);
}
// Read current wait base value into shadow register and return it.
extern "C" {
    pub fn host1x_syncpt_load_wait_base(sp: *mut host1x_syncpt) -> u32;
}
// Indicate future operations by incrementing the sync point max.
extern "C" {
    pub fn host1x_syncpt_incr_max(sp: *mut host1x_syncpt, incrs: u32) -> u32;
}
// Check if sync point id is valid.
