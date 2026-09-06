//! Automatically rewritten from C Header to Rust Module
//! Source: include/linux/genalloc.h
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
// Basic general purpose allocator for managing special purpose
// memory, for example, memory that is not managed by the regular
// kmalloc/kfree interface.  Uses for this includes on-device special
// memory, uncached memory etc.
//
// It is safe to use the allocator in NMI handlers and other special
// unblockable contexts that could otherwise deadlock on locks.  This
// is implemented by using atomic operations and retries on any
// conflicts.  The disadvantage is that there may be livelocks in
// extreme cases.  For better scalability, one allocator can be used
// for each CPU.
//
// The lockless operation only works if there is enough memory
// available.  If new memory is added to the pool a lock has to be
// still taken.  So any user relying on locklessness has to ensure
// that sufficient memory is preallocated.
//
// The basic atomic operation of this allocator is cmpxchg on long.
// On architectures that don't have NMI-safe cmpxchg implementation,
// the allocator can NOT be used in NMI handler.  So code uses the
// allocator in NMI handler should depend on
// CONFIG_ARCH_HAVE_NMI_SAFE_CMPXCHG.
//

//
// typedef genpool_algo_t: Allocation callback function type definition
// @map: Pointer to bitmap
// @size: The bitmap size in bits
// @start: The bitnumber to start searching at
// @nr: The number of zeroed bits we're looking for
// @data: optional additional data used by the callback
// @pool: the pool being allocated from
// @start_addr: start address of memory chunk
//
// General purpose special memory pool descriptor.
//
#[repr(C)]
#[derive(Copy, Clone)]
pub struct gen_pool {
    pub lock: spinlock_t,
    pub /: *mut *mut list_head chunks; / list of chunks in this pool,
    pub /: *mut *mut int min_alloc_order; / minimum allocation order,
    pub /: *mut *mut genpool_algo_t algo; / allocation function,
    pub data: *mut c_void,
    pub name: *const c_char,
}

//
// General purpose special memory pool chunk descriptor.
//
#[repr(C)]
#[derive(Copy, Clone)]
pub struct gen_pool_chunk {
    pub /: *mut *mut list_head next_chunk; / next chunk in pool,
    pub avail: atomic_long_t,
    pub /: *mut *mut phys_addr_t phys_addr; / physical starting address of memory chunk,
    pub /: *mut *mut *mut void owner; / private data to retrieve at alloc time,
    pub /: *mut *mut unsigned long start_addr; / start address of memory chunk,
    pub /: *mut *mut unsigned long end_addr; / end address of memory chunk (inclusive),
    pub /: *mut *mut unsigned long bits[]; / bitmap for allocating memory chunk,
}

//
// gen_pool data descriptor for gen_pool_first_fit_align.
//
#[repr(C)]
#[derive(Copy, Clone)]
pub struct genpool_data_align {
    pub /: *mut *mut int align; / alignment by bytes for starting address,
}

//
// gen_pool data descriptor for gen_pool_fixed_alloc.
//
#[repr(C)]
#[derive(Copy, Clone)]
pub struct genpool_data_fixed {
    pub /: *mut *mut unsigned long offset; / The offset of the specific region,
}

extern "C" {
    pub fn gen_pool_virt_to_phys(pool: *mut gen_pool, long: unsigned) -> phys_addr_t;
}
extern "C" {
    pub fn gen_pool_add_owner(_arg: pool, _arg: addr, _arg: phys, _arg: size, _arg: nid, _arg: NULL) -> return;
}
//
// gen_pool_add - add a new chunk of special memory to the pool
// @pool: pool to add new memory chunk to
// @addr: starting address of memory chunk to add to pool
// @size: size in bytes of the memory chunk to add to pool
// @nid: node id of the node the chunk structure and bitmap should be
// allocated on, or -1
//
// Add a new chunk of special memory to the specified pool.
//
// Returns 0 on success or a -ve errno on failure.
//
extern "C" {
    pub fn gen_pool_add_virt(_arg: pool, _arg: addr, _arg: -1, _arg: size, _arg: nid) -> return;
}
extern "C" {
    pub fn gen_pool_destroy(: *mut gen_pool);
}
extern "C" {
    pub fn gen_pool_alloc_algo_owner(_arg: pool, _arg: size, _arg: algo, _arg: data, _arg: NULL) -> return;
}
//
// gen_pool_alloc - allocate special memory from the pool
// @pool: pool to allocate from
// @size: number of bytes to allocate from the pool
//
// Allocate the requested number of bytes from the specified pool.
// Uses the pool allocation function (with first-fit algorithm by default).
// Can not be used in NMI handler on architectures without
// NMI-safe cmpxchg implementation.
//
extern "C" {
    pub fn gen_pool_alloc_algo(_arg: pool, _arg: size, _arg: pool->algo, _arg: pool->data) -> return;
}
extern "C" {
    pub fn gen_pool_avail(: *mut gen_pool) -> usize;
}
extern "C" {
    pub fn gen_pool_size(: *mut gen_pool) -> usize;
}

