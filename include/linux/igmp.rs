//! Automatically rewritten from C Header to Rust Module
//! Source: include/linux/igmp.h
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
// Linux NET3:	Internet Group Management Protocol  [IGMP]
//
// Authors:
// Alan Cox <alan@lxorguk.ukuu.org.uk>
//
// Extended to talk the BSD extended IGMP protocol of mrouted 3.6
//

#[repr(C)]
#[derive(Copy, Clone)]
pub struct ip_sf_socklist {
    pub sl_max: c_uint,
    pub sl_count: c_uint,
    pub rcu: rcu_head,
    pub __counted_by(sl_max): __be32 sl_addr[],
}

// ip_mc_socklist is real list now. Speed is not argument;
//
#[repr(C)]
#[derive(Copy, Clone)]
pub struct ip_mc_socklist {
    pub next_rcu: *mut ip_mc_socklist __rcu,
    pub multi: ip_mreqn,
    pub /: *mut *mut unsigned int sfmode; / MCAST_{INCLUDE,EXCLUDE},
    pub sflist: *mut ip_sf_socklist __rcu,
    pub rcu: rcu_head,
}

#[repr(C)]
#[derive(Copy, Clone)]
pub struct ip_sf_list {
    pub sf_next: *mut ip_sf_list __rcu,
    pub /: *mut *mut unsigned long sf_count[2]; / include/exclude counts,
    pub sf_inaddr: __be32,
    pub /: *mut *mut unsigned char sf_gsresp; / include in g & s response?,
    pub /: *mut *mut unsigned char sf_oldin; / change state,
    pub /: *mut *mut unsigned char sf_crcount; / retrans. left to send,
    pub rcu: rcu_head,
}

#[repr(C)]
#[derive(Copy, Clone)]
pub struct ip_mc_list {
    pub interface: *mut in_device,
    pub multiaddr: __be32,
    pub sfmode: c_uint,
    pub sources: *mut ip_sf_list __rcu,
    pub tomb: *mut ip_sf_list __rcu,
    pub sfcount: [c_ulong; 2],
    pub next: *mut ip_mc_list,
    pub next_rcu: *mut ip_mc_list __rcu,
}

// RFC3376, relevant sections:
// - 4.1.1. Maximum Response Code
// - 4.1.7. QQIC (Querier's Query Interval Code)
//
// For both MRC and QQIC, values >= 128 use the same floating-point
// encoding as follows:
//
// 0 1 2 3 4 5 6 7
// +-+-+-+-+-+-+-+-+
// |1| exp | mant  |
// +-+-+-+-+-+-+-+-+
//

