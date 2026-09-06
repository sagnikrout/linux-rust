//! Automatically rewritten from C to Rust
//! Source: tools/virtio/ringtest/ring.c
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
// Simple descriptor-based ring. virtio 0.9 compatible event index is used for
// signalling, unconditionally.
//
// Macro flag: #define _GNU_SOURCE

// Next - Where next entry will be written.
// Prev - "Next" value when event triggered previously.
// Event - Peer requested event after writing this entry.
//
    static inline bool need_event(unsigned short event,
    unsigned short next,
    unsigned short prev)
    {
    return (unsigned short)(next - event - 1) < (unsigned short)(next - prev);
    }
// Design:
// Guest adds descriptors with unique index values and DESC_HW in flags.
// Host overwrites used descriptors with correct len, index, and DESC_HW clear.
// Flags are always set last.
//
pub const DESC_HW: c_uint = 0x1;
#[repr(C)]
#[derive(Copy, Clone)]
pub struct desc {
    pub flags: c_ushort,
    pub index: c_ushort,
    pub len: unsigned,
    pub addr: c_ulonglong,
}

// how much padding is needed to avoid false cache sharing
pub const HOST_GUEST_PADDING: c_uint = 0x80;
// Mostly read
#[repr(C)]
#[derive(Copy, Clone)]
pub struct event {
    pub kick_index: c_ushort,
    pub 2]: unsigned char reserved0[HOST_GUEST_PADDING -,
    pub call_index: c_ushort,
    pub 2]: unsigned char reserved1[HOST_GUEST_PADDING -,
}

#[repr(C)]
#[derive(Copy, Clone)]
pub struct data {
    pub /: *mut *mut *mut void buf; / descriptor is writeable, we can't get buf from there,
    pub data: *mut c_void,
    pub data: *mut },
    pub ring: *mut desc,
    pub event: *mut event,
#[repr(C)]
#[derive(Copy, Clone)]
pub struct guest {
    pub avail_idx: unsigned,
    pub last_used_idx: unsigned,
    pub num_free: unsigned,
    pub kicked_avail_idx: unsigned,
    pub 12]: unsigned char reserved[HOST_GUEST_PADDING -,
    pub guest: },
#[repr(C)]
#[derive(Copy, Clone)]
pub struct host {
// we do not need to track last avail index
// unless we have more than one in flight.
//
    pub used_idx: unsigned,
    pub called_used_idx: unsigned,
    pub 4]: unsigned char reserved[HOST_GUEST_PADDING -,
    pub host: },
// implemented by ring
#[no_mangle]
pub unsafe extern "C" fn alloc_ring() {
    void alloc_ring(void)
    {
    pub ret: c_int,
    pub i: c_int,
    pub ring): *mut *mut *mut *mut ret = posix_memalign((void )&ring, 0x1000, ring_size  sizeof,
    if (ret) {
    pub buffer.\n"): perror("Unable to allocate ring,
    }
    pub sizeof(*event)): *mut event = calloc(1,,
    if (!event) {
    pub buffer.\n"): perror("Unable to allocate event,
    }
    pub 0: guest.avail_idx =,
    pub -1: guest.kicked_avail_idx =,
    pub 0: guest.last_used_idx =,
    pub 0: host.used_idx =,
    pub -1: host.called_used_idx =,
    pub {: for (i = 0; i < ring_size; ++i),
    struct desc desc = {
    .index = i,
}

    ring[i] = desc;
    }
    guest.num_free = ring_size;
    data = calloc(ring_size, sizeof(*data));
    if (!data) {
    perror("Unable to allocate data buffer.\n");
    exit(3);
    }
    }
// guest side
#[no_mangle]
pub unsafe extern "C" fn add_inbuf(len: unsigned, buf: *mut c_void, datap: *mut c_void) -> c_int {
    int add_inbuf(unsigned len, void *buf, void *datap)
    {
    unsigned head, index;
    if (!guest.num_free)
    return -1;
    guest.num_free--;
    head = (ring_size - 1) & (guest.avail_idx++);
// Start with a write. On MESI architectures this helps
// avoid a shared state with consumer that is polling this descriptor.
//
    ring[head].addr = (unsigned long)(void*)buf;
    ring[head].len = len;
// read below might bypass write above. That is OK because it's just an
// optimization. If this happens, we will get the cache line in a
// shared state which is unfortunate, but probably not worth it to
// add an explicit full barrier to avoid this.
//
    barrier();
    index = ring[head].index;
    data[index].buf = buf;
    data[index].data = datap;
// Barrier A (for pairing)
    smp_release();
    ring[head].flags = DESC_HW;
    return 0;
    }
    void *get_buf(unsigned *lenp, void **bufp)
    {
    let mut head: unsigned = (ring_size - 1) & guest.last_used_idx;
    unsigned index;
    void *datap;
    if (ring[head].flags & DESC_HW)
    return core::ptr::null_mut();
// Barrier B (for pairing)
    smp_acquire();
// lenp = ring[head].len;
    index = ring[head].index & (ring_size - 1);
    datap = data[index].data;
// bufp = data[index].buf;
    data[index].buf = core::ptr::null_mut();
    data[index].data = core::ptr::null_mut();
    guest.num_free++;
    guest.last_used_idx++;
    return datap;
    }
#[no_mangle]
pub unsafe extern "C" fn used_empty() -> bool {
    bool used_empty()
    {
    let mut head: unsigned = (ring_size - 1) & guest.last_used_idx;
    return (ring[head].flags & DESC_HW);
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
    event.call_index = guest.last_used_idx;
// Flush call index write
// Barrier D (for pairing)
    smp_mb();
    return used_empty();
    }
#[no_mangle]
pub unsafe extern "C" fn kick_available() {
    void kick_available(void)
    {
    bool need;
// Flush in previous flags write
// Barrier C (for pairing)
    smp_mb();
    need = need_event(event.kick_index,
    guest.avail_idx,
    guest.kicked_avail_idx);
    guest.kicked_avail_idx = guest.avail_idx;
    if (need)
    kick();
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
    event.kick_index = host.used_idx;
// Barrier C (for pairing)
    smp_mb();
    return avail_empty();
    }
#[no_mangle]
pub unsafe extern "C" fn avail_empty() -> bool {
    bool avail_empty()
    {
    let mut head: unsigned = (ring_size - 1) & host.used_idx;
    return !(ring[head].flags & DESC_HW);
    }
#[no_mangle]
pub unsafe extern "C" fn use_buf(lenp: *mut unsigned, bufp: *mut c_void) -> bool {
    bool use_buf(unsigned *lenp, void **bufp)
    {
    let mut head: unsigned = (ring_size - 1) & host.used_idx;
    if (!(ring[head].flags & DESC_HW))
    return false;
// make sure length read below is not speculated
// Barrier A (for pairing)
    smp_acquire();
// simple in-order completion: we don't need
// to touch index at all. This also means we
// can just modify the descriptor in-place.
//
    ring[head].len--;
// Make sure len is valid before flags.
// Note: alternative is to write len and flags in one access -
// possible on 64 bit architectures but wmb is free on Intel anyway
// so I have no way to test whether it's a gain.
//
// Barrier B (for pairing)
    smp_release();
    ring[head].flags = 0;
    host.used_idx++;
    return true;
    }
#[no_mangle]
pub unsafe extern "C" fn call_used() {
    void call_used(void)
    {
    bool need;
// Flush in previous flags write
// Barrier D (for pairing)
    smp_mb();
    need = need_event(event.call_index,
    host.used_idx,
    host.called_used_idx);
    host.called_used_idx = host.used_idx;
    if (need)
    call();
    }
