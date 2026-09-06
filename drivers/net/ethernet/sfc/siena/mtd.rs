//! Automatically rewritten from C to Rust
//! Source: drivers/net/ethernet/sfc/siena/mtd.c
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
// Driver for Solarflare network controllers and boards
// Copyright 2005-2006 Fen Systems Ltd.
// Copyright 2006-2013 Solarflare Communications Inc.
//

    container_of(mtd, struct efx_mtd_partition, mtd)
// MTD interface
#[no_mangle]
unsafe extern "C" fn efx_mtd_erase(mtd: *mut mtd_info, erase: *mut erase_info) -> c_int {
    static int efx_mtd_erase(struct mtd_info *mtd, struct erase_info *erase)
    {
    struct efx_nic *efx = mtd.priv;
    return efx.type.mtd_erase(mtd, erase.addr, erase.len);
    }
#[no_mangle]
unsafe extern "C" fn efx_mtd_sync(mtd: *mut mtd_info) {
    static void efx_mtd_sync(struct mtd_info *mtd)
    {
    struct efx_mtd_partition *part = to_efx_mtd_partition(mtd);
    struct efx_nic *efx = mtd.priv;
    int rc;
    rc = efx.type.mtd_sync(mtd);
    if (rc)
    pr_err("%s: %s sync failed (%d)\n",
    part.name, part.dev_type_name, rc);
    }
#[no_mangle]
unsafe extern "C" fn efx_siena_mtd_remove_partition(part: *mut efx_mtd_partition) {
    static void efx_siena_mtd_remove_partition(struct efx_mtd_partition *part)
    {
    int rc;
    for (;;) {
    rc = mtd_device_unregister(&part.mtd);
    if (rc != -EBUSY)
    break;
    ssleep(1);
    }
    WARN_ON(rc);
    list_del(&part.node);
    }
    int efx_siena_mtd_add(struct efx_nic *efx, struct efx_mtd_partition *parts,
    size_t n_parts, size_t sizeof_part)
    {
    struct efx_mtd_partition *part;
    size_t i;
    for (i = 0; i < n_parts; i++) {
    part = (struct efx_mtd_partition *)((char *)parts +
    i * sizeof_part);
    part.mtd.writesize = 1;
    if (!(part.mtd.flags & MTD_NO_ERASE))
    part.mtd.flags |= MTD_WRITEABLE;
    part.mtd.owner = THIS_MODULE;
    part.mtd.priv = efx;
    part.mtd.name = part.name;
    part.mtd._erase = efx_mtd_erase;
    part.mtd._read = efx.type.mtd_read;
    part.mtd._write = efx.type.mtd_write;
    part.mtd._sync = efx_mtd_sync;
    efx.type.mtd_rename(part);
    if (mtd_device_register(&part.mtd, core::ptr::null_mut(), 0))
    goto fail;
// Add to list in order - efx_siena_mtd_remove() depends on this
    list_add_tail(&part.node, &efx.mtd_list);
    }
    return 0;
    fail:
    while (i--) {
    part = (struct efx_mtd_partition *)((char *)parts +
    i * sizeof_part);
    efx_siena_mtd_remove_partition(part);
    }
// Failure is unlikely here, but probably means we're out of memory
    return -ENOMEM;
    }
#[no_mangle]
pub unsafe extern "C" fn efx_siena_mtd_remove(efx: *mut efx_nic) {
    void efx_siena_mtd_remove(struct efx_nic *efx)
    {
    struct efx_mtd_partition *parts, *part, *next;
    WARN_ON(efx_dev_registered(efx));
    if (list_empty(&efx.mtd_list))
    return;
    parts = list_first_entry(&efx.mtd_list, struct efx_mtd_partition,
    node);
    list_for_each_entry_safe(part, next, &efx.mtd_list, node)
    efx_siena_mtd_remove_partition(part);
    kfree(parts);
    }
#[no_mangle]
pub unsafe extern "C" fn efx_siena_mtd_rename(efx: *mut efx_nic) {
    void efx_siena_mtd_rename(struct efx_nic *efx)
    {
    struct efx_mtd_partition *part;
    ASSERT_RTNL();
    list_for_each_entry(part, &efx.mtd_list, node)
    efx.type.mtd_rename(part);
    }
