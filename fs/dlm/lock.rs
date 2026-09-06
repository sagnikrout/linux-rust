//! Automatically rewritten from C Header to Rust Module
//! Source: fs/dlm/lock.h
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
// Copyright (C) 2005-2007 Red Hat, Inc.  All rights reserved.
//
extern "C" {
    pub fn dlm_dump_rsb(r: *mut dlm_rsb);
}
extern "C" {
    pub fn dlm_dump_rsb_name(ls: *mut dlm_ls, name: *const c_char, len: c_int);
}
extern "C" {
    pub fn dlm_print_lkb(lkb: *mut dlm_lkb);
}
extern "C" {
    pub fn dlm_receive_buffer(p: *const dlm_packet, nodeid: c_int);
}
extern "C" {
    pub fn dlm_modes_compat(mode1: c_int, mode2: c_int) -> c_int;
}
extern "C" {
    pub fn free_inactive_rsb(r: *mut dlm_rsb);
}
extern "C" {
    pub fn dlm_put_rsb(r: *mut dlm_rsb);
}
extern "C" {
    pub fn dlm_hold_rsb(r: *mut dlm_rsb);
}
extern "C" {
    pub fn dlm_put_lkb(lkb: *mut dlm_lkb) -> c_int;
}
extern "C" {
    pub fn dlm_lock_recovery_try(ls: *mut dlm_ls) -> c_int;
}
extern "C" {
    pub fn dlm_lock_recovery(ls: *mut dlm_ls);
}
extern "C" {
    pub fn dlm_unlock_recovery(ls: *mut dlm_ls);
}
extern "C" {
    pub fn dlm_rsb_scan(timer: *mut timer_list);
}
extern "C" {
    pub fn resume_scan_timer(ls: *mut dlm_ls);
}
extern "C" {
    pub fn dlm_recover_purge(ls: *mut dlm_ls, root_list: *const list_head);
}
extern "C" {
    pub fn dlm_purge_mstcpy_locks(r: *mut dlm_rsb);
}
extern "C" {
    pub fn dlm_recover_grant(ls: *mut dlm_ls);
}
extern "C" {
    pub fn dlm_recover_waiters_post(ls: *mut dlm_ls) -> c_int;
}
extern "C" {
    pub fn dlm_recover_waiters_pre(ls: *mut dlm_ls);
}
extern "C" {
    pub fn dlm_user_deadlock(ls: *mut dlm_ls, flags: u32, lkid: u32) -> c_int;
}
extern "C" {
    pub fn dlm_clear_proc_locks(ls: *mut dlm_ls, proc: *mut dlm_user_proc);
}
