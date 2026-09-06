//! Automatically rewritten from C Header to Rust Module
//! Source: fs/quota/compat.h
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

#[repr(C)]
#[derive(Copy, Clone)]
pub struct compat_if_dqblk {
    pub dqb_bhardlimit: compat_u64,
    pub dqb_bsoftlimit: compat_u64,
    pub dqb_curspace: compat_u64,
    pub dqb_ihardlimit: compat_u64,
    pub dqb_isoftlimit: compat_u64,
    pub dqb_curinodes: compat_u64,
    pub dqb_btime: compat_u64,
    pub dqb_itime: compat_u64,
    pub dqb_valid: compat_uint_t,
}

#[repr(C)]
#[derive(Copy, Clone)]
pub struct compat_fs_qfilestat {
    pub dqb_bhardlimit: compat_u64,
    pub qfs_nblks: compat_u64,
    pub qfs_nextents: compat_uint_t,
}

#[repr(C)]
#[derive(Copy, Clone)]
pub struct compat_fs_quota_stat {
    pub qs_version: __s8,
    pub qs_flags: __u16,
    pub qs_pad: __s8,
    pub qs_uquota: compat_fs_qfilestat,
    pub qs_gquota: compat_fs_qfilestat,
    pub qs_incoredqs: compat_uint_t,
    pub qs_btimelimit: compat_int_t,
    pub qs_itimelimit: compat_int_t,
    pub qs_rtbtimelimit: compat_int_t,
    pub qs_bwarnlimit: __u16,
    pub qs_iwarnlimit: __u16,
}
