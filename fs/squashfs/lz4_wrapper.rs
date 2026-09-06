//! Automatically rewritten from C to Rust
//! Source: fs/squashfs/lz4_wrapper.c
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
// Copyright (c) 2013, 2014
// Phillip Lougher <phillip@squashfs.org.uk>
//

pub const LZ4_LEGACY: c_int = 1;
#[repr(C)]
#[derive(Copy, Clone)]
pub struct lz4_comp_opts {
    pub version: __le32,
    pub flags: __le32,
}

#[repr(C)]
#[derive(Copy, Clone)]
pub struct squashfs_lz4 {
    pub input: *mut c_void,
    pub output: *mut c_void,
}

    static void *lz4_comp_opts(struct squashfs_sb_info *msblk,
    void *buff, int len)
    {
    struct lz4_comp_opts *comp_opts = buff;
// LZ4 compressed filesystems always have compression options
    if (comp_opts == core::ptr::null_mut() || len < sizeof(*comp_opts))
    return ERR_PTR(-EIO);
    if (le32_to_cpu(comp_opts.version) != LZ4_LEGACY) {
// LZ4 format currently used by the kernel is the 'legacy'
// format
    ERROR("Unknown LZ4 version\n");
    return ERR_PTR(-EINVAL);
    }
    return core::ptr::null_mut();
    }
    static void *lz4_init(struct squashfs_sb_info *msblk, void *buff)
    {
    let mut block_size: c_int = max_t(int, msblk.block_size, SQUASHFS_METADATA_SIZE);
    struct squashfs_lz4 *stream;
    stream = kzalloc_obj(*stream);
    if (stream == core::ptr::null_mut())
    goto failed;
    stream.input = vmalloc(block_size);
    if (stream.input == core::ptr::null_mut())
    goto failed2;
    stream.output = vmalloc(block_size);
    if (stream.output == core::ptr::null_mut())
    goto failed3;
    return stream;
    failed3:
    vfree(stream.input);
    failed2:
    kfree(stream);
    failed:
    ERROR("Failed to initialise LZ4 decompressor\n");
    return ERR_PTR(-ENOMEM);
    }
#[no_mangle]
unsafe extern "C" fn lz4_free(strm: *mut c_void) {
    static void lz4_free(void *strm)
    {
    struct squashfs_lz4 *stream = strm;
    if (stream) {
    vfree(stream.input);
    vfree(stream.output);
    }
    kfree(stream);
    }
    static int lz4_uncompress(struct squashfs_sb_info *msblk, void *strm,
    struct bio *bio, int offset, int length,
    struct squashfs_page_actor *output)
    {
    let mut iter_all: bvec_iter_all = {};
    struct bio_vec *bvec = bvec_init_iter_all(&iter_all);
    struct squashfs_lz4 *stream = strm;
    void *buff = stream.input, *data;
    let mut bytes: c_int = length, res;
    while (bio_next_segment(bio, &iter_all)) {
    let mut avail: c_int = min(bytes, ((int)bvec.bv_len) - offset);
    data = bvec_virt(bvec);
    memcpy(buff, data + offset, avail);
    buff += avail;
    bytes -= avail;
    offset = 0;
    }
    res = LZ4_decompress_safe(stream.input, stream.output,
    length, output.length);
    if (res < 0)
    return -EIO;
    bytes = res;
    data = squashfs_first_page(output);
    buff = stream.output;
    while (data) {
    if (bytes <= PAGE_SIZE) {
    if (!IS_ERR(data))
    memcpy(data, buff, bytes);
    break;
    }
    if (!IS_ERR(data))
    memcpy(data, buff, PAGE_SIZE);
    buff += PAGE_SIZE;
    bytes -= PAGE_SIZE;
    data = squashfs_next_page(output);
    }
    squashfs_finish_page(output);
    return res;
    }
    const struct squashfs_decompressor squashfs_lz4_comp_ops = {
    .init = lz4_init,
    .comp_opts = lz4_comp_opts,
    .free = lz4_free,
    .decompress = lz4_uncompress,
    .id = LZ4_COMPRESSION,
    .name = "lz4",
    .alloc_buffer = 0,
    .supported = 1
    };
