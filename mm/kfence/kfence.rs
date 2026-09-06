//! Automatically rewritten from C Header to Rust Module
//! Source: mm/kfence/kfence.h
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
// Kernel Electric-Fence (KFENCE). For more info please see
// Documentation/dev-tools/kfence.rst.
//
// Copyright (C) 2020, Google LLC.
//

//
// Get the canary byte pattern for @addr. Use a pattern that varies based on the
// lower 3 bits of the address, to detect memory corruptions with higher
// probability, where similar constants are used.
//

//
// Define a continuous 8-byte canary starting from a multiple of 8. The canary
// of each byte is only related to the lowest three bits of its address, so the
// canary of every 8 bytes is the same. 64-bit memory can be filled and checked
// at a time instead of byte by byte to improve performance.
//

// Maximum stack depth for reports.
pub const KFENCE_STACK_DEPTH: c_int = 64;
// KFENCE object states.
#[repr(C)]
#[derive(Copy, Clone, PartialEq, Eq)]
pub enum kfence_object_state {
    KFENCE_OBJECT_UNUSED,		/* Object is unused. */
    KFENCE_OBJECT_ALLOCATED,	/* Object is currently allocated. */
    KFENCE_OBJECT_RCU_FREEING,	/* Object was allocated, and then being freed by rcu. */
    KFENCE_OBJECT_FREED,		/* Object was allocated, and then freed. */
}

// Alloc/free tracking information.
#[repr(C)]
#[derive(Copy, Clone)]
pub struct kfence_track {
    pub pid: pid_t,
    pub cpu: c_int,
    pub ts_nsec: u64,
    pub num_stack_entries: c_int,
    pub stack_entries: [c_ulong; KFENCE_STACK_DEPTH],
}

// KFENCE metadata per guarded allocation.
#[repr(C)]
#[derive(Copy, Clone)]
pub struct kfence_metadata {
    pub /: *mut *mut list_head list __guarded_by(&kfence_freelist_lock); / Freelist node.,
    pub /: *mut *mut rcu_head rcu_head; / For delayed freeing.,
//
// Lock protecting below data; to ensure consistency of the below data,
// since the following may execute concurrently: __kfence_alloc(),
// __kfence_free(), kfence_handle_page_fault(). However, note that we
// cannot grab the same metadata off the freelist twice, and multiple
// __kfence_alloc() cannot run concurrently on the same metadata.
//
    pub lock: raw_spinlock_t,
// The current state of the object; see above.
    pub state: kfence_object_state,
//
// Allocated object address; cannot be calculated from size, because of
// alignment requirements.
//
// Invariant: ALIGN_DOWN(addr, PAGE_SIZE) is constant.
//
    pub addr: c_ulong,
//
// The size of the original allocation.
//
    pub size: usize,
//
// The kmem_cache cache of the last allocation; NULL if never allocated
// or the cache has already been destroyed.
//
    pub cache: *mut kmem_cache,
//
// In case of an invalid access, the page that was unprotected; we
// optimistically only store one address.
//
    pub __guarded_by(&lock): unsigned long unprotected_page,
// Allocation and free stack information.
    pub __guarded_by(&lock): kfence_track alloc_track,
    pub __guarded_by(&lock): kfence_track free_track,
// For updating alloc_covered on frees.
    pub __guarded_by(&lock): u32 alloc_stack_hash,
}

// The checks do not affect performance; only called from slow-paths.
//
// May be an invalid index if called with an address at the edge of
// __kfence_pool, in which case we would report an "invalid access"
// error.
//
// KFENCE error types for report generation.
#[repr(C)]
#[derive(Copy, Clone, PartialEq, Eq)]
pub enum kfence_error_type {
    KFENCE_ERROR_OOB,		/* Detected a out-of-bounds access. */
    KFENCE_ERROR_UAF,		/* Detected a use-after-free access. */
    KFENCE_ERROR_CORRUPTION,	/* Detected a memory corruption on free. */
    KFENCE_ERROR_INVALID,		/* Invalid access of unknown type. */
    KFENCE_ERROR_INVALID_FREE,	/* Invalid free. */
}

#[repr(C)]
#[derive(Copy, Clone, PartialEq, Eq)]
pub enum kfence_fault {
    KFENCE_FAULT_NONE,
    KFENCE_FAULT_REPORT,
    KFENCE_FAULT_OOPS,
    KFENCE_FAULT_PANIC,
}

extern "C" {
    pub fn kfence_handle_fault(fault: kfence_fault);
}
extern "C" {
    pub fn kfence_print_object(seq: *mut seq_file, __must_hold(&meta->lock: *const *const kfence_metadata meta));
}
