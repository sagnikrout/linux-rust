//! Automatically rewritten from C Header to Rust Module
//! Source: tools/sched_ext/include/scx/user_exit_info.h
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
// Define struct user_exit_info which is shared between BPF and userspace parts
// to communicate exit status and other information.
//
// Copyright (c) 2022 Meta Platforms, Inc. and affiliates.
// Copyright (c) 2022 Tejun Heo <tj@kernel.org>
// Copyright (c) 2022 David Vernet <dvernet@meta.com>
//

// no need to call the following explicitly if SCX_OPS_LOAD() is used

// use __sync to force memory barrier */				\

//
// We can't import vmlinux.h while compiling user C code. Let's duplicate
// scx_exit_code definition.
//
#[repr(C)]
#[derive(Copy, Clone, PartialEq, Eq)]
pub enum scx_exit_code {
// Reasons
    SCX_ECODE_RSN_HOTPLUG		= 1LLU << 32,

// Actions
    SCX_ECODE_ACT_RESTART		= 1LLU << 48,
}

#[repr(C)]
#[derive(Copy, Clone, PartialEq, Eq)]
pub enum uei_ecode_mask {
    UEI_ECODE_USER_MASK		= ((1LLU << 32) - 1),
    UEI_ECODE_SYS_RSN_MASK		= ((1LLU << 16) - 1) << 32,
    UEI_ECODE_SYS_ACT_MASK		= ((1LLU << 16) - 1) << 48,
}

//
// These macro interpret the ecode returned from UEI_REPORT().
//

