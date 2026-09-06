//! Automatically rewritten from C Header to Rust Module
//! Source: include/net/tso.h
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

pub const TSO_HEADER_SIZE: c_int = 256;
#[repr(C)]
#[derive(Copy, Clone)]
pub struct tso_t {
    pub next_frag_idx: c_int,
    pub size: c_int,
    pub data: *mut c_void,
    pub ip_id: u16,
    pub /: *mut *mut u8 tlen; / transport header len,
    pub ipv6: bool,
    pub tcp_seq: u32,
}

// Calculate the worst case buffer count
extern "C" {
    pub fn tso_build_data(skb: *const sk_buff, tso: *mut tso_t, size: c_int);
}
extern "C" {
    pub fn tso_start(skb: *mut sk_buff, tso: *mut tso_t) -> c_int;
}
//
// struct tso_dma_map - DMA mapping state for GSO payload
// @dev: device used for DMA mapping
// @skb: the GSO skb being mapped
// @hdr_len: per-segment header length
// @iova_state: DMA IOVA state (when IOMMU available)
// @iova_offset: global byte offset into IOVA range (IOVA path only)
// @total_len: total payload length
// @frag_idx: current region (-1 = linear, 0..nr_frags-1 = frag)
// @offset: byte offset within current region
// @linear_dma: DMA address of the linear payload
// @linear_len: length of the linear payload
// @nr_frags: number of frags successfully DMA-mapped
// @frags: per-frag DMA address and length
//
// DMA-maps the payload regions of a GSO skb (linear data + frags).
// Prefers the DMA IOVA API for a single contiguous mapping with one
// IOTLB sync; falls back to per-region dma_map_phys() otherwise.
//
#[repr(C)]
#[derive(Copy, Clone)]
pub struct tso_dma_map {
    pub dev: *mut device,
    pub skb: *const sk_buff,
    pub hdr_len: c_uint,
// IOVA path
    pub iova_state: dma_iova_state,
    pub iova_offset: usize,
    pub total_len: usize,
// Fallback path if IOVA path fails
    pub frag_idx: c_int,
    pub offset: c_uint,
    pub linear_dma: dma_addr_t,
    pub linear_len: c_uint,
    pub nr_frags: c_uint,
    pub dma: dma_addr_t,
    pub len: c_uint,
    pub frags: [}; MAX_SKB_FRAGS],
}

//
// struct tso_dma_map_completion_state - Completion-time cleanup state
// @iova_state: DMA IOVA state (when IOMMU available)
// @total_len: total payload length of the IOVA mapping
//
// Drivers store this on their SW ring at xmit time via
// tso_dma_map_completion_save(), then call tso_dma_map_complete() at
// completion time.
//
#[repr(C)]
#[derive(Copy, Clone)]
pub struct tso_dma_map_completion_state {
    pub iova_state: dma_iova_state,
    pub total_len: usize,
}

extern "C" {
    pub fn tso_dma_map_cleanup(map: *mut tso_dma_map);
}
extern "C" {
    pub fn tso_dma_map_count(map: *mut tso_dma_map, len: c_uint) -> c_uint;
}
//
// tso_dma_map_completion_save - save state needed for completion-time cleanup
// @map: the xmit-time DMA map
// @cstate: driver-owned storage that persists until completion
//
// Should be called at xmit time to update the completion state and later passed
// to tso_dma_map_complete().
//
// tso_dma_map_complete - tear down mapping at completion time
// @dev: the device that owns the mapping
// @cstate: state saved by tso_dma_map_completion_save()
//
// Return: true if the IOVA path was used and the mapping has been
// destroyed; false if the fallback per-region path was used and the
// driver must unmap via its normal completion path.
//
