//! Automatically rewritten from C to Rust
//! Source: tools/virtio/ringtest/virtio_ring_0_9.c
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
// Copyright (C) 2016 Red Hat, Inc.
// Author: Michael S. Tsirkin <mst@redhat.com>
//
// Partial implementation of virtio 0.9. event index is used for signalling,
// unconditionally. Design roughly follows linux kernel implementation in order
// to be able to judge its performance.
//
// Macro flag: #define _GNU_SOURCE

#[repr(C)]
#[derive(Copy, Clone)]
pub struct data {
    pub data: *mut c_void,
    pub data: *mut },
    pub ring: vring,
// enabling the below activates experimental ring polling code
// (which skips index reads on consumer in favor of looking at
// high bits of ring id ^ 0x8000).
//
// #ifdef RING_POLL
// enabling the below activates experimental in-order code
// (which skips ring updates and reads and writes len in descriptor).
//
// #ifdef INORDER

// how much padding is needed to avoid false cache sharing
pub const HOST_GUEST_PADDING: c_uint = 0x80;
#[repr(C)]
#[derive(Copy, Clone)]
pub struct guest {
    pub avail_idx: c_ushort,
    pub last_used_idx: c_ushort,
    pub num_free: c_ushort,
    pub kicked_avail_idx: c_ushort,

    pub free_head: c_ushort,

    pub reserved_free_head: c_ushort,

    pub 10]: unsigned char reserved[HOST_GUEST_PADDING -,
    pub guest: },
#[repr(C)]
#[derive(Copy, Clone)]
pub struct host {
// we do not need to track last avail index
// unless we have more than one in flight.
//
    pub used_idx: c_ushort,
    pub called_used_idx: c_ushort,
    pub 4]: unsigned char reserved[HOST_GUEST_PADDING -,
    pub host: },
