//! Automatically rewritten from C Header to Rust Module
//! Source: include/net/geneve.h
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
pub const __NET_GENEVE_H: c_int = 1;

pub const GENEVE_UDP_PORT: c_int = 6081;
// Geneve Header:
// +-+-+-+-+-+-+-+-+-+-+-+-+-+-+-+-+-+-+-+-+-+-+-+-+-+-+-+-+-+-+-+-+
// |Ver|  Opt Len  |O|C|    Rsvd.  |          Protocol Type        |
// +-+-+-+-+-+-+-+-+-+-+-+-+-+-+-+-+-+-+-+-+-+-+-+-+-+-+-+-+-+-+-+-+
// |        Virtual Network Identifier (VNI)       |    Reserved   |
// +-+-+-+-+-+-+-+-+-+-+-+-+-+-+-+-+-+-+-+-+-+-+-+-+-+-+-+-+-+-+-+-+
// |                    Variable Length Options                    |
// +-+-+-+-+-+-+-+-+-+-+-+-+-+-+-+-+-+-+-+-+-+-+-+-+-+-+-+-+-+-+-+-+
//
// Option Header:
// +-+-+-+-+-+-+-+-+-+-+-+-+-+-+-+-+-+-+-+-+-+-+-+-+-+-+-+-+-+-+-+-+
// |          Option Class         |      Type     |R|R|R| Length  |
// +-+-+-+-+-+-+-+-+-+-+-+-+-+-+-+-+-+-+-+-+-+-+-+-+-+-+-+-+-+-+-+-+
// |                      Variable Option Data                     |
// +-+-+-+-+-+-+-+-+-+-+-+-+-+-+-+-+-+-+-+-+-+-+-+-+-+-+-+-+-+-+-+-+
//
#[repr(C)]
#[derive(Copy, Clone)]
pub struct geneve_opt {
    pub opt_class: __be16,
    pub type: u8,

    pub length:5: u8,
    pub r3:1: u8,
    pub r2:1: u8,
    pub r1:1: u8,

    pub r1:1: u8,
    pub r2:1: u8,
    pub r3:1: u8,
    pub length:5: u8,
    pub opt_data: [u8; ],
}

#[repr(C)]
#[derive(Copy, Clone)]
pub struct genevehdr {

    pub opt_len:6: u8,
    pub ver:2: u8,
    pub rsvd1:6: u8,
    pub critical:1: u8,
    pub oam:1: u8,

    pub ver:2: u8,
    pub opt_len:6: u8,
    pub oam:1: u8,
    pub critical:1: u8,
    pub rsvd1:6: u8,

    pub proto_type: __be16,
    pub vni: [u8; 3],
    pub rsvd2: u8,
    pub options: [u8; ],
}
