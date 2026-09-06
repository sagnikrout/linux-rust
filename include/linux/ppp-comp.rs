//! Automatically rewritten from C Header to Rust Module
//! Source: include/linux/ppp-comp.h
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
// ppp-comp.h - Definitions for doing PPP packet compression.
//
// Copyright 1994-1998 Paul Mackerras.
//

//
// The following symbols control whether we include code for
// various compression methods.
//

pub const DO_PREDICTOR_1: c_int = 0;
pub const DO_PREDICTOR_2: c_int = 0;
//
// Structure giving methods for compression/decompression.
//
#[repr(C)]
#[derive(Copy, Clone)]
pub struct compressor {
    pub /: *mut *mut int compress_proto; / CCP compression protocol number,
// Allocate space for a compressor (transmit side)
    pub opt_len): *mut *mut *mut *mut void (comp_alloc) (unsigned char options, int,
// Free space used by a compressor
    pub state): *mut *mut void (comp_free) (void,
// Initialize a compressor
    pub debug): int opt_len, int unit, int opthdr, int,
// Reset a compressor
    pub state): *mut *mut void (comp_reset) (void,
// Compress a packet
    pub osize): *mut *mut unsigned char obuf, int isize, int,
// Return compression statistics
    pub stats): *mut *mut *mut void (comp_stat) (void state, struct compstat,
// Allocate space for a decompressor (receive side)
    pub opt_len): *mut *mut *mut *mut void (decomp_alloc) (unsigned char options, int,
// Free space used by a decompressor
    pub state): *mut *mut void (decomp_free) (void,
// Initialize a decompressor
    pub debug): c_int,
// Reset a decompressor
    pub state): *mut *mut void (decomp_reset) (void,
// Decompress a packet.
    pub osize): *mut *mut unsigned char obuf, int,
// Update state for an incompressible packet received
    pub icnt): *mut *mut *mut *mut void (incomp) (void state, unsigned char ibuf, int,
// Return decompression statistics
    pub stats): *mut *mut *mut void (decomp_stat) (void state, struct compstat,
// Used in locking compressor modules
    pub owner: *mut module,
// Extra skb space needed by the compressor algorithm
    pub comp_extra: c_uint,
}

//
// The return value from decompress routine is the length of the
// decompressed packet if successful, otherwise DECOMP_ERROR
// or DECOMP_FATALERROR if an error occurred.
//
// We need to make this distinction so that we can disable certain
// useful functionality, namely sending a CCP reset-request as a result
// of an error detected after decompression.  This is to avoid infringing
// a patent held by Motorola.
// Don't you just lurve software patents.
//

extern "C" {
    pub fn ppp_register_compressor(: *mut compressor) -> c_int;
}
extern "C" {
    pub fn ppp_unregister_compressor(: *mut compressor);
}
