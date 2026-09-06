//! Automatically rewritten from C to Rust
//! Source: fs/ntfs3/lib/xpress_decompress.c
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
// (Huffman variant), which can be used in "System Compressed" files.  This is
// based on the code from wimlib.
//
// Copyright (C) 2015 Eric Biggers
//

pub const XPRESS_NUM_SYMBOLS: c_int = 512;
pub const XPRESS_MAX_CODEWORD_LEN: c_int = 15;
pub const XPRESS_MIN_MATCH_LEN: c_int = 3;
// This value is chosen for fast decompression.
pub const XPRESS_TABLEBITS: c_int = 12;
// Reusable heap-allocated memory for XPRESS decompression
#[repr(C)]
#[derive(Copy, Clone)]
pub struct xpress_decompressor {
// The Huffman decoding table
    pub XPRESS_NUM_SYMBOLS]: *mut *mut u16 decode_table[(1 << XPRESS_TABLEBITS) + 2,
// An array that maps symbols to codeword lengths
    pub lens: [u8; XPRESS_NUM_SYMBOLS],
// Temporary space for make_huffman_decode_table()
    u16 working_space[2 * (1 + XPRESS_MAX_CODEWORD_LEN) +
}

//
// xpress_allocate_decompressor - Allocate an XPRESS decompressor
//
// Return the pointer to the decompressor on success, or return NULL and set
// errno on failure.
//
    struct xpress_decompressor *xpress_allocate_decompressor(void)
    {
    return kmalloc_obj(struct xpress_decompressor, GFP_NOFS);
    }
//
// xpress_decompress - Decompress a buffer of XPRESS-compressed data
//
// @decompressor:       A decompressor that was allocated with
// xpress_allocate_decompressor()
// @compressed_data:	The buffer of data to decompress
// @compressed_size:	Number of bytes of compressed data
// @uncompressed_data:	The buffer in which to store the decompressed data
// @uncompressed_size:	The number of bytes the data decompresses into
//
// Return 0 on success, or return -1 and set errno on failure.
//
    int xpress_decompress(struct xpress_decompressor *decompressor,
    const void *compressed_data, size_t compressed_size,
    void *uncompressed_data, size_t uncompressed_size)
    {
    struct xpress_decompressor *d = decompressor;
    let mut in_begin: *const u8  const = compressed_data;
    let mut out_begin: *mut u8  const = uncompressed_data;
    u8 *out_next = out_begin;
    let mut out_end: *mut u8  const = out_begin + uncompressed_size;
    struct input_bitstream is;
    u32 i;
// Read the Huffman codeword lengths.
    if (compressed_size < XPRESS_NUM_SYMBOLS / 2)
    goto invalid;
    for (i = 0; i < XPRESS_NUM_SYMBOLS / 2; i++) {
    d.lens[i*2 + 0] = in_begin[i] & 0xF;
    d.lens[i*2 + 1] = in_begin[i] >> 4;
    }
// Build a decoding table for the Huffman code.
    if (make_huffman_decode_table(d.decode_table, XPRESS_NUM_SYMBOLS,
    XPRESS_TABLEBITS, d.lens,
    XPRESS_MAX_CODEWORD_LEN,
    d.working_space))
    goto invalid;
// Decode the matches and literals.
    init_input_bitstream(&is, in_begin + XPRESS_NUM_SYMBOLS / 2,
    compressed_size - XPRESS_NUM_SYMBOLS / 2);
    while (out_next != out_end) {
    u32 sym;
    u32 log2_offset;
    u32 length;
    u32 offset;
    sym = read_huffsym(&is, d.decode_table,
    XPRESS_TABLEBITS, XPRESS_MAX_CODEWORD_LEN);
    if (sym < 256) {
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
    if (offset > (size_t)(out_next - out_begin))
    goto invalid;
    if (length > (size_t)(out_end - out_next))
    goto invalid;
    out_next = lz_copy(out_next, length, offset, out_end,
    XPRESS_MIN_MATCH_LEN);
    }
    }
    return 0;
    invalid:
    return -1;
    }
//
// xpress_free_decompressor - Free an XPRESS decompressor
//
// @decompressor:       A decompressor that was allocated with
// xpress_allocate_decompressor(), or NULL.
//
#[no_mangle]
pub unsafe extern "C" fn xpress_free_decompressor(decompressor: *mut xpress_decompressor) {
    void xpress_free_decompressor(struct xpress_decompressor *decompressor)
    {
    kfree(decompressor);
    }
