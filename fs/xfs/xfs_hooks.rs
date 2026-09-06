//! Automatically rewritten from C Header to Rust Module
//! Source: fs/xfs/xfs_hooks.h
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


// SPDX-License-Identifier: GPL-2.0-or-later
//
// Copyright (c) 2022-2024 Oracle.  All Rights Reserved.
// Author: Darrick J. Wong <djwong@kernel.org>
//

#[repr(C)]
#[derive(Copy, Clone)]
pub struct xfs_hooks {
    pub head: blocking_notifier_head,
}

//
// If jump labels are enabled in Kconfig, the static key uses nop sleds and
// code patching to eliminate the overhead of taking the rwsem in
// blocking_notifier_call_chain when there are no hooks configured.  If not,
// the static key per-call overhead is an atomic read.  Most arches that can
// handle XFS also support jump labels.
//
// Note: Patching the kernel code requires taking the cpu hotplug lock.  Other
// parts of the kernel allocate memory with that lock held, which means that
// XFS callers cannot hold any locks that might be used by memory reclaim or
// writeback when calling the static_branch_{inc,dec} functions.
//

#[repr(C)]
#[derive(Copy, Clone)]
pub struct xfs_hook {
// This must come at the start of the structure.
    pub nb: notifier_block,
}

extern "C" {
    pub fn xfs_hooks_init(chain: *mut xfs_hooks);
}
extern "C" {
    pub fn xfs_hooks_add(chain: *mut xfs_hooks, hook: *mut xfs_hook) -> c_int;
}
extern "C" {
    pub fn xfs_hooks_del(chain: *mut xfs_hooks, hook: *mut xfs_hook);
}

#[repr(C)]
#[derive(Copy, Clone)]
pub struct xfs_hooks {

