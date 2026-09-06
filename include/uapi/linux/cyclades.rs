//! Automatically rewritten from C Header to Rust Module
//! Source: include/uapi/linux/cyclades.h
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


// SPDX-License-Identifier: GPL-2.0 WITH Linux-syscall-note

#[repr(C)]
#[derive(Copy, Clone)]
pub struct cyclades_monitor {
    pub int_count: c_ulong,
    pub char_count: c_ulong,
    pub char_max: c_ulong,
    pub char_last: c_ulong,
}

pub const CYGETMON: c_uint = 0x435901;
pub const CYGETTHRESH: c_uint = 0x435902;
pub const CYSETTHRESH: c_uint = 0x435903;
pub const CYGETDEFTHRESH: c_uint = 0x435904;
pub const CYSETDEFTHRESH: c_uint = 0x435905;
pub const CYGETTIMEOUT: c_uint = 0x435906;
pub const CYSETTIMEOUT: c_uint = 0x435907;
pub const CYGETDEFTIMEOUT: c_uint = 0x435908;
pub const CYSETDEFTIMEOUT: c_uint = 0x435909;
pub const CYSETRFLOW: c_uint = 0x43590a;
pub const CYGETRFLOW: c_uint = 0x43590b;
pub const CYSETRTSDTR_INV: c_uint = 0x43590c;
pub const CYGETRTSDTR_INV: c_uint = 0x43590d;
pub const CYZSETPOLLCYCLE: c_uint = 0x43590e;
pub const CYZGETPOLLCYCLE: c_uint = 0x43590f;
pub const CYGETCD1400VER: c_uint = 0x435910;
pub const CYSETWAIT: c_uint = 0x435912;
pub const CYGETWAIT: c_uint = 0x435913;
