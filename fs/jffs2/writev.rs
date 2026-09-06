//! Automatically rewritten from C to Rust
//! Source: fs/jffs2/writev.c
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
// JFFS2 -- Journalling Flash File System, Version 2.
//
// Copyright © 2001-2007 Red Hat, Inc.
//
// Created by David Woodhouse <dwmw2@infradead.org>
//
// For licensing information, see the file 'LICENCE' in this directory.
//

    int jffs2_flash_direct_writev(struct jffs2_sb_info *c, const struct kvec *vecs,
    unsigned long count, loff_t to, size_t *retlen)
    {
    if (!jffs2_is_writebuffered(c)) {
    if (jffs2_sum_active()) {
    int res;
    res = jffs2_sum_add_kvec(c, vecs, count, (uint32_t) to);
    if (res) {
    return res;
    }
    }
    }
    return mtd_writev(c.mtd, vecs, count, to, retlen);
    }
    int jffs2_flash_direct_write(struct jffs2_sb_info *c, loff_t ofs, size_t len,
    size_t *retlen, const u_char *buf)
    {
    int ret;
    ret = mtd_write(c.mtd, ofs, len, retlen, buf);
    if (jffs2_sum_active()) {
    struct kvec vecs[1];
    int res;
    vecs[0].iov_base = (unsigned char *) buf;
    vecs[0].iov_len = len;
    res = jffs2_sum_add_kvec(c, vecs, 1, (uint32_t) ofs);
    if (res) {
    return res;
    }
    }
    return ret;
    }
