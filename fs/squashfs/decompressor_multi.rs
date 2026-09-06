//! Automatically rewritten from C to Rust
//! Source: fs/squashfs/decompressor_multi.c
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
// Minchan Kim <minchan@kernel.org>
//

//
// This file implements multi-threaded decompression in the
// decompressor framework
//
// The reason that multiply two is that a CPU can request new I/O
// while it is waiting previous request.
//

#[no_mangle]
unsafe extern "C" fn squashfs_max_decompressors() -> c_int {
    static int squashfs_max_decompressors(void)
    {
    return MAX_DECOMPRESSOR;
    }
#[repr(C)]
#[derive(Copy, Clone)]
pub struct squashfs_stream {
    pub comp_opts: *mut c_void,
    pub strm_list: list_head,
    pub mutex: mutex,
    pub avail_decomp: c_int,
    pub wait: wait_queue_head_t,
}

#[repr(C)]
#[derive(Copy, Clone)]
pub struct decomp_stream {
    pub stream: *mut c_void,
    pub list: list_head,
}

    static void put_decomp_stream(struct decomp_stream *decomp_strm,
    struct squashfs_stream *stream)
    {
    mutex_lock(&stream.mutex);
    list_add(&decomp_strm.list, &stream.strm_list);
    mutex_unlock(&stream.mutex);
    wake_up(&stream.wait);
    }
    static void *squashfs_decompressor_create(struct squashfs_sb_info *msblk,
    void *comp_opts)
    {
    struct squashfs_stream *stream;
    struct decomp_stream *decomp_strm = core::ptr::null_mut();
    let mut err: c_int = -ENOMEM;
    stream = kzalloc_obj(*stream);
    if (!stream)
    goto out;
    stream.comp_opts = comp_opts;
    mutex_init(&stream.mutex);
    INIT_LIST_HEAD(&stream.strm_list);
    init_waitqueue_head(&stream.wait);
//
// We should have a decompressor at least as default
// so if we fail to allocate new decompressor dynamically,
// we could always fall back to default decompressor and
// file system works.
//
    decomp_strm = kmalloc_obj(*decomp_strm);
    if (!decomp_strm)
    goto out;
    decomp_strm.stream = msblk.decompressor.init(msblk,
    stream.comp_opts);
    if (IS_ERR(decomp_strm.stream)) {
    err = PTR_ERR(decomp_strm.stream);
    goto out;
    }
    list_add(&decomp_strm.list, &stream.strm_list);
    stream.avail_decomp = 1;
    return stream;
    out:
    kfree(decomp_strm);
    kfree(stream);
    return ERR_PTR(err);
    }
#[no_mangle]
unsafe extern "C" fn squashfs_decompressor_destroy(msblk: *mut squashfs_sb_info) {
    static void squashfs_decompressor_destroy(struct squashfs_sb_info *msblk)
    {
    struct squashfs_stream *stream = msblk.stream;
    if (stream) {
    struct decomp_stream *decomp_strm;
    while (!list_empty(&stream.strm_list)) {
    decomp_strm = list_entry(stream.strm_list.prev,
    struct decomp_stream, list);
    list_del(&decomp_strm.list);
    msblk.decompressor.free(decomp_strm.stream);
    kfree(decomp_strm);
    stream.avail_decomp--;
    }
    WARN_ON(stream.avail_decomp);
    kfree(stream.comp_opts);
    kfree(stream);
    }
    }
    static struct decomp_stream *get_decomp_stream(struct squashfs_sb_info *msblk,
    struct squashfs_stream *stream)
    {
    struct decomp_stream *decomp_strm;
    while (1) {
    mutex_lock(&stream.mutex);
// There is available decomp_stream
    if (!list_empty(&stream.strm_list)) {
    decomp_strm = list_entry(stream.strm_list.prev,
    struct decomp_stream, list);
    list_del(&decomp_strm.list);
    mutex_unlock(&stream.mutex);
    break;
    }
//
// If there is no available decomp and already full,
// let's wait for releasing decomp from other users.
//
    if (stream.avail_decomp >= msblk.max_thread_num)
    goto wait;
// Let's allocate new decomp
    decomp_strm = kmalloc_obj(*decomp_strm);
    if (!decomp_strm)
    goto wait;
    decomp_strm.stream = msblk.decompressor.init(msblk,
    stream.comp_opts);
    if (IS_ERR(decomp_strm.stream)) {
    kfree(decomp_strm);
    goto wait;
    }
    stream.avail_decomp++;
    WARN_ON(stream.avail_decomp > msblk.max_thread_num);
    mutex_unlock(&stream.mutex);
    break;
    wait:
//
// If system memory is tough, let's for other's
// releasing instead of hurting VM because it could
// make page cache thrashing.
//
    mutex_unlock(&stream.mutex);
    wait_event(stream.wait,
    !list_empty(&stream.strm_list));
    }
    return decomp_strm;
    }
    static int squashfs_decompress(struct squashfs_sb_info *msblk, struct bio *bio,
    int offset, int length,
    struct squashfs_page_actor *output)
    {
    int res;
    struct squashfs_stream *stream = msblk.stream;
    struct decomp_stream *decomp_stream = get_decomp_stream(msblk, stream);
    res = msblk.decompressor.decompress(msblk, decomp_stream.stream,
    bio, offset, length, output);
    put_decomp_stream(decomp_stream, stream);
    if (res < 0)
    ERROR("%s decompression failed, data probably corrupt\n",
    msblk.decompressor.name);
    return res;
    }
    const struct squashfs_decompressor_thread_ops squashfs_decompressor_multi = {
    .create = squashfs_decompressor_create,
    .destroy = squashfs_decompressor_destroy,
    .decompress = squashfs_decompress,
    .max_decompressors = squashfs_max_decompressors,
    };
