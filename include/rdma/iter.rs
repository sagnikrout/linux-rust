//! Automatically rewritten from C Header to Rust Module
//! Source: include/rdma/iter.h
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


// SPDX-License-Identifier: GPL-2.0 OR Linux-OpenIB
// Copyright (c) 2026, NVIDIA CORPORATION & AFFILIATES.

//
// IB block DMA iterator
//
// Iterates the DMA-mapped SGL in contiguous memory blocks aligned
// to a HW supported page size.
//
#[repr(C)]
#[derive(Copy, Clone)]
pub struct ib_block_iter {
// internal states
    pub /: *mut *mut *mut scatterlist __sg; / sg holding the current aligned block,
    pub /: *mut *mut dma_addr_t __dma_addr; / unaligned DMA address of this block,
    pub /: *mut *mut size_t __sg_numblocks; / ib_umem_num_dma_blocks(),
    pub /: *mut *mut unsigned int __sg_nents; / number of SG entries,
    pub /: *mut *mut unsigned int __sg_advance; / number of bytes to advance in sg in next step,
    pub /: *mut *mut unsigned int __pg_bit; / alignment of current block,
}

extern "C" {
    pub fn __rdma_block_iter_next(biter: *mut ib_block_iter) -> bool;
}
//
// rdma_block_iter_dma_address - get the aligned dma address of the current
// block held by the block iterator.
// @biter: block iterator holding the memory block
//
// rdma_for_each_block - iterate over contiguous memory blocks of the sg list
// @sglist: sglist to iterate over
// @biter: block iterator holding the memory block
// @nents: maximum number of sg entries to iterate over
// @pgsz: best HW supported page size to use
//
// Callers may use rdma_block_iter_dma_address() to get each
// blocks aligned DMA address.
//

//
// rdma_umem_for_each_dma_block - iterate over contiguous DMA blocks of the umem
// @umem: umem to iterate over
// @pgsz: Page size to split the list into
//
// pgsz must be <= PAGE_SIZE or computed by ib_umem_find_best_pgsz(). The
// returned DMA blocks will be aligned to pgsz and span the range:
// ALIGN_DOWN(umem->address, pgsz) to ALIGN(umem->address + umem->length, pgsz)
//
// Performs exactly ib_umem_num_dma_blocks() iterations.
//

