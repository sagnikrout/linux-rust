//! Automatically rewritten from C to Rust
//! Source: drivers/infiniband/hw/mlx5/mem.c
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


//
// Copyright (c) 2013-2015, Mellanox Technologies. All rights reserved.
//
// This software is available to you under a choice of one of two
// licenses.  You may choose to be licensed under the terms of the GNU
// General Public License (GPL) Version 2, available from the file
// COPYING in the main directory of this source tree, or the
// OpenIB.org BSD license below:
//
// Redistribution and use in source and binary forms, with or
// without modification, are permitted provided that the following
// conditions are met:
//
// - Redistributions of source code must retain the above
// copyright notice, this list of conditions and the following
// disclaimer.
//
// - Redistributions in binary form must reproduce the above
// copyright notice, this list of conditions and the following
// disclaimer in the documentation and/or other materials
// provided with the distribution.
//
// THE SOFTWARE IS PROVIDED "AS IS", WITHOUT WARRANTY OF ANY KIND,
// EXPRESS OR IMPLIED, INCLUDING BUT NOT LIMITED TO THE WARRANTIES OF
// MERCHANTABILITY, FITNESS FOR A PARTICULAR PURPOSE AND
// NONINFRINGEMENT. IN NO EVENT SHALL THE AUTHORS OR COPYRIGHT HOLDERS
// BE LIABLE FOR ANY CLAIM, DAMAGES OR OTHER LIABILITY, WHETHER IN AN
// ACTION OF CONTRACT, TORT OR OTHERWISE, ARISING FROM, OUT OF OR IN
// CONNECTION WITH THE SOFTWARE OR THE USE OR OTHER DEALINGS IN THE
// SOFTWARE.
//

//
// Fill in a physical address list. ib_umem_num_dma_blocks() entries will be
// filled in the pas array.
//
    void mlx5_ib_populate_pas(struct ib_umem *umem, size_t page_size, __be64 *pas,
    u64 access_flags)
    {
    struct ib_block_iter biter;
    rdma_umem_for_each_dma_block (umem, &biter, page_size) {
// pas = cpu_to_be64(rdma_block_iter_dma_address(&biter) |
    access_flags);
    pas++;
    }
    }
//
// Compute the page shift and page_offset for mailboxes that use a quantized
// page_offset. The granulatity of the page offset scales according to page
// size.
//
    unsigned long __mlx5_umem_find_best_quantized_pgoff(
    struct ib_umem *umem, unsigned long pgsz_bitmap,
    unsigned int page_offset_bits, u64 pgoff_bitmask, unsigned int scale,
    unsigned int *page_offset_quantized)
    {
    let mut page_offset_mask: u64 = (1UL << page_offset_bits) - 1;
    unsigned long page_size;
    u64 page_offset;
    page_size = ib_umem_find_best_pgoff(umem, pgsz_bitmap, pgoff_bitmask);
    if (!page_size)
    return 0;
//
// page size is the largest possible page size.
//
// Reduce the page_size, and thus the page_offset and quanta, until the
// page_offset fits into the mailbox field. Once page_size < scale this
// loop is guaranteed to terminate.
//
    page_offset = ib_umem_dma_offset(umem, page_size);
    while (page_offset & ~(u64)(page_offset_mask * (page_size / scale))) {
    page_size /= 2;
    page_offset = ib_umem_dma_offset(umem, page_size);
    }
//
// The address is not aligned, or otherwise cannot be represented by the
// page_offset.
//
    if (!(pgsz_bitmap & page_size))
    return 0;
// page_offset_quantized =
    (unsigned long)page_offset / (page_size / scale);
    if (WARN_ON(*page_offset_quantized > page_offset_mask))
    return 0;
    return page_size;
    }
