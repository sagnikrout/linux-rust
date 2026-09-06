//! Automatically rewritten from C Header to Rust Module
//! Source: include/linux/cpu_rmap.h
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
// cpu_rmap.c: CPU affinity reverse-map support
// Copyright 2011 Solarflare Communications Inc.
//

//
// struct cpu_rmap - CPU affinity reverse-map
// @refcount: kref for object
// @size: Number of objects to be reverse-mapped
// @obj: Pointer to array of object pointers
// @near: For each CPU, the index and distance to the nearest object,
// based on affinity masks
//
#[repr(C)]
#[derive(Copy, Clone)]
pub struct cpu_rmap {
    pub refcount: kref,
    pub size: u16,
    pub obj: *mut c_void,
    pub index: u16,
    pub dist: u16,
    pub near: [}; ],
}

pub const CPU_RMAP_DIST_INF: c_uint = 0xffff;
extern "C" {
    pub fn cpu_rmap_get(rmap: *mut cpu_rmap);
}
extern "C" {
    pub fn cpu_rmap_put(rmap: *mut cpu_rmap) -> c_int;
}
extern "C" {
    pub fn cpu_rmap_add(rmap: *mut cpu_rmap, obj: *mut c_void) -> c_int;
}
//
// alloc_irq_cpu_rmap - allocate CPU affinity reverse-map for IRQs
// @size: Number of objects to be mapped
//
// Must be called in process context.
//
extern "C" {
    pub fn alloc_cpu_rmap(_arg: size, _arg: GFP_KERNEL) -> return;
}
extern "C" {
    pub fn free_irq_cpu_rmap(rmap: *mut cpu_rmap);
}
extern "C" {
    pub fn irq_cpu_rmap_remove(rmap: *mut cpu_rmap, irq: c_int) -> c_int;
}
extern "C" {
    pub fn irq_cpu_rmap_add(rmap: *mut cpu_rmap, irq: c_int) -> c_int;
}
