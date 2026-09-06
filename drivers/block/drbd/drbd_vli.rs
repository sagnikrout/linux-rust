//! Automatically rewritten from C Header to Rust Module
//! Source: drivers/block/drbd/drbd_vli.h
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
// At a granularity of 4KiB storage represented per bit,
// and stroage sizes of several TiB,
// and possibly small-bandwidth replication,
// the bitmap transfer time can take much too long,
// if transmitted in plain text.
//
// We try to reduce the transferred bitmap information
// by encoding runlengths of bit polarity.
//
// We never actually need to encode a "zero" (runlengths are positive).
// But then we have to store the value of the first bit.
// The first bit of information thus shall encode if the first runlength
// gives the number of set or unset bits.
//
// We assume that large areas are either completely set or unset,
// which gives good compression with any runlength method,
// even when encoding the runlength as fixed size 32bit/64bit integers.
//
// Still, there may be areas where the polarity flips every few bits,
// and encoding the runlength sequence of those areas with fix size
// integers would be much worse than plaintext.
//
// We want to encode small runlength values with minimum code length,
// while still being able to encode a Huge run of all zeros.
//
// Thus we need a Variable Length Integer encoding, VLI.
//
// For some cases, we produce more code bits than plaintext input.
// We need to send incompressible chunks as plaintext, skip over them
// and then see if the next chunk compresses better.
//
// We don't care too much about "excellent" compression ratio for large
// runlengths (all set/all clear): whether we achieve a factor of 100
// or 1000 is not that much of an issue.
// We do not want to waste too much on short runlengths in the "noisy"
// parts of the bitmap, though.
//
// There are endless variants of VLI, we experimented with:
// * simple byte-based
// * various bit based with different code word length.
//
// To avoid yet an other configuration parameter (choice of bitmap compression
// algorithm) which was difficult to explain and tune, we just chose the one
// variant that turned out best in all test cases.
// Based on real world usage patterns, with device sizes ranging from a few GiB
// to several TiB, file server/mailserver/webserver/mysql/postgress,
// mostly idle to really busy, the all time winner (though sometimes only
// marginally better) is:
//
// encoding is "visualised" as
// __little endian__ bitstream, least significant bit first (left most)
//
// this particular encoding is chosen so that the prefix code
// starts as unary encoding the level, then modified so that
// 10 levels can be described in 8bit, with minimal overhead
// for the smaller levels.
//
// Number of data bits follow fibonacci sequence, with the exception of the
// last level (+1 data bit, so it makes 64bit total).  The only worse code when
// encoding bit polarity runlength is 1 plain bits => 2 code bits.
// maximum encodable value: 0x100000400202130 == 2**56 + some
// compression "table":

//
// LEVEL: (total bits, prefix bits, prefix value),
// sorted ascending by number of total bits.
// The rest of the code table is calculated at compiletime from this.
// fibonacci data 1, 1, ...

// finds a suitable level to decode the least significant part of in.
// returns number of bits consumed.
//
// BUG() for bad input, as that would mean a buggy code table.

// out = ((in & ((~0ULL) >> (64-t))) >> b) + adj;	\
// NOT REACHED, if VLI_LEVELS code table is defined properly

// return number of code bits needed,
// or negative error number

// out = ((in - adj) << b) | v;	\

// code from here down is independend of actually used bit code
//
// Code length is determined by some unique (e.g. unary) prefix.
// This encodes arbitrary bit length, not whole bytes: we have a bit-stream,
// not a byte stream.
//
// for the bitstream, we need a cursor
#[repr(C)]
#[derive(Copy, Clone)]
pub struct bitstream_cursor {
// the current byte
    pub b: *mut u8,
// the current bit within *b, nomalized: 0..7
    pub bit: c_uint,
}

// initialize cursor to point to first bit of stream
// advance cursor by that many bits; maximum expected input value: 64,
// but depending on VLI implementation, it may be more.
// the bitstream itself knows its length
#[repr(C)]
#[derive(Copy, Clone)]
pub struct bitstream {
    pub cur: bitstream_cursor,
    pub buf: *mut c_uchar,
    pub /: *mut *mut size_t buf_len; / in bytes,
// for input stream:
// number of trailing 0 bits for padding
// total number of valid bits in stream: buf_len * 8 - pad_bits
    pub pad_bits: c_uint,
}

// Put (at most 64) least significant bits of val into bitstream, and advance cursor.
// Ignores "pad_bits".
// Returns zero if bits == 0 (nothing to do).
// Returns number of bits used if successful.
//
// If there is not enough room left in bitstream,
// leaves bitstream unchanged and returns -ENOBUFS.
//
// paranoia: strip off hi bits; they should not be set anyways.
// b++ |= (val & 0xff) << bs->cur.bit;
// b++ |= (val >> tmp) & 0xff;
// Fetch (at most 64) bits from bitstream into *out, and advance cursor.
//
// If more than 64 bits are requested, returns -EINVAL and leave *out unchanged.
//
// If there are less than the requested number of valid bits left in the
// bitstream, still fetches all available bits.
//
// Returns number of actually fetched bits.
//
// out = 0;
// get the high bits
// n may be at most 9, if cur.bit + bits > 64
// which means this copies at most 8 byte
// we still need the low bits
// and mask out bits we don't want
// out = val;
// encodes @in as vli into @bs;
// return values
// > 0: number of bits successfully stored in bitstream
// -ENOBUFS @bs is full
// -EINVAL input zero (invalid)
// -EOVERFLOW input too large for this vli code (invalid)
//
extern "C" {
    pub fn bitstream_put_bits(_arg: bs, _arg: code, _arg: bits) -> return;
}
