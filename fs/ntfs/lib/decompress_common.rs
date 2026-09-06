//! Automatically rewritten from C Header to Rust Module
//! Source: fs/ntfs/lib/decompress_common.h
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


// SPDX-License-Identifier: MIT
//
// decompress_common.h - Code shared by the XPRESS and LZX decompressors
//
// This is a port of the upstream wimlib "decompress_common.h" which uses a
// subtable-based Huffman decode table format, as opposed to the older
// binary-tree-based format previously used in this library.
//
// Copyright (C) 2022 Eric Biggers
//

// "Force inline" macro (not required, but helpful for performance).

// Size of a machine word.

// UNALIGNED_ACCESS_IS_FAST should be 1 if unaligned memory accesses can be
// performed efficiently on the target platform.
//

// Deprecated name kept for compatibility with the upstream source.

// likely()/unlikely() are provided by <linux/compiler.h>.
// STATIC_ASSERT() - verify the truth of an expression at compile time.

// STATIC_ASSERT_ZERO() - like STATIC_ASSERT() but evaluates to 0 so it can be
// used in constant expressions.
//

// Unaligned word load/store helpers.
extern "C" {
    pub fn repeat_u16(b: ((u16)b << 8) |) -> return;
}
//
// Input bitstream for XPRESS and LZX
// ----------------------------------------------------------------------------
// Structure that encapsulates a block of in-memory data being interpreted as a
// stream of bits, optionally with interwoven literal bytes.  Bits are assumed
// to be stored in little endian 16-bit coding units, with the bits ordered high
// to low.
//
#[repr(C)]
#[derive(Copy, Clone)]
pub struct input_bitstream {
// Bits that have been read from the input buffer.  The bits are
// left-justified; the next bit is always bit 31.
//
    pub bitbuf: u32,
// Number of bits currently held in @bitbuf.
    pub bitsleft: u32,
// Pointer to the next byte to be retrieved from the input buffer.
    pub next: *const u8,
// Pointer past the end of the input buffer.
    pub end: *const u8,
}

// Initialize a bitstream to read from the specified input buffer.
// Note: for performance reasons, the following methods don't return error
// codes to the caller if the input buffer is overrun.  Instead, they just
// assume that all overrun data is zeroes.
//
// Ensure the bit buffer variable for the bitstream contains at least @num_bits
// bits.  Following this, bitstream_peek_bits() and/or bitstream_remove_bits()
// may be called on the bitstream to peek or remove up to @num_bits bits.  This
// works for at most 16 bits, which is sufficient for LZX (max codeword length
// 16) and XPRESS (max codeword length 15).
//
// Return the next @num_bits bits from the bitstream, without removing them.
// There must be at least @num_bits remaining in the buffer variable.
//
// Remove @num_bits from the bitstream.
// Remove and return @num_bits bits from the bitstream.
// Read and return the next @num_bits bits from the bitstream.
extern "C" {
    pub fn bitstream_pop_bits(_arg: is, _arg: num_bits) -> return;
}
// Read and return the next literal byte embedded in the bitstream.
// Read and return the next 16-bit integer embedded in the bitstream.
// Read and return the next 32-bit integer embedded in the bitstream.
// Read into @dst_buffer an array of literal bytes embedded in the bitstream.
// Return 0 if there were enough bytes remaining in the input, otherwise -1.
//
// Align the input bitstream on a coding-unit boundary.
//
// Huffman decoding
// ----------------------------------------------------------------------------
//
// Required alignment for the Huffman decode tables.  We require this alignment
// so that we can fill the entries with word instructions without having to deal
// with misaligned buffers.
//
pub const DECODE_TABLE_ALIGNMENT: c_int = 16;
//
// Each decode table entry is 16 bits divided into two fields: 'symbol' (high 12
// bits) and 'length' (low 4 bits).  See the comments in decompress_common.c for
// the precise meaning of these fields depending on the entry type.
//
pub const DECODE_TABLE_SYMBOL_SHIFT: c_int = 4;

//
// Read and return the next Huffman-encoded symbol from the given bitstream
// using the given decode table.  If the input data is exhausted, then the
// Huffman symbol will be decoded as if the missing bits were all zeroes.
//
// Preload the bitbuffer with 'max_codeword_len' bits.
// Index the root table by the next 'table_bits' bits of input.
// Extract the "symbol" and "length" from the entry.
// If the codeword is longer than 'table_bits', the root entry is a
// subtable pointer.  Discard the bits used to index the root table and
// index the subtable by the next 'length' bits.
//
// Discard the (remaining) bits of the codeword.
//
// DECODE_TABLE_ENOUGH() evaluates to the maximum number of decode table
// entries, including all subtable entries, that may be required for decoding a
// given Huffman code.  It is a compile-time mapping computed by the zlib
// 'enough' utility.  An unknown combination produces a build error.
//

// Wrapper around DECODE_TABLE_ENOUGH() that does additional compile-time
// validation.
//

// Declare the decode table for a Huffman code.

// Declare the temporary "working_space" array needed for building the decode
// table for a Huffman code.
//

//
// LZ match copying
// ----------------------------------------------------------------------------
//
// Copy an LZ77 match of 'length' bytes from the match source at 'out_next -
// offset' to the match destination at 'out_next'.  The source and destination
// may overlap.  This handles validating the length and offset; it returns 0 if
// the match was valid (and was copied), otherwise -1.
//
// Validate the offset.
// Fast path: copy a short, non-overlapping match whose end is not too
// close to the end of the buffer.
//
// Validate the length.
// Fall back to a bytewise copy.
// out_next++ = *src++;
