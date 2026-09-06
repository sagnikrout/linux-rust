//! Automatically rewritten from C Header to Rust Module
//! Source: include/net/checksum.h
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
// INET		An implementation of the TCP/IP protocol suite for the LINUX
// operating system.  INET is implemented using the  BSD Socket
// interface as the means of communication with the user level.
//
// Checksumming functions for IP, TCP, UDP and so on
//
// Authors:	Jorge Cwik, <jorge@laser.satlink.net>
// Arnt Gulbrandsen, <agulbra@nvg.unit.no>
// Borrows very liberally from tcp.c and ip.c, see those
// files for more names.
//

extern "C" {
    pub fn csum_partial(_arg: dst, _arg: len, _arg: ~0U) -> return;
}

extern "C" {
    pub fn csum_partial(_arg: dst, _arg: len, _arg: 0) -> return;
}

extern "C" {
    pub fn csum_add(_arg: csum, _arg: ~addend) -> return;
}
extern "C" {
    pub fn csum16_add(_arg: csum, _arg: ~addend) -> return;
}

// rotate sum to align it with a 16b boundary

extern "C" {
    pub fn csum_add(_arg: csum, _arg: csum_shift(csum2, _arg: offset)) -> return;
}
extern "C" {
    pub fn csum_block_add(_arg: csum, _arg: ~csum2, _arg: offset) -> return;
}

// sum = csum_fold(csum_add(diff, ~csum_unfold(*sum)));
// sum = csum_fold(csum_add(tmp, ( __wsum)to));
// Implements RFC 1624 (Incremental Internet Checksum)
// 3. Discussion states :
// HC' = ~(~HC + ~m + m')
// m : old value of a 16bit field
// m' : new value of a 16bit field
//
// sum = ~csum16_add(csum16_sub(~(*sum), old), new);
// csum = csum_add(csum_sub(*csum, old), new);
// Subtract out checksum up to start
// Set derived checksum in packet
// psum = csum_fold(csum);
// psum = csum_fold(csum_sub(delta, ( __wsum)*psum));
