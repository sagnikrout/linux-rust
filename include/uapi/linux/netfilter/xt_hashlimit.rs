//! Automatically rewritten from C Header to Rust Module
//! Source: include/uapi/linux/netfilter/xt_hashlimit.h
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

// timings are in milliseconds.
pub const XT_HASHLIMIT_SCALE: c_int = 10000;

// 1/10,000 sec period => max of 10,000/sec.  Min rate is then 429490
// seconds, or one packet every 59 hours.
//
// packet length accounting is done in 16-byte steps
pub const XT_HASHLIMIT_BYTE_SHIFT: c_int = 4;
// details of this structure hidden by the implementation
#[repr(C)]
#[derive(Copy, Clone)]
pub struct hashlimit_cfg {
    pub /: *mut *mut *mut __u32 mode; / bitmask of XT_HASHLIMIT_HASH_,
    pub /: *mut *mut *mut __u32 avg; / Average secs between packets  scale,
    pub /: *mut *mut __u32 burst; / Period multiplier for upper limit.,
// user specified
    pub /: *mut *mut __u32 size; / how many buckets,
    pub /: *mut *mut __u32 max; / max number of entries,
    pub /: *mut *mut __u32 gc_interval; / gc interval,
    pub /: *mut *mut __u32 expire; / when do entries expire?,
}

#[repr(C)]
#[derive(Copy, Clone)]
pub struct xt_hashlimit_info {
    pub /: *mut *mut char name [IFNAMSIZ]; / name,
    pub cfg: hashlimit_cfg,
// Used internally by the kernel
    pub hinfo: *mut xt_hashlimit_htable,
    pub ptr: *mut c_void,
    pub master: *mut xt_hashlimit_info,
    pub u: },
}

#[repr(C)]
#[derive(Copy, Clone)]
pub struct hashlimit_cfg1 {
    pub /: *mut *mut *mut __u32 mode; / bitmask of XT_HASHLIMIT_HASH_,
    pub /: *mut *mut *mut __u32 avg; / Average secs between packets  scale,
    pub /: *mut *mut __u32 burst; / Period multiplier for upper limit.,
// user specified
    pub /: *mut *mut __u32 size; / how many buckets,
    pub /: *mut *mut __u32 max; / max number of entries,
    pub /: *mut *mut __u32 gc_interval; / gc interval,
    pub /: *mut *mut __u32 expire; / when do entries expire?,
    pub dstmask: __u8 srcmask,,
}

#[repr(C)]
#[derive(Copy, Clone)]
pub struct hashlimit_cfg2 {
    pub /: *mut *mut *mut __u64 avg; / Average secs between packets  scale,
    pub /: *mut *mut __u64 burst; / Period multiplier for upper limit.,
    pub /: *mut *mut *mut __u32 mode; / bitmask of XT_HASHLIMIT_HASH_,
// user specified
    pub /: *mut *mut __u32 size; / how many buckets,
    pub /: *mut *mut __u32 max; / max number of entries,
    pub /: *mut *mut __u32 gc_interval; / gc interval,
    pub /: *mut *mut __u32 expire; / when do entries expire?,
    pub dstmask: __u8 srcmask,,
}

#[repr(C)]
#[derive(Copy, Clone)]
pub struct hashlimit_cfg3 {
    pub /: *mut *mut *mut __u64 avg; / Average secs between packets  scale,
    pub /: *mut *mut __u64 burst; / Period multiplier for upper limit.,
    pub /: *mut *mut *mut __u32 mode; / bitmask of XT_HASHLIMIT_HASH_,
// user specified
    pub /: *mut *mut __u32 size; / how many buckets,
    pub /: *mut *mut __u32 max; / max number of entries,
    pub /: *mut *mut __u32 gc_interval; / gc interval,
    pub /: *mut *mut __u32 expire; / when do entries expire?,
    pub interval: __u32,
    pub dstmask: __u8 srcmask,,
}

#[repr(C)]
#[derive(Copy, Clone)]
pub struct xt_hashlimit_mtinfo1 {
    pub name: [c_char; IFNAMSIZ],
    pub cfg: hashlimit_cfg1,
// Used internally by the kernel
    pub __attribute__((aligned(8))): *mut *mut xt_hashlimit_htable hinfo,
}

#[repr(C)]
#[derive(Copy, Clone)]
pub struct xt_hashlimit_mtinfo2 {
    pub name: [c_char; NAME_MAX],
    pub cfg: hashlimit_cfg2,
// Used internally by the kernel
    pub __attribute__((aligned(8))): *mut *mut xt_hashlimit_htable hinfo,
}

#[repr(C)]
#[derive(Copy, Clone)]
pub struct xt_hashlimit_mtinfo3 {
    pub name: [c_char; NAME_MAX],
    pub cfg: hashlimit_cfg3,
// Used internally by the kernel
    pub __attribute__((aligned(8))): *mut *mut xt_hashlimit_htable hinfo,
}
