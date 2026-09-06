//! Automatically rewritten from C Header to Rust Module
//! Source: include/uapi/linux/netfilter_bridge/ebt_among.h
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


// SPDX-License-Identifier: GPL-2.0 WITH Linux-syscall-note

pub const EBT_AMONG_DST: c_uint = 0x01;
pub const EBT_AMONG_SRC: c_uint = 0x02;
// Grzegorz Borowiak <grzes@gnu.univ.gda.pl> 2003
//
// Write-once-read-many hash table, used for checking if a given
// MAC address belongs to a set or not and possibly for checking
// if it is related with a given IPv4 address.
//
// The hash value of an address is its last byte.
//
// In real-world ethernet addresses, values of the last byte are
// evenly distributed and there is no need to consider other bytes.
// It would only slow the routines down.
//
// For MAC address comparison speedup reasons, we introduce a trick.
// MAC address is mapped onto an array of two 32-bit integers.
// This pair of integers is compared with MAC addresses in the
// hash table, which are stored also in form of pairs of integers
// (in `cmp' array). This is quick as it requires only two elementary
// number comparisons in worst case. Further, we take advantage of
// fact that entropy of 3 last bytes of address is larger than entropy
// of 3 first bytes. So first we compare 4 last bytes of addresses and
// if they are the same we compare 2 first.
//
// Yes, it is a memory overhead, but in 2003 AD, who cares?
//
#[repr(C)]
#[derive(Copy, Clone)]
pub struct ebt_mac_wormhash_tuple {
    pub cmp: [__u32; 2],
    pub ip: __be32,
}

#[repr(C)]
#[derive(Copy, Clone)]
pub struct ebt_mac_wormhash {
    pub table: [c_int; 257],
    pub poolsize: c_int,
    pub pool: [ebt_mac_wormhash_tuple; ],
}

#[repr(C)]
#[derive(Copy, Clone)]
pub struct ebt_among_info {
    pub wh_dst_ofs: c_int,
    pub wh_src_ofs: c_int,
    pub bitmask: c_int,
}

pub const EBT_AMONG_DST_NEG: c_uint = 0x1;
pub const EBT_AMONG_SRC_NEG: c_uint = 0x2;

