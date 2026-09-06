//! Automatically rewritten from C Header to Rust Module
//! Source: fs/ntfs3/lib/decompress_common.h
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
// decompress_common.h - Code shared by the XPRESS and LZX decompressors
//
// Copyright (C) 2015 Eric Biggers
//

// "Force inline" macro (not required, but helpful for performance)

// Enable whole-word match copying on selected architectures

// Size of a machine word

// Generate a "word" with platform-dependent size whose bytes all contain the
// value 'b'.
//
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
// Pointer to just past the end of the input buffer.
    pub end: *const u8,
}

// Initialize a bitstream to read from the specified input buffer.
// Ensure the bit buffer variable for the bitstream contains at least @num_bits
// bits.  Following this, bitstream_peek_bits() and/or bitstream_remove_bits()
// may be called on the bitstream to peek or remove up to @num_bits bits.  Note
// that @num_bits must be <= 16.
//
// Return the next @num_bits bits from the bitstream, without removing them.
// There must be at least @num_bits remaining in the buffer variable, from a
// previous call to bitstream_ensure_bits().
//
// Remove @num_bits from the bitstream.  There must be at least @num_bits
// remaining in the buffer variable, from a previous call to
// bitstream_ensure_bits().
//
// Remove and return @num_bits bits from the bitstream.  There must be at least
// @num_bits remaining in the buffer variable, from a previous call to
// bitstream_ensure_bits().
//
// Read and return the next @num_bits bits from the bitstream.
extern "C" {
    pub fn bitstream_pop_bits(_arg: is, _arg: num_bits) -> return;
}
// Read and return the next literal byte embedded in the bitstream.
// Read and return the next 16-bit integer embedded in the bitstream.
// Read and return the next 32-bit integer embedded in the bitstream.
// Read into @dst_buffer an array of literal bytes embedded in the bitstream.
// Return either a pointer to the byte past the last written, or NULL if the
// read overflows the input buffer.
//
// Align the input bitstream on a coding-unit boundary.
// Reads and returns the next Huffman-encoded symbol from a bitstream.  If the
// input data is exhausted, the Huffman symbol is decoded as if the missing bits
// are all zeroes.
//
// Index the decode table by the next table_bits bits of the input.
// Fast case: The decode table directly provided the
// symbol and codeword length.  The low 11 bits are the
// symbol, and the high 5 bits are the codeword length.
//
// Slow case: The codeword for the symbol is longer than
// table_bits, so the symbol does not have an entry
// directly in the first (1 << table_bits) entries of the
// decode table.  Traverse the appropriate binary tree
// bit-by-bit to decode the symbol.
//
// Copy an LZ77 match at (dst - offset) to dst.
//
// The length and offset must be already validated --- that is, (dst - offset)
// can't underrun the output buffer, and (dst + length) can't overrun the output
// buffer.  Also, the length cannot be 0.
//
// @bufend points to the byte past the end of the output buffer.  This function
// won't write any data beyond this position.
//
// Returns dst + length.
//
// Try to copy one machine word at a time.  On i386 and x86_64 this is
// faster than copying one byte at a time, unless the data is
// near-random and all the matches have very short lengths.  Note that
// since this requires unaligned memory accesses, it won't necessarily
// be faster on every architecture.
//
// Also note that we might copy more than the length of the match.  For
// example, if a word is 8 bytes and the match is of length 5, then
// we'll simply copy 8 bytes.  This is okay as long as we don't write
// beyond the end of the output buffer, hence the check for (bufend -
// end >= WORDBYTES - 1).
//

// The source and destination words don't overlap.
// To improve branch prediction, one iteration of this
// loop is unrolled.  Most matches are short and will
// fail the first check.  But if that check passes, then
// it becomes increasing likely that the match is long
// and we'll need to continue copying.
//
// Offset 1 matches are equivalent to run-length
// encoding of the previous byte.  This case is common
// if the data contains many repeated bytes.
//
// We don't bother with special cases for other 'offset <
// WORDBYTES', which are usually rarer than 'offset == 1'.  Extra
// checks will just slow things down.  Actually, it's possible
// to handle all the 'offset < WORDBYTES' cases using the same
// code, but it still becomes more complicated doesn't seem any
// faster overall; it definitely slows down the more common
// 'offset == 1' case.
//

// Fall back to a bytewise copy.
// dst++ = *src++;
