//! Automatically rewritten from C Header to Rust Module
//! Source: arch/powerpc/include/asm/rtas-work-area.h
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
// struct rtas_work_area - RTAS work area descriptor.
//
// Descriptor for a "work area" in PAPR terminology that satisfies
// RTAS addressing requirements.
//
#[repr(C)]
#[derive(Copy, Clone)]
pub struct rtas_work_area {
// private: Use the APIs provided below.
    pub buf: *mut c_char,
    pub size: usize,
}

// Maximum allocation size, enforced at build time.
//
// rtas_work_area_alloc() - Acquire a work area of the requested size.
// @size_: Allocation size. Must be compile-time constant and not more
// than %RTAS_WORK_AREA_MAX_ALLOC_SZ.
//
// Allocate a buffer suitable for passing to RTAS functions that have
// a memory address parameter, often (but not always) referred to as a
// "work area" in PAPR. Although callers are allowed to block while
// holding a work area, the amount of memory reserved for this purpose
// is limited, and allocations should be short-lived. A good guideline
// is to release any allocated work area before returning from a
// system call.
//
// This function does not fail. It blocks until the allocation
// succeeds. To prevent deadlocks, callers are discouraged from
// allocating more than one work area simultaneously in a single task
// context.
//
// Context: This function may sleep.
// Return: A &struct rtas_work_area descriptor for the allocated work area.
//

//
// Do not call __rtas_work_area_alloc() directly. Use
// rtas_work_area_alloc().
//
// rtas_work_area_free() - Release a work area.
// @area: Work area descriptor as returned from rtas_work_area_alloc().
//
// Return a work area buffer to the pool.
//
extern "C" {
    pub fn rtas_work_area_free(area: *mut rtas_work_area);
}
extern "C" {
    pub fn __pa(_arg: area->buf) -> return;
}
//
// Early setup for the work area allocator. Call from
// rtas_initialize() only.
//

extern "C" {
    pub fn rtas_work_area_reserve_arena(limit: phys_addr_t);
}

