//! Automatically rewritten from C Header to Rust Module
//! Source: tools/testing/selftests/bpf/xsk.h
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


// SPDX-License-Identifier: (LGPL-2.1 OR BSD-2-Clause)
//
// AF_XDP user-space access library.
//
// Copyright (c) 2018 - 2019 Intel Corporation.
// Copyright (c) 2019 Facebook
//
// Author(s): Magnus Karlsson <magnus.karlsson@intel.com>
//

// Do not access these members directly. Use the functions below.

#[repr(C)]
#[derive(Copy, Clone)]
pub struct name {
    pub \: __u32 cached_prod;,
    pub \: __u32 cached_cons;,
    pub \: __u32 mask;,
    pub \: __u32 size;,
    pub \: *mut *mut __u32 producer;,
    pub \: *mut *mut __u32 consumer;,
    pub \: *mut *mut void ring;,
    pub \: *mut *mut __u32 flags;,
// For a detailed explanation on the memory barriers associated with the
// ring, please take a look at net/xdp/xsk_queue.h.
//
    pub xsk_umem: struct,
    pub xsk_socket: struct,
    pub )fill->ring: *mut *mut __u64 addrs = (__u64,
    pub fill->mask]: return &addrs[idx &,
    pub )comp->ring: *const *const __u64 addrs = (__u64,
    pub comp->mask]: return &addrs[idx &,
    pub )tx->ring: *mut *mut xdp_desc descs = (xdp_desc,
    pub tx->mask]: return &descs[idx &,
    pub )rx->ring: *const *const xdp_desc descs = (xdp_desc,
    pub rx->mask]: return &descs[idx &,
    pub XDP_RING_NEED_WAKEUP: *mut *mut return r->flags &,
    pub r->cached_prod: __u32 free_entries = r->cached_cons -,
    pub free_entries: return,
// Refresh the local tail pointer.
// cached_cons is r->size bigger than the real consumer pointer so
// that this addition can be avoided in the more frequently
// executed code that computes free_entries in the beginning of
// this function. Without this optimization it would have been
// free_entries = r->cached_prod - r->cached_cons + r->size.
//
    pub __ATOMIC_ACQUIRE): r->cached_cons = __atomic_load_n(r->consumer,,
    pub r->size: r->cached_cons +=,
    pub r->cached_prod: return r->cached_cons -,
    pub r->cached_cons: __u32 entries = r->cached_prod -,
    pub __ATOMIC_ACQUIRE): r->cached_prod = __atomic_load_n(r->producer,,
    pub r->cached_cons: entries = r->cached_prod -,
    pub entries: return (entries > nb) ? nb :,
    pub 0: return,
// idx = prod->cached_prod;
    pub nb: prod->cached_prod +=,
    pub nb: return,
// Make sure everything has been written to the ring before indicating
// this to the kernel by writing the producer pointer.
//
    pub __ATOMIC_RELEASE): *mut *mut __atomic_store_n(prod->producer, prod->producer + nb,,
    pub nb: prod->cached_prod -=,
    pub nb): __u32 entries = xsk_cons_nb_avail(cons,,
// idx = cons->cached_cons;
    pub entries: cons->cached_cons +=,
    pub entries: return,
    pub nb: cons->cached_cons -=,
// Make sure data has been read before indicating we are done
// with the entries by updating the consumer pointer.
//
    pub __ATOMIC_RELEASE): *mut *mut __atomic_store_n(cons->consumer, cons->consumer + nb,,
    pub )umem_area)[addr]: *mut return &((char,
    pub XSK_UNALIGNED_BUF_ADDR_MASK: return addr &,
    pub XSK_UNALIGNED_BUF_OFFSET_SHIFT: return addr >>,
    pub xsk_umem__extract_offset(addr): return xsk_umem__extract_addr(addr) +,
    pub umem): *const int xsk_umem__fd(struct xsk_umem,
    pub xsk): *const int xsk_socket__fd(struct xsk_socket,
pub const XSK_RING_CONS__DEFAULT_NUM_DESCS: c_int = 2048;
pub const XSK_RING_PROD__DEFAULT_NUM_DESCS: c_int = 2048;

pub const XSK_UMEM__DEFAULT_FRAME_HEADROOM: c_int = 0;
pub const XSK_UMEM__DEFAULT_FLAGS: c_int = 0;
#[repr(C)]
#[derive(Copy, Clone)]
pub struct xsk_umem_config {
    pub fill_size: __u32,
    pub comp_size: __u32,
    pub frame_size: __u32,
    pub frame_headroom: __u32,
    pub flags: __u32,
    pub tx_metadata_len: __u32,
}

extern "C" {
    pub fn xsk_attach_xdp_program(prog: *mut bpf_program, ifindex: c_int, xdp_flags: u32) -> c_int;
}
extern "C" {
    pub fn xsk_detach_xdp_program(ifindex: c_int, xdp_flags: u32);
}
extern "C" {
    pub fn xsk_update_xskmap(map: *mut bpf_map, xsk: *mut xsk_socket, index: u32) -> c_int;
}
extern "C" {
    pub fn xsk_clear_xskmap(map: *mut bpf_map);
}
extern "C" {
    pub fn xsk_is_in_mode(ifindex: u32, mode: c_int) -> bool;
}
#[repr(C)]
#[derive(Copy, Clone)]
pub struct xsk_socket_config {
    pub rx_size: __u32,
    pub tx_size: __u32,
    pub bind_flags: __u16,
}

// Set config to NULL to get the default configuration.
// Returns 0 for success and -EBUSY if the umem is still in use.
extern "C" {
    pub fn xsk_umem__delete(umem: *mut xsk_umem) -> c_int;
}
extern "C" {
    pub fn xsk_socket__delete(xsk: *mut xsk_socket);
}
extern "C" {
    pub fn xsk_set_mtu(ifindex: c_int, mtu: c_int) -> c_int;
}

