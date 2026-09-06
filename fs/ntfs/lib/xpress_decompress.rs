//! Automatically rewritten from C to Rust
//! Source: fs/ntfs/lib/xpress_decompress.c
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
// xpress_decompress.c - A decompressor for the XPRESS compression format
// (Huffman variant), which can be used in "System Compressed" (WOF) files.
//
// This is a port of the upstream wimlib "xpress_decompress.c" which uses a
// subtable-based Huffman decode table format.  The decode table and the
// codeword-length array share a union since the lengths are fully consumed
// before the table is written.
//
// Copyright (C) 2012-2016 Eric Biggers
//

pub const XPRESS_NUM_CHARS: c_int = 256;
pub const XPRESS_NUM_SYMBOLS: c_int = 512;
pub const XPRESS_MAX_CODEWORD_LEN: c_int = 15;
pub const XPRESS_MIN_MATCH_LEN: c_int = 3;
// This value is chosen for fast decompression.
pub const XPRESS_TABLEBITS: c_int = 11;
// Reusable heap-allocated memory for XPRESS decompression.  The decode table
// and the codeword-length array alias each other in a union: all lengths are
// consumed into the working space before any decode-table entry is written.
//
#[repr(C)]
#[derive(Copy, Clone)]
pub struct xpress_decompressor {
    union {
    DECODE_TABLE(decode_table, XPRESS_NUM_SYMBOLS, XPRESS_TABLEBITS,
    pub lens: [u8; XPRESS_NUM_SYMBOLS],
}

    DECODE_TABLE_WORKING_SPACE(working_space, XPRESS_NUM_SYMBOLS,
    XPRESS_MAX_CODEWORD_LEN);
    } __aligned(DECODE_TABLE_ALIGNMENT);
    int xpress_decompress(struct xpress_decompressor *d,
    const void *compressed_data, size_t compressed_size,
    void *uncompressed_data, size_t uncompressed_size)
    {
    let mut in_begin: *const u8 const = compressed_data;
    let mut out_begin: *mut u8 const = uncompressed_data;
    u8 *out_next = out_begin;
    let mut out_end: *mut u8 const = out_begin + uncompressed_size;
    struct input_bitstream is;
    u32 i;
// Read the Huffman codeword lengths (512 4-bit values packed into 256
// bytes).
//
    if (compressed_size < XPRESS_NUM_SYMBOLS / 2)
    return -1;
    for (i = 0; i < XPRESS_NUM_SYMBOLS / 2; i++) {
    d.lens[2 * i + 0] = in_begin[i] & 0xf;
    d.lens[2 * i + 1] = in_begin[i] >> 4;
    }
// Build a decoding table for the Huffman code.
    if (make_huffman_decode_table(d.decode_table, XPRESS_NUM_SYMBOLS,
    XPRESS_TABLEBITS, d.lens,
    XPRESS_MAX_CODEWORD_LEN,
    d.working_space,
    ARRAY_SIZE(d.decode_table)))
    return -1;
// Decode the matches and literals.
    init_input_bitstream(&is, in_begin + XPRESS_NUM_SYMBOLS / 2,
    compressed_size - XPRESS_NUM_SYMBOLS / 2);
    while (out_next != out_end) {
    u32 sym;
    u32 log2_offset;
    u32 length;
    u32 offset;
    sym = read_huffsym(&is, d.decode_table, XPRESS_TABLEBITS,
    XPRESS_MAX_CODEWORD_LEN);
    if (sym < XPRESS_NUM_CHARS) {
// Literal
// out_next++ = sym;
    } else {
// Match
    length = sym & 0xf;
    log2_offset = (sym >> 4) & 0xf;
    bitstream_ensure_bits(&is, 16);
    offset = ((u32)1 << log2_offset) |
    bitstream_pop_bits(&is, log2_offset);
    if (length == 0xf) {
    length += bitstream_read_byte(&is);
    if (length == 0xf + 0xff)
    length = bitstream_read_u16(&is);
    }
    length += XPRESS_MIN_MATCH_LEN;
    if (unlikely(lz_copy(length, offset, out_begin, out_next,
    out_end, XPRESS_MIN_MATCH_LEN)))
    return -1;
    out_next += length;
    }
    }
    return 0;
    }
    struct xpress_decompressor *xpress_allocate_decompressor(void)
    {
    return kmalloc_obj(struct xpress_decompressor, GFP_NOFS);
    }
#[no_mangle]
pub unsafe extern "C" fn xpress_free_decompressor(d: *mut xpress_decompressor) {
    void xpress_free_decompressor(struct xpress_decompressor *d)
    {
    kfree(d);
    }
#[no_mangle]
unsafe extern "C" fn xpress_scratch_size(chunk_size: u32) -> usize {
    static size_t xpress_scratch_size(u32 chunk_size)
    {
    return sizeof(struct xpress_decompressor);
    }
    static int xpress_decompress_chunk(void *scratch, const void *src,
    size_t src_len, void *dst, size_t dst_len,
    u32 chunk_size)
    {
    return xpress_decompress(scratch, src, src_len, dst, dst_len);
    }
    const struct ntfs_codec_ops ntfs_xpress4k_codec_ops = {
    .id = NTFS_CODEC_XPRESS4K,
    .name = "xpress4k",
    .scratch_size = xpress_scratch_size,
    .decompress_chunk = xpress_decompress_chunk,
    };
    const struct ntfs_codec_ops ntfs_xpress8k_codec_ops = {
    .id = NTFS_CODEC_XPRESS8K,
    .name = "xpress8k",
    .scratch_size = xpress_scratch_size,
    .decompress_chunk = xpress_decompress_chunk,
    };
    const struct ntfs_codec_ops ntfs_xpress16k_codec_ops = {
    .id = NTFS_CODEC_XPRESS16K,
    .name = "xpress16k",
    .scratch_size = xpress_scratch_size,
    .decompress_chunk = xpress_decompress_chunk,
    };
