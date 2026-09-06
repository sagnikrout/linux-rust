//! Automatically rewritten from C Header to Rust Module
//! Source: include/linux/dma-map-ops.h
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
// This header is for implementations of dma_map_ops and related code.
// It should not be included in drivers just using the DMA API.
//

#[repr(C)]
#[derive(Copy, Clone)]
pub struct dma_map_ops {
    pub attrs): c_ulong,
    pub attrs): dma_addr_t dma_handle, unsigned long,
    pub gfp): gfp_t,
    pub dir): dma_addr_t dma_handle, enum dma_data_direction,
    pub attrs): *mut *mut void , dma_addr_t, size_t, unsigned long,
    pub attrs): c_ulong,
    pub attrs): c_ulong,
    pub attrs): c_ulong,
//
// map_sg should return a negative error code on error. See
// dma_map_sgtable() for a list of appropriate error codes
// and their meanings.
//
    pub attrs): dma_data_direction dir, unsigned long,
    pub attrs): dma_data_direction dir, unsigned long,
    pub dir): size_t size, enum dma_data_direction,
    pub dir): dma_data_direction,
    pub dir): int nents, enum dma_data_direction,
    pub dir): int nents, enum dma_data_direction,
    pub direction): dma_data_direction,
    pub mask): *mut *mut *mut int (dma_supported)(struct device dev, u64,
    pub dev): *mut *mut u64 (get_required_mask)(struct device,
    pub dev): *mut *mut size_t (max_mapping_size)(struct device,
    pub (*opt_mapping_size)(void): *mut usize,
    pub dev): *mut *mut unsigned long (get_merge_boundary)(struct device,
}

extern "C" {
    pub fn get_arch_dma_ops() -> return;
}

extern "C" {
    pub fn dma_contiguous_reserve(addr_limit: phys_addr_t);
}
extern "C" {
    pub fn dma_free_contiguous(dev: *mut device, page: *mut page, size: usize);
}
extern "C" {
    pub fn dma_contiguous_early_fixup(base: phys_addr_t, size: c_ulong);
}

// Use fallback alloc() and free() when CONFIG_DMA_CMA=n

extern "C" {
    pub fn dma_release_coherent_memory(dev: *mut device);
}
extern "C" {
    pub fn dma_release_from_dev_coherent(dev: *mut device, order: c_int, vaddr: *mut c_void) -> c_int;
}

extern "C" {
    pub fn dma_release_from_global_coherent(order: c_int, vaddr: *mut c_void) -> c_int;
}
extern "C" {
    pub fn dma_init_global_coherent(phys_addr: phys_addr_t, size: usize) -> c_int;
}

extern "C" {
    pub fn dma_common_free_remap(cpu_addr: *mut c_void, size: usize);
}
extern "C" {
    pub fn dma_free_from_pool(dev: *mut device, start: *mut c_void, size: usize) -> bool;
}
extern "C" {
    pub fn dma_free_from_pool_page(dev: *mut device, page: *mut page, size: usize) -> bool;
}

extern "C" {
    pub fn dev_dma_coherent(_arg: dev) -> return;
}

// Reset it only once so that the function can be called on hotpath

//
// Check whether potential kmalloc() buffers are safe for non-coherent DMA.
//
// If DMA bouncing of kmalloc() buffers is disabled, the kmalloc()
// caches have already been aligned to a DMA-safe size.
//
// kmalloc() buffers are DMA-safe irrespective of size if the device
// is coherent or the direction is DMA_TO_DEVICE (non-desctructive
// cache maintenance and benign cache line evictions).
//
// Check whether the given size, assuming it is for a kmalloc()'ed buffer, is
// sufficiently aligned for non-coherent DMA.
//
// Larger kmalloc() sizes are guaranteed to be aligned to
// ARCH_DMA_MINALIGN.
//
// Check whether the given object size may have originated from a kmalloc()
// buffer with a slab alignment below the DMA-safe alignment and needs
// bouncing for non-coherent DMA. The pointer alignment is not considered and
// in-structure DMA-safe offsets are the responsibility of the caller. Such
// code should use the static ARCH_DMA_MINALIGN for compiler annotations.
//
// The heuristics can have false positives, bouncing unnecessarily, though the
// buffers would be small. False negatives are theoretically possible if, for
// example, multiple small kmalloc() buffers are coalesced into a larger
// buffer that passes the alignment check. There are no such known constructs
// in the kernel.
//

extern "C" {
    pub fn arch_dma_set_mask(dev: *mut device, mask: u64);
}

//
// Page protection so that devices that can't snoop CPU caches can use the
// memory coherently.  We default to pgprot_noncached which is usually used
// for ioremap as a safe bet, but architectures can override this with less
// strict semantics if possible.
//

extern "C" {
    pub fn dma_pgprot(dev: *mut device, prot: pgprot_t, attrs: c_ulong) -> pgprot_t;
}

extern "C" {
    pub fn arch_sync_dma_for_cpu_all();
}

extern "C" {
    pub fn arch_dma_prep_coherent(page: *mut page, size: usize);
}

extern "C" {
    pub fn arch_dma_clear_uncached(addr: *mut c_void, size: usize);
}

extern "C" {
    pub fn arch_dma_map_phys_direct(dev: *mut device, addr: phys_addr_t) -> bool;
}
extern "C" {
    pub fn arch_dma_unmap_phys_direct(dev: *mut device, dma_handle: dma_addr_t) -> bool;
}
extern "C" {
    pub fn arch_dma_alloc_direct(dev: *mut device) -> bool;
}
extern "C" {
    pub fn arch_dma_free_direct(dev: *mut device, dma_handle: dma_addr_t) -> bool;
}

extern "C" {
    pub fn arch_setup_dma_ops(dev: *mut device, coherent: bool);
}

extern "C" {
    pub fn arch_teardown_dma_ops(dev: *mut device);
}

extern "C" {
    pub fn dma_debug_add_bus(bus: *const bus_type);
}
extern "C" {
    pub fn debug_dma_dump_mappings(dev: *mut device);
}

