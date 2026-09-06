//! Automatically rewritten from C Header to Rust Module
//! Source: tools/virtio/ringtest/main.h
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
// Copyright (C) 2016 Red Hat, Inc.
// Author: Michael S. Tsirkin <mst@redhat.com>
//
// Common macros and functions for ring benchmarking.
//

pub const VMEXIT_CYCLES: c_int = 500;
pub const VMENTRY_CYCLES: c_int = 500;

extern "C" {
    pub fn volatile(%0: "0: brctg, (cycles): 0b" : : "d") -> asm;
}
// tweak me
pub const VMEXIT_CYCLES: c_int = 200;
pub const VMENTRY_CYCLES: c_int = 200;

pub const VMEXIT_CYCLES: c_int = 0;
pub const VMENTRY_CYCLES: c_int = 0;

// implemented by ring
extern "C" {
    pub fn alloc_ring();
}
// guest side
extern "C" {
    pub fn add_inbuf(_arg: unsigned, : *mut c_void, : *mut c_void) -> c_int;
}
extern "C" {
    pub fn disable_call();
}
extern "C" {
    pub fn used_empty() -> bool;
}
extern "C" {
    pub fn enable_call() -> bool;
}
extern "C" {
    pub fn kick_available();
}
// host side
extern "C" {
    pub fn disable_kick();
}
extern "C" {
    pub fn avail_empty() -> bool;
}
extern "C" {
    pub fn enable_kick() -> bool;
}
extern "C" {
    pub fn use_buf(: *mut unsigned, : *mut c_void) -> bool;
}
extern "C" {
    pub fn call_used();
}
// implemented by main
extern "C" {
    pub fn kick();
}
extern "C" {
    pub fn wait_for_kick();
}
extern "C" {
    pub fn call();
}
extern "C" {
    pub fn wait_for_call();
}
// Compiler barrier - similar to what Linux uses

// Is there a portable way to do this?

// prevent compiler from removing busy loops

//
// Not using __ATOMIC_SEQ_CST since gcc docs say they are only synchronized
// with other __ATOMIC_SEQ_CST calls.
//

//
// This abuses the atomic builtins for thread fences, and
// adds a compiler barrier.
//

