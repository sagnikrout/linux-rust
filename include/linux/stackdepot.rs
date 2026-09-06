//! Automatically rewritten from C Header to Rust Module
//! Source: include/linux/stackdepot.h
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
// Stack depot - a stack trace storage that avoids duplication.
//
// Stack depot is intended to be used by subsystems that need to store and
// later retrieve many potentially duplicated stack traces without wasting
// memory.
//
// For example, KASAN needs to save allocation and free stack traces for each
// object. Storing two stack traces per object requires a lot of memory (e.g.
// SLUB_DEBUG needs 256 bytes per object for that). Since allocation and free
// stack traces often repeat, using stack depot allows to save about 100x space.
//
// Author: Alexander Potapenko <glider@google.com>
// Copyright (C) 2016 Google, Inc.
//
// Based on the code by Dmitry Chernenkov.
//

pub type depot_stack_handle_t = u32;
//
// Number of bits in the handle that stack depot doesn't use. Users may store
// information in them via stack_depot_set/get_extra_bits.
//
pub const STACK_DEPOT_EXTRA_BITS: c_int = 5;

pub const DEPOT_STACK_ALIGN: c_int = 4;

// Compact structure that stores a reference to a stack.
#[repr(C)]
#[derive(Copy, Clone)]
pub union handle_parts {
    pub handle: depot_stack_handle_t,
    pub DEPOT_POOL_INDEX_BITS: u32 pool_index_plus_1 :,
    pub DEPOT_OFFSET_BITS: u32 offset :,
    pub STACK_DEPOT_EXTRA_BITS: u32 extra :,
}

#[repr(C)]
#[derive(Copy, Clone)]
pub struct stack_record {
    pub /: *mut *mut list_head hash_list; / Links in the hash table,
    pub /: *mut *mut u32 hash; / Hash in hash table,
    pub /: *mut *mut u32 size; / Number of stored frames,
    pub /: *mut *mut handle_parts handle; / Constant after initialization,
    pub count: refcount_t,
    pub /: *mut *mut unsigned long entries[CONFIG_STACKDEPOT_MAX_FRAMES]; / Frames,
//
// An important invariant of the implementation is to
// only place a stack record onto the freelist iff its
// refcount is zero. Because stack records with a zero
// refcount are never considered as valid, it is safe to
// union @entries and freelist management state below.
// Conversely, as soon as an entry is off the freelist
// and its refcount becomes non-zero, the below must not
// be accessed until being placed back on the freelist.
//
    pub /: *mut *mut list_head free_list; / Links in the freelist,
    pub /: *mut *mut unsigned long rcu_state; / RCU cookie,
}

pub type depot_flags_t = u32;
//
// Flags that can be passed to stack_depot_save_flags(); see the comment next
// to its declaration for more details.
//

pub const STACK_DEPOT_FLAGS_NUM: c_int = 2;

//
// Using stack depot requires its initialization, which can be done in 3 ways:
//
// 1. Selecting CONFIG_STACKDEPOT_ALWAYS_INIT. This option is suitable in
// scenarios where it's known at compile time that stack depot will be used.
// Enabling this config makes the kernel initialize stack depot in mm_init().
//
// 2. Calling stack_depot_request_early_init() during early boot, before
// stack_depot_early_init() in mm_init() completes. For example, this can
// be done when evaluating kernel boot parameters.
//
// 3. Calling stack_depot_init(). Possible after boot is complete. This option
// is recommended for modules initialized later in the boot process, after
// mm_init() completes.
//
// stack_depot_init() and stack_depot_request_early_init() can be called
// regardless of whether CONFIG_STACKDEPOT is enabled and are no-op when this
// config is disabled. The save/fetch/print stack depot functions can only be
// called from the code that makes sure CONFIG_STACKDEPOT is enabled _and_
// initializes stack depot via one of the ways listed above.
//

extern "C" {
    pub fn stack_depot_init() -> c_int;
}
extern "C" {
    pub fn stack_depot_request_early_init() -> void __init;
}
// Must be only called from mm_init().
extern "C" {
    pub fn stack_depot_early_init() -> int __init;
}

