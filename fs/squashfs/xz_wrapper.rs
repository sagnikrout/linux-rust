//! Automatically rewritten from C to Rust
//! Source: fs/squashfs/xz_wrapper.c
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
//
// Squashfs - a compressed read only filesystem for Linux
//
// Copyright (c) 2002, 2003, 2004, 2005, 2006, 2007, 2008, 2009, 2010
// Phillip Lougher <phillip@squashfs.org.uk>
//
// xz_wrapper.c
//

#[repr(C)]
#[derive(Copy, Clone)]
pub struct squashfs_xz {
    pub state: *mut xz_dec,
    pub buf: xz_buf,
}

#[repr(C)]
#[derive(Copy, Clone)]
pub struct disk_comp_opts {
    pub dictionary_size: __le32,
    pub flags: __le32,
}

#[repr(C)]
#[derive(Copy, Clone)]
pub struct comp_opts {
    pub dict_size: c_int,
}

    static void *squashfs_xz_comp_opts(struct squashfs_sb_info *msblk,
    void *buff, int len)
    {
    struct disk_comp_opts *comp_opts = buff;
    struct comp_opts *opts;
    let mut err: c_int = 0, n;
    opts = kmalloc_obj(*opts);
    if (opts == core::ptr::null_mut()) {
    err = -ENOMEM;
    goto out2;
    }
    if (comp_opts) {
// check compressor options are the expected length
    if (len < sizeof(*comp_opts)) {
    err = -EIO;
    goto out;
    }
    opts.dict_size = le32_to_cpu(comp_opts.dictionary_size);
// the dictionary size should be 2^n or 2^n+2^(n+1)
    n = ffs(opts.dict_size) - 1;
    if (opts.dict_size != (1 << n) && opts.dict_size != (1 << n) +
    (1 << (n + 1))) {
    err = -EIO;
    goto out;
    }
    } else
// use defaults
    opts.dict_size = max_t(int, msblk.block_size,
    SQUASHFS_METADATA_SIZE);
    return opts;
    out:
    kfree(opts);
    out2:
    return ERR_PTR(err);
    }
    static void *squashfs_xz_init(struct squashfs_sb_info *msblk, void *buff)
    {
    struct comp_opts *comp_opts = buff;
    struct squashfs_xz *stream;
    int err;
    stream = kmalloc_obj(*stream);
    if (stream == core::ptr::null_mut()) {
    err = -ENOMEM;
    goto failed;
    }
    stream.state = xz_dec_init(XZ_PREALLOC, comp_opts.dict_size);
    if (stream.state == core::ptr::null_mut()) {
    kfree(stream);
    err = -ENOMEM;
    goto failed;
    }
    return stream;
    failed:
    ERROR("Failed to initialise xz decompressor\n");
    return ERR_PTR(err);
    }
#[no_mangle]
unsafe extern "C" fn squashfs_xz_free(strm: *mut c_void) {
    static void squashfs_xz_free(void *strm)
    {
    struct squashfs_xz *stream = strm;
    if (stream) {
    xz_dec_end(stream.state);
    kfree(stream);
    }
    }
    static int squashfs_xz_uncompress(struct squashfs_sb_info *msblk, void *strm,
    struct bio *bio, int offset, int length,
    struct squashfs_page_actor *output)
    {
    let mut iter_all: bvec_iter_all = {};
    struct bio_vec *bvec = bvec_init_iter_all(&iter_all);
    let mut total: c_int = 0, error = 0;
    struct squashfs_xz *stream = strm;
    xz_dec_reset(stream.state);
    stream.buf.in_pos = 0;
    stream.buf.in_size = 0;
    stream.buf.out_pos = 0;
    stream.buf.out_size = PAGE_SIZE;
    stream.buf.out = squashfs_first_page(output);
    if (IS_ERR(stream.buf.out)) {
    error = PTR_ERR(stream.buf.out);
    goto finish;
    }
    for (;;) {
    enum xz_ret xz_err;
    if (stream.buf.in_pos == stream.buf.in_size) {
    const void *data;
    int avail;
    if (!bio_next_segment(bio, &iter_all)) {
// XZ_STREAM_END must be reached.
    error = -EIO;
    break;
    }
    avail = min(length, ((int)bvec.bv_len) - offset);
    data = bvec_virt(bvec);
    length -= avail;
    stream.buf.in = data + offset;
    stream.buf.in_size = avail;
    stream.buf.in_pos = 0;
    offset = 0;
    }
    if (stream.buf.out_pos == stream.buf.out_size) {
    stream.buf.out = squashfs_next_page(output);
    if (IS_ERR(stream.buf.out)) {
    error = PTR_ERR(stream.buf.out);
    break;
    } else if (stream.buf.out != core::ptr::null_mut()) {
    stream.buf.out_pos = 0;
    total += PAGE_SIZE;
    }
    }
    xz_err = xz_dec_run(stream.state, &stream.buf);
    if (xz_err == XZ_STREAM_END)
    break;
    if (xz_err != XZ_OK) {
    error = -EIO;
    break;
    }
    }
    finish:
    squashfs_finish_page(output);
    return error ? error : total + stream.buf.out_pos;
    }
    const struct squashfs_decompressor squashfs_xz_comp_ops = {
    .init = squashfs_xz_init,
    .comp_opts = squashfs_xz_comp_opts,
    .free = squashfs_xz_free,
    .decompress = squashfs_xz_uncompress,
    .id = XZ_COMPRESSION,
    .name = "xz",
    .alloc_buffer = 1,
    .supported = 1
    };
