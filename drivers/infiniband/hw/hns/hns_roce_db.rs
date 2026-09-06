//! Automatically rewritten from C to Rust
//! Source: drivers/infiniband/hw/hns/hns_roce_db.c
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


// SPDX-License-Identifier: (GPL-2.0 OR BSD-2-Clause)
//
// Copyright (c) 2017 Hisilicon Limited.
// Copyright (c) 2007, 2008 Mellanox Technologies. All rights reserved.
//

    int hns_roce_db_map_user(struct hns_roce_ucontext *context, unsigned long virt,
    struct hns_roce_db *db)
    {
    let mut page_addr: c_ulong = virt & PAGE_MASK;
    struct hns_roce_user_db_page *page;
    unsigned int offset;
    let mut ret: c_int = 0;
    mutex_lock(&context.page_mutex);
    list_for_each_entry(page, &context.page_list, list)
    if (page.user_virt == page_addr)
    goto found;
    page = kmalloc_obj(*page);
    if (!page) {
    ret = -ENOMEM;
    goto out;
    }
    refcount_set(&page.refcount, 1);
    page.user_virt = page_addr;
    page.umem = ib_umem_get_va(context.ibucontext.device, page_addr,
    PAGE_SIZE, 0);
    if (IS_ERR(page.umem)) {
    ret = PTR_ERR(page.umem);
    kfree(page);
    goto out;
    }
    list_add(&page.list, &context.page_list);
    found:
    offset = virt - page_addr;
    db.dma = sg_dma_address(page.umem.sgt_append.sgt.sgl) + offset;
    db.virt_addr = sg_virt(page.umem.sgt_append.sgt.sgl) + offset;
    db.u.user_page = page;
    refcount_inc(&page.refcount);
    out:
    mutex_unlock(&context.page_mutex);
    return ret;
    }
    void hns_roce_db_unmap_user(struct hns_roce_ucontext *context,
    struct hns_roce_db *db)
    {
    mutex_lock(&context.page_mutex);
    refcount_dec(&db.u.user_page.refcount);
    if (refcount_dec_if_one(&db.u.user_page.refcount)) {
    list_del(&db.u.user_page.list);
    ib_umem_release(db.u.user_page.umem);
    kfree(db.u.user_page);
    }
    mutex_unlock(&context.page_mutex);
    }
    static struct hns_roce_db_pgdir *hns_roce_alloc_db_pgdir(
    struct device *dma_device)
    {
    struct hns_roce_db_pgdir *pgdir;
    pgdir = kzalloc_obj(*pgdir);
    if (!pgdir)
    return core::ptr::null_mut();
    bitmap_fill(pgdir.order1,
    HNS_ROCE_DB_PER_PAGE / HNS_ROCE_DB_TYPE_COUNT);
    pgdir.bits[0] = pgdir.order0;
    pgdir.bits[1] = pgdir.order1;
    pgdir.page = dma_alloc_coherent(dma_device, PAGE_SIZE,
    &pgdir.db_dma, GFP_KERNEL);
    if (!pgdir.page) {
    kfree(pgdir);
    return core::ptr::null_mut();
    }
    return pgdir;
    }
    static int hns_roce_alloc_db_from_pgdir(struct hns_roce_db_pgdir *pgdir,
    struct hns_roce_db *db, int order)
    {
    unsigned long o;
    unsigned long i;
    for (o = order; o <= 1; ++o) {
    i = find_first_bit(pgdir.bits[o], HNS_ROCE_DB_PER_PAGE >> o);
    if (i < HNS_ROCE_DB_PER_PAGE >> o)
    goto found;
    }
    return -ENOMEM;
    found:
    clear_bit(i, pgdir.bits[o]);
    i <<= o;
    if (o > order)
    set_bit(i ^ 1, pgdir.bits[order]);
    db.u.pgdir	= pgdir;
    db.index	= i;
    db.db_record	= pgdir.page + db.index;
    db.dma		= pgdir.db_dma  + db.index * HNS_ROCE_DB_UNIT_SIZE;
    db.order	= order;
    return 0;
    }
    int hns_roce_alloc_db(struct hns_roce_dev *hr_dev, struct hns_roce_db *db,
    int order)
    {
    struct hns_roce_db_pgdir *pgdir;
    let mut ret: c_int = 0;
    mutex_lock(&hr_dev.pgdir_mutex);
    list_for_each_entry(pgdir, &hr_dev.pgdir_list, list)
    if (!hns_roce_alloc_db_from_pgdir(pgdir, db, order))
    goto out;
    pgdir = hns_roce_alloc_db_pgdir(hr_dev.dev);
    if (!pgdir) {
    ret = -ENOMEM;
    goto out;
    }
    list_add(&pgdir.list, &hr_dev.pgdir_list);
// This should never fail -- we just allocated an empty page:
    WARN_ON(hns_roce_alloc_db_from_pgdir(pgdir, db, order));
    out:
    mutex_unlock(&hr_dev.pgdir_mutex);
    return ret;
    }
#[no_mangle]
pub unsafe extern "C" fn hns_roce_free_db(hr_dev: *mut hns_roce_dev, db: *mut hns_roce_db) {
    void hns_roce_free_db(struct hns_roce_dev *hr_dev, struct hns_roce_db *db)
    {
    unsigned long o;
    unsigned long i;
    mutex_lock(&hr_dev.pgdir_mutex);
    o = db.order;
    i = db.index;
    if (db.order == 0 && test_bit(i ^ 1, db.u.pgdir.order0)) {
    clear_bit(i ^ 1, db.u.pgdir.order0);
    ++o;
    }
    i >>= o;
    set_bit(i, db.u.pgdir.bits[o]);
    if (bitmap_full(db.u.pgdir.order1,
    HNS_ROCE_DB_PER_PAGE / HNS_ROCE_DB_TYPE_COUNT)) {
    dma_free_coherent(hr_dev.dev, PAGE_SIZE, db.u.pgdir.page,
    db.u.pgdir.db_dma);
    list_del(&db.u.pgdir.list);
    kfree(db.u.pgdir);
    }
    mutex_unlock(&hr_dev.pgdir_mutex);
    }
