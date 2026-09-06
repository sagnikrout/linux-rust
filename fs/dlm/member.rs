//! Automatically rewritten from C Header to Rust Module
//! Source: fs/dlm/member.h
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
// Copyright (C) 2005-2011 Red Hat, Inc.  All rights reserved.
//
extern "C" {
    pub fn dlm_ls_stop(ls: *mut dlm_ls) -> c_int;
}
extern "C" {
    pub fn dlm_ls_start(ls: *mut dlm_ls) -> c_int;
}
extern "C" {
    pub fn dlm_clear_members(ls: *mut dlm_ls);
}
extern "C" {
    pub fn dlm_clear_members_gone(ls: *mut dlm_ls);
}
extern "C" {
    pub fn dlm_recover_members(ls: *mut dlm_ls, rv: *mut dlm_recover, neg_out: *mut c_int) -> c_int;
}
extern "C" {
    pub fn dlm_is_removed(ls: *mut dlm_ls, nodeid: c_int) -> c_int;
}
extern "C" {
    pub fn dlm_is_member(ls: *mut dlm_ls, nodeid: c_int) -> c_int;
}
extern "C" {
    pub fn dlm_slots_version(h: *const dlm_header) -> c_int;
}
extern "C" {
    pub fn dlm_slots_copy_out(ls: *mut dlm_ls, rc: *mut dlm_rcom);
}
extern "C" {
    pub fn dlm_slots_copy_in(ls: *mut dlm_ls) -> c_int;
}
extern "C" {
    pub fn dlm_lsop_recover_done(ls: *mut dlm_ls);
}
