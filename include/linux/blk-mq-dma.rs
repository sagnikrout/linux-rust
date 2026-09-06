//! Automatically rewritten from C Header to Rust Module
//! Source: include/linux/blk-mq-dma.h
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

#[repr(C)]
#[derive(Copy, Clone)]
pub struct blk_map_iter {
    pub iter: bvec_iter,
    pub bio: *mut bio,
    pub bvecs: *mut bio_vec,
    pub is_integrity: bool,
}

#[repr(C)]
#[derive(Copy, Clone)]
pub struct blk_dma_iter {
// Output address range for this iteration
    pub addr: dma_addr_t,
    pub len: u32,
    pub p2pdma: pci_p2pdma_map_state,
// Status code. Only valid when blk_rq_dma_map_iter_* returned false
    pub status: blk_status_t,
// Internal to blk_rq_dma_map_iter_*
    pub iter: blk_map_iter,
}

//
// blk_rq_dma_map_coalesce - were all segments coalesced?
// @state: DMA state to check
//
// Returns true if blk_rq_dma_map_iter_start coalesced all segments into a
// single DMA range.
//
extern "C" {
    pub fn dma_use_iova(_arg: state) -> return;
}
//
// blk_rq_dma_unmap - try to DMA unmap a request
// @req:	request to unmap
// @dma_dev:	device to unmap from
// @state:	DMA IOVA state
// @mapped_len: number of bytes to unmap
// @map:	peer-to-peer mapping type
//
// Returns %false if the callers need to manually unmap every DMA segment
// mapped using @iter or %true if no work is left to be done.
//
