//! Automatically rewritten from C Header to Rust Module
//! Source: arch/riscv/include/asm/atomic.h
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
// Copyright (C) 2007 Red Hat, Inc. All Rights Reserved.
// Copyright (C) 2012 Regents of the University of California
// Copyright (C) 2017 SiFive
//

extern "C" {
    pub fn __volatile__("memory": RISCV_RELEASE_BARRIER "" :::) -> __asm__;
}
extern "C" {
    pub fn READ_ONCE(_arg: v->counter) -> return;
}

extern "C" {
    pub fn READ_ONCE(_arg: v->counter) -> return;
}

//
// First, the atomic ops that have no ordering constraints and therefore don't
// have the AQ or RL bits set.  These don't return anything, so there's only
// one version to worry about.
//

//
// Atomic ops that have ordered, relaxed, acquire, and release variants.
// There's two flavors of these: the arithmetic ops have both fetch and return
// versions, while the logical ops only have fetch versions.
//

// This is required to provide a full barrier on success.

