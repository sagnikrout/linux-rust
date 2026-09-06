//! Automatically rewritten from C Header to Rust Module
//! Source: tools/perf/util/namespaces.h
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
// Copyright (C) 2017 Hari Bathini, IBM Corporation
//

extern "C" {
    pub fn setns(fd: c_int, nstype: c_int) -> c_int;
}

#[repr(C)]
#[derive(Copy, Clone)]
pub struct namespaces {
    pub list: list_head,
    pub end_time: u64,
    pub link_info: [perf_ns_link_info; ],
}

extern "C" {
    pub fn namespaces__free(namespaces: *mut namespaces);
}
#[repr(C)]
#[derive(Copy, Clone)]
pub struct nscookie {
    pub oldns: c_int,
    pub newns: c_int,
    pub oldcwd: *mut c_char,
}

extern "C" {
    pub fn nsinfo__init(nsi: *mut nsinfo) -> c_int;
}
extern "C" {
    pub fn nsinfo__put(nsi: *mut nsinfo);
}
extern "C" {
    pub fn nsinfo__need_setns(nsi: *const nsinfo) -> bool;
}
extern "C" {
    pub fn nsinfo__clear_need_setns(nsi: *mut nsinfo);
}
extern "C" {
    pub fn nsinfo__tgid(nsi: *const nsinfo) -> pid_t;
}
extern "C" {
    pub fn nsinfo__nstgid(nsi: *const nsinfo) -> pid_t;
}
extern "C" {
    pub fn nsinfo__pid(nsi: *const nsinfo) -> pid_t;
}
extern "C" {
    pub fn nsinfo__in_pidns(nsi: *const nsinfo) -> bool;
}
extern "C" {
    pub fn nsinfo__set_in_pidns(nsi: *mut nsinfo);
}
extern "C" {
    pub fn nsinfo__mountns_enter(nsi: *mut nsinfo, nc: *mut nscookie);
}
extern "C" {
    pub fn nsinfo__mountns_exit(nc: *mut nscookie);
}
extern "C" {
    pub fn nsinfo__stat(filename: *const c_char, st: *mut stat, nsi: *mut nsinfo) -> c_int;
}
extern "C" {
    pub fn nsinfo__is_in_root_namespace() -> bool;
}
// nsip = NULL;