// implemented by ring
#[no_mangle]
pub unsafe extern "C" fn alloc_ring() {
    void alloc_ring(void)
    {
    pub ret: c_int,
    pub i: c_int,
    pub p: *mut c_void,
    pub 0x1000)): ret = posix_memalign(&p, 0x1000, vring_size(ring_size,,
    if (ret) {
    pub buffer.\n"): perror("Unable to allocate ring,
    }
    pub 0x1000)): memset(p, 0, vring_size(ring_size,,
    pub 0x1000): vring_init(&ring, ring_size, p,,
    pub 0: guest.avail_idx =,
    pub -1: guest.kicked_avail_idx =,
    pub 0: guest.last_used_idx =,

// Put everything in free lists.
    pub 0: guest.free_head =,

    pub i++): for (i = 0; i < ring_size - 1;,
    pub 1: ring.desc[i].next = i +,
    pub 0: host.used_idx =,
    pub -1: host.called_used_idx =,
    pub ring_size: guest.num_free =,
    pub data): *mut *mut data = malloc(ring_size  sizeof,
    if (!data) {
    pub buffer.\n"): perror("Unable to allocate data,
    }
    pub data): *mut *mut memset(data, 0, ring_size  sizeof,
    }
// guest side
#[no_mangle]
pub unsafe extern "C" fn add_inbuf(len: unsigned, buf: *mut c_void, datap: *mut c_void) -> c_int {
    int add_inbuf(unsigned len, void *buf, void *datap)
    {
    pub head: unsigned,

    pub avail: unsigned,

    pub desc: *mut vring_desc,
    if (!guest.num_free)
    pub -1: return,

    pub (guest.avail_idx++): head = (ring_size - 1) &,

    pub guest.free_head: head =,

    pub ring.desc: desc =,
    pub VRING_DESC_F_NEXT: desc[head].flags =,
    pub )buf: *mut desc[head].addr = (unsigned long)(void,
    pub len: desc[head].len =,
// We do it like this to simulate the way
// we'd have to flip it if we had multiple
// descriptors.
//
    pub ~VRING_DESC_F_NEXT: desc[head].flags &=,

    pub desc[head].next: guest.free_head =,

    pub datap: data[head].data =,

// Barrier A (for pairing)
    pub guest.avail_idx++: avail =,
    ring.avail.ring[avail & (ring_size - 1)] =
    pub 0x8000: (head | (avail & ~(ring_size - 1))) ^,

// Barrier A (for pairing)
    pub (guest.avail_idx++): avail = (ring_size - 1) &,
    pub head: ring.avail->ring[avail] =,

// Barrier A (for pairing)

    pub guest.avail_idx: ring.avail->idx =,
    pub 0: return,
    }
    void *get_buf(unsigned *lenp, void **bufp)
    {
    pub head: unsigned,
    pub index: unsigned,
    pub datap: *mut c_void,

    pub guest.last_used_idx: head = (ring_size - 1) &,
    pub ring.used->ring[head].id: index =,
    if ((index ^ guest.last_used_idx ^ 0x8000) & ~(ring_size - 1))
    pub NULL: return,
// Barrier B (for pairing)
    pub 1: index &= ring_size -,

    if (ring.used.idx == guest.last_used_idx)
    pub NULL: return,
// Barrier B (for pairing)

    pub guest.last_used_idx: head = (ring_size - 1) &,
    pub head: index =,

    pub guest.last_used_idx: head = (ring_size - 1) &,
    pub ring.used->ring[head].id: index =,

// lenp = ring.desc[index].len;

// lenp = ring.used->ring[head].len;

    pub data[index].data: datap =,
// bufp = (void*)(unsigned long)ring.desc[index].addr;
    pub NULL: data[index].data =,

    pub guest.free_head: ring.desc[index].next =,
    pub index: guest.free_head =,

    pub datap: return,
    }
#[no_mangle]
pub unsafe extern "C" fn used_empty() -> bool {
    bool used_empty()
    {
    pub guest.last_used_idx: unsigned short last_used_idx =,

    pub 1): unsigned short head = last_used_idx & (ring_size -,
    pub ring.used->ring[head].id: unsigned index =,
    pub 1): return (index ^ last_used_idx ^ 0x8000) & ~(ring_size -,

    pub last_used_idx: return ring.used->idx ==,

    }
#[no_mangle]
pub unsafe extern "C" fn disable_call() {
    void disable_call()
    {
// Doing nothing to disable calls might cause
// extra interrupts, but reduces the number of cache misses.
//
    }
#[no_mangle]
pub unsafe extern "C" fn enable_call() -> bool {
    bool enable_call()
    {
    pub guest.last_used_idx: vring_used_event(&ring) =,
// Flush call index write
// Barrier D (for pairing)
    pub used_empty(): return,
    }
#[no_mangle]
pub unsafe extern "C" fn kick_available() {
    void kick_available(void)
    {
    pub need: bool,
// Flush in previous flags write
// Barrier C (for pairing)
    need = vring_need_event(vring_avail_event(&ring),
    guest.avail_idx,
    pub guest.avail_idx: guest.kicked_avail_idx =,
    if (need)
    }
// host side
#[no_mangle]
pub unsafe extern "C" fn disable_kick() {
    void disable_kick()
    {
// Doing nothing to disable kicks might cause
// extra interrupts, but reduces the number of cache misses.
//
    }
#[no_mangle]
pub unsafe extern "C" fn enable_kick() -> bool {
    bool enable_kick()
    {
    pub host.used_idx: vring_avail_event(&ring) =,
// Barrier C (for pairing)
    pub avail_empty(): return,
    }
#[no_mangle]
pub unsafe extern "C" fn avail_empty() -> bool {
    bool avail_empty()
    {
    pub host.used_idx: unsigned head =,

    pub 1)]: unsigned index = ring.avail->ring[head & (ring_size -,
    pub 1)): return ((index ^ head ^ 0x8000) & ~(ring_size -,

    pub ring.avail->idx: return head ==,

    }
#[no_mangle]
pub unsafe extern "C" fn use_buf(lenp: *mut unsigned, bufp: *mut c_void) -> bool {
    bool use_buf(unsigned *lenp, void **bufp)
    {
    pub host.used_idx: unsigned used_idx =,
    pub desc: *mut vring_desc,
    pub head: unsigned,

    pub 1)]: head = ring.avail->ring[used_idx & (ring_size -,
    if ((used_idx ^ head ^ 0x8000) & ~(ring_size - 1))
    pub false: return,
// Barrier A (for pairing)
    pub 1: used_idx &= ring_size -,
    pub 1)]: desc = &ring.desc[head & (ring_size -,

    if (used_idx == ring.avail.idx)
    pub false: return,
// Barrier A (for pairing)
    pub 1: used_idx &= ring_size -,

    pub used_idx: head =,

    pub ring.avail->ring[used_idx]: head =,

    pub &ring.desc[head]: desc =,

// lenp = desc->len;
// bufp = (void *)(unsigned long)desc->addr;

    pub 1: desc->len = desc->len -,

// now update used ring
    pub head: ring.used->ring[used_idx].id =,
    pub 1: ring.used->ring[used_idx].len = desc->len -,

// Barrier B (for pairing)
    pub host.used_idx: ring.used->idx =,
    pub true: return,
    }
#[no_mangle]
pub unsafe extern "C" fn call_used() {
    void call_used(void)
    {
    pub need: bool,
// Flush in previous flags write
// Barrier D (for pairing)
    need = vring_need_event(vring_used_event(&ring),
    host.used_idx,
    pub host.used_idx: host.called_used_idx =,
    if (need)
    }
