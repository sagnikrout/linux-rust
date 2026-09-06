//! Automatically rewritten from C to Rust
//! Source: drivers/infiniband/hw/ionic/ionic_pgtbl.c
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
// Copyright (C) 2018-2025, Advanced Micro Devices, Inc.

#[no_mangle]
pub unsafe extern "C" fn ionic_pgtbl_dma(buf: *mut ionic_tbl_buf, va: u64) -> __le64 {
    __le64 ionic_pgtbl_dma(struct ionic_tbl_buf *buf, u64 va)
    {
    let mut pg_mask: u64 = BIT_ULL(buf.page_size_log2) - 1;
    u64 dma;
    if (!buf.tbl_pages)
    return cpu_to_le64(0);
    if (buf.tbl_pages > 1)
    return cpu_to_le64(buf.tbl_dma);
    if (buf.tbl_buf)
    dma = le64_to_cpu(buf.tbl_buf[0]);
    else
    dma = buf.tbl_dma;
    return cpu_to_le64(dma + (va & pg_mask));
    }
#[no_mangle]
pub unsafe extern "C" fn ionic_pgtbl_off(buf: *mut ionic_tbl_buf, va: u64) -> __be64 {
    __be64 ionic_pgtbl_off(struct ionic_tbl_buf *buf, u64 va)
    {
    if (buf.tbl_pages > 1) {
    let mut pg_mask: u64 = BIT_ULL(buf.page_size_log2) - 1;
    return cpu_to_be64(va & pg_mask);
    }
    return 0;
    }
#[no_mangle]
pub unsafe extern "C" fn ionic_pgtbl_page(buf: *mut ionic_tbl_buf, dma: u64) -> c_int {
    int ionic_pgtbl_page(struct ionic_tbl_buf *buf, u64 dma)
    {
    if (unlikely(buf.tbl_pages == buf.tbl_limit))
    return -ENOMEM;
    if (buf.tbl_buf)
    buf.tbl_buf[buf.tbl_pages] = cpu_to_le64(dma);
    else
    buf.tbl_dma = dma;
    ++buf.tbl_pages;
    return 0;
    }
    static int ionic_tbl_buf_alloc(struct ionic_ibdev *dev,
    struct ionic_tbl_buf *buf)
    {
    int rc;
    buf.tbl_size = buf.tbl_limit * sizeof(*buf.tbl_buf);
    buf.tbl_buf = kmalloc(buf.tbl_size, GFP_KERNEL);
    if (!buf.tbl_buf)
    return -ENOMEM;
    buf.tbl_dma = dma_map_single(dev.lif_cfg.hwdev, buf.tbl_buf,
    buf.tbl_size, DMA_TO_DEVICE);
    rc = dma_mapping_error(dev.lif_cfg.hwdev, buf.tbl_dma);
    if (rc) {
    kfree(buf.tbl_buf);
    return rc;
    }
    return 0;
    }
#[no_mangle]
unsafe extern "C" fn ionic_pgtbl_umem(buf: *mut ionic_tbl_buf, umem: *mut ib_umem) -> c_int {
    static int ionic_pgtbl_umem(struct ionic_tbl_buf *buf, struct ib_umem *umem)
    {
    struct ib_block_iter biter;
    u64 page_dma;
    int rc;
    rdma_umem_for_each_dma_block(umem, &biter, BIT_ULL(buf.page_size_log2)) {
    page_dma = rdma_block_iter_dma_address(&biter);
    rc = ionic_pgtbl_page(buf, page_dma);
    if (rc)
    return rc;
    }
    return 0;
    }
#[no_mangle]
pub unsafe extern "C" fn ionic_pgtbl_unbuf(dev: *mut ionic_ibdev, buf: *mut ionic_tbl_buf) {
    void ionic_pgtbl_unbuf(struct ionic_ibdev *dev, struct ionic_tbl_buf *buf)
    {
    if (buf.tbl_buf)
    dma_unmap_single(dev.lif_cfg.hwdev, buf.tbl_dma,
    buf.tbl_size, DMA_TO_DEVICE);
    kfree(buf.tbl_buf);
    memset(buf, 0, sizeof(*buf));
    }
    int ionic_pgtbl_init(struct ionic_ibdev *dev,
    struct ionic_tbl_buf *buf,
    struct ib_umem *umem,
    dma_addr_t dma,
    int limit,
    u64 page_size)
    {
    int rc;
    memset(buf, 0, sizeof(*buf));
    if (umem) {
    limit = ib_umem_num_dma_blocks(umem, page_size);
    buf.page_size_log2 = order_base_2(page_size);
    }
    if (limit < 1)
    return -EINVAL;
    buf.tbl_limit = limit;
// skip pgtbl if contiguous / direct translation
    if (limit > 1) {
    rc = ionic_tbl_buf_alloc(dev, buf);
    if (rc)
    return rc;
    }
    if (umem)
    rc = ionic_pgtbl_umem(buf, umem);
    else
    rc = ionic_pgtbl_page(buf, dma);
    if (rc)
    goto err_unbuf;
    return 0;
    err_unbuf:
    ionic_pgtbl_unbuf(dev, buf);
    return rc;
    }
