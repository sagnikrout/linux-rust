//! Automatically rewritten from C to Rust
//! Source: drivers/infiniband/hw/mana/wq.c
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
// Copyright (c) 2022, Microsoft Corporation. All rights reserved.
//

    struct ib_wq *mana_ib_create_wq(struct ib_pd *pd,
    struct ib_wq_init_attr *init_attr,
    struct ib_udata *udata)
    {
    struct mana_ib_dev *mdev =
    container_of(pd.device, struct mana_ib_dev, ib_dev);
    struct mana_ib_create_wq ucmd;
    struct mana_ib_wq *wq;
    int err;
    err = ib_copy_validate_udata_in(udata, ucmd, reserved);
    if (err)
    return ERR_PTR(err);
    wq = kzalloc_obj(*wq);
    if (!wq)
    return ERR_PTR(-ENOMEM);
    ibdev_dbg(&mdev.ib_dev, "ucmd wq_buf_addr 0x%llx\n", ucmd.wq_buf_addr);
    err = mana_ib_create_queue(mdev, ucmd.wq_buf_addr, ucmd.wq_buf_size, &wq.queue);
    if (err) {
    ibdev_dbg(&mdev.ib_dev,
    "Failed to create queue for create wq, %d\n", err);
    goto err_free_wq;
    }
    wq.wqe = init_attr.max_wr;
    wq.wq_buf_size = ucmd.wq_buf_size;
    wq.rx_object = INVALID_MANA_HANDLE;
    return &wq.ibwq;
    err_free_wq:
    kfree(wq);
    return ERR_PTR(err);
    }
    int mana_ib_modify_wq(struct ib_wq *wq, struct ib_wq_attr *wq_attr,
    u32 wq_attr_mask, struct ib_udata *udata)
    {
// modify_wq is not supported by this version of the driver
    return -EOPNOTSUPP;
    }
#[no_mangle]
pub unsafe extern "C" fn mana_ib_destroy_wq(ibwq: *mut ib_wq, udata: *mut ib_udata) -> c_int {
    int mana_ib_destroy_wq(struct ib_wq *ibwq, struct ib_udata *udata)
    {
    struct mana_ib_wq *wq = container_of(ibwq, struct mana_ib_wq, ibwq);
    struct ib_device *ib_dev = ibwq.device;
    struct mana_ib_dev *mdev;
    int err;
    err = ib_no_udata_io(udata);
    if (err)
    return err;
    mdev = container_of(ib_dev, struct mana_ib_dev, ib_dev);
    mana_ib_destroy_queue(mdev, &wq.queue);
    kfree(wq);
    return 0;
    }
    int mana_ib_create_rwq_ind_table(struct ib_rwq_ind_table *ib_rwq_ind_table,
    struct ib_rwq_ind_table_init_attr *init_attr,
    struct ib_udata *udata)
    {
    int err;
    err = ib_no_udata_io(udata);
    if (err)
    return err;
//
// There is no additional data in ind_table to be maintained by this
// driver, do nothing
//
    return 0;
    }
#[no_mangle]
pub unsafe extern "C" fn mana_ib_destroy_rwq_ind_table(ib_rwq_ind_tbl: *mut ib_rwq_ind_table) -> c_int {
    int mana_ib_destroy_rwq_ind_table(struct ib_rwq_ind_table *ib_rwq_ind_tbl)
    {
//
// There is no additional data in ind_table to be maintained by this
// driver, do nothing
//
    return 0;
    }
