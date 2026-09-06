//! Automatically rewritten from C to Rust
//! Source: drivers/md/dm-verity-loadpin.c
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

    LIST_HEAD(dm_verity_loadpin_trusted_root_digests);
#[no_mangle]
unsafe extern "C" fn is_trusted_verity_target(ti: *mut dm_target) -> bool {
    static bool is_trusted_verity_target(struct dm_target *ti)
    {
    int verity_mode;
    u8 *root_digest;
    unsigned int digest_size;
    struct dm_verity_loadpin_trusted_root_digest *trd;
    let mut trusted: bool = false;
    if (!dm_is_verity_target(ti))
    return false;
    verity_mode = dm_verity_get_mode(ti);
    if ((verity_mode != DM_VERITY_MODE_EIO) &&
    (verity_mode != DM_VERITY_MODE_RESTART) &&
    (verity_mode != DM_VERITY_MODE_PANIC))
    return false;
    if (dm_verity_get_root_digest(ti, &root_digest, &digest_size))
    return false;
    list_for_each_entry(trd, &dm_verity_loadpin_trusted_root_digests, node) {
    if ((trd.len == digest_size) &&
    !memcmp(trd.data, root_digest, digest_size)) {
    trusted = true;
    break;
    }
    }
    kfree(root_digest);
    return trusted;
    }
//
// Determines whether the file system of a superblock is located on
// a verity device that is trusted by LoadPin.
//
#[no_mangle]
pub unsafe extern "C" fn dm_verity_loadpin_is_bdev_trusted(bdev: *mut block_device) -> bool {
    bool dm_verity_loadpin_is_bdev_trusted(struct block_device *bdev)
    {
    struct mapped_device *md;
    struct dm_table *table;
    struct dm_target *ti;
    int srcu_idx;
    let mut trusted: bool = false;
    if (bdev == core::ptr::null_mut())
    return false;
    if (list_empty(&dm_verity_loadpin_trusted_root_digests))
    return false;
    md = dm_get_md(bdev.bd_dev);
    if (!md)
    return false;
    table = dm_get_live_table(md, &srcu_idx);
    if (!table || table.num_targets != 1)
    goto out;
    ti = dm_table_get_target(table, 0);
    if (is_trusted_verity_target(ti))
    trusted = true;
    out:
    dm_put_live_table(md, srcu_idx);
    dm_put(md);
    return trusted;
    }