//
// stack_depot_save_flags - Save a stack trace to stack depot
//
// @entries:		Pointer to the stack trace
// @nr_entries:		Number of frames in the stack
// @alloc_flags:	Allocation GFP flags
// @depot_flags:	Stack depot flags
//
// Saves a stack trace from @entries array of size @nr_entries.
//
// If STACK_DEPOT_FLAG_CAN_ALLOC is set in @depot_flags, stack depot can
// replenish the stack pools in case no space is left (allocates using GFP
// flags of @alloc_flags). Otherwise, stack depot avoids any allocations and
// fails if no space is left to store the stack trace.
//
// If STACK_DEPOT_FLAG_GET is set in @depot_flags, stack depot will increment
// the refcount on the saved stack trace if it already exists in stack depot.
// Users of this flag must also call stack_depot_put() when keeping the stack
// trace is no longer required to avoid overflowing the refcount.
//
// If the provided stack trace comes from the interrupt context, only the part
// up to the interrupt entry is saved.
//
// Context: Any context, but unsetting STACK_DEPOT_FLAG_CAN_ALLOC is required if
// alloc_pages() cannot be used from the current context. Currently
// this is the case for contexts where neither %GFP_ATOMIC nor
// %GFP_NOWAIT can be used (NMI, raw_spin_lock).
//
// Return: Handle of the stack struct stored in depot, 0 on failure
//
// stack_depot_save - Save a stack trace to stack depot
//
// @entries:		Pointer to the stack trace
// @nr_entries:		Number of frames in the stack
// @alloc_flags:	Allocation GFP flags
//
// Does not increment the refcount on the saved stack trace; see
// stack_depot_save_flags() for more details.
//
// Context: Contexts where allocations via alloc_pages() are allowed;
// see stack_depot_save_flags() for more details.
//
// Return: Handle of the stack trace stored in depot, 0 on failure
//
// __stack_depot_get_stack_record - Get a pointer to a stack_record struct
//
// @handle: Stack depot handle
//
// This function is only for internal purposes.
//
// Return: Returns a pointer to a stack_record struct
//
// stack_depot_fetch - Fetch a stack trace from stack depot
//
// @handle:	Stack depot handle returned from stack_depot_save()
// @entries:	Pointer to store the address of the stack trace
//
// Return: Number of frames for the fetched stack
//
// stack_depot_print - Print a stack trace from stack depot
//
// @stack:	Stack depot handle returned from stack_depot_save()
//
extern "C" {
    pub fn stack_depot_print(stack: depot_stack_handle_t);
}
//
// stack_depot_snprint - Print a stack trace from stack depot into a buffer
//
// @handle:	Stack depot handle returned from stack_depot_save()
// @buf:	Pointer to the print buffer
// @size:	Size of the print buffer
// @spaces:	Number of leading spaces to print
//
// Return:	Number of bytes printed
//
// stack_depot_put - Drop a reference to a stack trace from stack depot
//
// @handle:	Stack depot handle returned from stack_depot_save()
//
// The stack trace is evicted from stack depot once all references to it have
// been dropped (once the number of stack_depot_evict() calls matches the
// number of stack_depot_save_flags() calls with STACK_DEPOT_FLAG_GET set for
// this stack trace).
//
extern "C" {
    pub fn stack_depot_put(handle: depot_stack_handle_t);
}
//
// stack_depot_set_extra_bits - Set extra bits in a stack depot handle
//
// @handle:	Stack depot handle returned from stack_depot_save()
// @extra_bits:	Value to set the extra bits
//
// Return: Stack depot handle with extra bits set
//
// Stack depot handles have a few unused bits, which can be used for storing
// user-specific information. These bits are transparent to the stack depot.
//
// stack_depot_get_extra_bits - Retrieve extra bits from a stack depot handle
//
// @handle:	Stack depot handle with extra bits saved
//
// Return: Extra bits retrieved from the stack depot handle
//
extern "C" {
    pub fn stack_depot_get_extra_bits(handle: depot_stack_handle_t) -> c_uint;
}
