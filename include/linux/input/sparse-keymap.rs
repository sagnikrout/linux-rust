//! Automatically rewritten from C Header to Rust Module
//! Source: include/linux/input/sparse-keymap.h
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
// Copyright (c) 2009 Dmitry Torokhov
//

//
// struct key_entry - keymap entry for use in sparse keymap
// @type: Type of the key entry (KE_KEY, KE_SW, KE_VSW, KE_END);
// drivers are allowed to extend the list with their own
// private definitions.
// @code: Device-specific data identifying the button/switch
// @keycode: KEY_* code assigned to a key/button
// @sw: struct with code/value used by KE_SW and KE_VSW
// @sw.code: SW_* code assigned to a switch
// @sw.value: Value that should be sent in an input even when KE_SW
// switch is toggled. KE_VSW switches ignore this field and
// expect driver to supply value for the event.
//
// This structure defines an entry in a sparse keymap used by some
// input devices for which traditional table-based approach is not
// suitable.
//
#[repr(C)]
#[derive(Copy, Clone)]
pub struct key_entry {
    pub /: *mut *mut *mut int type; / See KE_ above,
    pub code: u32,
    pub /: *mut *mut u16 keycode; / For KE_KEY,
    pub code: u8,
    pub /: *mut *mut u8 value; / For KE_SW, ignored by KE_VSW,
    pub sw: },
}
