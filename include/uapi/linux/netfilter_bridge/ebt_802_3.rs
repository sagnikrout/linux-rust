//! Automatically rewritten from C Header to Rust Module
//! Source: include/uapi/linux/netfilter_bridge/ebt_802_3.h
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

pub const EBT_802_3_SAP: c_uint = 0x01;
pub const EBT_802_3_TYPE: c_uint = 0x02;

//
// If frame has DSAP/SSAP value 0xaa you must check the SNAP type
// to discover what kind of packet we're carrying.
//
pub const CHECK_TYPE: c_uint = 0xaa;
//
// Control field may be one or two bytes.  If the first byte has
// the value 0x03 then the entire length is one byte, otherwise it is two.
// One byte controls are used in Unnumbered Information frames.
// Two byte controls are used in Numbered Information frames.
//
pub const IS_UI: c_uint = 0x03;

// ui has one byte ctrl, ni has two
#[repr(C)]
#[derive(Copy, Clone)]
pub struct hdr_ui {
    pub dsap: __u8,
    pub ssap: __u8,
    pub ctrl: __u8,
    pub orig: [__u8; 3],
    pub type: __be16,
}

#[repr(C)]
#[derive(Copy, Clone)]
pub struct hdr_ni {
    pub dsap: __u8,
    pub ssap: __u8,
    pub ctrl: __be16,
    pub orig: [__u8; 3],
    pub type: __be16,
}

#[repr(C)]
#[derive(Copy, Clone)]
pub struct ebt_802_3_hdr {
    pub daddr: [__u8; ETH_ALEN],
    pub saddr: [__u8; ETH_ALEN],
    pub len: __be16,
    pub ui: hdr_ui,
    pub ni: hdr_ni,
    pub llc: },
}

#[repr(C)]
#[derive(Copy, Clone)]
pub struct ebt_802_3_info {
    pub sap: __u8,
    pub type: __be16,
    pub bitmask: __u8,
    pub invflags: __u8,
}
