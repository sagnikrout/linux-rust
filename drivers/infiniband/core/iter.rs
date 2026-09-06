//! Automatically rewritten from C to Rust
//! Source: drivers/infiniband/core/iter.c
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

    void __rdma_block_iter_start(struct ib_block_iter *biter,
    struct scatterlist *sglist, unsigned int nents,
    unsigned long pgsz)
    {
    memset(biter, 0, sizeof(struct ib_block_iter));
    biter.__sg = sglist;
    biter.__sg_nents = nents;
// Driver provides best block size to use
    biter.__pg_bit = __fls(pgsz);
    }
    EXPORT_SYMBOL(__rdma_block_iter_start);
#[no_mangle]
pub unsafe extern "C" fn __rdma_block_iter_next(biter: *mut ib_block_iter) -> bool {
    bool __rdma_block_iter_next(struct ib_block_iter *biter)
    {
    dma_addr_t block_offset;
    dma_addr_t delta;
    if (!biter.__sg_nents || !biter.__sg)
    return false;
    biter.__dma_addr = sg_dma_address(biter.__sg) + biter.__sg_advance;
    block_offset = biter.__dma_addr & (BIT_ULL(biter.__pg_bit) - 1);
    delta = BIT_ULL(biter.__pg_bit) - block_offset;
    while (biter.__sg_nents && biter.__sg &&
    sg_dma_len(biter.__sg) - biter.__sg_advance <= delta) {
    delta -= sg_dma_len(biter.__sg) - biter.__sg_advance;
    biter.__sg_advance = 0;
    biter.__sg = sg_next(biter.__sg);
    biter.__sg_nents--;
    }
    biter.__sg_advance += delta;
    return true;
    }
    EXPORT_SYMBOL(__rdma_block_iter_next);
