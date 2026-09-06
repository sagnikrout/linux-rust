//! Automatically rewritten from C Header to Rust Module
//! Source: include/net/mld.h
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


// SPDX-License-Identifier: GPL-2.0

// MLDv1 Query/Report/Done
#[repr(C)]
#[derive(Copy, Clone)]
pub struct mld_msg {
    pub mld_hdr: icmp6hdr,
    pub mld_mca: in6_addr,
}

// Multicast Listener Discovery version 2 headers
// MLDv2 Report
#[repr(C)]
#[derive(Copy, Clone)]
pub struct mld2_grec {
    pub grec_type: __u8,
    pub grec_auxwords: __u8,
    pub grec_nsrcs: __be16,
    pub grec_mca: in6_addr,
    pub grec_src: [in6_addr; ],
}

#[repr(C)]
#[derive(Copy, Clone)]
pub struct mld2_report {
    pub mld2r_hdr: icmp6hdr,
    pub mld2r_grec: [mld2_grec; ],
}

// MLDv2 Query
#[repr(C)]
#[derive(Copy, Clone)]
pub struct mld2_query {
    pub mld2q_hdr: icmp6hdr,
    pub mld2q_mca: in6_addr,

    pub mld2q_qqic: __u8,
    pub mld2q_nsrcs: __be16,
    pub mld2q_srcs: [in6_addr; ],
}

// RFC3810, 5.1.3. Maximum Response Code:
//
// If Maximum Response Code >= 32768, Maximum Response Code represents a
// floating-point value as follows:
//
// 0 1 2 3 4 5 6 7 8 9 A B C D E F
// +-+-+-+-+-+-+-+-+-+-+-+-+-+-+-+-+
// |1| exp |          mant         |
// +-+-+-+-+-+-+-+-+-+-+-+-+-+-+-+-+
//

// RFC3810, 5.1.9. QQIC (Querier's Query Interval Code):
//
// If QQIC >= 128, QQIC represents a floating-point value as follows:
//
// 0 1 2 3 4 5 6 7
// +-+-+-+-+-+-+-+-+
// |1| exp | mant  |
// +-+-+-+-+-+-+-+-+
//

// MLDv2 QQIC floating-point exponential field min threshold
pub const MLD_QQIC_MIN_THRESHOLD: c_int = 128;
// MLDv2 QQIC FP max threshold (mant = 0xF, exp = 7) -> 31744
pub const MLD_QQIC_MAX_THRESHOLD: c_int = 31744;
// MLDv2 MRC floating-point exponential field min threshold

// MLDv2 MRC FP max threshold (mant = 0xFFF, exp = 7) -> 8387584
pub const MLD_MRC_MAX_THRESHOLD: c_int = 8387584;

