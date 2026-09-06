//! Automatically rewritten from C Header to Rust Module
//! Source: include/linux/crc8.h
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


//
// Copyright (c) 2011 Broadcom Corporation
//
// Permission to use, copy, modify, and/or distribute this software for any
// purpose with or without fee is hereby granted, provided that the above
// copyright notice and this permission notice appear in all copies.
//
// THE SOFTWARE IS PROVIDED "AS IS" AND THE AUTHOR DISCLAIMS ALL WARRANTIES
// WITH REGARD TO THIS SOFTWARE INCLUDING ALL IMPLIED WARRANTIES OF
// MERCHANTABILITY AND FITNESS. IN NO EVENT SHALL THE AUTHOR BE LIABLE FOR ANY
// SPECIAL, DIRECT, INDIRECT, OR CONSEQUENTIAL DAMAGES OR ANY DAMAGES
// WHATSOEVER RESULTING FROM LOSS OF USE, DATA OR PROFITS, WHETHER IN AN ACTION
// OF CONTRACT, NEGLIGENCE OR OTHER TORTIOUS ACTION, ARISING OUT OF OR IN
// CONNECTION WITH THE USE OR PERFORMANCE OF THIS SOFTWARE.
//

// see usage of this value in crc8() description
pub const CRC8_INIT_VALUE: c_uint = 0xFF;
//
// Return value of crc8() indicating valid message+crc. This is true
// if a CRC is inverted before transmission. The CRC computed over the
// whole received bitstream is _table[x], where x is the bit pattern
// of the modification (almost always 0xff).
//

// required table size for crc8 algorithm
pub const CRC8_TABLE_SIZE: c_int = 256;
// helper macro assuring right table size is used

//
// crc8_populate_lsb - fill crc table for given polynomial in regular bit order.
//
// @table:	table to be filled.
// @polynomial:	polynomial for which table is to be filled.
//
// This function fills the provided table according the polynomial provided for
// regular bit order (lsb first). Polynomials in CRC algorithms are typically
// represented as shown below.
//
// poly = x^8 + x^7 + x^6 + x^4 + x^2 + 1
//
// For lsb first direction x^7 maps to the lsb. So the polynomial is as below.
//
// - lsb first: poly = 10101011(1) = 0xAB
//
extern "C" {
    pub fn crc8_populate_lsb(table[CRC8_TABLE_SIZE]: u8, polynomial: u8);
}
//
// crc8_populate_msb - fill crc table for given polynomial in reverse bit order.
//
// @table:	table to be filled.
// @polynomial:	polynomial for which table is to be filled.
//
// This function fills the provided table according the polynomial provided for
// reverse bit order (msb first). Polynomials in CRC algorithms are typically
// represented as shown below.
//
// poly = x^8 + x^7 + x^6 + x^4 + x^2 + 1
//
// For msb first direction x^7 maps to the msb. So the polynomial is as below.
//
// - msb first: poly = (1)11010101 = 0xD5
//
extern "C" {
    pub fn crc8_populate_msb(table[CRC8_TABLE_SIZE]: u8, polynomial: u8);
}
//
// crc8() - calculate a crc8 over the given input data.
//
// @table:	crc table used for calculation.
// @pdata:	pointer to data buffer.
// @nbytes:	number of bytes in data buffer.
// @crc:	previous returned crc8 value.
//
// The CRC8 is calculated using the polynomial given in crc8_populate_msb()
// or crc8_populate_lsb().
//
// The caller provides the initial value (either %CRC8_INIT_VALUE
// or the previous returned value) to allow for processing of
// discontiguous blocks of data.  When generating the CRC the
// caller is responsible for complementing the final return value
// and inserting it into the byte stream.  When validating a byte
// stream (including CRC8), a final return value of %CRC8_GOOD_VALUE
// indicates the byte stream data can be considered valid.
//
// Reference:
// "A Painless Guide to CRC Error Detection Algorithms", ver 3, Aug 1993
// Williams, Ross N., ross<at>ross.net
// (see URL http://www.ross.net/crc/download/crc_v3.txt).
//
extern "C" {
    pub fn crc8(table[CRC8_TABLE_SIZE]: u8, pdata: *const u8, nbytes: usize, crc: u8) -> u8;
}
