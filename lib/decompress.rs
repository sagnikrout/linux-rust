//! Automatically rewritten from C to Rust
//! Source: lib/decompress.c
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
// decompress.c
//
// Detect the decompression method based on magic number
//

#[repr(C)]
#[derive(Copy, Clone)]
pub struct compress_format {
    pub magic: [c_uchar; 2],
    pub name: *const c_char,
    pub decompressor: decompress_fn,
}

    static const struct compress_format compressed_formats[] __initconst = {
    { .magic = {0x1f, 0x8b}, .name = "gzip", .decompressor = gunzip },
    { .magic = {0x1f, 0x9e}, .name = "gzip", .decompressor = gunzip },
    { .magic = {0x42, 0x5a}, .name = "bzip2", .decompressor = bunzip2 },
    { .magic = {0x5d, 0x00}, .name = "lzma", .decompressor = unlzma },
    { .magic = {0xfd, 0x37}, .name = "xz", .decompressor = unxz },
    { .magic = {0x89, 0x4c}, .name = "lzo", .decompressor = unlzo },
    { .magic = {0x02, 0x21}, .name = "lz4", .decompressor = unlz4 },
    { .magic = {0x28, 0xb5}, .name = "zstd", .decompressor = unzstd },
    { /* sentinel */ }
    };
    decompress_fn __init decompress_method(const unsigned char *inbuf, long len,
    const char **name)
    {
    const struct compress_format *cf;
    if (len < 2) {
    if (name)
// name = NULL;
    return core::ptr::null_mut();	/* Need at least this much... */
    }
    pr_debug("Compressed data magic: %#.2x %#.2x\n", inbuf[0], inbuf[1]);
    for (cf = compressed_formats; cf.name; cf++)
    if (!memcmp(inbuf, cf.magic, 2))
    break;
    if (name)
// name = cf->name;
    return cf.decompressor;
    }
