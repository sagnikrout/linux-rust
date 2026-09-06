//! Automatically rewritten from C Header to Rust Module
//! Source: include/linux/qat/qat_mig_dev.h
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
// Copyright(c) 2024 Intel Corporation
#[repr(C)]
#[derive(Copy, Clone)]
pub struct qat_mig_dev {
    pub parent_accel_dev: *mut c_void,
    pub state: *mut u8,
    pub setup_size: u32,
    pub remote_setup_size: u32,
    pub state_size: u32,
    pub vf_id: i32,
}

extern "C" {
    pub fn qat_vfmig_init(mdev: *mut qat_mig_dev) -> c_int;
}
extern "C" {
    pub fn qat_vfmig_cleanup(mdev: *mut qat_mig_dev);
}
extern "C" {
    pub fn qat_vfmig_reset(mdev: *mut qat_mig_dev);
}
extern "C" {
    pub fn qat_vfmig_open(mdev: *mut qat_mig_dev) -> c_int;
}
extern "C" {
    pub fn qat_vfmig_close(mdev: *mut qat_mig_dev);
}
extern "C" {
    pub fn qat_vfmig_suspend(mdev: *mut qat_mig_dev) -> c_int;
}
extern "C" {
    pub fn qat_vfmig_resume(mdev: *mut qat_mig_dev) -> c_int;
}
extern "C" {
    pub fn qat_vfmig_save_state(mdev: *mut qat_mig_dev) -> c_int;
}
extern "C" {
    pub fn qat_vfmig_save_setup(mdev: *mut qat_mig_dev) -> c_int;
}
extern "C" {
    pub fn qat_vfmig_load_state(mdev: *mut qat_mig_dev) -> c_int;
}
extern "C" {
    pub fn qat_vfmig_load_setup(mdev: *mut qat_mig_dev, size: c_int) -> c_int;
}
extern "C" {
    pub fn qat_vfmig_destroy(mdev: *mut qat_mig_dev);
}
