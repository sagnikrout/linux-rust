//! Automatically rewritten from C Header to Rust Module
//! Source: include/linux/mm_types_task.h
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
// Here are the definitions of the MM data types that are embedded in 'struct task_struct'.
//
// (These are defined separately to decouple sched.h from mm_types.h as much as possible.)
//

//
// When updating this, please also update struct resident_page_types[] in
// kernel/fork.c
//
#[repr(C)]
#[derive(Copy, Clone)]
pub struct page_frag {
    pub page: *mut page,

    pub offset: __u32,
    pub size: __u32,

    pub offset: __u16,
    pub size: __u16,

}

#[repr(C)]
#[derive(Copy, Clone)]
pub struct page_frag_cache {
// encoded_page consists of the virtual address, pfmemalloc bit and
// order of a page.
//
    pub encoded_page: c_ulong,
// we maintain a pagecount bias, so that we dont dirty cache line
// containing page->_refcount every time we allocate a fragment.
//

    pub offset: __u16,
    pub pagecnt_bias: __u16,

    pub offset: __u32,
    pub pagecnt_bias: __u32,

}

// Track pages that require TLB flushes
#[repr(C)]
#[derive(Copy, Clone)]
pub struct tlbflush_unmap_batch {

//
// The arch code makes the following promise: generic code can modify a
// PTE, then call arch_tlbbatch_add_pending() (which internally provides
// all needed barriers), then call arch_tlbbatch_flush(), and the entries
// will be flushed on all CPUs by the time that arch_tlbbatch_flush()
// returns.
//
    pub arch: arch_tlbflush_unmap_batch,
// True if a flush is needed.
    pub flush_required: bool,
//
// If true then the PTE was dirty when unmapped. The entry must be
// flushed before IO is initiated or a stale TLB entry potentially
// allows an update without redirtying the page.
//
    pub writable: bool,

}

#[repr(C)]
#[derive(Copy, Clone)]
pub struct lazy_mmu_state {
    pub enable_count: u8,
    pub pause_count: u8,
}
