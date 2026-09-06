//! Automatically rewritten from C Header to Rust Module
//! Source: include/linux/kfence.h
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
// Kernel Electric-Fence (KFENCE). Public interface for allocator and fault
// handler integration. For more info see Documentation/dev-tools/kfence.rst.
//
// Copyright (C) 2020, Google LLC.
//

//
// We allocate an even number of pages, as it simplifies calculations to map
// address to metadata indices; effectively, the very first page serves as an
// extended guard page, but otherwise has no special purpose.
//

//
// is_kfence_address() - check if an address belongs to KFENCE pool
// @addr: address to check
//
// Return: true or false depending on whether the address is within the KFENCE
// object range.
//
// KFENCE objects live in a separate page range and are not to be intermixed
// with regular heap objects (e.g. KFENCE objects must never be added to the
// allocator freelists). Failing to do so may and will result in heap
// corruptions, therefore is_kfence_address() must be used to check whether
// an object requires specific handling.
//
// Note: This function may be used in fast-paths, and is performance critical.
// Future changes should take this into account; for instance, we want to avoid
// introducing another load and therefore need to keep KFENCE_POOL_SIZE a
// constant (until immediate patching support is added to the kernel).
//
// The __kfence_pool != NULL check is required to deal with the case
// where __kfence_pool == NULL && addr < KFENCE_POOL_SIZE. Keep it in
// the slow-path after the range-check!
//
extern "C" {
    pub fn unlikely(__kfence_pool: *mut *mut (unsigned long)((char )addr - __kfence_pool) < KFENCE_POOL_SIZE &&) -> return;
}
//
// kfence_alloc_pool_and_metadata() - allocate the KFENCE pool and KFENCE
// metadata via memblock
//
extern "C" {
    pub fn kfence_alloc_pool_and_metadata() -> void __init;
}
//
// kfence_init() - perform KFENCE initialization at boot time
//
// Requires that kfence_alloc_pool_and_metadata() was called before. This sets
// up the allocation gate timer, and requires that workqueues are available.
//
extern "C" {
    pub fn kfence_init() -> void __init;
}
//
// kfence_shutdown_cache() - handle shutdown_cache() for KFENCE objects
// @s: cache being shut down
//
// Before shutting down a cache, one must ensure there are no remaining objects
// allocated from it. Because KFENCE objects are not referenced from the cache
// directly, we need to check them here.
//
// Note that shutdown_cache() is internal to SL*B, and kmem_cache_destroy() does
// not return if allocated objects still exist: it prints an error message and
// simply aborts destruction of a cache, leaking memory.
//
// If the only such objects are KFENCE objects, we will not leak the entire
// cache, but instead try to provide more useful debug info by making allocated
// objects "zombie allocations". Objects may then still be used or freed (which
// is handled gracefully), but usage will result in showing KFENCE error reports
// which include stack traces to the user of the object, the original allocation
// site, and caller to shutdown_cache().
//
extern "C" {
    pub fn kfence_shutdown_cache(s: *mut kmem_cache);
}
//
// Allocate a KFENCE object. Allocators must not call this function directly,
// use kfence_alloc() instead.
//
// kfence_alloc() - allocate a KFENCE object with a low probability
// @s:     struct kmem_cache with object requirements
// @size:  exact size of the object to allocate (can be less than @s->size
// e.g. for kmalloc caches)
// @flags: GFP flags
//
// Return:
// * NULL     - must proceed with allocating as usual,
// * non-NULL - pointer to a KFENCE object.
//
// kfence_alloc() should be inserted into the heap allocation fast path,
// allowing it to transparently return KFENCE-allocated objects with a low
// probability using a static branch (the probability is controlled by the
// kfence.sample_interval boot parameter).
//

extern "C" {
    pub fn __kfence_alloc(_arg: s, _arg: size, _arg: flags) -> return;
}
//
// kfence_ksize() - get actual amount of memory allocated for a KFENCE object
// @addr: pointer to a heap object
//
// Return:
// * 0     - not a KFENCE object, must call __ksize() instead,
// * non-0 - this many bytes can be accessed without causing a memory error.
//
// kfence_ksize() returns the number of bytes requested for a KFENCE object at
// allocation time. This number may be less than the object size of the
// corresponding struct kmem_cache.
//
extern "C" {
    pub fn kfence_ksize(addr: *const c_void) -> usize;
}
//
// kfence_object_start() - find the beginning of a KFENCE object
// @addr: address within a KFENCE-allocated object
//
// Return: address of the beginning of the object.
//
// SL[AU]B-allocated objects are laid out within a page one by one, so it is
// easy to calculate the beginning of an object given a pointer inside it and
// the object size. The same is not true for KFENCE, which places a single
// object at either end of the page. This helper function is used to find the
// beginning of a KFENCE-allocated object.
//
// __kfence_free() - release a KFENCE heap object to KFENCE pool
// @addr: object to be freed
//
// Requires: is_kfence_address(addr)
//
// Release a KFENCE object and mark it as freed.
//
extern "C" {
    pub fn __kfence_free(addr: *mut c_void);
}
//
// kfence_free() - try to release an arbitrary heap object to KFENCE pool
// @addr: object to be freed
//
// Return:
// * false - object doesn't belong to KFENCE pool and was ignored,
// * true  - object was released to KFENCE pool.
//
// Release a KFENCE object and mark it as freed. May be called on any object,
// even non-KFENCE objects, to simplify integration of the hooks into the
// allocator's free codepath. The allocator must check the return value to
// determine if it was a KFENCE object or not.
//
// kfence_handle_page_fault() - perform page fault handling for KFENCE pages
// @addr: faulting address
// @is_write: is access a write
// @regs: current struct pt_regs (can be NULL, but shows full stack trace)
//
// Return:
// * false - address outside KFENCE pool,
// * true  - page fault handled by KFENCE, no additional handling required.
//
// A page fault inside KFENCE pool indicates a memory error, such as an
// out-of-bounds access, a use-after-free or an invalid memory access. In these
// cases KFENCE prints an error message and marks the offending page as
// present, so that the kernel can proceed.
//
extern "C" {
    pub fn kfence_handle_page_fault(addr: c_ulong, is_write: bool, regs: *mut pt_regs) -> bool __must_check;
}

//
// __kfence_obj_info() - fill kmem_obj_info struct
// @kpp: kmem_obj_info to be filled
// @object: the object
// @slab: the slab
//
// Return:
// * false - not a KFENCE object
// * true - a KFENCE object, filled @kpp
//
// Copies information to @kpp for KFENCE objects.
//
extern "C" {
    pub fn __kfence_obj_info(kpp: *mut kmem_obj_info, object: *mut c_void, slab: *mut slab) -> bool;
}

