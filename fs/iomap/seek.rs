//! Automatically rewritten from C to Rust
//! Source: fs/iomap/seek.c
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
// Copyright (C) 2017 Red Hat, Inc.
// Copyright (c) 2018-2021 Christoph Hellwig.
//

    static int iomap_seek_hole_iter(struct iomap_iter *iter,
    loff_t *hole_pos)
    {
    let mut length: loff_t = iomap_length(iter);
    switch (iter.iomap.type) {
    case IOMAP_UNWRITTEN:
// hole_pos = mapping_seek_hole_data(iter->inode->i_mapping,
    iter.pos, iter.pos + length, SEEK_HOLE);
    if (*hole_pos == iter.pos + length)
    return iomap_iter_advance(iter, length);
    return 0;
    case IOMAP_HOLE:
// hole_pos = iter->pos;
    return 0;
    default:
    return iomap_iter_advance(iter, length);
    }
    }
    loff_t
    iomap_seek_hole(struct inode *inode, loff_t pos, const struct iomap_ops *ops)
    {
    let mut size: loff_t = i_size_read(inode);
    struct iomap_iter iter = {
    .inode	= inode,
    .pos	= pos,
    .flags	= IOMAP_REPORT,
    };
    int ret;
// Nothing to be found before or beyond the end of the file.
    if (pos < 0 || pos >= size)
    return -ENXIO;
    iter.len = size - pos;
    while ((ret = iomap_iter(&iter, ops)) > 0)
    iter.status = iomap_seek_hole_iter(&iter, &pos);
    if (ret < 0)
    return ret;
    if (iter.len) /* found hole before EOF */
    return pos;
    return size;
    }
    EXPORT_SYMBOL_GPL(iomap_seek_hole);
    static int iomap_seek_data_iter(struct iomap_iter *iter,
    loff_t *hole_pos)
    {
    let mut length: loff_t = iomap_length(iter);
    switch (iter.iomap.type) {
    case IOMAP_HOLE:
    return iomap_iter_advance(iter, length);
    case IOMAP_UNWRITTEN:
// hole_pos = mapping_seek_hole_data(iter->inode->i_mapping,
    iter.pos, iter.pos + length, SEEK_DATA);
    if (*hole_pos < 0)
    return iomap_iter_advance(iter, length);
    return 0;
    default:
// hole_pos = iter->pos;
    return 0;
    }
    }
    loff_t
    iomap_seek_data(struct inode *inode, loff_t pos, const struct iomap_ops *ops)
    {
    let mut size: loff_t = i_size_read(inode);
    struct iomap_iter iter = {
    .inode	= inode,
    .pos	= pos,
    .flags	= IOMAP_REPORT,
    };
    int ret;
// Nothing to be found before or beyond the end of the file.
    if (pos < 0 || pos >= size)
    return -ENXIO;
    iter.len = size - pos;
    while ((ret = iomap_iter(&iter, ops)) > 0)
    iter.status = iomap_seek_data_iter(&iter, &pos);
    if (ret < 0)
    return ret;
    if (iter.len) /* found data before EOF */
    return pos;
// We've reached the end of the file without finding data
    return -ENXIO;
    }
    EXPORT_SYMBOL_GPL(iomap_seek_data);
