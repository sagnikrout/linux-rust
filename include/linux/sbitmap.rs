//! Automatically rewritten from C Header to Rust Module
//! Source: include/linux/sbitmap.h
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
// Fast and scalable bitmaps.
//
// Copyright (C) 2016 Facebook
// Copyright (C) 2013-2014 Jens Axboe
//

//
// struct sbitmap_word - Word in a &struct sbitmap.
//
#[repr(C)]
#[derive(Copy, Clone)]
pub struct sbitmap_word {
//
// @word: word holding free bits
//
    pub word: c_ulong,
//
// @cleared: word holding cleared bits
//
    pub ____cacheline_aligned_in_smp: unsigned long cleared,
//
// @swap_lock: serializes simultaneous updates of ->word and ->cleared
//
    pub swap_lock: raw_spinlock_t,
    pub ____cacheline_aligned_in_smp: },
//
// struct sbitmap - Scalable bitmap.
//
// A &struct sbitmap is spread over multiple cachelines to avoid ping-pong. This
// trades off higher memory usage for better scalability.
//
#[repr(C)]
#[derive(Copy, Clone)]
pub struct sbitmap {
//
// @depth: Number of bits used in the whole bitmap.
//
    pub depth: c_uint,
//
// @shift: log2(number of bits used per word)
//
    pub shift: c_uint,
//
// @map_nr: Number of words (cachelines) being used for the bitmap.
//
    pub map_nr: c_uint,
//
// @round_robin: Allocate bits in strict round-robin order.
//
    pub round_robin: bool,
//
// @map: Allocated bitmap.
//
    pub map: *mut sbitmap_word,
//
// @alloc_hint: Cache of last successfully allocated or freed bit.
//
// This is per-cpu, which allows multiple users to stick to different
// cachelines until the map is exhausted.
//
    pub alloc_hint: *mut unsigned int __percpu,
}

pub const SBQ_WAIT_QUEUES: c_int = 8;
pub const SBQ_WAKE_BATCH: c_int = 8;
//
// struct sbq_wait_state - Wait queue in a &struct sbitmap_queue.
//
#[repr(C)]
#[derive(Copy, Clone)]
pub struct sbq_wait_state {
//
// @wait: Wait queue.
//
    pub wait: wait_queue_head_t,
    pub ____cacheline_aligned_in_smp: },
//
// struct sbitmap_queue - Scalable bitmap with the added ability to wait on free
// bits.
//
// A &struct sbitmap_queue uses multiple wait queues and rolling wakeups to
// avoid contention on the wait queue spinlock. This ensures that we don't hit a
// scalability wall when we run out of free bits and have to start putting tasks
// to sleep.
//
#[repr(C)]
#[derive(Copy, Clone)]
pub struct sbitmap_queue {
//
// @sb: Scalable bitmap.
//
    pub sb: sbitmap,
//
// @wake_batch: Number of bits which must be freed before we wake up any
// waiters.
//
    pub wake_batch: c_uint,
//
// @wake_index: Next wait queue in @ws to wake up.
//
    pub wake_index: core::sync::atomic::AtomicI32,
//
// @ws: Wait queues.
//
    pub ws: *mut sbq_wait_state,
//
// @ws_active: count of currently active ws waitqueues
//
    pub ws_active: core::sync::atomic::AtomicI32,
//
// @min_shallow_depth: The minimum shallow depth which may be passed to
// sbitmap_queue_get_shallow()
//
    pub min_shallow_depth: c_uint,
//
// @completion_cnt: Number of bits cleared passed to the
// wakeup function.
//
    pub completion_cnt: core::sync::atomic::AtomicI32,
//
// @wakeup_cnt: Number of thread wake ups issued.
//
    pub wakeup_cnt: core::sync::atomic::AtomicI32,
}

//
// sbitmap_init_node() - Initialize a &struct sbitmap on a specific memory node.
// @sb: Bitmap to initialize.
// @depth: Number of bits to allocate.
// @shift: Use 2^@shift bits per word in the bitmap; if a negative number if
// given, a good default is chosen.
// @flags: Allocation flags.
// @node: Memory node to allocate on.
// @round_robin: If true, be stricter about allocation order; always allocate
// starting from the last allocated bit. This is less efficient
// than the default behavior (false).
// @alloc_hint: If true, apply percpu hint for where to start searching for
// a free bit.
//
// Return: Zero on success or negative errno on failure.
//
// sbitmap internal helper
//
// sbitmap_free() - Free memory used by a &struct sbitmap.
// @sb: Bitmap to free.
//
// sbitmap_resize() - Resize a &struct sbitmap.
// @sb: Bitmap to resize.
// @depth: New number of bits to resize to.
//
// Doesn't reallocate anything. It's up to the caller to ensure that the new
// depth doesn't exceed the depth that the sb was initialized with.
//
extern "C" {
    pub fn sbitmap_resize(sb: *mut sbitmap, depth: c_uint);
}
//
// sbitmap_get() - Try to allocate a free bit from a &struct sbitmap.
// @sb: Bitmap to allocate from.
//
// This operation provides acquire barrier semantics if it succeeds.
//
// Return: Non-negative allocated bit number if successful, -1 otherwise.
//
extern "C" {
    pub fn sbitmap_get(sb: *mut sbitmap) -> c_int;
}
//
// sbitmap_any_bit_set() - Check for a set bit in a &struct sbitmap.
// @sb: Bitmap to check.
//
// Return: true if any bit in the bitmap is set, false otherwise.
//
extern "C" {
    pub fn sbitmap_any_bit_set(sb: *const sbitmap) -> bool;
}

