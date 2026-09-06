//! Automatically rewritten from C Header to Rust Module
//! Source: io_uring/notif.h
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

pub const IO_NOTIF_SPLICE_BATCH: c_int = 32;
#[repr(C)]
#[derive(Copy, Clone)]
pub struct io_notif_data {
    pub file: *mut file,
    pub uarg: ubuf_info,
    pub next: *mut io_notif_data,
    pub head: *mut io_notif_data,
    pub account_pages: unsigned,
    pub zc_report: bool,
    pub zc_used: bool,
    pub zc_copied: bool,
}

extern "C" {
    pub fn io_kiocb_to_cmd(_arg: notif, io_notif_data: struct) -> return;
}