pub const MLD_MAX_QUEUE: c_int = 8;
pub const MLD_MAX_SKBS: c_int = 32;
// V2 exponential field encoding
//
// Calculate Maximum Response Code from Maximum Response Delay
//
// MRC represents the 16-bit encoded form of Maximum Response Delay (MRD);
// once decoded, the resulting value is in milliseconds.
//
// RFC3810, 5.1.3. defines only the decoding formula:
// Maximum Response Delay = (mant | 0x1000) << (exp + 3)
//
// but does NOT define the encoding procedure. To derive exponent:
//
// For the 16-bit MRC, the "hidden bit" (0x1000) is left shifted by 12 to
// sit above the 12-bit mantissa. The RFC then shifts this entire block
// left by (exp + 3) to reconstruct the value. So, 'hidden bit' is the
// MSB which is shifted by (12 + exp + 3).
//
// Total left shift of the hidden bit = 12 + (exp + 3) = exp + 15.
// This is the MSB at the 0-based bit position: (exp + 15).
// Since fls() is 1-based, fls(value) - 1 = exp + 15.
//
// Therefore:
// exp  = fls(value) - 16
// mant = (value >> (exp + 3)) & 0x0FFF
//
// Final encoding formula:
// 0x8000 | (exp << 12) | mant
//
// Example (value = 1311744):
// 0               1               2               3
// 0 1 2 3 4 5 6 7 0 1 2 3 4 5 6 7 0 1 2 3 4 5 6 7 0 1 2 3 4 5 6 7
// +-+-+-+-+-+-+-+-+-+-+-+-+-+-+-+-+-+-+-+-+-+-+-+-+-+-+-+-+-+-+-+-+
// |0 0 0 0 0 0 0 0 0 0 0 1 0 1 0 0 0 0 0 0 0 1 0 0 0 0 0 0 0 0 0 0| 1311744
// |                      ^-^--------mant---------^ ^...(exp+3)...^| exp=5
// +-+-+-+-+-+-+-+-+-+-+-+-+-+-+-+-+-+-+-+-+-+-+-+-+-+-+-+-+-+-+-+-+
//
// Encoded:
// 0x8000 | (5 << 12) | 0x404 = 0xD404
//
// MRC < 32768 is literal
// Saturate at max representable (mant = 0xFFF, exp = 7) -> 8387584
//
// Calculate Querier's Query Interval Code from Querier's Query Interval
//
// QQIC represents the 8-bit encoded form of Querier's Query Interval (QQI);
// once decoded, the resulting value is in seconds.
//
// RFC3810, 5.1.9. defines only the decoding formula:
// QQI = (mant | 0x10) << (exp + 3)
//
// but does NOT define the encoding procedure. To derive exponent:
//
// For any value of mantissa and exponent, the decoding formula indicates
// that the "hidden bit" (0x10) is shifted 4 bits left to sit above the
// 4-bit mantissa. The RFC again shifts this entire block left by (exp + 3)
// to reconstruct the value. So, 'hidden bit' is the MSB which is shifted
// by (4 + exp + 3).
//
// Total left shift of the 'hidden bit' = 4 + (exp + 3) = exp + 7.
// This is the MSB at the 0-based bit position: (exp + 7).
// Since fls() is 1-based, fls(value) - 1 = exp + 7.
//
// Therefore:
// exp  = fls(value) - 8
// mant = (value >> (exp + 3)) & 0x0F
//
// Final encoding formula:
// 0x80 | (exp << 4) | mant
//
// Example (value = 3200):
// 0               1
// 0 1 2 3 4 5 6 7 0 1 2 3 4 5 6 7
// +-+-+-+-+-+-+-+-+-+-+-+-+-+-+-+-+
// |0 0 0 0 1 1 0 0 1 0 0 0 0 0 0 0| (value = 3200)
// |        ^-^-mant^ ^..(exp+3)..^| exp = 4, mant = 9
// +-+-+-+-+-+-+-+-+-+-+-+-+-+-+-+-+
//
// Encoded:
// 0x80 | (4 << 4) | 9 = 0xC9
//
// QQIC < 128 is literal
// Saturate at max representable (mant = 0xF, exp = 7) -> 31744
// V2 exponential field decoding
// Calculate Maximum Response Delay from Maximum Response Code
//
// RFC3810, relevant sections:
// - 5.1.3. Maximum Response Code defines the decoding formula:
// 0 1 2 3 4 5 6 7 8 9 A B C D E F
// +-+-+-+-+-+-+-+-+-+-+-+-+-+-+-+-+
// |1| exp |          mant         |
// +-+-+-+-+-+-+-+-+-+-+-+-+-+-+-+-+
// Maximum Response Delay = (mant | 0x1000) << (exp+3)
// - 9.3. Query Response Interval
//
// After decode, MRC represents the Maximum Response Delay (MRD) in
// units of milliseconds.
//
// Calculate Querier's Query Interval from Querier's Query Interval Code
//
// RFC3810, relevant sections:
// - 5.1.9. QQIC (Querier's Query Interval Code) defines the decoding formula:
// 0 1 2 3 4 5 6 7
// +-+-+-+-+-+-+-+-+
// |1| exp | mant  |
// +-+-+-+-+-+-+-+-+
// QQI = (mant | 0x10) << (exp + 3)
// - 9.2. Query Interval
// - 9.12. Older Version Querier Present Timeout
// (the [Query Interval] in the last Query received)
//
// After decode, QQIC represents the Querier's Query Interval in units
// of seconds.
//
