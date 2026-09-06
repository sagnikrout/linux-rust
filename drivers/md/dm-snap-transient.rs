//! Automatically rewritten from C to Rust
//! Source: drivers/md/dm-snap-transient.c
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
// Copyright (C) 2001-2002 Sistina Software (UK) Limited.
// Copyright (C) 2006-2008 Red Hat GmbH
//
// This file is released under the GPL.
//

//
// ---------------------------------------------------------------
// Implementation of the store for non-persistent snapshots.
// ---------------------------------------------------------------
//
#[repr(C)]
#[derive(Copy, Clone)]
pub struct transient_c {
    pub next_free: sector_t,
}

#[no_mangle]
unsafe extern "C" fn transient_dtr(store: *mut dm_exception_store) {
    static void transient_dtr(struct dm_exception_store *store)
    {
    kfree(store.context);
    }
    static int transient_read_metadata(struct dm_exception_store *store,
    int (*callback)(void *callback_context,
    chunk_t old, chunk_t new),
    void *callback_context)
    {
    return 0;
    }
    static int transient_prepare_exception(struct dm_exception_store *store,
    struct dm_exception *e)
    {
    struct transient_c *tc = store.context;
    let mut size: sector_t = get_dev_size(dm_snap_cow(store.snap).bdev);
    if (size < (tc.next_free + store.chunk_size))
    return -1;
    e.new_chunk = sector_to_chunk(store, tc.next_free);
    tc.next_free += store.chunk_size;
    return 0;
    }
    static void transient_commit_exception(struct dm_exception_store *store,
    struct dm_exception *e, int valid,
    void (*callback)(void *, int success),
    void *callback_context)
    {
// Just succeed
    callback(callback_context, valid);
    }
    static void transient_usage(struct dm_exception_store *store,
    sector_t *total_sectors,
    sector_t *sectors_allocated,
    sector_t *metadata_sectors)
    {
// sectors_allocated = ((struct transient_c *) store->context)->next_free;
// total_sectors = get_dev_size(dm_snap_cow(store->snap)->bdev);
// metadata_sectors = 0;
    }
#[no_mangle]
unsafe extern "C" fn transient_ctr(store: *mut dm_exception_store, options: *mut c_char) -> c_int {
    static int transient_ctr(struct dm_exception_store *store, char *options)
    {
    struct transient_c *tc;
    tc = kmalloc_obj(struct transient_c);
    if (!tc)
    return -ENOMEM;
    tc.next_free = 0;
    store.context = tc;
    return 0;
    }
    static unsigned int transient_status(struct dm_exception_store *store,
    status_type_t status, char *result,
    unsigned int maxlen)
    {
    let mut sz: c_uint = 0;
    switch (status) {
    case STATUSTYPE_INFO:
    break;
    case STATUSTYPE_TABLE:
    DMEMIT(" N %llu", (unsigned long long)store.chunk_size);
    break;
    case STATUSTYPE_IMA:
// result = '\0';
    break;
    }
    return sz;
    }
    static struct dm_exception_store_type _transient_type = {
    .name = "transient",
    .module = THIS_MODULE,
    .ctr = transient_ctr,
    .dtr = transient_dtr,
    .read_metadata = transient_read_metadata,
    .prepare_exception = transient_prepare_exception,
    .commit_exception = transient_commit_exception,
    .usage = transient_usage,
    .status = transient_status,
    };
    static struct dm_exception_store_type _transient_compat_type = {
    .name = "N",
    .module = THIS_MODULE,
    .ctr = transient_ctr,
    .dtr = transient_dtr,
    .read_metadata = transient_read_metadata,
    .prepare_exception = transient_prepare_exception,
    .commit_exception = transient_commit_exception,
    .usage = transient_usage,
    .status = transient_status,
    };
#[no_mangle]
pub unsafe extern "C" fn dm_transient_snapshot_init() -> c_int {
    int dm_transient_snapshot_init(void)
    {
    int r;
    r = dm_exception_store_type_register(&_transient_type);
    if (r) {
    DMWARN("Unable to register transient exception store type");
    return r;
    }
    r = dm_exception_store_type_register(&_transient_compat_type);
    if (r) {
    DMWARN("Unable to register old-style transient exception store type");
    dm_exception_store_type_unregister(&_transient_type);
    return r;
    }
    return r;
    }
#[no_mangle]
pub unsafe extern "C" fn dm_transient_snapshot_exit() {
    void dm_transient_snapshot_exit(void)
    {
    dm_exception_store_type_unregister(&_transient_type);
    dm_exception_store_type_unregister(&_transient_compat_type);
    }