// IGMPv3 floating-point exponential field min threshold
pub const IGMPV3_EXP_MIN_THRESHOLD: c_int = 128;
// IGMPv3 FP max threshold (mant = 0xF, exp = 7) -> 31744
pub const IGMPV3_EXP_MAX_THRESHOLD: c_int = 31744;
// V3 exponential field encoding
// IGMPv3 MRC/QQIC 8-bit exponential field encode
//
// RFC3376, 4.1.1 & 4.1.7. defines only the decoding formula:
// MRT/QQI = (mant | 0x10) << (exp + 3)
//
// but does NOT define the encoding procedure. To derive exponent:
//
// For any value of mantissa and exponent, the decoding formula
// indicates that the "hidden bit" (0x10) is shifted 4 bits left
// to sit above the 4-bit mantissa. The RFC again shifts this
// entire block left by (exp + 3) to reconstruct the value.
// So, 'hidden bit' is the MSB which is shifted by (4 + exp + 3).
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
// MRC/QQIC < 128 is literal
// Saturate at max representable (mant = 0xF, exp = 7) -> 31744
// Calculate Maximum Response Code from Max Resp Time
//
// RFC3376, relevant sections:
// - 4.1.1. Maximum Response Code
// - 8.3. Query Response Interval
//
// MRC represents the encoded form of Max Resp Time (MRT); once
// decoded, the resulting value is in units of 0.1 seconds (100 ms).
//
extern "C" {
    pub fn igmpv3_exp_field_encode(_arg: mrt) -> return;
}
// Calculate Querier's Query Interval Code from Querier's Query Interval
//
// RFC3376, relevant sections:
// - 4.1.7. QQIC (Querier's Query Interval Code)
// - 8.2. Query Interval
// - 8.12. Older Version Querier Present Timeout
// (the [Query Interval] in the last Query received)
//
// QQIC represents the encoded form of Querier's Query Interval (QQI);
// once decoded, the resulting value is in units of seconds.
//
extern "C" {
    pub fn igmpv3_exp_field_encode(_arg: qi) -> return;
}
// V3 exponential field decoding
// IGMPv3 MRC/QQIC 8-bit exponential field decode
//
// RFC3376, 4.1.1 & 4.1.7. defines the decoding formula:
// 0 1 2 3 4 5 6 7
// +-+-+-+-+-+-+-+-+
// |1| exp | mant  |
// +-+-+-+-+-+-+-+-+
// Max Resp Time = (mant | 0x10) << (exp + 3)
// QQI = (mant | 0x10) << (exp + 3)
//
// Calculate Max Resp Time from Maximum Response Code
//
// RFC3376, relevant sections:
// - 4.1.1. Maximum Response Code
// - 8.3. Query Response Interval
//
// After decode, MRC represents the Maximum Response Time (MRT) in
// units of 0.1 seconds (100 ms).
//
extern "C" {
    pub fn igmpv3_exp_field_decode(_arg: ih3->code) -> return;
}
// Calculate Querier's Query Interval from Querier's Query Interval Code
//
// RFC3376, relevant sections:
// - 4.1.7. QQIC (Querier's Query Interval Code)
// - 8.2. Query Interval
// - 8.12. Older Version Querier Present Timeout
// (the [Query Interval] in the last Query received)
//
// After decode, QQIC represents the Querier's Query Interval in units
// of seconds.
//
extern "C" {
    pub fn igmpv3_exp_field_decode(_arg: ih3->qqic) -> return;
}
extern "C" {
    pub fn pskb_may_pull(_arg: skb, _arg: len) -> return;
}
extern "C" {
    pub fn ip_check_mc_rcu(dev: *mut in_device, mc_addr: __be32, src_addr: __be32, proto: u8) -> c_int;
}
extern "C" {
    pub fn igmp_rcv(: *mut sk_buff) -> c_int;
}
extern "C" {
    pub fn ip_mc_join_group(sk: *mut sock, imr: *mut ip_mreqn) -> c_int;
}
extern "C" {
    pub fn ip_mc_leave_group(sk: *mut sock, imr: *mut ip_mreqn) -> c_int;
}
extern "C" {
    pub fn ip_mc_drop_socket(sk: *mut sock);
}
extern "C" {
    pub fn ip_mc_msfilter(sk: *mut sock, msf: *mut ip_msfilter, ifindex: c_int) -> c_int;
}
extern "C" {
    pub fn ip_mc_init_dev(: *mut in_device);
}
extern "C" {
    pub fn ip_mc_destroy_dev(: *mut in_device);
}
extern "C" {
    pub fn ip_mc_up(: *mut in_device);
}
extern "C" {
    pub fn ip_mc_down(: *mut in_device);
}
extern "C" {
    pub fn ip_mc_unmap(: *mut in_device);
}
extern "C" {
    pub fn ip_mc_remap(: *mut in_device);
}
extern "C" {
    pub fn __ip_mc_dec_group(in_dev: *mut in_device, addr: __be32, gfp: gfp_t);
}
extern "C" {
    pub fn __ip_mc_dec_group(_arg: in_dev, _arg: addr, _arg: GFP_KERNEL) -> return;
}
extern "C" {
    pub fn ip_mc_inc_group(in_dev: *mut in_device, addr: __be32);
}
extern "C" {
    pub fn ip_mc_check_igmp(skb: *mut sk_buff) -> c_int;
}
