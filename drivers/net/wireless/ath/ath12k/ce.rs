//! Automatically rewritten from C Header to Rust Module
//! Source: drivers/net/wireless/ath/ath12k/ce.h
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


// SPDX-License-Identifier: BSD-3-Clause-Clear
//
// Copyright (c) 2018-2021 The Linux Foundation. All rights reserved.
// Copyright (c) Qualcomm Technologies, Inc. and/or its subsidiaries.
//
pub const CE_COUNT_MAX: c_int = 16;
// Byte swap data words
pub const CE_ATTR_BYTE_SWAP_DATA: c_int = 2;
// no interrupt on copy completion
pub const CE_ATTR_DIS_INTR: c_int = 8;
// Host software's Copy Engine configuration.
pub const CE_ATTR_FLAGS: c_int = 0;
// Threshold to poll for tx completion in case of Interrupt disabled CE's
pub const ATH12K_CE_USAGE_THRESHOLD: c_int = 32;
// Directions for interconnect pipe configuration.
// These definitions may be used during configuration and are shared
// between Host and Target.
//
// Pipe Directions are relative to the Host, so PIPEDIR_IN means
// "coming IN over air through Target to Host" as with a WiFi Rx operation.
// Conversely, PIPEDIR_OUT means "going OUT from Host through Target over air"
// as with a WiFi Tx operation. This is somewhat awkward for the "middle-man"
// Target since things that are "PIPEDIR_OUT" are coming IN to the Target
// over the interconnect.
//
pub const PIPEDIR_NONE: c_int = 0;

// IPQ5332 CE address/mask
pub const CE_HOST_IPQ5332_IE_ADDRESS: c_uint = 0x75804C;
pub const CE_HOST_IPQ5332_IE_2_ADDRESS: c_uint = 0x758050;

// IPQ5424 CE address/mask
pub const CE_HOST_IPQ5424_IE_ADDRESS: c_uint = 0x21804C;
pub const CE_HOST_IPQ5424_IE_2_ADDRESS: c_uint = 0x218050;

pub const CE_HOST_IE_3_SHIFT: c_uint = 0xC;

pub const ATH12K_CE_RX_POST_RETRY_JIFFIES: c_int = 50;
// Establish a mapping between a service/direction and a pipe.
// Configuration information for a Copy Engine pipe and services.
// Passed from Host to Target through QMI message and must be in
// little endian format.
//
#[repr(C)]
#[derive(Copy, Clone)]
pub struct service_to_pipe {
    pub service_id: __le32,
    pub pipedir: __le32,
    pub pipenum: __le32,
}

// Configuration information for a Copy Engine pipe.
// Passed from Host to Target through QMI message during startup (one per CE).
//
// NOTE: Structure is shared between Host software and Target firmware!
//
#[repr(C)]
#[derive(Copy, Clone)]
pub struct ce_pipe_config {
    pub pipenum: __le32,
    pub pipedir: __le32,
    pub nentries: __le32,
    pub nbytes_max: __le32,
    pub flags: __le32,
    pub reserved: __le32,
}

#[repr(C)]
#[derive(Copy, Clone)]
pub struct ce_ie_addr {
    pub ie1_reg_addr: u32,
    pub ie2_reg_addr: u32,
    pub ie3_reg_addr: u32,
}

#[repr(C)]
#[derive(Copy, Clone)]
pub struct ce_remap {
    pub base: u32,
    pub size: u32,
    pub cmem_offset: u32,
}

#[repr(C)]
#[derive(Copy, Clone)]
pub struct ce_attr {
// CE_ATTR_* values
    pub flags: c_uint,
// #entries in source ring - Must be a power of 2
    pub src_nentries: c_uint,
// Max source send size for this CE.
// This is also the minimum size of a destination buffer.
//
    pub src_sz_max: c_uint,
// #entries in destination ring - Must be a power of 2
    pub dest_nentries: c_uint,
    pub skb): *mut *mut *mut void (recv_cb)(struct ath12k_base ab, struct sk_buff,
}

pub const CE_DESC_RING_ALIGN: c_int = 8;
#[repr(C)]
#[derive(Copy, Clone)]
pub struct ath12k_ce_ring {
// Number of entries in this ring; must be power of 2
    pub nentries: c_uint,
    pub nentries_mask: c_uint,
// For dest ring, this is the next index to be processed
// by software after it was/is received into.
//
// For src ring, this is the last descriptor that was sent
// and completion processed by software.
//
// Regardless of src or dest ring, this is an invariant
// (modulo ring size):
// write index >= read index >= sw_index
//
    pub sw_index: c_uint,
// cached copy
    pub write_index: c_uint,
// Start of DMA-coherent area reserved for descriptors
// Host address space
    pub base_addr_owner_space_unaligned: *mut c_void,
// CE address space
    pub base_addr_ce_space_unaligned: dma_addr_t,
// Actual start of descriptors.
// Aligned to descriptor-size boundary.
// Points into reserved DMA-coherent area, above.
//
// Host address space
    pub base_addr_owner_space: *mut c_void,
// CE address space
    pub base_addr_ce_space: dma_addr_t,
// HAL ring id
    pub hal_ring_id: u32,
// keep last
    pub skb: [*mut sk_buff; ],
}

#[repr(C)]
#[derive(Copy, Clone)]
pub struct ath12k_ce_pipe {
    pub ab: *mut ath12k_base,
    pub pipe_num: u16,
    pub attr_flags: c_uint,
    pub buf_sz: c_uint,
    pub rx_buf_needed: c_uint,
    pub pipe): *mut *mut void (send_cb)(struct ath12k_ce_pipe,
    pub skb): *mut *mut *mut void (recv_cb)(struct ath12k_base ab, struct sk_buff,
    pub intr_wq: work_struct,
    pub src_ring: *mut ath12k_ce_ring,
    pub dest_ring: *mut ath12k_ce_ring,
    pub status_ring: *mut ath12k_ce_ring,
    pub timestamp: u64,
}

#[repr(C)]
#[derive(Copy, Clone)]
pub struct ath12k_ce {
    pub ce_pipe: [ath12k_ce_pipe; CE_COUNT_MAX],
// Protects rings of all ce pipes
    pub ce_lock: spinlock_t,
    pub hp_timer: [ath12k_hp_update_timer; CE_COUNT_MAX],
}

extern "C" {
    pub fn ath12k_ce_cleanup_pipes(ab: *mut ath12k_base);
}
extern "C" {
    pub fn ath12k_ce_rx_replenish_retry(t: *mut timer_list);
}
extern "C" {
    pub fn ath12k_ce_per_engine_service(ab: *mut ath12k_base, ce_id: u16);
}
extern "C" {
    pub fn ath12k_ce_rx_post_buf(ab: *mut ath12k_base);
}
extern "C" {
    pub fn ath12k_ce_init_pipes(ab: *mut ath12k_base) -> c_int;
}
extern "C" {
    pub fn ath12k_ce_alloc_pipes(ab: *mut ath12k_base) -> c_int;
}
extern "C" {
    pub fn ath12k_ce_free_pipes(ab: *mut ath12k_base);
}
extern "C" {
    pub fn ath12k_ce_get_attr_flags(ab: *mut ath12k_base, ce_id: c_int) -> c_int;
}
extern "C" {
    pub fn ath12k_ce_poll_send_completed(ab: *mut ath12k_base, pipe_id: u8);
}
