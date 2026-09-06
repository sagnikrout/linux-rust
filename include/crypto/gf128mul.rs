//! Automatically rewritten from C Header to Rust Module
//! Source: include/crypto/gf128mul.h
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


// gf128mul.h - GF(2^128) multiplication functions
//
// Copyright (c) 2003, Dr Brian Gladman, Worcester, UK.
// Copyright (c) 2006 Rik Snel <rsnel@cube.dyndns.org>
//
// Based on Dr Brian Gladman's (GPL'd) work published at
// http://fp.gladman.plus.com/cryptography_technology/index.htm
// See the original copyright notice below.
//
// This program is free software; you can redistribute it and/or modify it
// under the terms of the GNU General Public License as published by the Free
// Software Foundation; either version 2 of the License, or (at your option)
// any later version.
//

// Comment by Rik:
//
// For some background on GF(2^128) see for example:
// http://csrc.nist.gov/groups/ST/toolkit/BCM/documents/proposedmodes/gcm/gcm-revised-spec.pdf
//
// The elements of GF(2^128) := GF(2)[X]/(X^128-X^7-X^2-X^1-1) can
// be mapped to computer memory in a variety of ways. Let's examine
// three common cases.
//
// Take a look at the 16 binary octets below in memory order. The msb's
// are left and the lsb's are right. char b[16] is an array and b[0] is
// the first octet.
//
// 10000000 00000000 00000000 00000000 .... 00000000 00000000 00000000
// b[0]     b[1]     b[2]     b[3]          b[13]    b[14]    b[15]
//
// Every bit is a coefficient of some power of X. We can store the bits
// in every byte in little-endian order and the bytes themselves also in
// little endian order. I will call this lle (little-little-endian).
// The above buffer represents the polynomial 1, and X^7+X^2+X^1+1 looks
// like 11100001 00000000 .... 00000000 = { 0xE1, 0x00, }.
// This format was originally implemented in gf128mul and is used
// in GCM (Galois/Counter mode) and in ABL (Arbitrary Block Length).
//
// Another convention says: store the bits in bigendian order and the
// bytes also. This is bbe (big-big-endian). Now the buffer above
// represents X^127. X^7+X^2+X^1+1 looks like 00000000 .... 10000111,
// b[15] = 0x87 and the rest is 0. LRW uses this convention and bbe
// is partly implemented.
//
// Both of the above formats are easy to implement on big-endian
// machines.
//
// XTS and EME (the latter of which is patent encumbered) use the ble
// format (bits are stored in big endian order and the bytes in little
// endian). The above buffer represents X^7 in this case and the
// primitive polynomial is b[0] = 0x87.
//
// The common machine word-size is smaller than 128 bits, so to make
// an efficient implementation we must split into machine word sizes.
// This implementation uses 64-bit words for the moment. Machine
// endianness comes into play. The lle format in relation to machine
// endianness is discussed below by the original author of gf128mul Dr
// Brian Gladman.
//
// Let's look at the bbe and ble format on a little endian machine.
//
// bbe on a little endian machine u32 x[4]:
//
// MS            x[0]           LS  MS            x[1]		  LS
// ms   ls ms   ls ms   ls ms   ls  ms   ls ms   ls ms   ls ms   ls
// 103..96 111.104 119.112 127.120  71...64 79...72 87...80 95...88
//
// MS            x[2]           LS  MS            x[3]		  LS
// ms   ls ms   ls ms   ls ms   ls  ms   ls ms   ls ms   ls ms   ls
// 39...32 47...40 55...48 63...56  07...00 15...08 23...16 31...24
//
// ble on a little endian machine
//
// MS            x[0]           LS  MS            x[1]		  LS
// ms   ls ms   ls ms   ls ms   ls  ms   ls ms   ls ms   ls ms   ls
// 31...24 23...16 15...08 07...00  63...56 55...48 47...40 39...32
//
// MS            x[2]           LS  MS            x[3]		  LS
// ms   ls ms   ls ms   ls ms   ls  ms   ls ms   ls ms   ls ms   ls
// 95...88 87...80 79...72 71...64  127.120 199.112 111.104 103..96
//
// Multiplications in GF(2^128) are mostly bit-shifts, so you see why
// ble (and lbe also) are easier to implement on a little-endian
// machine than on a big-endian machine. The converse holds for bbe
// and lle.
//
// Note: to have good alignment, it seems to me that it is sufficient
// to keep elements of GF(2^128) in type u64[2]. On 32-bit wordsize
// machines this will automatically aligned to wordsize and on a 64-bit
// machine also.
//
// Multiply a GF(2^128) field element by x. Field elements are
//
// A slow generic version of gf_mul, implemented for lle
// It multiplies a and b and puts the result in a
extern "C" {
    pub fn gf128mul_lle(a: *mut be128, b: *const be128);
}
//
// The following functions multiply a field element by x in
// the polynomial field representation.  They use 64-bit word operations
// to gain speed but compensate for machine endianness and hence work
// correctly on both styles of machine.
//
// They are defined here for performance.
//
// a constant-time version of 'x & ((u64)1 << which) ? (u64)-1 : 0'
// equivalent to gf128mul_table_le[(b << 7) & 0xff] << 48
// (see crypto/gf128mul.c):
// equivalent to gf128mul_table_be[a >> 63] (see crypto/gf128mul.c):
// needed by XTS
// equivalent to gf128mul_table_be[b >> 63] (see crypto/gf128mul.c):
extern "C" {
    pub fn gf128mul_x8_ble(r: *mut le128, x: *const le128);
}
// 64k table optimization, implemented for bbe
#[repr(C)]
#[derive(Copy, Clone)]
pub struct gf128mul_64k {
    pub t: [be128; 256],
    pub t: [*mut }; 16],
}

// First initialize with the constant factor with which you
// want to multiply and then call gf128mul_64k_bbe with the other
// factor in the first argument, and the table in the second.
// Afterwards, the result is stored in *a.
//
extern "C" {
    pub fn gf128mul_free_64k(t: *mut gf128mul_64k);
}
extern "C" {
    pub fn gf128mul_64k_bbe(a: *mut be128, t: *const gf128mul_64k);
}
