//! Automatically rewritten from C Header to Rust Module
//! Source: drivers/mmc/core/pwrseq.h
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
// Copyright (C) 2014 Linaro Ltd
//
// Author: Ulf Hansson <ulf.hansson@linaro.org>
//

#[repr(C)]
#[derive(Copy, Clone)]
pub struct mmc_pwrseq_ops {
    pub host): *mut *mut void (pre_power_on)(struct mmc_host,
    pub host): *mut *mut void (post_power_on)(struct mmc_host,
    pub host): *mut *mut void (power_off)(struct mmc_host,
    pub host): *mut *mut void (reset)(struct mmc_host,
}

#[repr(C)]
#[derive(Copy, Clone)]
pub struct mmc_pwrseq {
    pub ops: *const mmc_pwrseq_ops,
    pub dev: *mut device,
    pub pwrseq_node: list_head,
    pub owner: *mut module,
}

extern "C" {
    pub fn mmc_pwrseq_register(pwrseq: *mut mmc_pwrseq) -> c_int;
}
extern "C" {
    pub fn mmc_pwrseq_unregister(pwrseq: *mut mmc_pwrseq);
}
extern "C" {
    pub fn mmc_pwrseq_alloc(host: *mut mmc_host) -> c_int;
}
extern "C" {
    pub fn mmc_pwrseq_pre_power_on(host: *mut mmc_host);
}
extern "C" {
    pub fn mmc_pwrseq_post_power_on(host: *mut mmc_host);
}
extern "C" {
    pub fn mmc_pwrseq_power_off(host: *mut mmc_host);
}
extern "C" {
    pub fn mmc_pwrseq_reset(host: *mut mmc_host);
}
extern "C" {
    pub fn mmc_pwrseq_free(host: *mut mmc_host);
}

