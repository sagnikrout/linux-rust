//! Automatically rewritten from C Header to Rust Module
//! Source: include/uapi/linux/netfilter/xt_SECMARK.h
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

// Macro flag: #define _XT_SECMARK_H_target

//
// This is intended for use by various security subsystems (but not
// at the same time).
//
// 'mode' refers to the specific security subsystem which the
// packets are being marked for.
//
pub const SECMARK_MODE_SEL: c_uint = 0x01		/* SELinux */;
pub const SECMARK_SECCTX_MAX: c_int = 256;
#[repr(C)]
#[derive(Copy, Clone)]
pub struct xt_secmark_target_info {
    pub mode: __u8,
    pub secid: __u32,
    pub secctx: [c_char; SECMARK_SECCTX_MAX],
}

#[repr(C)]
#[derive(Copy, Clone)]
pub struct xt_secmark_target_info_v1 {
    pub mode: __u8,
    pub secctx: [c_char; SECMARK_SECCTX_MAX],
    pub secid: __u32,
}
