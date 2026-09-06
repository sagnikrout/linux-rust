//! Automatically rewritten from C Header to Rust Module
//! Source: include/linux/pim.h
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

// Message types - V1

pub const PIM_V1_REGISTER: c_int = 1;
// Message types - V2
pub const PIM_VERSION: c_int = 2;
// RFC7761, sec 4.9:
// Type
// Types for specific PIM messages.  PIM Types are:
//
// Message Type                          Destination
// ---------------------------------------------------------------------
// 0 = Hello                             Multicast to ALL-PIM-ROUTERS
// 1 = Register                          Unicast to RP
// 2 = Register-Stop                     Unicast to source of Register
// packet
// 3 = Join/Prune                        Multicast to ALL-PIM-ROUTERS
// 4 = Bootstrap                         Multicast to ALL-PIM-ROUTERS
// 5 = Assert                            Multicast to ALL-PIM-ROUTERS
// 6 = Graft (used in PIM-DM only)       Unicast to RPF'(S)
// 7 = Graft-Ack (used in PIM-DM only)   Unicast to source of Graft
// packet
// 8 = Candidate-RP-Advertisement        Unicast to Domain's BSR
//

// RFC7761, sec 4.9:
// The PIM header common to all PIM messages is:
// 0                   1                   2                   3
// 0 1 2 3 4 5 6 7 8 9 0 1 2 3 4 5 6 7 8 9 0 1 2 3 4 5 6 7 8 9 0 1
// +-+-+-+-+-+-+-+-+-+-+-+-+-+-+-+-+-+-+-+-+-+-+-+-+-+-+-+-+-+-+-+-+
// |PIM Ver| Type  |   Reserved    |           Checksum            |
// +-+-+-+-+-+-+-+-+-+-+-+-+-+-+-+-+-+-+-+-+-+-+-+-+-+-+-+-+-+-+-+-+
//
#[repr(C)]
#[derive(Copy, Clone)]
pub struct pimhdr {
    pub type: __u8,
    pub reserved: __u8,
    pub csum: __be16,
}

// PIMv2 register message header layout (ietf-draft-idmr-pimvsm-v2-00.ps
#[repr(C)]
#[derive(Copy, Clone)]
pub struct pimreghdr {
    pub type: __u8,
    pub reserved: __u8,
    pub csum: __be16,
    pub flags: __be32,
}

extern "C" {
    pub fn pim_rcv_v1(skb: *mut sk_buff) -> c_int;
}
extern "C" {
    pub fn IS_BUILTIN(IS_BUILTIN(CONFIG_IP_PIMSM_V2: CONFIG_IP_PIMSM_V1) ||) -> return;
}
// check if the address is 224.0.0.13, RFC7761 sec 4.3.1