extern "C" {
    pub fn bool(: *mut *mut sb_for_each_fn)(struct sbitmap, int: unsigned, : *mut c_void) -> typedef;
}
//
// __sbitmap_for_each_set() - Iterate over each set bit in a &struct sbitmap.
// @start: Where to start the iteration.
// @sb: Bitmap to iterate over.
// @fn: Callback. Should return true to continue or false to break early.
// @data: Pointer to pass to callback.
//
// This is inline even though it's non-trivial so that the function calls to the
// callback will hopefully get optimized away.
//
// On the first iteration of the outer loop, we need to add the
// bit offset back to the size of the word for find_next_bit().
// On all other iterations, nr is zero, so this is a noop.
//
// sbitmap_for_each_set() - Iterate over each set bit in a &struct sbitmap.
// @sb: Bitmap to iterate over.
// @fn: Callback. Should return true to continue or false to break early.
// @data: Pointer to pass to callback.
//
// Helpers equivalent to the operations in asm/bitops.h and linux/bitmap.h
//
// This one is special, since it doesn't actually clear the bit, rather it
// sets the corresponding bit in the ->cleared mask instead. Paired with
// the caller doing sbitmap_deferred_clear() if a given index is full, which
// will clear the previously freed entries in the corresponding ->word.
//
// Pair of sbitmap_get, and this one applies both cleared bit and
// allocation hint.
//
// raw_cpu_ptr(sb->alloc_hint) = bitnr;
extern "C" {
    pub fn test_bit(_arg: SB_NR_TO_BIT(sb, _arg: bitnr), _arg: __sbitmap_word(sb, _arg: bitnr)) -> return;
}
//
// If the bitmap is small, shrink the number of bits per word so
// we spread over a few cachelines, at least. If less than 4
// bits, just forget about it, it's not going to work optimally
// anyway.
//
// sbitmap_show() - Dump &struct sbitmap information to a &struct seq_file.
// @sb: Bitmap to show.
// @m: struct seq_file to write to.
//
// This is intended for debugging. The format may change at any time.
//
extern "C" {
    pub fn sbitmap_show(sb: *mut sbitmap, m: *mut seq_file);
}
//
// sbitmap_weight() - Return how many set and not cleared bits in a &struct
// sbitmap.
// @sb: Bitmap to check.
//
// Return: How many set and not cleared bits set
//
extern "C" {
    pub fn sbitmap_weight(sb: *const sbitmap) -> c_uint;
}
//
// sbitmap_bitmap_show() - Write a hex dump of a &struct sbitmap to a &struct
// seq_file.
// @sb: Bitmap to show.
// @m: struct seq_file to write to.
//
// This is intended for debugging. The output isn't guaranteed to be internally
// consistent.
//
extern "C" {
    pub fn sbitmap_bitmap_show(sb: *mut sbitmap, m: *mut seq_file);
}
//
// sbitmap_queue_init_node() - Initialize a &struct sbitmap_queue on a specific
// memory node.
// @sbq: Bitmap queue to initialize.
// @depth: See sbitmap_init_node().
// @shift: See sbitmap_init_node().
// @round_robin: See sbitmap_get().
// @flags: Allocation flags.
// @node: Memory node to allocate on.
//
// Return: Zero on success or negative errno on failure.
//
// sbitmap_queue_free() - Free memory used by a &struct sbitmap_queue.
//
// @sbq: Bitmap queue to free.
//
// sbitmap_queue_recalculate_wake_batch() - Recalculate wake batch
// @sbq: Bitmap queue to recalculate wake batch.
// @users: Number of shares.
//
// Like sbitmap_queue_update_wake_batch(), this will calculate wake batch
// by depth. This interface is for HCTX shared tags or queue shared tags.
//
// sbitmap_queue_resize() - Resize a &struct sbitmap_queue.
// @sbq: Bitmap queue to resize.
// @depth: New number of bits to resize to.
//
// Like sbitmap_resize(), this doesn't reallocate anything. It has to do
// some extra work on the &struct sbitmap_queue, so it's not safe to just
// resize the underlying &struct sbitmap.
//
extern "C" {
    pub fn sbitmap_queue_resize(sbq: *mut sbitmap_queue, depth: c_uint);
}
//
// __sbitmap_queue_get() - Try to allocate a free bit from a &struct
// sbitmap_queue with preemption already disabled.
// @sbq: Bitmap queue to allocate from.
//
// Return: Non-negative allocated bit number if successful, -1 otherwise.
//
extern "C" {
    pub fn __sbitmap_queue_get(sbq: *mut sbitmap_queue) -> c_int;
}
//
// __sbitmap_queue_get_batch() - Try to allocate a batch of free bits
// @sbq: Bitmap queue to allocate from.
// @nr_tags: number of tags requested
// @offset: offset to add to returned bits
//
// Return: Mask of allocated tags, 0 if none are found. Each tag allocated is
// a bit in the mask returned, and the caller must add @offset to the value to
// get the absolute tag value.
//
// sbitmap_queue_get_shallow() - Try to allocate a free bit from a &struct
// sbitmap_queue, limiting the depth used from each word, with preemption
// already disabled.
// @sbq: Bitmap queue to allocate from.
// @shallow_depth: The maximum number of bits to allocate from the queue.
// See sbitmap_get_shallow().
//
// If you call this, make sure to call sbitmap_queue_min_shallow_depth() after
// initializing @sbq.
//
// Return: Non-negative allocated bit number if successful, -1 otherwise.
//
// sbitmap_queue_get() - Try to allocate a free bit from a &struct
// sbitmap_queue.
// @sbq: Bitmap queue to allocate from.
// @cpu: Output parameter; will contain the CPU we ran on (e.g., to be passed to
// sbitmap_queue_clear()).
//
// Return: Non-negative allocated bit number if successful, -1 otherwise.
//
// cpu = get_cpu();
//
// sbitmap_queue_min_shallow_depth() - Inform a &struct sbitmap_queue of the
// minimum shallow depth that will be used.
// @sbq: Bitmap queue in question.
// @min_shallow_depth: The minimum shallow depth that will be passed to
// sbitmap_queue_get_shallow() or __sbitmap_queue_get_shallow().
//
// sbitmap_queue_clear() batches wakeups as an optimization. The batch size
// depends on the depth of the bitmap. Since the shallow allocation functions
// effectively operate with a different depth, the shallow depth must be taken
// into account when calculating the batch size. This function must be called
// with the minimum shallow depth that will be used. Failure to do so can result
// in missed wakeups.
//
// sbitmap_queue_clear() - Free an allocated bit and wake up waiters on a
// &struct sbitmap_queue.
// @sbq: Bitmap to free from.
// @nr: Bit number to free.
// @cpu: CPU the bit was allocated on.
//
// sbitmap_queue_clear_batch() - Free a batch of allocated bits
// &struct sbitmap_queue.
// @sbq: Bitmap to free from.
// @offset: offset for each tag in array
// @tags: array of tags
// @nr_tags: number of tags in array
//
// sbq_wait_ptr() - Get the next wait queue to use for a &struct
// sbitmap_queue.
// @sbq: Bitmap queue to wait on.
// @wait_index: A counter per "user" of @sbq.
//
// Return: Next wait queue to be used
//
// sbitmap_queue_wake_all() - Wake up everything waiting on a &struct
// sbitmap_queue.
// @sbq: Bitmap queue to wake up.
//
extern "C" {
    pub fn sbitmap_queue_wake_all(sbq: *mut sbitmap_queue);
}
//
// sbitmap_queue_wake_up() - Wake up some of waiters in one waitqueue
// on a &struct sbitmap_queue.
// @sbq: Bitmap queue to wake up.
// @nr: Number of bits cleared.
//
extern "C" {
    pub fn sbitmap_queue_wake_up(sbq: *mut sbitmap_queue, nr: c_int);
}
//
// sbitmap_queue_show() - Dump &struct sbitmap_queue information to a &struct
// seq_file.
// @sbq: Bitmap queue to show.
// @m: struct seq_file to write to.
//
// This is intended for debugging. The format may change at any time.
//
extern "C" {
    pub fn sbitmap_queue_show(sbq: *mut sbitmap_queue, m: *mut seq_file);
}
#[repr(C)]
#[derive(Copy, Clone)]
pub struct sbq_wait {
    pub /: *mut *mut *mut sbitmap_queue sbq; / if set, sbq_wait is accounted,
    pub wait: wait_queue_entry,
}

//
// Wrapper around prepare_to_wait_exclusive(), which maintains some extra
// internal state.
//
// Must be paired with sbitmap_prepare_to_wait().
//
// Wrapper around add_wait_queue(), which maintains some extra internal state
//
// Must be paired with sbitmap_add_wait_queue()
//
extern "C" {
    pub fn sbitmap_del_wait_queue(sbq_wait: *mut sbq_wait);
}
