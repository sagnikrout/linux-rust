//! Automatically rewritten from C Header to Rust Module
//! Source: include/linux/dev_printk.h
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
// dev_printk.h - printk messages helpers for devices
//
// Copyright (c) 2001-2003 Patrick Mochel <mochel@osdl.org>
// Copyright (c) 2004-2009 Greg Kroah-Hartman <gregkh@suse.de>
// Copyright (c) 2008-2009 Novell Inc.
//

pub const PRINTK_INFO_SUBSYSTEM_LEN: c_int = 16;
pub const PRINTK_INFO_DEVICE_LEN: c_int = 48;
#[repr(C)]
#[derive(Copy, Clone)]
pub struct dev_printk_info {
    pub subsystem: [c_char; PRINTK_INFO_SUBSYSTEM_LEN],
    pub device: [c_char; PRINTK_INFO_DEVICE_LEN],
}

extern "C" {
    pub fn dev_printk_emit(level: c_int, dev: *const device, fmt: *const c_char, ...) -> c_int;
}
extern "C" {
    pub fn _dev_emerg(dev: *const device, fmt: *const c_char, ...);
}
extern "C" {
    pub fn _dev_alert(dev: *const device, fmt: *const c_char, ...);
}
extern "C" {
    pub fn _dev_crit(dev: *const device, fmt: *const c_char, ...);
}
extern "C" {
    pub fn _dev_err(dev: *const device, fmt: *const c_char, ...);
}
extern "C" {
    pub fn _dev_warn(dev: *const device, fmt: *const c_char, ...);
}
extern "C" {
    pub fn _dev_notice(dev: *const device, fmt: *const c_char, ...);
}
extern "C" {
    pub fn _dev_info(dev: *const device, fmt: *const c_char, ...);
}

//
// Need to take variadic arguments even though we don't use them, as dev_fmt()
// may only just have been expanded and may result in multiple arguments.
//

//
// Some callsites directly call dev_printk rather than going through the
// dev_<level> infrastructure, so we need to emit here as well as inside those
// level-specific macros. Only one index entry will be produced, either way,
// since dev_printk's `fmt` isn't known at compile time if going through the
// dev_<level> macros.
//
// dev_fmt() isn't called for dev_printk when used directly, as it's used by
// the dev_<level> macros internally which already have dev_fmt() processed.
//
// We also can't use dev_printk_index_wrap directly, because we have a separate
// level to process.
//

//
// Dummy dev_printk for disabled debugging statements to use whilst maintaining
// gcc's format checking.
//

//
// #defines for all the dev_<level> macros to prefix with whatever
// possible use of #define dev_fmt(fmt) ...
//

// descriptor check is first to prevent flooding with "callbacks suppressed"

//
// dev_WARN*() acts like dev_printk(), but with the key difference of
// using WARN/WARN_ONCE to include file/line information and a backtrace.
//

// Simple helper for dev_err_probe() when ERR_PTR() is to be returned.

// Simple helper for dev_err_probe() when ERR_CAST() is to be returned.

