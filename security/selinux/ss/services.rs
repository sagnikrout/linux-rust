//! Automatically rewritten from C Header to Rust Module
//! Source: security/selinux/ss/services.h
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
// Implementation of the security services.
//
// Author : Stephen Smalley, <stephen.smalley.work@gmail.com>
//

// Mapping for a single class
#[repr(C)]
#[derive(Copy, Clone)]
pub struct selinux_mapping {
    pub /: *mut *mut u16 value; / policy value for class,
    pub /: *mut *mut u16 num_perms; / number of permissions in class,
    pub /: *mut *mut *mut u32 perms[sizeof(u32)  8]; / policy values for permissions,
}

// Map for all of the classes, with array size
#[repr(C)]
#[derive(Copy, Clone)]
pub struct selinux_map {
    pub /: *mut *mut *mut selinux_mapping mapping; / indexed by class,
    pub /: *mut *mut u16 size; / array size of mapping,
}

#[repr(C)]
#[derive(Copy, Clone)]
pub struct selinux_policy {
    pub sidtab: *mut sidtab,
    pub policydb: policydb,
    pub map: selinux_map,
    pub latest_granting: u32,
    pub __randomize_layout: },
#[repr(C)]
#[derive(Copy, Clone)]
pub struct convert_context_args {
    pub oldp: *mut policydb,
    pub newp: *mut policydb,
}
