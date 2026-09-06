//! Automatically rewritten from C Header to Rust Module
//! Source: net/xdp/xsk.h
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
// Copyright(c) 2019 Intel Corporation.
#[repr(C)]
#[derive(Copy, Clone)]
pub struct xdp_ring_offset_v1 {
    pub producer: __u64,
    pub consumer: __u64,
    pub desc: __u64,
}

#[repr(C)]
#[derive(Copy, Clone)]
pub struct xdp_mmap_offsets_v1 {
    pub rx: xdp_ring_offset_v1,
    pub tx: xdp_ring_offset_v1,
    pub fr: xdp_ring_offset_v1,
    pub cr: xdp_ring_offset_v1,
}

// Nodes are linked in the struct xdp_sock map_list field, and used to
// track which maps a certain socket reside in.
//
#[repr(C)]
#[derive(Copy, Clone)]
pub struct xsk_map_node {
    pub node: list_head,
    pub map: *mut xsk_map,
    pub map_entry: *mut xdp_sock __rcu,
}

extern "C" {
    pub fn xsk_clear_pool_at_qid(dev: *mut net_device, queue_id: u16);
}
