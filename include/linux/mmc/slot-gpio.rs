//! Automatically rewritten from C Header to Rust Module
//! Source: include/linux/mmc/slot-gpio.h
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
// Generic GPIO card-detect helper header
//
// Copyright (C) 2011, Guennadi Liakhovetski <g.liakhovetski@gmx.de>
//

extern "C" {
    pub fn mmc_gpio_get_ro(host: *mut mmc_host) -> c_int;
}
extern "C" {
    pub fn mmc_gpio_get_cd(host: *mut mmc_host) -> c_int;
}
extern "C" {
    pub fn mmc_gpio_set_cd_irq(host: *mut mmc_host, irq: c_int);
}
extern "C" {
    pub fn mmc_gpiod_set_cd_config(host: *mut mmc_host, config: c_ulong) -> c_int;
}
extern "C" {
    pub fn mmc_gpio_set_cd_wake(host: *mut mmc_host, on: bool) -> c_int;
}
extern "C" {
    pub fn mmc_gpiod_request_cd_irq(host: *mut mmc_host);
}
extern "C" {
    pub fn mmc_host_can_gpio_cd(host: *mut mmc_host) -> bool;
}
extern "C" {
    pub fn mmc_host_can_gpio_ro(host: *mut mmc_host) -> bool;
}
