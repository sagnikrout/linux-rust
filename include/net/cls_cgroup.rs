//! Automatically rewritten from C Header to Rust Module
//! Source: include/net/cls_cgroup.h
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
// cls_cgroup.h			Control Group Classifier
//
// Authors:	Thomas Graf <tgraf@suug.ch>
//

#[repr(C)]
#[derive(Copy, Clone)]
pub struct cgroup_cls_state {
    pub css: cgroup_subsys_state,
    pub classid: u32,
}

// Due to the nature of the classifier it is required to ignore all
// packets originating from softirq context as accessing `current'
// would lead to false results.
//
// This test assumes that all callers of dev_queue_xmit() explicitly
// disable bh. Knowing this, it is possible to detect softirq based
// calls by looking at the number of nested bh disable calls because
// softirqs always disables bh.
//
// If there is an sock_cgroup_classid we'll use that.

