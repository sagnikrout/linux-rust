//! Automatically rewritten from C Header to Rust Module
//! Source: drivers/net/ethernet/alibaba/eea/eea_ring.h
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
// Driver for Alibaba Elastic Ethernet Adapter.
//
// Copyright (C) 2025 Alibaba Inc.
//

// These two values define the bounds for the queue depth returned by the
// hardware.
//

pub const EEA_NET_IO_HW_RING_DEPTH_MIN: c_int = 128;
// This value constrains the minimum queue depth that the driver configures for
// the hardware, which typically applies to user-provided settings. Naturally,
// the configured depth must also not exceed the maximum capacity supported by
// the hardware.
//
pub const EEA_NET_IO_RING_DEPTH_MIN: c_int = 64;
#[repr(C)]
#[derive(Copy, Clone)]
pub struct eea_common_desc {
    pub flags: __le16,
    pub id: __le16,
}

#[repr(C)]
#[derive(Copy, Clone)]
pub struct eea_ring_sq {
    pub desc: *mut c_void,
    pub head: u16,
    pub hw_idx: u16,
    pub shadow_idx: u16,
    pub shadow_id: __le16,
    pub shadow_num: u16,
    pub desc_size: u8,
    pub desc_size_shift: u8,
    pub dma_addr: dma_addr_t,
    pub dma_size: u32,
}

#[repr(C)]
#[derive(Copy, Clone)]
pub struct eea_ring_cq {
    pub desc: *mut c_void,
    pub head: u16,
    pub hw_idx: u16,
    pub phase: u8,
    pub desc_size_shift: u8,
    pub desc_size: u8,
    pub dma_addr: dma_addr_t,
    pub dma_size: u32,
}

#[repr(C)]
#[derive(Copy, Clone)]
pub struct eea_ring {
    pub name: *const c_char,
    pub edev: *mut eea_device,
    pub index: u32,
    pub db: *mut void __iomem,
    pub msix_vec: u16,
    pub num: u32,
    pub num_free: u32,
    pub sq: eea_ring_sq,
    pub cq: eea_ring_cq,
}

extern "C" {
    pub fn eea_ering_free(ering: *mut eea_ring);
}
extern "C" {
    pub fn eea_ering_kick(ering: *mut eea_ring);
}
extern "C" {
    pub fn eea_ering_sq_commit_desc(ering: *mut eea_ring);
}
extern "C" {
    pub fn eea_ering_sq_cancel(ering: *mut eea_ring);
}
extern "C" {
    pub fn eea_ering_cq_ack_desc(ering: *mut eea_ring, num: u32);
}
extern "C" {
    pub fn eea_ering_irq_active(ering: *mut eea_ring, tx_ering: *mut eea_ring);
}
