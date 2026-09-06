//! Automatically rewritten from C to Rust
//! Source: fs/cramfs/uncompress.c
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
// uncompress.c
//
// (C) Copyright 1999 Linus Torvalds
//
// cramfs interfaces to the uncompression library. There's really just
// three entrypoints:
//
// - cramfs_uncompress_init() - called to initialize the thing.
// - cramfs_uncompress_exit() - tell me when you're done
// - cramfs_uncompress_block() - uncompress a block.
//
// NOTE NOTE NOTE! The uncompression is entirely single-threaded. We
// only have one stream, and we'll initialize it only once even if it
// then is used by multiple filesystems.
//

    static z_stream stream;
    static int initialized;
// Returns length of decompressed data.
#[no_mangle]
pub unsafe extern "C" fn cramfs_uncompress_block(dst: *mut c_void, dstlen: c_int, src: *mut c_void, srclen: c_int) -> c_int {
    int cramfs_uncompress_block(void *dst, int dstlen, void *src, int srclen)
    {
    int err;
    stream.next_in = src;
    stream.avail_in = srclen;
    stream.next_out = dst;
    stream.avail_out = dstlen;
    err = zlib_inflateReset(&stream);
    if (err != Z_OK) {
    pr_err("zlib_inflateReset error %d\n", err);
    zlib_inflateEnd(&stream);
    zlib_inflateInit(&stream);
    }
    err = zlib_inflate(&stream, Z_FINISH);
    if (err != Z_STREAM_END)
    goto err;
    return stream.total_out;
    err:
    pr_err("Error %d while decompressing!\n", err);
    pr_err("%p(%d).%p(%d)\n", src, srclen, dst, dstlen);
    return -EIO;
    }
#[no_mangle]
pub unsafe extern "C" fn cramfs_uncompress_init() -> c_int {
    int cramfs_uncompress_init(void)
    {
    if (!initialized++) {
    stream.workspace = vmalloc(zlib_inflate_workspacesize());
    if (!stream.workspace) {
    initialized = 0;
    return -ENOMEM;
    }
    stream.next_in = core::ptr::null_mut();
    stream.avail_in = 0;
    zlib_inflateInit(&stream);
    }
    return 0;
    }
#[no_mangle]
pub unsafe extern "C" fn cramfs_uncompress_exit() {
    void cramfs_uncompress_exit(void)
    {
    if (!--initialized) {
    zlib_inflateEnd(&stream);
    vfree(stream.workspace);
    }
    }
