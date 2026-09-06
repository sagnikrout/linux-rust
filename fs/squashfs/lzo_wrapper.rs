//! Automatically rewritten from C to Rust
//! Source: fs/squashfs/lzo_wrapper.c
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
// Copyright (c) 2010 LG Electronics
// Chan Jeong <chan.jeong@lge.com>
//
// lzo_wrapper.c
//

#[repr(C)]
#[derive(Copy, Clone)]
pub struct squashfs_lzo {
    pub input: *mut c_void,
    pub output: *mut c_void,
}

    static void *lzo_init(struct squashfs_sb_info *msblk, void *buff)
    {
    let mut block_size: c_int = max_t(int, msblk.block_size, SQUASHFS_METADATA_SIZE);
    struct squashfs_lzo *stream = kzalloc_obj(*stream);
    if (stream == core::ptr::null_mut())
    goto failed;
    stream.input = vmalloc(block_size);
    if (stream.input == core::ptr::null_mut())
    goto failed;
    stream.output = vmalloc(block_size);
    if (stream.output == core::ptr::null_mut())
    goto failed2;
    return stream;
    failed2:
    vfree(stream.input);
    failed:
    ERROR("Failed to allocate lzo workspace\n");
    kfree(stream);
    return ERR_PTR(-ENOMEM);
    }
#[no_mangle]
unsafe extern "C" fn lzo_free(strm: *mut c_void) {
    static void lzo_free(void *strm)
    {
    struct squashfs_lzo *stream = strm;
    if (stream) {
    vfree(stream.input);
    vfree(stream.output);
    }
    kfree(stream);
    }
    static int lzo_uncompress(struct squashfs_sb_info *msblk, void *strm,
    struct bio *bio, int offset, int length,
    struct squashfs_page_actor *output)
    {
    let mut iter_all: bvec_iter_all = {};
    struct bio_vec *bvec = bvec_init_iter_all(&iter_all);
    struct squashfs_lzo *stream = strm;
    void *buff = stream.input, *data;
    let mut bytes: c_int = length, res;
    let mut out_len: usize = output.length;
    while (bio_next_segment(bio, &iter_all)) {
    let mut avail: c_int = min(bytes, ((int)bvec.bv_len) - offset);
    data = bvec_virt(bvec);
    memcpy(buff, data + offset, avail);
    buff += avail;
    bytes -= avail;
    offset = 0;
    }
    res = lzo1x_decompress_safe(stream.input, (size_t)length,
    stream.output, &out_len);
    if (res != LZO_E_OK)
    goto failed;
    res = bytes = (int)out_len;
    data = squashfs_first_page(output);
    buff = stream.output;
    while (data) {
    if (bytes <= PAGE_SIZE) {
    if (!IS_ERR(data))
    memcpy(data, buff, bytes);
    break;
    } else {
    if (!IS_ERR(data))
    memcpy(data, buff, PAGE_SIZE);
    buff += PAGE_SIZE;
    bytes -= PAGE_SIZE;
    data = squashfs_next_page(output);
    }
    }
    squashfs_finish_page(output);
    return res;
    failed:
    return -EIO;
    }
    const struct squashfs_decompressor squashfs_lzo_comp_ops = {
    .init = lzo_init,
    .free = lzo_free,
    .decompress = lzo_uncompress,
    .id = LZO_COMPRESSION,
    .name = "lzo",
    .alloc_buffer = 0,
    .supported = 1
    };
