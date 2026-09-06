//! Automatically rewritten from C to Rust
//! Source: fs/squashfs/zstd_wrapper.c
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
// Copyright (c) 2016-present, Facebook, Inc.
// All rights reserved.
//
// zstd_wrapper.c
//

#[repr(C)]
#[derive(Copy, Clone)]
pub struct workspace {
    pub mem: *mut c_void,
    pub mem_size: usize,
    pub window_size: usize,
}

    static void *zstd_init(struct squashfs_sb_info *msblk, void *buff)
    {
    struct workspace *wksp = kmalloc_obj(*wksp);
    if (wksp == core::ptr::null_mut())
    goto failed;
    wksp.window_size = max_t(size_t,
    msblk.block_size, SQUASHFS_METADATA_SIZE);
    wksp.mem_size = zstd_dstream_workspace_bound(wksp.window_size);
    wksp.mem = vmalloc(wksp.mem_size);
    if (wksp.mem == core::ptr::null_mut())
    goto failed;
    return wksp;
    failed:
    ERROR("Failed to allocate zstd workspace\n");
    kfree(wksp);
    return ERR_PTR(-ENOMEM);
    }
#[no_mangle]
unsafe extern "C" fn zstd_free(strm: *mut c_void) {
    static void zstd_free(void *strm)
    {
    struct workspace *wksp = strm;
    if (wksp)
    vfree(wksp.mem);
    kfree(wksp);
    }
    static int zstd_uncompress(struct squashfs_sb_info *msblk, void *strm,
    struct bio *bio, int offset, int length,
    struct squashfs_page_actor *output)
    {
    struct workspace *wksp = strm;
    zstd_dstream *stream;
    let mut total_out: usize = 0;
    let mut error: c_int = 0;
    let mut in_buf: zstd_in_buffer = { core::ptr::null_mut(), 0, 0 };
    let mut out_buf: zstd_out_buffer = { core::ptr::null_mut(), 0, 0 };
    let mut iter_all: bvec_iter_all = {};
    struct bio_vec *bvec = bvec_init_iter_all(&iter_all);
    stream = zstd_init_dstream(wksp.window_size, wksp.mem, wksp.mem_size);
    if (!stream) {
    ERROR("Failed to initialize zstd decompressor\n");
    return -EIO;
    }
    out_buf.size = PAGE_SIZE;
    out_buf.dst = squashfs_first_page(output);
    if (IS_ERR(out_buf.dst)) {
    error = PTR_ERR(out_buf.dst);
    goto finish;
    }
    for (;;) {
    size_t zstd_err;
    if (in_buf.pos == in_buf.size) {
    const void *data;
    int avail;
    if (!bio_next_segment(bio, &iter_all)) {
    error = -EIO;
    break;
    }
    avail = min(length, ((int)bvec.bv_len) - offset);
    data = bvec_virt(bvec);
    length -= avail;
    in_buf.src = data + offset;
    in_buf.size = avail;
    in_buf.pos = 0;
    offset = 0;
    }
    if (out_buf.pos == out_buf.size) {
    out_buf.dst = squashfs_next_page(output);
    if (IS_ERR(out_buf.dst)) {
    error = PTR_ERR(out_buf.dst);
    break;
    } else if (out_buf.dst == core::ptr::null_mut()) {
// Shouldn't run out of pages
// before stream is done.
//
    error = -EIO;
    break;
    }
    out_buf.pos = 0;
    out_buf.size = PAGE_SIZE;
    }
    total_out -= out_buf.pos;
    zstd_err = zstd_decompress_stream(stream, &out_buf, &in_buf);
    total_out += out_buf.pos; /* add the additional data produced */
    if (zstd_err == 0)
    break;
    if (zstd_is_error(zstd_err)) {
    ERROR("zstd decompression error: %d\n",
    (int)zstd_get_error_code(zstd_err));
    error = -EIO;
    break;
    }
    }
    finish:
    squashfs_finish_page(output);
    return error ? error : total_out;
    }
    const struct squashfs_decompressor squashfs_zstd_comp_ops = {
    .init = zstd_init,
    .free = zstd_free,
    .decompress = zstd_uncompress,
    .id = ZSTD_COMPRESSION,
    .name = "zstd",
    .alloc_buffer = 1,
    .supported = 1
    };
