//! Automatically rewritten from C to Rust
//! Source: lib/decompress_unlzo.c
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
// LZO decompressor for the Linux kernel. Code borrowed from the lzo
// implementation by Markus Franz Xaver Johannes Oberhumer.
//
// Linux kernel adaptation:
// Copyright (C) 2009
// Albin Tonnerre, Free Electrons <albin.tonnerre@free-electrons.com>
//
// Original code:
// Copyright (C) 1996-2005 Markus Franz Xaver Johannes Oberhumer
// All Rights Reserved.
//
// Markus F.X.J. Oberhumer
// <markus@oberhumer.com>
// http://www.oberhumer.com/opensource/lzop
//

// Macro flag: #define PREBOOT

    static const unsigned char lzop_magic[] = {
    0x89, 0x4c, 0x5a, 0x4f, 0x00, 0x0d, 0x0a, 0x1a, 0x0a };

pub const HEADER_HAS_FILTER: c_uint = 0x00000800L;

#[no_mangle]
pub unsafe extern "C" fn parse_header(input: *mut u8, skip: *mut c_long, in_len: c_long) -> STATIC inline long INIT {
    STATIC inline long INIT parse_header(u8 *input, long *skip, long in_len)
    {
    int l;
    u8 *parse = input;
    u8 *end = input + in_len;
    u16 version;
//
// Check that there's enough input to possibly have a valid header.
// Then it is possible to parse several fields until the minimum
// size may have been used.
//
    if (in_len < HEADER_SIZE_MIN)
    return 0;
// read magic: 9 first bits
    for (l = 0; l < 9; l++) {
    if (*parse++ != lzop_magic[l])
    return 0;
    }
// get version (2bytes), skip library version (2),
// 'need to be extracted' version (2) and
// method (1)
    version = get_unaligned_be16(parse);
    parse += 7;
    if (version >= 0x0940)
    parse++;
    if (get_unaligned_be32(parse) & HEADER_HAS_FILTER)
    parse += 8; /* flags + filter info */
    else
    parse += 4; /* flags */
//
// At least mode, mtime_low, filename length, and checksum must
// be left to be parsed. If also mtime_high is present, it's OK
// because the next input buffer check is after reading the
// filename length.
//
    if (end - parse < 8 + 1 + 4)
    return 0;
// skip mode and mtime_low
    parse += 8;
    if (version >= 0x0940)
    parse += 4;	/* skip mtime_high */
    l = *parse++;
// don't care about the file name, and skip checksum
    if (end - parse < l + 4)
    return 0;
    parse += l + 4;
// skip = parse - input;
    return 1;
    }
    STATIC int INIT unlzo(u8 *input, long in_len,
    long (*fill)(void *, unsigned long),
    long (*flush)(void *, unsigned long),
    u8 *output, long *posp,
    void (*error) (char *x))
    {
    let mut r: u8 = 0;
    let mut skip: c_long = 0;
    u32 src_len, dst_len;
    size_t tmp;
    u8 *in_buf, *in_buf_save, *out_buf;
    let mut ret: c_int = -1;
    if (output) {
    out_buf = output;
    } else if (!flush) {
    error("core::ptr::null_mut() output pointer and no flush function provided");
    goto exit;
    } else {
    out_buf = malloc(LZO_BLOCK_SIZE);
    if (!out_buf) {
    error("Could not allocate output buffer");
    goto exit;
    }
    }
    if (input && fill) {
    error("Both input pointer and fill function provided, don't know what to do");
    goto exit_1;
    } else if (input) {
    in_buf = input;
    } else if (!fill) {
    error("core::ptr::null_mut() input pointer and missing fill function");
    goto exit_1;
    } else {
    in_buf = malloc(lzo1x_worst_compress(LZO_BLOCK_SIZE));
    if (!in_buf) {
    error("Could not allocate input buffer");
    goto exit_1;
    }
    }
    in_buf_save = in_buf;
    if (posp)
// posp = 0;
    if (fill) {
//
// Start from in_buf + HEADER_SIZE_MAX to make it possible
// to use memcpy() to copy the unused data to the beginning
// of the buffer. This way memmove() isn't needed which
// is missing from pre-boot environments of most archs.
//
    in_buf += HEADER_SIZE_MAX;
    in_len = fill(in_buf, HEADER_SIZE_MAX);
    }
    if (!parse_header(in_buf, &skip, in_len)) {
    error("invalid header");
    goto exit_2;
    }
    in_buf += skip;
    in_len -= skip;
    if (fill) {
// Move the unused data to the beginning of the buffer.
    memcpy(in_buf_save, in_buf, in_len);
    in_buf = in_buf_save;
    }
    if (posp)
// posp = skip;
    for (;;) {
// read uncompressed block size
    if (fill && in_len < 4) {
    skip = fill(in_buf + in_len, 4 - in_len);
    if (skip > 0)
    in_len += skip;
    }
    if (in_len < 4) {
    error("file corrupted");
    goto exit_2;
    }
    dst_len = get_unaligned_be32(in_buf);
    in_buf += 4;
    in_len -= 4;
// exit if last block
    if (dst_len == 0) {
    if (posp)
// posp += 4;
    break;
    }
    if (dst_len > LZO_BLOCK_SIZE) {
    error("dest len longer than block size");
    goto exit_2;
    }
// read compressed block size, and skip block checksum info
    if (fill && in_len < 8) {
    skip = fill(in_buf + in_len, 8 - in_len);
    if (skip > 0)
    in_len += skip;
    }
    if (in_len < 8) {
    error("file corrupted");
    goto exit_2;
    }
    src_len = get_unaligned_be32(in_buf);
    in_buf += 8;
    in_len -= 8;
    if (src_len <= 0 || src_len > dst_len) {
    error("file corrupted");
    goto exit_2;
    }
// decompress
    if (fill && in_len < src_len) {
    skip = fill(in_buf + in_len, src_len - in_len);
    if (skip > 0)
    in_len += skip;
    }
    if (in_len < src_len) {
    error("file corrupted");
    goto exit_2;
    }
    tmp = dst_len;
// When the input data is not compressed at all,
// lzo1x_decompress_safe will fail, so call memcpy()
// instead
    if (unlikely(dst_len == src_len))
    memcpy(out_buf, in_buf, src_len);
    else {
    r = lzo1x_decompress_safe((u8 *) in_buf, src_len,
    out_buf, &tmp);
    if (r != LZO_E_OK || dst_len != tmp) {
    error("Compressed data violation");
    goto exit_2;
    }
    }
    if (flush && flush(out_buf, dst_len) != dst_len)
    goto exit_2;
    if (output)
    out_buf += dst_len;
    if (posp)
// posp += src_len + 12;
    in_buf += src_len;
    in_len -= src_len;
    if (fill) {
//
// If there happens to still be unused data left in
// in_buf, move it to the beginning of the buffer.
// Use a loop to avoid memmove() dependency.
//
    if (in_len > 0)
    for (skip = 0; skip < in_len; ++skip)
    in_buf_save[skip] = in_buf[skip];
    in_buf = in_buf_save;
    }
    }
    ret = 0;
    exit_2:
    if (!input)
    free(in_buf_save);
    exit_1:
    if (!output)
    free(out_buf);
    exit:
    return ret;
    }

    STATIC int INIT __decompress(unsigned char *buf, long len,
    long (*fill)(void*, unsigned long),
    long (*flush)(void*, unsigned long),
    unsigned char *out_buf, long olen,
    long *pos,
    void (*error)(char *x))
    {
    return unlzo(buf, len, fill, flush, out_buf, pos, error);
    }
