//! Automatically rewritten from C to Rust
//! Source: fs/squashfs/zlib_wrapper.c
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
// Copyright (c) 2002, 2003, 2004, 2005, 2006, 2007, 2008, 2009
// Phillip Lougher <phillip@squashfs.org.uk>
//
// zlib_wrapper.c
//

    static void *zlib_init(struct squashfs_sb_info *dummy, void *buff)
    {
    z_stream *stream = kmalloc_obj(z_stream);
    if (stream == core::ptr::null_mut())
    goto failed;
    stream.workspace = vmalloc(zlib_inflate_workspacesize());
    if (stream.workspace == core::ptr::null_mut())
    goto failed;
    return stream;
    failed:
    ERROR("Failed to allocate zlib workspace\n");
    kfree(stream);
    return ERR_PTR(-ENOMEM);
    }
#[no_mangle]
unsafe extern "C" fn zlib_free(strm: *mut c_void) {
    static void zlib_free(void *strm)
    {
    z_stream *stream = strm;
    if (stream)
    vfree(stream.workspace);
    kfree(stream);
    }
    static int zlib_uncompress(struct squashfs_sb_info *msblk, void *strm,
    struct bio *bio, int offset, int length,
    struct squashfs_page_actor *output)
    {
    let mut iter_all: bvec_iter_all = {};
    struct bio_vec *bvec = bvec_init_iter_all(&iter_all);
    let mut zlib_init: c_int = 0, error = 0;
    z_stream *stream = strm;
    stream.avail_out = PAGE_SIZE;
    stream.next_out = squashfs_first_page(output);
    stream.avail_in = 0;
    if (IS_ERR(stream.next_out)) {
    error = PTR_ERR(stream.next_out);
    goto finish;
    }
    for (;;) {
    int zlib_err;
    if (stream.avail_in == 0) {
    const void *data;
    int avail;
    if (!bio_next_segment(bio, &iter_all)) {
// Z_STREAM_END must be reached.
    error = -EIO;
    break;
    }
    avail = min(length, ((int)bvec.bv_len) - offset);
    data = bvec_virt(bvec);
    length -= avail;
    stream.next_in = data + offset;
    stream.avail_in = avail;
    offset = 0;
    }
    if (stream.avail_out == 0) {
    stream.next_out = squashfs_next_page(output);
    if (IS_ERR(stream.next_out)) {
    error = PTR_ERR(stream.next_out);
    break;
    } else if (stream.next_out != core::ptr::null_mut())
    stream.avail_out = PAGE_SIZE;
    }
    if (!zlib_init) {
    zlib_err = zlib_inflateInit(stream);
    if (zlib_err != Z_OK) {
    error = -EIO;
    break;
    }
    zlib_init = 1;
    }
    zlib_err = zlib_inflate(stream, Z_SYNC_FLUSH);
    if (zlib_err == Z_STREAM_END)
    break;
    if (zlib_err != Z_OK) {
    error = -EIO;
    break;
    }
    }
    finish:
    squashfs_finish_page(output);
    if (!error)
    if (zlib_inflateEnd(stream) != Z_OK)
    error = -EIO;
    return error ? error : stream.total_out;
    }
    const struct squashfs_decompressor squashfs_zlib_comp_ops = {
    .init = zlib_init,
    .free = zlib_free,
    .decompress = zlib_uncompress,
    .id = ZLIB_COMPRESSION,
    .name = "zlib",
    .alloc_buffer = 1,
    .supported = 1
    };
