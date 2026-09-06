//! Automatically rewritten from C Header to Rust Module
//! Source: drivers/usb/mtu3/mtu3_debug.h
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
// mtu3_debug.h - debug header
//
// Copyright (C) 2019 MediaTek Inc.
//
// Author: Chunfeng Yun <chunfeng.yun@mediatek.com>
//

pub const MTU3_DEBUGFS_NAME_LEN: c_int = 32;
#[repr(C)]
#[derive(Copy, Clone)]
pub struct mtu3_regset {
    pub name: [c_char; MTU3_DEBUGFS_NAME_LEN],
    pub regset: debugfs_regset32,
}

#[repr(C)]
#[derive(Copy, Clone)]
pub struct mtu3_file_map {
    pub name: *const c_char,
    pub unused): *mut *mut *mut int (show)(struct seq_file s, void,
}

extern "C" {
    pub fn ssusb_dev_debugfs_init(ssusb: *mut ssusb_mtk);
}
extern "C" {
    pub fn ssusb_dr_debugfs_init(ssusb: *mut ssusb_mtk);
}
extern "C" {
    pub fn ssusb_debugfs_create_root(ssusb: *mut ssusb_mtk);
}
extern "C" {
    pub fn ssusb_debugfs_remove_root(ssusb: *mut ssusb_mtk);
}

extern "C" {
    pub fn mtu3_dbg_trace(dev: *mut device, fmt: *const c_char, ...);
}

