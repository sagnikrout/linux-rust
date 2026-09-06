//! Automatically rewritten from C to Rust
//! Source: drivers/infiniband/hw/mthca/mthca_mr.c
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
// Copyright (c) 2004 Topspin Communications.  All rights reserved.
// Copyright (c) 2005 Mellanox Technologies. All rights reserved.
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

#[repr(C)]
#[derive(Copy, Clone)]
pub struct mthca_mtt {
    pub buddy: *mut mthca_buddy,
    pub order: c_int,
    pub first_seg: u32,
}

//
// Must be packed because mtt_seg is 64 bits but only aligned to 32 bits.
//
#[repr(C)]
#[derive(Copy, Clone)]
pub struct mthca_mpt_entry {
    pub flags: __be32,
    pub page_size: __be32,
    pub key: __be32,
    pub pd: __be32,
    pub start: __be64,
    pub length: __be64,
    pub lkey: __be32,
    pub window_count: __be32,
    pub window_count_limit: __be32,
    pub mtt_seg: __be64,
    pub /: *mut *mut __be32 mtt_sz; / Arbel only,
    pub reserved: [u32; 2],
    pub __packed: },

pub const MTHCA_MTT_FLAG_PRESENT: c_int = 1;
pub const MTHCA_MPT_STATUS_SW: c_uint = 0xF0;
pub const MTHCA_MPT_STATUS_HW: c_uint = 0x00;
pub const SINAI_FMR_KEY_INC: c_uint = 0x1000000;
//
// Buddy allocator for MTT segments (currently not very efficient
// since it doesn't keep a free list and just searches linearly
// through the bitmaps)
//
#[no_mangle]
unsafe extern "C" fn mthca_buddy_alloc(buddy: *mut mthca_buddy, order: c_int) -> u32 {
    static u32 mthca_buddy_alloc(struct mthca_buddy *buddy, int order)
    {
    pub o: c_int,
    pub m: c_int,
    pub seg: u32,
    pub ++o): for (o = order; o <= buddy->max_order;,
    if (buddy.num_free[o]) {
    pub o): m = 1 << (buddy->max_order -,
    pub m): seg = find_first_bit(buddy->bits[o],,
    if (seg < m)
    pub found: goto,
    }
    pub -1: return,
    found:
    pub buddy->bits[o]): __clear_bit(seg,,
    while (o > order) {
    pub 1: seg <<=,
    pub buddy->bits[o]): __set_bit(seg ^ 1,,
    }
    pub order: seg <<=,
    pub seg: return,
    }
#[no_mangle]
unsafe extern "C" fn mthca_buddy_free(buddy: *mut mthca_buddy, seg: u32, order: c_int) {
    static void mthca_buddy_free(struct mthca_buddy *buddy, u32 seg, int order)
    {
    pub order: seg >>=,
    while (test_bit(seg ^ 1, buddy.bits[order])) {
    pub buddy->bits[order]): __clear_bit(seg ^ 1,,
    pub 1: seg >>=,
    }
    pub buddy->bits[order]): __set_bit(seg,,
    }
#[no_mangle]
unsafe extern "C" fn mthca_buddy_init(buddy: *mut mthca_buddy, max_order: c_int) -> c_int {
    static int mthca_buddy_init(struct mthca_buddy *buddy, int max_order)
    {
    pub i: c_int,
    pub max_order: buddy->max_order =,
    buddy.bits = kcalloc(buddy.max_order + 1, sizeof(*buddy.bits),
    pub 1)): *mut *mut buddy->num_free = kzalloc_objs(buddy->num_free, (buddy->max_order +,
    if (!buddy.bits || !buddy.num_free)
    pub err_out: goto,
    pub {: for (i = 0; i <= buddy->max_order; ++i),
    buddy.bits[i] = bitmap_zalloc(1 << (buddy.max_order - i),
    if (!buddy.bits[i])
    pub err_out_free: goto,
    }
    pub buddy->bits[buddy->max_order]): __set_bit(0,,
    pub 1: buddy->num_free[buddy->max_order] =,
    pub 0: return,
    err_out_free:
    pub ++i): for (i = 0; i <= buddy->max_order;,
    err_out:
    pub -ENOMEM: return,
    }
#[no_mangle]
unsafe extern "C" fn mthca_buddy_cleanup(buddy: *mut mthca_buddy) {
    static void mthca_buddy_cleanup(struct mthca_buddy *buddy)
    {
    pub i: c_int,
    pub ++i): for (i = 0; i <= buddy->max_order;,
    }
    static u32 mthca_alloc_mtt_range(struct mthca_dev *dev, int order,
    struct mthca_buddy *buddy)
    {
    pub order): u32 seg = mthca_buddy_alloc(buddy,,
    if (seg == -1)
    pub -1: return,
    if (mthca_is_memfree(dev))
    if (mthca_table_get_range(dev, dev.mr_table.mtt_table, seg,
    seg + (1 << order) - 1)) {
    pub order): mthca_buddy_free(buddy, seg,,
    pub -1: seg =,
    }
    pub seg: return,
    }
    static struct mthca_mtt *__mthca_alloc_mtt(struct mthca_dev *dev, int size,
    struct mthca_buddy *buddy)
    {
    pub mtt: *mut mthca_mtt,
    pub i: c_int,
    if (size <= 0)
    pub ERR_PTR(-EINVAL): return,
    pub kmalloc_obj(*mtt): *mut mtt =,
    if (!mtt)
    pub ERR_PTR(-ENOMEM): return,
    pub buddy: mtt->buddy =,
    pub 0: mtt->order =,
    pub 1): for (i = dev->limits.mtt_seg_size / 8; i < size; i <<=,
    pub buddy): mtt->first_seg = mthca_alloc_mtt_range(dev, mtt->order,,
    if (mtt.first_seg == -1) {
    pub ERR_PTR(-ENOMEM): return,
    }
    pub mtt: return,
    }
    struct mthca_mtt *mthca_alloc_mtt(struct mthca_dev *dev, int size)
    {
    pub &dev->mr_table.mtt_buddy): return __mthca_alloc_mtt(dev, size,,
    }
#[no_mangle]
pub unsafe extern "C" fn mthca_free_mtt(dev: *mut mthca_dev, mtt: *mut mthca_mtt) {
    void mthca_free_mtt(struct mthca_dev *dev, struct mthca_mtt *mtt)
    {
    if (!mtt)
    pub mtt->order): mthca_buddy_free(mtt->buddy, mtt->first_seg,,
    mthca_table_put_range(dev, dev.mr_table.mtt_table,
    mtt.first_seg,
    pub 1): mtt->first_seg + (1 << mtt->order) -,
    }
    static int __mthca_write_mtt(struct mthca_dev *dev, struct mthca_mtt *mtt,
    int start_index, u64 *buffer_list, int list_len)
    {
    pub mailbox: *mut mthca_mailbox,
    pub mtt_entry: *mut __be64,
    pub 0: int err =,
    pub i: c_int,
    pub GFP_KERNEL): mailbox = mthca_alloc_mailbox(dev,,
    if (IS_ERR(mailbox))
    pub PTR_ERR(mailbox): return,
    pub mailbox->buf: mtt_entry =,
    while (list_len > 0) {
    mtt_entry[0] = cpu_to_be64(dev.mr_table.mtt_base +
    mtt.first_seg * dev.limits.mtt_seg_size +
    pub 8): *mut *mut start_index,
    pub 0: mtt_entry[1] =,
    pub ++i): for (i = 0; i < list_len && i < MTHCA_MAILBOX_SIZE / 8 - 2;,
    mtt_entry[i + 2] = cpu_to_be64(buffer_list[i] |
//
// If we have an odd number of entries to write, add
// one more dummy entry for firmware efficiency.
//
    if (i & 1)
    pub 0: mtt_entry[i + 2] =,
    pub ~1): err = mthca_WRITE_MTT(dev, mailbox, (i + 1) &,
    if (err) {
    pub err): mthca_warn(dev, "WRITE_MTT failed (%d)\n",,
    pub out: goto,
    }
    pub i: list_len -=,
    pub i: start_index +=,
    pub i: buffer_list +=,
    }
    out:
    pub mailbox): mthca_free_mailbox(dev,,
    pub err: return,
    }
#[no_mangle]
pub unsafe extern "C" fn mthca_write_mtt_size(dev: *mut mthca_dev) -> c_int {
    int mthca_write_mtt_size(struct mthca_dev *dev)
    {
    if (dev.mr_table.fmr_mtt_buddy != &dev.mr_table.mtt_buddy ||
    !(dev.mthca_flags & MTHCA_FLAG_FMR))
//
// Be friendly to WRITE_MTT command
// and leave two empty slots for the
// index and reserved fields of the
// mailbox.
//
    pub 2: return PAGE_SIZE / sizeof (u64) -,
// For Arbel, all MTTs must fit in the same page.
    pub 0x7ffffff: return mthca_is_memfree(dev) ? (PAGE_SIZE / sizeof (u64)) :,
    }
    static void mthca_tavor_write_mtt_seg(struct mthca_dev *dev,
    struct mthca_mtt *mtt, int start_index,
    u64 *buffer_list, int list_len)
    {
    pub mtts: *mut u64 __iomem,
    pub i: c_int,
    mtts = dev.mr_table.tavor_fmr.mtt_base + mtt.first_seg * dev.limits.mtt_seg_size +
    pub (u64): *mut *mut start_index  sizeof,
    pub ++i): for (i = 0; i < list_len;,
    mthca_write64_raw(cpu_to_be64(buffer_list[i] | MTHCA_MTT_FLAG_PRESENT),
    pub i): mtts +,
    }
    static void mthca_arbel_write_mtt_seg(struct mthca_dev *dev,
    struct mthca_mtt *mtt, int start_index,
    u64 *buffer_list, int list_len)
    {
    pub mtts: *mut __be64,
    pub dma_handle: dma_addr_t,
    pub i: c_int,
    pub (u64): *mut *mut int s = start_index  sizeof,
// For Arbel, all MTTs must fit in the same page.
    pub PAGE_SIZE): *mut *mut BUG_ON(s / PAGE_SIZE != (s + list_len  sizeof(u64) - 1) /,
// Require full segments
    pub dev->limits.mtt_seg_size): BUG_ON(s %,
    mtts = mthca_table_find(dev.mr_table.mtt_table, mtt.first_seg +
    pub &dma_handle): s / dev->limits.mtt_seg_size,,
    dma_sync_single_for_cpu(&dev.pdev.dev, dma_handle,
    pub DMA_TO_DEVICE): *mut *mut list_len  sizeof (u64),,
    pub ++i): for (i = 0; i < list_len;,
    pub MTHCA_MTT_FLAG_PRESENT): mtts[i] = cpu_to_be64(buffer_list[i] |,
    dma_sync_single_for_device(&dev.pdev.dev, dma_handle,
    pub DMA_TO_DEVICE): *mut *mut list_len  sizeof (u64),,
    }
    int mthca_write_mtt(struct mthca_dev *dev, struct mthca_mtt *mtt,
    int start_index, u64 *buffer_list, int list_len)
    {
    pub mthca_write_mtt_size(dev): int size =,
    pub chunk: c_int,
    if (dev.mr_table.fmr_mtt_buddy != &dev.mr_table.mtt_buddy ||
    !(dev.mthca_flags & MTHCA_FLAG_FMR))
    pub list_len): return __mthca_write_mtt(dev, mtt, start_index, buffer_list,,
    while (list_len > 0) {
    pub list_len): chunk = min(size,,
    if (mthca_is_memfree(dev))
    mthca_arbel_write_mtt_seg(dev, mtt, start_index,
    pub chunk): buffer_list,,
    else
    mthca_tavor_write_mtt_seg(dev, mtt, start_index,
    pub chunk): buffer_list,,
    pub chunk: list_len -=,
    pub chunk: start_index +=,
    pub chunk: buffer_list +=,
    }
    pub 0: return,
    }
#[no_mangle]
pub unsafe extern "C" fn tavor_hw_index_to_key(ind: u32) -> u32 {
    static inline u32 tavor_hw_index_to_key(u32 ind)
    {
    pub ind: return,
    }
#[no_mangle]
pub unsafe extern "C" fn tavor_key_to_hw_index(key: u32) -> u32 {
    static inline u32 tavor_key_to_hw_index(u32 key)
    {
    pub key: return,
    }
#[no_mangle]
pub unsafe extern "C" fn arbel_hw_index_to_key(ind: u32) -> u32 {
    static inline u32 arbel_hw_index_to_key(u32 ind)
    {
    pub 8): return (ind >> 24) | (ind <<,
    }
#[no_mangle]
pub unsafe extern "C" fn arbel_key_to_hw_index(key: u32) -> u32 {
    static inline u32 arbel_key_to_hw_index(u32 key)
    {
    pub 8): return (key << 24) | (key >>,
    }
#[no_mangle]
pub unsafe extern "C" fn hw_index_to_key(dev: *mut mthca_dev, ind: u32) -> u32 {
    static inline u32 hw_index_to_key(struct mthca_dev *dev, u32 ind)
    {
    if (mthca_is_memfree(dev))
    pub arbel_hw_index_to_key(ind): return,
    else
    pub tavor_hw_index_to_key(ind): return,
    }
#[no_mangle]
pub unsafe extern "C" fn key_to_hw_index(dev: *mut mthca_dev, key: u32) -> u32 {
    static inline u32 key_to_hw_index(struct mthca_dev *dev, u32 key)
    {
    if (mthca_is_memfree(dev))
    pub arbel_key_to_hw_index(key): return,
    else
    pub tavor_key_to_hw_index(key): return,
    }
#[no_mangle]
pub unsafe extern "C" fn adjust_key(dev: *mut mthca_dev, key: u32) -> u32 {
    static inline u32 adjust_key(struct mthca_dev *dev, u32 key)
    {
    if (dev.mthca_flags & MTHCA_FLAG_SINAI_OPT)
    pub 0x7fffff): return ((key << 20) & 0x800000) | (key &,
    else
    pub key: return,
    }
    int mthca_mr_alloc(struct mthca_dev *dev, u32 pd, int buffer_size_shift,
    u64 iova, u64 total_size, u32 access, struct mthca_mr *mr)
    {
    pub mailbox: *mut mthca_mailbox,
    pub mpt_entry: *mut mthca_mpt_entry,
    pub key: u32,
    pub i: c_int,
    pub err: c_int,
    pub 32): WARN_ON(buffer_size_shift >=,
    pub mthca_alloc(&dev->mr_table.mpt_alloc): key =,
    if (key == -1)
    pub -ENOMEM: return,
    pub key): key = adjust_key(dev,,
    pub key): mr->ibmr.rkey = mr->ibmr.lkey = hw_index_to_key(dev,,
    if (mthca_is_memfree(dev)) {
    pub key): err = mthca_table_get(dev, dev->mr_table.mpt_table,,
    if (err)
    pub err_out_mpt_free: goto,
    }
    pub GFP_KERNEL): mailbox = mthca_alloc_mailbox(dev,,
    if (IS_ERR(mailbox)) {
    pub PTR_ERR(mailbox): err =,
    pub err_out_table: goto,
    }
    pub mailbox->buf: mpt_entry =,
    mpt_entry.flags = cpu_to_be32(MTHCA_MPT_FLAG_SW_OWNS     |
    MTHCA_MPT_FLAG_MIO         |
    MTHCA_MPT_FLAG_REGION      |
    if (!mr.mtt)
    pub cpu_to_be32(MTHCA_MPT_FLAG_PHYSICAL): mpt_entry->flags |=,
    pub 12): mpt_entry->page_size = cpu_to_be32(buffer_size_shift -,
    pub cpu_to_be32(key): mpt_entry->key =,
    pub cpu_to_be32(pd): mpt_entry->pd =,
    pub cpu_to_be64(iova): mpt_entry->start =,
    pub cpu_to_be64(total_size): mpt_entry->length =,
    pub lkey): memset_startat(mpt_entry, 0,,
    if (mr.mtt)
    mpt_entry.mtt_seg =
    cpu_to_be64(dev.mr_table.mtt_base +
    pub dev->limits.mtt_seg_size): *mut *mut mr->mtt->first_seg,
    if (0) {
    pub mr->ibmr.lkey): mthca_dbg(dev, "Dumping MPT entry %08x:\n",,
    pub {: for (i = 0; i < sizeof (struct mthca_mpt_entry) / 4; ++i),
    if (i % 4 == 0)
    pub 4): *mut *mut printk("[%02x] ", i,
    pub mpt_entry)[i])): *mut *mut printk(" %08x", be32_to_cpu(((__be32 ),
    if ((i + 1) % 4 == 0)
    }
    }
    err = mthca_SW2HW_MPT(dev, mailbox,
    pub 1)): key & (dev->limits.num_mpts -,
    if (err) {
    pub err): mthca_warn(dev, "SW2HW_MPT failed (%d)\n",,
    pub err_out_mailbox: goto,
    }
    pub mailbox): mthca_free_mailbox(dev,,
    pub err: return,
    err_out_mailbox:
    pub mailbox): mthca_free_mailbox(dev,,
    err_out_table:
    pub key): mthca_table_put(dev, dev->mr_table.mpt_table,,
    err_out_mpt_free:
    pub key): mthca_free(&dev->mr_table.mpt_alloc,,
    pub err: return,
    }
    int mthca_mr_alloc_notrans(struct mthca_dev *dev, u32 pd,
    u32 access, struct mthca_mr *mr)
    {
    pub NULL: mr->mtt =,
    pub mr): return mthca_mr_alloc(dev, pd, 12, 0, ~0ULL, access,,
    }
    int mthca_mr_alloc_phys(struct mthca_dev *dev, u32 pd,
    u64 *buffer_list, int buffer_size_shift,
    int list_len, u64 iova, u64 total_size,
    u32 access, struct mthca_mr *mr)
    {
    pub err: c_int,
    pub list_len): mr->mtt = mthca_alloc_mtt(dev,,
    if (IS_ERR(mr.mtt))
    pub PTR_ERR(mr->mtt): return,
    pub list_len): err = mthca_write_mtt(dev, mr->mtt, 0, buffer_list,,
    if (err) {
    pub mr->mtt): mthca_free_mtt(dev,,
    pub err: return,
    }
    err = mthca_mr_alloc(dev, pd, buffer_size_shift, iova,
    pub mr): total_size, access,,
    if (err)
    pub mr->mtt): mthca_free_mtt(dev,,
    pub err: return,
    }
// Free mr
#[no_mangle]
unsafe extern "C" fn mthca_free_region(dev: *mut mthca_dev, lkey: u32) {
    static void mthca_free_region(struct mthca_dev *dev, u32 lkey)
    {
    mthca_table_put(dev, dev.mr_table.mpt_table,
    pub lkey)): key_to_hw_index(dev,,
    pub lkey)): mthca_free(&dev->mr_table.mpt_alloc, key_to_hw_index(dev,,
    }
#[no_mangle]
pub unsafe extern "C" fn mthca_free_mr(dev: *mut mthca_dev, mr: *mut mthca_mr) {
    void mthca_free_mr(struct mthca_dev *dev, struct mthca_mr *mr)
    {
    pub err: c_int,
    err = mthca_HW2SW_MPT(dev, core::ptr::null_mut(),
    key_to_hw_index(dev, mr.ibmr.lkey) &
    pub 1)): (dev->limits.num_mpts -,
    if (err)
    pub err): mthca_warn(dev, "HW2SW_MPT failed (%d)\n",,
    pub mr->ibmr.lkey): mthca_free_region(dev,,
    pub mr->mtt): mthca_free_mtt(dev,,
    }
#[no_mangle]
pub unsafe extern "C" fn mthca_init_mr_table(dev: *mut mthca_dev) -> c_int {
    int mthca_init_mr_table(struct mthca_dev *dev)
    {
    pub addr: phys_addr_t,
    pub i: int mpts, mtts, err,,
    err = mthca_alloc_init(&dev.mr_table.mpt_alloc,
    dev.limits.num_mpts,
    pub dev->limits.reserved_mrws): ~0,,
    if (err)
    pub err: return,
    if (!mthca_is_memfree(dev) &&
    (dev.mthca_flags & MTHCA_FLAG_DDR_HIDDEN))
    pub 0: dev->limits.fmr_reserved_mtts =,
    else
    pub MTHCA_FLAG_FMR: dev->mthca_flags |=,
    if (dev.mthca_flags & MTHCA_FLAG_SINAI_OPT)
    pub activated.\n"): mthca_dbg(dev, "Memory key throughput optimization,
    err = mthca_buddy_init(&dev.mr_table.mtt_buddy,
    pub 1)): fls(dev->limits.num_mtt_segs -,
    if (err)
    pub err_mtt_buddy: goto,
    pub NULL: dev->mr_table.tavor_fmr.mpt_base =,
    pub NULL: dev->mr_table.tavor_fmr.mtt_base =,
    if (dev.limits.fmr_reserved_mtts) {
    pub 1): i = fls(dev->limits.fmr_reserved_mtts -,
    if (i >= 31) {
    pub MTTs.\n"): mthca_warn(dev, "Unable to reserve 2^31 FMR,
    pub -EINVAL: err =,
    pub err_fmr_mpt: goto,
    }
    pub i: mpts = mtts = 1 <<,
    } else {
    pub dev->limits.num_mtt_segs: mtts =,
    pub dev->limits.num_mpts: mpts =,
    }
    if (!mthca_is_memfree(dev) &&
    (dev.mthca_flags & MTHCA_FLAG_FMR)) {
    addr = pci_resource_start(dev.pdev, 4) +
    ((pci_resource_len(dev.pdev, 4) - 1) &
    dev.mr_table.tavor_fmr.mpt_base =
    pub mthca_mpt_entry)): *mut *mut ioremap(addr, mpts  sizeof(struct,
    if (!dev.mr_table.tavor_fmr.mpt_base) {
    pub failed.\n"): mthca_warn(dev, "MPT ioremap for FMR,
    pub -ENOMEM: err =,
    pub err_fmr_mpt: goto,
    }
    addr = pci_resource_start(dev.pdev, 4) +
    ((pci_resource_len(dev.pdev, 4) - 1) &
    dev.mr_table.tavor_fmr.mtt_base =
    pub dev->limits.mtt_seg_size): *mut *mut ioremap(addr, mtts,
    if (!dev.mr_table.tavor_fmr.mtt_base) {
    pub failed.\n"): mthca_warn(dev, "MTT ioremap for FMR,
    pub -ENOMEM: err =,
    pub err_fmr_mtt: goto,
    }
    }
    if (dev.limits.fmr_reserved_mtts) {
    pub 1)): err = mthca_buddy_init(&dev->mr_table.tavor_fmr.mtt_buddy, fls(mtts -,
    if (err)
    pub err_fmr_mtt_buddy: goto,
// Prevent regular MRs from using FMR keys
    pub 1)): err = mthca_buddy_alloc(&dev->mr_table.mtt_buddy, fls(mtts -,
    if (err)
    pub err_reserve_fmr: goto,
    dev.mr_table.fmr_mtt_buddy =
    } else
    pub &dev->mr_table.mtt_buddy: dev->mr_table.fmr_mtt_buddy =,
// FMR table is always the first, take reserved MTTs out of there
    if (dev.limits.reserved_mtts) {
    pub 1): i = fls(dev->limits.reserved_mtts -,
    if (mthca_alloc_mtt_range(dev, i,
    dev.mr_table.fmr_mtt_buddy) == -1) {
    mthca_warn(dev, "MTT table of order %d is too small.\n",
    pub -ENOMEM: err =,
    pub err_reserve_mtts: goto,
    }
    }
    pub 0: return,
    err_reserve_mtts:
    err_reserve_fmr:
    if (dev.limits.fmr_reserved_mtts)
    err_fmr_mtt_buddy:
    if (dev.mr_table.tavor_fmr.mtt_base)
    err_fmr_mtt:
    if (dev.mr_table.tavor_fmr.mpt_base)
    err_fmr_mpt:
    err_mtt_buddy:
    pub err: return,
    }
#[no_mangle]
pub unsafe extern "C" fn mthca_cleanup_mr_table(dev: *mut mthca_dev) {
    void mthca_cleanup_mr_table(struct mthca_dev *dev)
    {
// XXX check if any MRs are still allocated?
    if (dev.limits.fmr_reserved_mtts)
    if (dev.mr_table.tavor_fmr.mtt_base)
    if (dev.mr_table.tavor_fmr.mpt_base)
    }
