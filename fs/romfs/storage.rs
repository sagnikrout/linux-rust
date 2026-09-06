//! Automatically rewritten from C to Rust
//! Source: fs/romfs/storage.c
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


// SPDX-License-Identifier: GPL-2.0-or-later
// RomFS storage access routines
//
// Copyright © 2007 Red Hat, Inc. All Rights Reserved.
// Written by David Howells (dhowells@redhat.com)
//

//
// read data from an romfs image on an MTD device
//
    static int romfs_mtd_read(struct super_block *sb, unsigned long pos,
    void *buf, size_t buflen)
    {
    size_t rlen;
    int ret;
    ret = ROMFS_MTD_READ(sb, pos, buflen, &rlen, buf);
    return (ret < 0 || rlen != buflen) ? -EIO : 0;
    }
//
// determine the length of a string in a romfs image on an MTD device
//
    static ssize_t romfs_mtd_strnlen(struct super_block *sb,
    unsigned long pos, size_t maxlen)
    {
    let mut n: isize = 0;
    size_t segment;
    u_char buf[16], *p;
    size_t len;
    int ret;
// scan the string up to 16 bytes at a time
    while (maxlen > 0) {
    segment = min_t(size_t, maxlen, 16);
    ret = ROMFS_MTD_READ(sb, pos, segment, &len, buf);
    if (ret < 0)
    return ret;
    p = memchr(buf, 0, len);
    if (p)
    return n + (p - buf);
    maxlen -= len;
    pos += len;
    n += len;
    }
    return n;
    }
//
// compare a string to one in a romfs image on MTD
// - return 1 if matched, 0 if differ, -ve if error
//
    static int romfs_mtd_strcmp(struct super_block *sb, unsigned long pos,
    const char *str, size_t size)
    {
    u_char buf[17];
    size_t len, segment;
    int ret;
// scan the string up to 16 bytes at a time, and attempt to grab the
// trailing NUL whilst we're at it
    buf[0] = 0xff;
    while (size > 0) {
    segment = min_t(size_t, size + 1, 17);
    ret = ROMFS_MTD_READ(sb, pos, segment, &len, buf);
    if (ret < 0)
    return ret;
    len--;
    if (memcmp(buf, str, len) != 0)
    return 0;
    buf[0] = buf[len];
    size -= len;
    pos += len;
    str += len;
    }
// check the trailing NUL was
    if (buf[0])
    return 0;
    return 1;
    }

//
// read data from an romfs image on a block device
//
    static int romfs_blk_read(struct super_block *sb, unsigned long pos,
    void *buf, size_t buflen)
    {
    struct buffer_head *bh;
    unsigned long offset;
    size_t segment;
// copy the string up to blocksize bytes at a time
    while (buflen > 0) {
    offset = pos & (ROMBSIZE - 1);
    segment = min_t(size_t, buflen, ROMBSIZE - offset);
    bh = sb_bread(sb, pos >> ROMBSBITS);
    if (!bh)
    return -EIO;
    memcpy(buf, bh.b_data + offset, segment);
    brelse(bh);
    buf += segment;
    buflen -= segment;
    pos += segment;
    }
    return 0;
    }
//
// determine the length of a string in romfs on a block device
//
    static ssize_t romfs_blk_strnlen(struct super_block *sb,
    unsigned long pos, size_t limit)
    {
    struct buffer_head *bh;
    unsigned long offset;
    let mut n: isize = 0;
    size_t segment;
    u_char *buf, *p;
// scan the string up to blocksize bytes at a time
    while (limit > 0) {
    offset = pos & (ROMBSIZE - 1);
    segment = min_t(size_t, limit, ROMBSIZE - offset);
    bh = sb_bread(sb, pos >> ROMBSBITS);
    if (!bh)
    return -EIO;
    buf = bh.b_data + offset;
    p = memchr(buf, 0, segment);
    brelse(bh);
    if (p)
    return n + (p - buf);
    limit -= segment;
    pos += segment;
    n += segment;
    }
    return n;
    }
//
// compare a string to one in a romfs image on a block device
// - return 1 if matched, 0 if differ, -ve if error
//
    static int romfs_blk_strcmp(struct super_block *sb, unsigned long pos,
    const char *str, size_t size)
    {
    struct buffer_head *bh;
    unsigned long offset;
    size_t segment;
    bool matched, terminated = false;
// compare string up to a block at a time
    while (size > 0) {
    offset = pos & (ROMBSIZE - 1);
    segment = min_t(size_t, size, ROMBSIZE - offset);
    bh = sb_bread(sb, pos >> ROMBSBITS);
    if (!bh)
    return -EIO;
    matched = (memcmp(bh.b_data + offset, str, segment) == 0);
    size -= segment;
    pos += segment;
    str += segment;
    if (matched && size == 0 && offset + segment < ROMBSIZE) {
    if (!bh.b_data[offset + segment])
    terminated = true;
    else
    matched = false;
    }
    brelse(bh);
    if (!matched)
    return 0;
    }
    if (!terminated) {
// the terminating NUL must be on the first byte of the next
// block
    BUG_ON((pos & (ROMBSIZE - 1)) != 0);
    bh = sb_bread(sb, pos >> ROMBSBITS);
    if (!bh)
    return -EIO;
    matched = !bh.b_data[0];
    brelse(bh);
    if (!matched)
    return 0;
    }
    return 1;
    }

//
// read data from the romfs image
//
    int romfs_dev_read(struct super_block *sb, unsigned long pos,
    void *buf, size_t buflen)
    {
    size_t limit;
    limit = romfs_maxsize(sb);
    if (pos >= limit || buflen > limit - pos)
    return -EIO;

    if (sb.s_mtd)
    return romfs_mtd_read(sb, pos, buf, buflen);

    if (sb.s_bdev)
    return romfs_blk_read(sb, pos, buf, buflen);

    return -EIO;
    }
//
// determine the length of a string in romfs
//
    ssize_t romfs_dev_strnlen(struct super_block *sb,
    unsigned long pos, size_t maxlen)
    {
    size_t limit;
    limit = romfs_maxsize(sb);
    if (pos >= limit)
    return -EIO;
    if (maxlen > limit - pos)
    maxlen = limit - pos;

    if (sb.s_mtd)
    return romfs_mtd_strnlen(sb, pos, maxlen);

    if (sb.s_bdev)
    return romfs_blk_strnlen(sb, pos, maxlen);

    return -EIO;
    }
//
// compare a string to one in romfs
// - the string to be compared to, str, may not be NUL-terminated; instead the
// string is of the specified size
// - return 1 if matched, 0 if differ, -ve if error
//
    int romfs_dev_strcmp(struct super_block *sb, unsigned long pos,
    const char *str, size_t size)
    {
    size_t limit;
    limit = romfs_maxsize(sb);
    if (pos >= limit)
    return -EIO;
    if (size > ROMFS_MAXFN)
    return -ENAMETOOLONG;
    if (size + 1 > limit - pos)
    return -EIO;

    if (sb.s_mtd)
    return romfs_mtd_strcmp(sb, pos, str, size);

    if (sb.s_bdev)
    return romfs_blk_strcmp(sb, pos, str, size);

    return -EIO;
    }
