//! Automatically rewritten from C Header to Rust Module
//! Source: include/linux/uts_namespace.h
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
pub struct uts_namespace {
    pub name: new_utsname,
    pub user_ns: *mut user_namespace,
    pub ucounts: *mut ucounts,
    pub ns: ns_common,
    pub __randomize_layout: },
    pub init_uts_ns: extern struct uts_namespace,

    pub ns): return container_of(ns, struct uts_namespace,,
    pub old_ns): *mut *mut user_namespace user_ns, uts_namespace,
    pub ns): *mut extern void free_uts_ns(struct uts_namespace,
    pub uts_ns_init(void): c_void,

    pub ERR_PTR(-EINVAL): return,
    pub old_ns: return,

