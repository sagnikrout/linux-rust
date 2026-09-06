//! Automatically rewritten from C Header to Rust Module
//! Source: include/net/bluetooth/sco.h
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
//
// SCO defaults
pub const SCO_DEFAULT_MTU: c_int = 500;
// SCO socket address
#[repr(C)]
#[derive(Copy, Clone)]
pub struct sockaddr_sco {
    pub sco_family: sa_family_t,
    pub sco_bdaddr: bdaddr_t,
}

// SCO socket options
pub const SCO_OPTIONS: c_uint = 0x01;
#[repr(C)]
#[derive(Copy, Clone)]
pub struct sco_options {
    pub mtu: __u16,
}

pub const SCO_CONNINFO: c_uint = 0x02;
#[repr(C)]
#[derive(Copy, Clone)]
pub struct sco_conninfo {
    pub hci_handle: __u16,
    pub dev_class: [__u8; 3],
}
