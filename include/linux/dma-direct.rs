//! Automatically rewritten from C Header to Rust Module
//! Source: include/linux/dma-direct.h
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
//
// Internals of the DMA direct mapping implementation.  Only for use by the
// DMA mapping code and IOMMU drivers.
//
pub const _LINUX_DMA_DIRECT_H: c_int = 1;

//
// Record the mapping of CPU physical to DMA addresses for a given region.
//
#[repr(C)]
#[derive(Copy, Clone)]
pub struct bus_dma_region {
    pub cpu_start: phys_addr_t,
    pub dma_start: dma_addr_t,
    pub size: u64,
}

// make sure dma_capable fails when no translation is available

extern "C" {
    pub fn translate_phys_to_dma(_arg: dev, _arg: paddr) -> return;
}
extern "C" {
    pub fn dma_addr_unencrypted(_arg: __phys_to_dma(dev, _arg: paddr)) -> return;
}
extern "C" {
    pub fn dma_addr_encrypted(_arg: __phys_to_dma(dev, _arg: paddr)) -> return;
}
//
// If memory encryption is supported, phys_to_dma will set the memory encryption
// bit in the DMA address, and dma_to_phys will clear it.
// phys_to_dma_unencrypted is for use on special unencrypted memory like swiotlb
// buffers.
//
extern "C" {
    pub fn dma_addr_encrypted(_arg: __phys_to_dma(dev, _arg: paddr)) -> return;
}

extern "C" {
    pub fn force_dma_unencrypted(dev: *mut device) -> bool;
}

//
// The DMA address was derived from encrypted RAM, but this device
// requires unencrypted DMA addresses. Treat it as not DMA-capable
// so the caller can fall back to a suitable SWIOTLB pool.
//
extern "C" {
    pub fn dma_direct_get_required_mask(dev: *mut device) -> u64;
}
extern "C" {
    pub fn dma_direct_supported(dev: *mut device, mask: u64) -> c_int;
}
