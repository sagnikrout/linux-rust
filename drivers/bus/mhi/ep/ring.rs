//! Automatically rewritten from C to Rust
//! Source: drivers/bus/mhi/ep/ring.c
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
// Copyright (C) 2022 Linaro Ltd.
// Author: Manivannan Sadhasivam <manivannan.sadhasivam@linaro.org>
//

#[no_mangle]
pub unsafe extern "C" fn mhi_ep_ring_addr2offset(ring: *mut mhi_ep_ring, ptr: u64) -> usize {
    size_t mhi_ep_ring_addr2offset(struct mhi_ep_ring *ring, u64 ptr)
    {
    return (ptr - ring.rbase) / sizeof(struct mhi_ring_element);
    }
#[no_mangle]
unsafe extern "C" fn mhi_ep_ring_num_elems(ring: *mut mhi_ep_ring) -> u32 {
    static u32 mhi_ep_ring_num_elems(struct mhi_ep_ring *ring)
    {
    __le64 rlen;
    memcpy_fromio(&rlen, (void __iomem *) &ring.ring_ctx.generic.rlen, sizeof(u64));
    return le64_to_cpu(rlen) / sizeof(struct mhi_ring_element);
    }
#[no_mangle]
pub unsafe extern "C" fn mhi_ep_ring_inc_index(ring: *mut mhi_ep_ring) {
    void mhi_ep_ring_inc_index(struct mhi_ep_ring *ring)
    {
    ring.rd_offset = (ring.rd_offset + 1) % ring.ring_size;
    }
#[no_mangle]
unsafe extern "C" fn __mhi_ep_cache_ring(ring: *mut mhi_ep_ring, end: usize) -> c_int {
    static int __mhi_ep_cache_ring(struct mhi_ep_ring *ring, size_t end)
    {
    struct mhi_ep_cntrl *mhi_cntrl = ring.mhi_cntrl;
    struct device *dev = &mhi_cntrl.mhi_dev.dev;
    let mut buf_info: mhi_ep_buf_info = {};
    size_t start;
    int ret;
// Don't proceed in the case of event ring. This happens during mhi_ep_ring_start().
    if (ring.type == RING_TYPE_ER)
    return 0;
// No need to cache the ring if write pointer is unmodified
    if (ring.wr_offset == end)
    return 0;
    start = ring.wr_offset;
    if (start < end) {
    buf_info.size = (end - start) * sizeof(struct mhi_ring_element);
    buf_info.host_addr = ring.rbase + (start * sizeof(struct mhi_ring_element));
    buf_info.dev_addr = &ring.ring_cache[start];
    ret = mhi_cntrl.read_sync(mhi_cntrl, &buf_info);
    if (ret)
    return ret;
    } else {
    buf_info.size = (ring.ring_size - start) * sizeof(struct mhi_ring_element);
    buf_info.host_addr = ring.rbase + (start * sizeof(struct mhi_ring_element));
    buf_info.dev_addr = &ring.ring_cache[start];
    ret = mhi_cntrl.read_sync(mhi_cntrl, &buf_info);
    if (ret)
    return ret;
    if (end) {
    buf_info.host_addr = ring.rbase;
    buf_info.dev_addr = &ring.ring_cache[0];
    buf_info.size = end * sizeof(struct mhi_ring_element);
    ret = mhi_cntrl.read_sync(mhi_cntrl, &buf_info);
    if (ret)
    return ret;
    }
    }
    dev_dbg(dev, "Cached ring: start %zu end %zu size %zu\n", start, end, buf_info.size);
    return 0;
    }
#[no_mangle]
unsafe extern "C" fn mhi_ep_cache_ring(ring: *mut mhi_ep_ring, wr_ptr: u64) -> c_int {
    static int mhi_ep_cache_ring(struct mhi_ep_ring *ring, u64 wr_ptr)
    {
    size_t wr_offset;
    int ret;
    wr_offset = mhi_ep_ring_addr2offset(ring, wr_ptr);
// Cache the host ring till write offset
    ret = __mhi_ep_cache_ring(ring, wr_offset);
    if (ret)
    return ret;
    ring.wr_offset = wr_offset;
    return 0;
    }
#[no_mangle]
pub unsafe extern "C" fn mhi_ep_update_wr_offset(ring: *mut mhi_ep_ring) -> c_int {
    int mhi_ep_update_wr_offset(struct mhi_ep_ring *ring)
    {
    u64 wr_ptr;
    wr_ptr = mhi_ep_mmio_get_db(ring);
    return mhi_ep_cache_ring(ring, wr_ptr);
    }
// TODO: Support for adding multiple ring elements to the ring
#[no_mangle]
pub unsafe extern "C" fn mhi_ep_ring_add_element(ring: *mut mhi_ep_ring, el: *mut mhi_ring_element) -> c_int {
    int mhi_ep_ring_add_element(struct mhi_ep_ring *ring, struct mhi_ring_element *el)
    {
    struct mhi_ep_cntrl *mhi_cntrl = ring.mhi_cntrl;
    struct device *dev = &mhi_cntrl.mhi_dev.dev;
    let mut buf_info: mhi_ep_buf_info = {};
    let mut old_offset: usize = 0;
    u32 num_free_elem;
    __le64 rp;
    int ret;
    ret = mhi_ep_update_wr_offset(ring);
    if (ret) {
    dev_err(dev, "Error updating write pointer\n");
    return ret;
    }
    if (ring.rd_offset < ring.wr_offset)
    num_free_elem = (ring.wr_offset - ring.rd_offset) - 1;
    else
    num_free_elem = ((ring.ring_size - ring.rd_offset) + ring.wr_offset) - 1;
// Check if there is space in ring for adding at least an element
    if (!num_free_elem) {
    dev_err(dev, "No space left in the ring\n");
    return -ENOSPC;
    }
    old_offset = ring.rd_offset;
    dev_dbg(dev, "Adding an element to ring at offset (%zu)\n", ring.rd_offset);
    buf_info.host_addr = ring.rbase + (old_offset * sizeof(*el));
    buf_info.dev_addr = el;
    buf_info.size = sizeof(*el);
    ret = mhi_cntrl.write_sync(mhi_cntrl, &buf_info);
    if (ret)
    return ret;
    mhi_ep_ring_inc_index(ring);
// Update rp in ring context
    rp = cpu_to_le64(ring.rd_offset * sizeof(*el) + ring.rbase);
    memcpy_toio((void __iomem *) &ring.ring_ctx.generic.rp, &rp, sizeof(u64));
    return ret;
    }
#[no_mangle]
pub unsafe extern "C" fn mhi_ep_ring_init(ring: *mut mhi_ep_ring, type: enum mhi_ep_ring_type, id: u32) {
    void mhi_ep_ring_init(struct mhi_ep_ring *ring, enum mhi_ep_ring_type type, u32 id)
    {
    ring.type = type;
    if (ring.type == RING_TYPE_CMD) {
    ring.db_offset_h = EP_CRDB_HIGHER;
    ring.db_offset_l = EP_CRDB_LOWER;
    } else if (ring.type == RING_TYPE_CH) {
    ring.db_offset_h = CHDB_HIGHER_n(id);
    ring.db_offset_l = CHDB_LOWER_n(id);
    ring.ch_id = id;
    } else {
    ring.db_offset_h = ERDB_HIGHER_n(id);
    ring.db_offset_l = ERDB_LOWER_n(id);
    }
    }
#[no_mangle]
unsafe extern "C" fn mhi_ep_raise_irq(work: *mut work_struct) {
    static void mhi_ep_raise_irq(struct work_struct *work)
    {
    struct mhi_ep_ring *ring = container_of(work, struct mhi_ep_ring, intmodt_work.work);
    struct mhi_ep_cntrl *mhi_cntrl = ring.mhi_cntrl;
    mhi_cntrl.raise_irq(mhi_cntrl, ring.irq_vector);
    WRITE_ONCE(ring.irq_pending, false);
    }
    int mhi_ep_ring_start(struct mhi_ep_cntrl *mhi_cntrl, struct mhi_ep_ring *ring,
    union mhi_ep_ring_ctx *ctx)
    {
    struct device *dev = &mhi_cntrl.mhi_dev.dev;
    __le64 val;
    int ret;
    ring.mhi_cntrl = mhi_cntrl;
    ring.ring_ctx = ctx;
    ring.ring_size = mhi_ep_ring_num_elems(ring);
    memcpy_fromio(&val, (void __iomem *) &ring.ring_ctx.generic.rbase, sizeof(u64));
    ring.rbase = le64_to_cpu(val);
    if (ring.type == RING_TYPE_CH)
    ring.er_index = le32_to_cpu(ring.ring_ctx.ch.erindex);
    if (ring.type == RING_TYPE_ER) {
    ring.irq_vector = le32_to_cpu(ring.ring_ctx.ev.msivec);
    ring.intmodt = FIELD_GET(EV_CTX_INTMODT_MASK,
    le32_to_cpu(ring.ring_ctx.ev.intmod));
    INIT_DELAYED_WORK(&ring.intmodt_work, mhi_ep_raise_irq);
    }
// During ring init, both rp and wp are equal
    memcpy_fromio(&val, (void __iomem *) &ring.ring_ctx.generic.rp, sizeof(u64));
    ring.rd_offset = mhi_ep_ring_addr2offset(ring, le64_to_cpu(val));
    ring.wr_offset = mhi_ep_ring_addr2offset(ring, le64_to_cpu(val));
// Allocate ring cache memory for holding the copy of host ring
    ring.ring_cache = kzalloc_objs(struct mhi_ring_element,
    ring.ring_size);
    if (!ring.ring_cache)
    return -ENOMEM;
    memcpy_fromio(&val, (void __iomem *) &ring.ring_ctx.generic.wp, sizeof(u64));
    ret = mhi_ep_cache_ring(ring, le64_to_cpu(val));
    if (ret) {
    dev_err(dev, "Failed to cache ring\n");
    kfree(ring.ring_cache);
    return ret;
    }
    ring.started = true;
    return 0;
    }
#[no_mangle]
pub unsafe extern "C" fn mhi_ep_ring_reset(mhi_cntrl: *mut mhi_ep_cntrl, ring: *mut mhi_ep_ring) {
    void mhi_ep_ring_reset(struct mhi_ep_cntrl *mhi_cntrl, struct mhi_ep_ring *ring)
    {
    if (ring.type == RING_TYPE_ER)
    cancel_delayed_work_sync(&ring.intmodt_work);
    ring.started = false;
    kfree(ring.ring_cache);
    ring.ring_cache = core::ptr::null_mut();
    }
