//! Automatically rewritten from C Header to Rust Module
//! Source: net/xdp/xsk_queue.h
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
// XDP user-space ring structure
// Copyright(c) 2018 Intel Corporation.
//

#[repr(C)]
#[derive(Copy, Clone)]
pub struct xdp_ring {
    pub ____cacheline_aligned_in_smp: u32 producer,
// Hinder the adjacent cache prefetcher to prefetch the consumer
// pointer if the producer pointer is touched and vice versa.
//
    pub ____cacheline_aligned_in_smp: u32 pad1,
    pub ____cacheline_aligned_in_smp: u32 consumer,
    pub ____cacheline_aligned_in_smp: u32 pad2,
    pub flags: u32,
    pub ____cacheline_aligned_in_smp: u32 pad3,
}

// Used for the RX and TX queues for packets
#[repr(C)]
#[derive(Copy, Clone)]
pub struct xdp_rxtx_ring {
    pub ptrs: xdp_ring,
    pub ____cacheline_aligned_in_smp: xdp_desc desc[],
}

// Used for the fill and completion queues for buffers
#[repr(C)]
#[derive(Copy, Clone)]
pub struct xdp_umem_ring {
    pub ptrs: xdp_ring,
    pub ____cacheline_aligned_in_smp: u64 desc[],
}

#[repr(C)]
#[derive(Copy, Clone)]
pub struct xsk_queue {
    pub ring_mask: u32,
    pub nentries: u32,
    pub cached_prod: u32,
    pub cached_cons: u32,
    pub ring: *mut xdp_ring,
    pub invalid_descs: u64,
    pub queue_empty_descs: u64,
    pub ring_vmalloc_size: usize,
// Mutual exclusion of the completion ring in the SKB mode.
// Protect: when sockets share a single cq when the same netdev
// and queue id is shared.
//
    pub cq_cached_prod_lock: spinlock_t,
}

#[repr(C)]
#[derive(Copy, Clone)]
pub struct parsed_desc {
    pub mb: u32,
    pub valid: u32,
}

#[repr(C)]
#[derive(Copy, Clone)]
pub struct xsk_tx_batch {
    pub tx_descs: u32,
    pub reclaim_descs: u32,
    pub budget_limited: bool,
}

// The structure of the shared state of the rings are a simple
// circular buffer, as outlined in
// Documentation/core-api/circular-buffers.rst. For the Rx and
// completion ring, the kernel is the producer and user space is the
// consumer. For the Tx and fill rings, the kernel is the consumer and
// user space is the producer.
//
// producer                         consumer
//
// if (LOAD ->consumer) {  (A)      LOAD.acq ->producer  (C)
// STORE $data                   LOAD $data
// STORE.rel ->producer (B)      STORE.rel ->consumer (D)
// }
//
// (A) pairs with (D), and (B) pairs with (C).
//
// Starting with (B), it protects the data from being written after
// the producer pointer. If this barrier was missing, the consumer
// could observe the producer pointer being set and thus load the data
// before the producer has written the new data. The consumer would in
// this case load the old data.
//
// (C) protects the consumer from speculatively loading the data before
// the producer pointer actually has been read. If we do not have this
// barrier, some architectures could load old data as speculative loads
// are not discarded as the CPU does not know there is a dependency
// between ->producer and data.
//
// (A) is a control dependency that separates the load of ->consumer
// from the stores of $data. In case ->consumer indicates there is no
// room in the buffer to store $data we do not. The dependency will
// order both of the stores after the loads. So no barrier is needed.
//
// (D) protects the load of the data to be observed to happen after the
// store of the consumer pointer. If we did not have this memory
// barrier, the producer could observe the consumer pointer being set
// and overwrite the data with a new value before the consumer got the
// chance to read the old value. The consumer would thus miss reading
// the old entry and very likely read the new entry twice, once right
// now and again after circling through the ring.
//
// The operations on the rings are the following:
//
// producer                           consumer
//
// RESERVE entries                    PEEK in the ring for entries
// WRITE data into the ring           READ data from the ring
// SUBMIT entries                     RELEASE entries
//
// The producer reserves one or more entries in the ring. It can then
// fill in these entries and finally submit them so that they can be
// seen and read by the consumer.
//
// The consumer peeks into the ring to see if the producer has written
// any new entries. If so, the consumer can then read these entries
// and when it is done reading them release them back to the producer
// so that the producer can use these slots to fill in new entries.
//
// The function names below reflect these operations.
//
// Functions that read and validate content from consumer rings.
// addr = ring->desc[idx];
// Can overflow if desc->addr < pool->tx_metadata_len
//
// Can't overflow: @offset is guaranteed to be < ``U32_MAX``
// (pool->chunk_size is ``u32``), @len is guaranteed
// to be <= ``U32_MAX``.
//
// Can't overflow: @len is guaranteed to be <= ``U32_MAX``
// Can overflow if desc->addr is close to 0
// Can overflow if pool->addrs_cnt is high enough
// desc = ring->desc[idx];
extern "C" {
    pub fn xskq_cons_is_valid_desc(_arg: q, _arg: desc, _arg: pool) -> return;
}
// Release valid plus any invalid entries
// Functions for consumers
// Refresh the local pointer
extern "C" {
    pub fn xskq_cons_read_addr_unchecked(_arg: q, _arg: addr) -> return;
}
extern "C" {
    pub fn xskq_cons_read_desc(_arg: q, _arg: desc, _arg: pool) -> return;
}
// To improve performance in the xskq_cons_release functions, only update local state here.
// Reflect this to global state when we get new entries from the ring in
// xskq_cons_get_entries() and whenever Rx or Tx processing are completed in the NAPI loop.
//
// No barriers needed since data is not accessed
extern "C" {
    pub fn READ_ONCE(READ_ONCE(q->ring->consumer: q->ring->producer) -) -> return;
}
// Functions for producers
extern "C" {
    pub fn READ_ONCE(_arg: q->ring->producer) -> return;
}
// Refresh the local tail pointer
// A, matches D
// No barriers needed since data is not accessed
extern "C" {
    pub fn READ_ONCE(READ_ONCE(q->ring->producer: q->ring->consumer) ==) -> return;
}
// For both producers and consumers
extern "C" {
    pub fn xskq_destroy(q_ops: *mut xsk_queue);
}
