//! Automatically rewritten from C to Rust
//! Source: drivers/infiniband/core/mr_pool.c
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


// SPDX-License-Identifier: GPL-2.0-only
//
// Copyright (c) 2016 HGST, a Western Digital Company.
//

    struct ib_mr *ib_mr_pool_get(struct ib_qp *qp, struct list_head *list)
    {
    struct ib_mr *mr;
    unsigned long flags;
    spin_lock_irqsave(&qp.mr_lock, flags);
    mr = list_first_entry_or_null(list, struct ib_mr, qp_entry);
    if (mr) {
    list_del(&mr.qp_entry);
    qp.mrs_used++;
    }
    spin_unlock_irqrestore(&qp.mr_lock, flags);
    return mr;
    }
    EXPORT_SYMBOL(ib_mr_pool_get);
#[no_mangle]
pub unsafe extern "C" fn ib_mr_pool_put(qp: *mut ib_qp, list: *mut list_head, mr: *mut ib_mr) {
    void ib_mr_pool_put(struct ib_qp *qp, struct list_head *list, struct ib_mr *mr)
    {
    unsigned long flags;
    spin_lock_irqsave(&qp.mr_lock, flags);
    list_add(&mr.qp_entry, list);
    qp.mrs_used--;
    spin_unlock_irqrestore(&qp.mr_lock, flags);
    }
    EXPORT_SYMBOL(ib_mr_pool_put);
    int ib_mr_pool_init(struct ib_qp *qp, struct list_head *list, int nr,
    enum ib_mr_type type, u32 max_num_sg, u32 max_num_meta_sg)
    {
    struct ib_mr *mr;
    unsigned long flags;
    int ret, i;
    for (i = 0; i < nr; i++) {
    if (type == IB_MR_TYPE_INTEGRITY)
    mr = ib_alloc_mr_integrity(qp.pd, max_num_sg,
    max_num_meta_sg);
    else
    mr = ib_alloc_mr(qp.pd, type, max_num_sg);
    if (IS_ERR(mr)) {
    ret = PTR_ERR(mr);
    goto out;
    }
    spin_lock_irqsave(&qp.mr_lock, flags);
    list_add_tail(&mr.qp_entry, list);
    spin_unlock_irqrestore(&qp.mr_lock, flags);
    }
    return 0;
    out:
    ib_mr_pool_destroy(qp, list);
    return ret;
    }
    EXPORT_SYMBOL(ib_mr_pool_init);
#[no_mangle]
pub unsafe extern "C" fn ib_mr_pool_destroy(qp: *mut ib_qp, list: *mut list_head) {
    void ib_mr_pool_destroy(struct ib_qp *qp, struct list_head *list)
    {
    struct ib_mr *mr;
    unsigned long flags;
    spin_lock_irqsave(&qp.mr_lock, flags);
    while (!list_empty(list)) {
    mr = list_first_entry(list, struct ib_mr, qp_entry);
    list_del(&mr.qp_entry);
    spin_unlock_irqrestore(&qp.mr_lock, flags);
    ib_dereg_mr(mr);
    spin_lock_irqsave(&qp.mr_lock, flags);
    }
    spin_unlock_irqrestore(&qp.mr_lock, flags);
    }
    EXPORT_SYMBOL(ib_mr_pool_destroy);
