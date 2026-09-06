//! Automatically rewritten from C Header to Rust Module
//! Source: include/linux/alloc_tag.h
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
// allocation tagging
//

#[repr(C)]
#[derive(Copy, Clone)]
pub struct alloc_tag_counters {
    pub bytes: u64,
    pub calls: u64,
}

//
// An instance of this structure is created in a special ELF section at every
// allocation callsite. At runtime, the special section is treated as
// an array of these. Embedded codetag utilizes codetag framework.
//
#[repr(C)]
#[derive(Copy, Clone)]
pub struct alloc_tag {
    pub ct: codetag,
    pub counters: *mut alloc_tag_counters __percpu,
    pub __aligned(8): },
#[repr(C)]
#[derive(Copy, Clone)]
pub struct alloc_tag_kernel_section {
    pub first_tag: *mut alloc_tag,
    pub count: c_ulong,
}

#[repr(C)]
#[derive(Copy, Clone)]
pub struct alloc_tag_module_section {
    pub start_addr: c_ulong,
    pub first_tag: *mut alloc_tag,
}

// used size

#[repr(C)]
#[derive(Copy, Clone)]
pub struct codetag_bytes {
    pub ct: *mut codetag,
    pub bytes: i64,
}

extern "C" {
    pub fn alloc_tag_top_users(tags: *mut codetag_bytes, count: usize, can_sleep: bool) -> usize;
}
extern "C" {
    pub fn container_of(_arg: ct, alloc_tag: struct, _arg: ct) -> return;
}

//
// When percpu variables are required to be defined as weak, static percpu
// variables can't be used inside a function (see comments for DECLARE_PER_CPU_SECTION).
// Instead we will account all module allocations to a single counter.
//

extern "C" {
    pub fn mem_alloc_profiling_permanently_disabled() -> bool;
}

extern "C" {
    pub fn alloc_tag_add_early_pfn(pfn: c_ulong, alloc_flags: c_uint);
}

// Caller should verify both ref and tag to be valid
//
// We need in increment the call counter every time we have a new
// allocation or when we split a large allocation into smaller ones.
// Each new reference for every sub-allocation needs to increment call
// counter because when we free each part the counter will be decremented.
//

// Macro flag: #define DEFINE_ALLOC_TAG(_alloc_tag)

