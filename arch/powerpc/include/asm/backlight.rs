//! Automatically rewritten from C Header to Rust Module
//! Source: arch/powerpc/include/asm/backlight.h
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
//
// Routines for handling backlight control on PowerBooks
//
// For now, implementation resides in
// arch/powerpc/platforms/powermac/backlight.c
//

// For locking instructions, see the implementation file
extern "C" {
    pub fn pmac_has_backlight_type(type: *const c_char) -> c_int;
}
extern "C" {
    pub fn pmac_backlight_key(direction: c_int);
}
extern "C" {
    pub fn pmac_backlight_set_legacy_brightness_pmu(brightness: c_int);
}
extern "C" {
    pub fn pmac_backlight_set_legacy_brightness(brightness: c_int) -> c_int;
}
extern "C" {
    pub fn pmac_backlight_get_legacy_brightness() -> c_int;
}
extern "C" {
    pub fn pmac_backlight_enable();
}
extern "C" {
    pub fn pmac_backlight_disable();
}

