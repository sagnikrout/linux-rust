//! Automatically rewritten from C Header to Rust Module
//! Source: sound/soc/pxa/pxa2xx-lib.h
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

// PCM
// AC97
extern "C" {
    pub fn pxa2xx_ac97_read(slot: c_int, reg: c_ushort) -> c_int;
}
extern "C" {
    pub fn pxa2xx_ac97_write(slot: c_int, reg: c_ushort, val: c_ushort) -> c_int;
}
extern "C" {
    pub fn pxa2xx_ac97_try_warm_reset() -> bool;
}
extern "C" {
    pub fn pxa2xx_ac97_try_cold_reset() -> bool;
}
extern "C" {
    pub fn pxa2xx_ac97_finish_reset();
}
extern "C" {
    pub fn pxa2xx_ac97_hw_suspend() -> c_int;
}
extern "C" {
    pub fn pxa2xx_ac97_hw_resume() -> c_int;
}
extern "C" {
    pub fn pxa2xx_ac97_hw_probe(dev: *mut platform_device) -> c_int;
}
extern "C" {
    pub fn pxa2xx_ac97_hw_remove(dev: *mut platform_device);
}
