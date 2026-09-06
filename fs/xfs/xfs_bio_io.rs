//! Automatically rewritten from C to Rust
//! Source: fs/xfs/xfs_bio_io.c
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
// Copyright (c) 2019 Christoph Hellwig.
//

#[no_mangle]
pub unsafe extern "C" fn bio_max_vecs(count: c_uint) -> c_uint {
    static inline unsigned int bio_max_vecs(unsigned int count)
    {
    return bio_max_segs(howmany(count, PAGE_SIZE));
    }
    int
    xfs_rw_bdev(
    struct block_device	*bdev,
    sector_t		sector,
    unsigned int		count,
    char			*data,
    enum req_op		op)
    {
    let mut done: c_uint = 0, added;
    int			error;
    struct bio		*bio;
    op |= REQ_META | REQ_SYNC;
    if (!is_vmalloc_addr(data))
    return bdev_rw_virt(bdev, sector, data, count, op);
    bio = bio_alloc(bdev, bio_max_vecs(count), op, GFP_KERNEL);
    bio.bi_iter.bi_sector = sector;
    do {
    added = bio_add_vmalloc_chunk(bio, data + done, count - done);
    if (!added) {
    struct bio	*prev = bio;
    bio = bio_alloc(prev.bi_bdev,
    bio_max_vecs(count - done),
    prev.bi_opf, GFP_KERNEL);
    bio.bi_iter.bi_sector = bio_end_sector(prev);
    bio_chain(prev, bio);
    submit_bio(prev);
    }
    done += added;
    } while (done < count);
    error = submit_bio_wait(bio);
    bio_put(bio);
    if (op == REQ_OP_READ)
    invalidate_kernel_vmap_range(data, count);
    return error;
    }
