//! Automatically rewritten from C Header to Rust Module
//! Source: drivers/i3c/master/mipi-i3c-hci/dat.h
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


// SPDX-License-Identifier: BSD-3-Clause
//
// Copyright (c) 2020, MIPI Alliance, Inc.
//
// Author: Nicolas Pitre <npitre@baylibre.com>
//
// Common DAT related stuff
//
// Global DAT flags

#[repr(C)]
#[derive(Copy, Clone)]
pub struct hci_dat_ops {
    pub hci): *mut *mut int (init)(struct i3c_hci,
    pub hci): *mut *mut int (alloc_entry)(struct i3c_hci,
    pub dat_idx): *mut *mut *mut void (free_entry)(struct i3c_hci hci, unsigned int,
    pub addr): *mut *mut *mut void (set_dynamic_addr)(struct i3c_hci hci, unsigned int dat_idx, u8,
    pub addr): *mut *mut *mut void (set_static_addr)(struct i3c_hci hci, unsigned int dat_idx, u8,
    pub w1): *mut *mut *mut void (set_flags)(struct i3c_hci hci, unsigned int dat_idx, u32 w0, u32,
    pub w1): *mut *mut *mut void (clear_flags)(struct i3c_hci hci, unsigned int dat_idx, u32 w0, u32,
    pub address): *mut *mut *mut int (get_index)(struct i3c_hci hci, u8,
    pub hci): *mut *mut void (restore)(struct i3c_hci,
}
