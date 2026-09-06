//! Automatically rewritten from C to Rust
//! Source: rust/helpers/dma.c
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

    __rust_helper void *rust_helper_dma_alloc_attrs(struct device *dev, size_t size,
    dma_addr_t *dma_handle,
    gfp_t flag, unsigned long attrs)
    {
    return dma_alloc_attrs(dev, size, dma_handle, flag, attrs);
    }
    __rust_helper void rust_helper_dma_free_attrs(struct device *dev, size_t size,
    void *cpu_addr,
    dma_addr_t dma_handle,
    unsigned long attrs)
    {
    dma_free_attrs(dev, size, cpu_addr, dma_handle, attrs);
    }
    __rust_helper int rust_helper_dma_set_mask_and_coherent(struct device *dev,
    u64 mask)
    {
    return dma_set_mask_and_coherent(dev, mask);
    }
#[no_mangle]
pub unsafe extern "C" fn rust_helper_dma_set_mask(dev: *mut device, mask: u64) -> __rust_helper int {
    __rust_helper int rust_helper_dma_set_mask(struct device *dev, u64 mask)
    {
    return dma_set_mask(dev, mask);
    }
#[no_mangle]
pub unsafe extern "C" fn rust_helper_dma_set_coherent_mask(dev: *mut device, mask: u64) -> __rust_helper int {
    __rust_helper int rust_helper_dma_set_coherent_mask(struct device *dev, u64 mask)
    {
    return dma_set_coherent_mask(dev, mask);
    }
    __rust_helper int rust_helper_dma_map_sgtable(struct device *dev, struct sg_table *sgt,
    enum dma_data_direction dir, unsigned long attrs)
    {
    return dma_map_sgtable(dev, sgt, dir, attrs);
    }
#[no_mangle]
pub unsafe extern "C" fn rust_helper_dma_max_mapping_size(dev: *mut device) -> __rust_helper size_t {
    __rust_helper size_t rust_helper_dma_max_mapping_size(struct device *dev)
    {
    return dma_max_mapping_size(dev);
    }
    __rust_helper void rust_helper_dma_set_max_seg_size(struct device *dev,
    unsigned int size)
    {
    dma_set_max_seg_size(dev, size);
    }
