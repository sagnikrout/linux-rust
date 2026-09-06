//! Automatically rewritten from C Header to Rust Module
//! Source: tools/testing/selftests/bpf/xdp_metadata.h
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

pub const ETH_P_IP: c_uint = 0x0800;

pub const ETH_P_IPV6: c_uint = 0x86DD;

pub const ETH_P_8021Q: c_uint = 0x8100;

pub const ETH_P_8021AD: c_uint = 0x88A8;

// Non-existent checksum status

#[repr(C)]
#[derive(Copy, Clone, PartialEq, Eq)]
pub enum xdp_meta_field {
    XDP_META_FIELD_TS	= BIT(0),
    XDP_META_FIELD_RSS	= BIT(1),
    XDP_META_FIELD_VLAN_TAG	= BIT(2),
}

#[repr(C)]
#[derive(Copy, Clone)]
pub struct xdp_meta {
    pub rx_timestamp: __u64,
    pub rx_timestamp_err: __s32,
}
