//! Automatically rewritten from C Header to Rust Module
//! Source: include/linux/hmm-dma.h
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
// Copyright (c) 2024 NVIDIA Corporation & Affiliates

//
// struct hmm_dma_map - array of PFNs and DMA addresses
//
// @state: DMA IOVA state
// @pfns: array of PFNs
// @dma_list: array of DMA addresses
// @dma_entry_size: size of each DMA entry in the array
//
#[repr(C)]
#[derive(Copy, Clone)]
pub struct hmm_dma_map {
    pub state: dma_iova_state,
    pub pfn_list: *mut c_ulong,
    pub dma_list: *mut dma_addr_t,
    pub dma_entry_size: usize,
}

extern "C" {
    pub fn hmm_dma_map_free(dev: *mut device, map: *mut hmm_dma_map);
}
extern "C" {
    pub fn hmm_dma_unmap_pfn(dev: *mut device, map: *mut hmm_dma_map, idx: usize) -> bool;
}
