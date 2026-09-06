//! Automatically rewritten from C Header to Rust Module
//! Source: security/selinux/ss/avtab.h
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
// An access vector table (avtab) is a hash table
// of access vectors and transition types indexed
// by a type pair and a class.  An access vector
// table is used to represent the type enforcement
// tables.
//
// Author : Stephen Smalley, <stephen.smalley.work@gmail.com>
//
// Updated: Frank Mayer <mayerf@tresys.com> and
// Karl MacMillan <kmacmillan@tresys.com>
// Added conditional policy language extensions
// Copyright (C) 2003 Tresys Technology, LLC
//
// Updated: Yuichi Nakamura <ynakam@hitachisoft.jp>
// Tuned number of hash slots for avtab to reduce memory usage
//

#[repr(C)]
#[derive(Copy, Clone)]
pub struct avtab_key {
    pub /: *mut *mut u16 source_type; / source type,
    pub /: *mut *mut u16 target_type; / target type,
    pub /: *mut *mut u16 target_class; / target object class,
pub const AVTAB_ALLOWED: c_uint = 0x0001;
pub const AVTAB_AUDITALLOW: c_uint = 0x0002;
pub const AVTAB_AUDITDENY: c_uint = 0x0004;

pub const AVTAB_TRANSITION: c_uint = 0x0010;
pub const AVTAB_MEMBER: c_uint = 0x0020;
pub const AVTAB_CHANGE: c_uint = 0x0040;

// extended permissions
pub const AVTAB_XPERMS_ALLOWED: c_uint = 0x0100;
pub const AVTAB_XPERMS_AUDITALLOW: c_uint = 0x0200;
pub const AVTAB_XPERMS_DONTAUDIT: c_uint = 0x0400;

pub const AVTAB_ENABLED_OLD: c_uint = 0x80000000 /* reserved for used in cond_avtab */;
pub const AVTAB_ENABLED: c_uint = 0x8000 /* reserved for used in cond_avtab */;

    pub /: *mut *mut u16 specified; / what field is specified,
}

//
// For operations that require more than the 32 permissions provided by the avc
// extended permissions may be used to provide 256 bits of permissions.
//
#[repr(C)]
#[derive(Copy, Clone)]
pub struct avtab_extended_perms {
// These are not flags. All 256 values may be used
pub const AVTAB_XPERMS_IOCTLFUNCTION: c_uint = 0x01;
pub const AVTAB_XPERMS_IOCTLDRIVER: c_uint = 0x02;
pub const AVTAB_XPERMS_NLMSG: c_uint = 0x03;
// extension of the avtab_key specified
    pub /: *mut *mut u8 specified; / ioctl, netfilter, ...,
//
// if 256 bits is not adequate as is often the case with ioctls, then
// multiple extended perms may be used and the driver field
// specifies which permissions are included.
//
    pub driver: u8,
// 256 bits of permissions
    pub perms: extended_perms_data,
}

#[repr(C)]
#[derive(Copy, Clone)]
pub struct avtab_datum {
    pub /: *mut *mut u32 data; / access vector or type value,
    pub xperms: *mut avtab_extended_perms,
    pub u: },
}

#[repr(C)]
#[derive(Copy, Clone)]
pub struct avtab_node {
    pub key: avtab_key,
    pub datum: avtab_datum,
    pub next: *mut avtab_node,
}

#[repr(C)]
#[derive(Copy, Clone)]
pub struct avtab {
    pub htable: *mut avtab_node,
    pub /: *mut *mut u32 nel; / number of elements,
    pub /: *mut *mut u32 nslot; / number of hash slots,
    pub /: *mut *mut u32 mask; / mask to compute hash func,
}

extern "C" {
    pub fn avtab_init(h: *mut avtab);
}
extern "C" {
    pub fn avtab_alloc(h: *mut avtab, nrules: u32) -> c_int;
}
extern "C" {
    pub fn avtab_alloc_dup(new: *mut avtab, orig: *const avtab) -> c_int;
}
extern "C" {
    pub fn avtab_destroy(h: *mut avtab);
}
pub const MAX_AVTAB_HASH_BITS: c_int = 16;

extern "C" {
    pub fn avtab_hash_eval(h: *mut avtab, tag: *const c_char);
}

extern "C" {
    pub fn avtab_read(a: *mut avtab, fp: *mut policy_file, pol: *mut policydb) -> c_int;
}
extern "C" {
    pub fn avtab_write(p: *mut policydb, a: *mut avtab, fp: *mut policy_file) -> c_int;
}
