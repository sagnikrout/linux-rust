//! Automatically rewritten from C to Rust
//! Source: fs/squashfs/decompressor_single.c
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
// Copyright (c) 2013
// Phillip Lougher <phillip@squashfs.org.uk>
//

//
// This file implements single-threaded decompression in the
// decompressor framework
//
#[repr(C)]
#[derive(Copy, Clone)]
pub struct squashfs_stream {
    pub stream: *mut c_void,
    pub mutex: mutex,
}

    static void *squashfs_decompressor_create(struct squashfs_sb_info *msblk,
    void *comp_opts)
    {
    struct squashfs_stream *stream;
    let mut err: c_int = -ENOMEM;
    stream = kmalloc_obj(*stream);
    if (stream == core::ptr::null_mut())
    goto out;
    stream.stream = msblk.decompressor.init(msblk, comp_opts);
    if (IS_ERR(stream.stream)) {
    err = PTR_ERR(stream.stream);
    goto out;
    }
    kfree(comp_opts);
    mutex_init(&stream.mutex);
    return stream;
    out:
    kfree(stream);
    return ERR_PTR(err);
    }
#[no_mangle]
unsafe extern "C" fn squashfs_decompressor_destroy(msblk: *mut squashfs_sb_info) {
    static void squashfs_decompressor_destroy(struct squashfs_sb_info *msblk)
    {
    struct squashfs_stream *stream = msblk.stream;
    if (stream) {
    msblk.decompressor.free(stream.stream);
    kfree(stream);
    }
    }
    static int squashfs_decompress(struct squashfs_sb_info *msblk, struct bio *bio,
    int offset, int length,
    struct squashfs_page_actor *output)
    {
    int res;
    struct squashfs_stream *stream = msblk.stream;
    mutex_lock(&stream.mutex);
    res = msblk.decompressor.decompress(msblk, stream.stream, bio,
    offset, length, output);
    mutex_unlock(&stream.mutex);
    if (res < 0)
    ERROR("%s decompression failed, data probably corrupt\n",
    msblk.decompressor.name);
    return res;
    }
#[no_mangle]
unsafe extern "C" fn squashfs_max_decompressors() -> c_int {
    static int squashfs_max_decompressors(void)
    {
    return 1;
    }
    const struct squashfs_decompressor_thread_ops squashfs_decompressor_single = {
    .create = squashfs_decompressor_create,
    .destroy = squashfs_decompressor_destroy,
    .decompress = squashfs_decompress,
    .max_decompressors = squashfs_max_decompressors,
    };
