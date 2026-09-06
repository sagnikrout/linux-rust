//! Automatically rewritten from C to Rust
//! Source: rust/helpers/scatterlist.c
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

#[no_mangle]
pub unsafe extern "C" fn rust_helper_sg_dma_address(sg: *mut scatterlist) -> __rust_helper dma_addr_t {
    __rust_helper dma_addr_t rust_helper_sg_dma_address(struct scatterlist *sg)
    {
    return sg_dma_address(sg);
    }
#[no_mangle]
pub unsafe extern "C" fn rust_helper_sg_dma_len(sg: *mut scatterlist) -> __rust_helper unsigned int {
    __rust_helper unsigned int rust_helper_sg_dma_len(struct scatterlist *sg)
    {
    return sg_dma_len(sg);
    }
    __rust_helper struct scatterlist *rust_helper_sg_next(struct scatterlist *sg)
    {
    return sg_next(sg);
    }
    __rust_helper void rust_helper_dma_unmap_sgtable(struct device *dev,
    struct sg_table *sgt,
    enum dma_data_direction dir,
    unsigned long attrs)
    {
    return dma_unmap_sgtable(dev, sgt, dir, attrs);
    }
