//! Automatically rewritten from C Header to Rust Module
//! Source: kernel/bpf/percpu_freelist.h
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
// Copyright (c) 2016 Facebook
//

#[repr(C)]
#[derive(Copy, Clone)]
pub struct pcpu_freelist_head {
    pub first: *mut pcpu_freelist_node,
    pub lock: rqspinlock_t,
}

#[repr(C)]
#[derive(Copy, Clone)]
pub struct pcpu_freelist {
    pub freelist: *mut pcpu_freelist_head __percpu,
}

#[repr(C)]
#[derive(Copy, Clone)]
pub struct pcpu_freelist_node {
    pub next: *mut pcpu_freelist_node,
}

// pcpu_freelist_* do spin_lock_irqsave.
extern "C" {
    pub fn pcpu_freelist_push(: *mut pcpu_freelist, : *mut pcpu_freelist_node);
}
// __pcpu_freelist_* do spin_lock only. caller must disable irqs.
extern "C" {
    pub fn __pcpu_freelist_push(: *mut pcpu_freelist, : *mut pcpu_freelist_node);
}
extern "C" {
    pub fn pcpu_freelist_init(: *mut pcpu_freelist) -> c_int;
}
extern "C" {
    pub fn pcpu_freelist_destroy(s: *mut pcpu_freelist);
}
